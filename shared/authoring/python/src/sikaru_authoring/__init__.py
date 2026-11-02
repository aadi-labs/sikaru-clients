"""Compile explicitly selected customer source files for the public agent API.

This module performs no network requests and contains no execution runtime.
Pass ``bundle.definition`` and ``bundle.content_digest`` to the generated
managed-agent creation client. Only files named in sikaru.json are read.
"""
from __future__ import annotations

import base64
from dataclasses import dataclass
import hashlib
import json
import os
import stat
from pathlib import Path
import re
from typing import Any

_LIMIT = 1_000_000


@dataclass(frozen=True)
class AgentBundle:
    name: str
    definition: dict[str, Any]
    content_digest: str


def _canonical(value: Any) -> bytes:
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False, allow_nan=False).encode()


def _read(root: Path, relative: str) -> bytes:
    if not relative or "\\" in relative or any(part in ("", ".", "..") for part in relative.split("/")):
        raise ValueError("Source paths must be relative without traversal")
    path = root
    for part in relative.split("/"):
        path = path / part
        if path.is_symlink():
            raise ValueError("Source symlinks are not supported")
    flags = os.O_RDONLY | getattr(os, "O_NONBLOCK", 0) | getattr(os, "O_NOFOLLOW", 0)
    with os.fdopen(os.open(path, flags), "rb") as stream:
        metadata = os.fstat(stream.fileno())
        if not stat.S_ISREG(metadata.st_mode) or metadata.st_size > _LIMIT:
            raise ValueError("Source must be a regular file of at most 1 MB")
        body = stream.read(_LIMIT + 1)
    if len(body) > _LIMIT:
        raise ValueError("Source exceeds 1 MB")
    return body


# Capability sections: the agent's declared reach. The service applies the same
# rules; checking here reports problems before upload.
_SECTIONS = ("web", "tools", "setup")
_TOOLS = ("bash", "workspace", "agents", "memory", "web_search", "web_fetch")
_POLICIES = ("allow", "require_approval", "deny")
_REFERENCE = re.compile(r"[A-Za-z0-9][A-Za-z0-9_.:-]{0,127}")
_LABEL = re.compile(r"[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?")


def _section(value: Any, field: str, keys: tuple[str, ...]) -> dict[str, Any]:
    if not isinstance(value, dict) or not set(value) <= set(keys):
        raise ValueError(f"Invalid capability section {field}")
    return value


def _strings(value: Any, field: str, most: int) -> list[str]:
    if not isinstance(value, list) or not all(isinstance(item, str) for item in value):
        raise ValueError(f"{field} must be a list of strings")
    if len(value) > most:
        raise ValueError(f"{field} allows at most {most} entries")
    return value


def _is_reference(value: Any) -> bool:
    return isinstance(value, str) and _REFERENCE.fullmatch(value) is not None


def _is_domain(value: str) -> bool:
    labels = value.split(".")
    return len(value) <= 253 and len(labels) >= 2 and all(_LABEL.fullmatch(label) for label in labels)


def _check_provider(provider: Any) -> None:
    if provider == "sikaru":
        return
    tool = _section(provider, "web.provider", ("connection_id", "tool"))
    if not _is_reference(tool.get("connection_id")):
        raise ValueError("web.provider.connection_id is not a valid reference")
    if not isinstance(tool.get("tool"), str) or not 1 <= len(tool["tool"]) <= 256:
        raise ValueError("web.provider.tool must be 1-256 characters")


def _check_web(web: Any) -> None:
    web = _section(web, "web", ("provider", "allow_domains", "block_domains"))
    _check_provider(web.get("provider", "sikaru"))
    lists = {field: _strings(web.get(field, []), f"web.{field}", 100) for field in ("allow_domains", "block_domains")}
    for field, domains in lists.items():
        bad = next((domain for domain in domains if not _is_domain(domain)), None)
        if bad is not None:
            raise ValueError(f"Invalid domain {bad!r} in web.{field}; use a bare lowercase hostname")
    if set(lists["allow_domains"]) & set(lists["block_domains"]):
        raise ValueError("Domains cannot be in both allow and block lists")


def _check_tools(tools: Any) -> None:
    for tool, setting in _section(tools, "tools", _TOOLS).items():
        setting = _section(setting, f"tools.{tool}", ("enabled", "policy"))
        if not isinstance(setting.get("enabled", True), bool):
            raise ValueError(f"tools.{tool}.enabled must be true or false")
        if setting.get("policy", "allow") not in _POLICIES:
            raise ValueError(f"tools.{tool}.policy must be one of {', '.join(_POLICIES)}")


