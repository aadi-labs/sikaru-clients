// Local public command-surface checks; no request is sent to a service.
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { resolve } from "node:path";

const binary = resolve(process.argv[2] ?? "target/debug/sikaru");
const env = { ...process.env };
for (const key of Object.keys(env)) {
  if (key.startsWith("SIKARU_")) delete env[key];
}
env.SIKARU_API_KEY = "local-validation-only";

function spawn(args) {
  const result = spawnSync(binary, args, { env, encoding: "utf8", timeout: 15_000 });
  if (result.error) throw result.error;
  return result;
}
function run(args) {
  const result = spawn(args);
  assert.equal(result.status, 0, `${args.join(" ")}: ${result.stderr || result.stdout}`);
  return result.stdout;
}
function commands(help) {
  const section = help.split("Commands:")[1]?.split(/\n\s*\n/)[0] ?? "";
  return new Set(section.split("\n").map((line) => line.trim().split(/\s+/)[0]).filter(Boolean));
}

for (const flag of ["--spec", "--spec-raw"]) {
  const result = spawn([flag]);
  assert.notEqual(result.status, 0);
  const error = JSON.parse(result.stdout);
  assert.deepEqual(Object.keys(error), ["error"]);
  assert.match(error.error.message, /no binding has an embedded API spec/);
}

const groups = {
  inference: ["chat", "models"],
  agent: ["list", "get", "create", "run", "logs", "stop", "pull", "push", "validate", "review", "publish"],
  eval: ["run", "list", "get", "samples", "logs", "stop"],
  traces: ["upload", "get"],
  environment: ["list", "info", "push", "pull"],
  config: ["show", "set"],
};
const topLevel = commands(run(["--help"]));
for (const command of [...Object.keys(groups), "login", "logout", "doctor"]) {
  assert.ok(topLevel.has(command), `Missing command: ${command}`);
}
for (const stale of ["agents", "execution-sessions", "runs", "tool-providers", "datasets", "specialists",
  "organizations", "exec", "compute", "agent-budgets"]) {
  assert.ok(!topLevel.has(stale), `Unexpected command: ${stale}`);
}
for (const [group, verbs] of Object.entries(groups)) {
  const available = commands(run([group, "--help"]));
  for (const verb of verbs) {
    assert.ok(available.has(verb), `Missing command: ${group} ${verb}`);
    run([group, verb, "--help"]);
  }
}

const agentRun = run(["agent", "run", "--help"]);
assert.match(agentRun, /--compute <compute>/);
assert.match(agentRun, /--model <model>/);
assert.match(run(["login", "--help"]), /--with-token/);
assert.match(run(["login", "--help"]), /--no-browser/);
assert.match(run(["doctor", "--help"]), /--workspace/);
run(["logout", "--help"]);

console.log("Public command surface and help checks passed.");
