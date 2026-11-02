# Generated compute lifecycle wire contracts

These authored programs call the real generated SDK packages and CLI. `wire.py`
starts a bounded loopback HTTP server, runs one program, and validates all thirteen
requests including controller and executor bearer credentials, request bodies,
the controller's claim and executor credential issuance. Each driver checks the typed claim
owner and epoch, submits that authority to issuance, and uses the returned token
for executor requests. The oracle randomizes that token on every run so a
hardcoded executor credential cannot bypass issuance. The suites also verify
explicit identical receipt replay and exactly one unavailable mutation despite
caller retry configuration. Programs check generated responses for readiness,
recovery-required/lost-process state, terminal polling and scoped-auth failure.

Run from the repository root:

```sh
python shared/compute-contracts/wire.py python shared/compute-contracts/python_contract.py
sdk/typescript/node_modules/.bin/tsc --project shared/compute-contracts/tsconfig.json
python shared/compute-contracts/wire.py node shared/compute-contracts/typescript_contract.js
COMPUTE_CONTRACT_BINARY=cli/target/debug/sikaru python shared/compute-contracts/wire.py python shared/compute-contracts/cli_contract.py
```

Build or install each generated client first; the CLI contract needs `cargo build --locked --bin sikaru` in `cli/`.
These checks prove local client serialization and transport behavior, not hosted
compute authorization, durable leases, execution or billing acceptance.