def _is_package(name: str) -> bool:
    return (1 <= len(name) <= 200 and (name[0] == "@" or re.fullmatch("[A-Za-z0-9]", name[0]) is not None)
            and not any(char.isspace() for char in name))


def _check_packages(packages: Any) -> None:
    packages = _section(packages, "setup.packages", ("pip", "npm"))
    for manager in ("pip", "npm"):
        names = _strings(packages.get(manager, []), f"setup.packages.{manager}", 100)
        bad = next((name for name in names if not _is_package(name)), None)
        if bad is not None:
            raise ValueError(f"Invalid setup package {bad!r}")


def _check_commands(commands: Any) -> None:
    for command in _strings(commands, "setup.commands", 50):
        if len(command) > 4000 or not command.strip() or "\0" in command:
            raise ValueError("Setup commands must be nonempty text up to 4000 characters")


def _check_repo_url(url: Any) -> None:
    """Accept only credential-free https repository URLs on the default port."""
    if not isinstance(url, str):
        raise ValueError("Setup repo URL must be text")
    scheme, separator, rest = url.partition("://")
    if not separator or scheme.encode().lower() != b"https":
        raise ValueError(f"Setup repo URL {url!r} must use https")
    authority, slash, path = rest.partition("/")
    host, colon, port = authority.partition(":")
    problems = (
        (len(url.encode()) > 2048, "is too long"),
        ("?" in rest or "#" in rest, "must name a repository without query or fragment"),
        ("@" in authority, "must not embed credentials; use git_credential"),
        (bool(colon) and port != "443", "must use the default https port"),
        (not host, "must name a host"),
        (not (slash + path).strip("/"), "must name a repository"),
    )
    reason = next((reason for found, reason in problems if found), None)
    if reason is not None:
        raise ValueError(f"Setup repo URL {url!r} {reason}")


def _check_workspace_path(path: Any) -> None:
    relative = (isinstance(path, str) and len(path.encode()) <= 512 and "\\" not in path
                and not path.startswith("/") and all(part not in ("", ".", "..") for part in path.split("/")))
    if not relative:
        raise ValueError(f"Setup repo path {path!r} must be a relative workspace path")


def _check_repo(repo: Any) -> str:
    repo = _section(repo, "setup.repos", ("url", "path", "git_credential"))
    _check_repo_url(repo.get("url"))
    _check_workspace_path(repo.get("path"))
    credential = repo.get("git_credential")
    if credential is not None and not _is_reference(credential):
        raise ValueError(f"Git credential for {repo['path']} is not a valid reference")
    return repo["path"]


def _check_repos(repos: Any) -> None:
    if not isinstance(repos, list):
        raise ValueError("setup.repos must be a list")
    if len(repos) > 20:
        raise ValueError("setup.repos allows at most 20 entries")
    paths = sorted(_check_repo(repo) + "/" for repo in repos)
    # Sorted, a path that equals or contains another is immediately followed by it.
    for path, following in zip(paths, paths[1:]):
        if following.startswith(path):
            raise ValueError(f"Setup repo path {path[:-1]!r} overlaps another repo path")


def _check_setup(setup: Any) -> None:
    setup = _section(setup, "setup", ("packages", "commands", "repos"))
    _check_packages(setup.get("packages", {}))
    _check_commands(setup.get("commands", []))
    _check_repos(setup.get("repos", []))


_SECTION_CHECKS = {"web": _check_web, "tools": _check_tools, "setup": _check_setup}


def _capability_sections(manifest: dict[str, Any]) -> dict[str, Any]:
    """The manifest's validated ``web``, ``tools`` and ``setup`` sections, as authored.

    A section set to null is treated as absent.
    """
    sections = {key: manifest[key] for key in _SECTIONS if manifest.get(key) is not None}
    for key, value in sections.items():
        _SECTION_CHECKS[key](value)
    return sections


_ROOT_INSTRUCTIONS = ("instructions.md", "AGENTS.md")
_NAMED = re.compile(r"agents/([A-Za-z0-9][A-Za-z0-9_-]*)/(.+)")
_ASSET = re.compile(r"skills/([^/]+)/(scripts|references|assets)/(.+)")


def _instruction_path(path: str, local: str, named: bool) -> bool:
    return local == "instructions.md" if named else path in _ROOT_INSTRUCTIONS


def _skill_path(path: str, local: str, named: bool) -> bool:
    return local.startswith("skills/") and local.endswith(".md")


def _eval_path(path: str, local: str, named: bool) -> bool:
    return not named and path.startswith("evals/") and path.endswith(".md")


def _asset_path(path: str, local: str, named: bool) -> bool:
    return _ASSET.fullmatch(local) is not None


