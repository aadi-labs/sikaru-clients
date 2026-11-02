"""Exact UTF-8 document IO over Sikaru's generated public operations.

Compilation and validation belong to the server. Save conflicts propagate the
server's newer draft instead of overwriting it or retrying the mutation.
"""
from pathlib import Path


def pull_document(client, project_id: str, agent_slug: str, path: str | Path):
    draft = client.agent_documents.get(project_id=project_id, agent_slug=agent_slug)
    Path(path).write_bytes(draft.document.encode('utf-8'))
    return draft


def push_document(client, project_id: str, agent_slug: str, path: str | Path, *, expected_revision: int):
    return client.agent_documents.save(project_id=project_id, agent_slug=agent_slug,
        document=Path(path).read_bytes().decode('utf-8'), expected_revision=expected_revision)


def validate_document(client, project_id: str, agent_slug: str, path: str | Path):
    return client.agent_documents.validate(project_id=project_id, agent_slug=agent_slug,
        document=Path(path).read_bytes().decode('utf-8'))


def publish_document(client, project_id: str, agent_slug: str, *, revision: int,
                     expected_live_version_id: str | None, acknowledge_widening=False, acknowledge_removals=False):
    return client.agent_documents.publish(project_id=project_id, agent_slug=agent_slug,
        revision=revision, expected_live_version_id=expected_live_version_id,
        acknowledge_widening=acknowledge_widening, acknowledge_removals=acknowledge_removals)