_KINDS = {
    "agent_md": (_instruction_path, "Invalid instruction path"),
    "agent_skill": (_skill_path, "Skills must be Markdown below skills/"),
    "eval_md": (_eval_path, "Evals must be Markdown below evals/"),
    "skill_asset": (_asset_path, "Asset requires a skill package directory"),
}


def _required_package(path: str, local: str) -> str:
    """The SKILL.md an asset at ``path`` belongs to."""
    asset = _ASSET.fullmatch(local)
    return path[: -len(local)] + "skills/" + asset[1] + "/SKILL.md"


def _kind_rule(kind: Any):
    if not isinstance(kind, str) or kind not in _KINDS:
        raise ValueError("Unsupported customer source kind")
    return _KINDS[kind]


def _source(root: Path, entry: Any, seen: set[str], packages: set[str]) -> dict[str, Any]:
    """Validate one manifest entry and read it as a definition source."""
    if not isinstance(entry, dict) or set(entry) != {"path", "kind"}:
        raise ValueError("Source requires only path and kind")
    path, kind = entry["path"], entry["kind"]
    if not isinstance(path, str) or path in seen:
        raise ValueError("Invalid or duplicate source path")
    seen.add(path)
    named = _NAMED.fullmatch(path)
    local = named[2] if named else path
    allowed, message = _kind_rule(kind)
    if not allowed(path, local, bool(named)):
        raise ValueError(message)
    body = _read(root, path)
    if kind == "skill_asset":
        packages.add(_required_package(path, local))
        return {"path": path, "kind": kind, "content": base64.b64encode(body).decode("ascii"), "encoding": "base64"}
    content = body.decode("utf-8")
    if "\0" in content:
        raise ValueError("Text sources cannot contain NUL")
    return {"path": path, "kind": kind, "content": content}


def _is_root_instruction(source: dict[str, Any]) -> bool:
    return source["kind"] == "agent_md" and source["path"] in _ROOT_INSTRUCTIONS and bool(source["content"].strip())


def _check_bundle(sources: list[dict[str, Any]], packages: set[str]) -> None:
    """Checks across sources: root instructions, asset packages, named-agent instructions."""
    by_path = {source["path"]: source["kind"] for source in sources}
    if not any(_is_root_instruction(source) for source in sources):
        raise ValueError("Nonempty root instructions are required")
    if any(by_path.get(package) != "agent_skill" for package in packages):
        raise ValueError("Every asset requires its customer-authored SKILL.md")
    agents = {path.split("/")[1] for path in by_path if path.startswith("agents/")}
    if any(by_path.get(f"agents/{agent}/instructions.md") != "agent_md" for agent in agents):
        raise ValueError("Every named agent requires instructions.md")


def _load_manifest(root: Path) -> dict[str, Any]:
    manifest = json.loads(_read(root, "sikaru.json"))
    if not isinstance(manifest, dict) or not {"name", "sources"} <= set(manifest):
        raise ValueError("Manifest requires name and sources")
    unknown = set(manifest) - {"name", "sources", *_SECTIONS}
    if unknown:
        raise ValueError(f"Unsupported manifest keys: {', '.join(sorted(unknown))}")
    name = manifest["name"]
    if not isinstance(name, str) or not re.fullmatch(r"[a-z0-9][a-z0-9-]{0,63}", name):
        raise ValueError("Invalid agent name")
    entries = manifest["sources"]
    if not isinstance(entries, list) or not 1 <= len(entries) <= 100:
        raise ValueError("Manifest requires 1 to 100 sources")
    return manifest


def compile_directory(directory: str | Path) -> AgentBundle:
    """Validate sikaru.json and return a deterministic, private-file-free bundle.

    Manifest: ``{"name":"support","sources":[{"path":"instructions.md",
    "kind":"agent_md"}]}``. Assets use kind ``skill_asset`` and are encoded
    as base64. Skills, assets and named agents retain their relative paths.
    Optional ``web``, ``tools`` and ``setup`` sections are validated locally
    and included in the definition as authored.
    """
    root = Path(directory)
    if root.is_symlink() or not root.is_dir():
        raise ValueError("Agent directory must be a real directory")
    manifest = _load_manifest(root)
    seen: set[str] = set()
    packages: set[str] = set()
    sources = [_source(root, entry, seen, packages) for entry in manifest["sources"]]
    _check_bundle(sources, packages)
    definition = {"schema": "sikaru.agent.contract.v1", "sources": sorted(sources, key=lambda source: source["path"])}
    definition.update(_capability_sections(manifest))
    raw = _canonical(definition)
    if len(raw) > _LIMIT:
        raise ValueError("Customer definition exceeds 1 MB")
    return AgentBundle(manifest["name"], definition, "sha256:" + hashlib.sha256(raw).hexdigest())
