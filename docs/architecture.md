# Architecture

| Layer | Path | Responsibility |
|---|---|---|
| Application | `src/app/` | CLI boundary, command orchestration, dependency wiring |
| Domain | `src/domain/` | Pure rules, command invariants, execution planning, interfaces |
| Ports | `src/domain/ports/` | Interface boundaries required by domain/application |
| Adapters | `src/adapters/` | Process execution, file I/O, catalog loading, runtime asset materialization |
| Internal dep | `crates/mev-internal/` | Internal command domain implementations reused by mev |
| Source assets | `src/assets/` | Source-of-truth Ansible playbooks and roles |
| Release assets | `GitHub Releases` | `mev-darwin-aarch64` binary distribution |

## Package Structure

```text
src/
├── main.rs               # Binary entry point
├── lib.rs                 # Library root
├── app/
│   ├── cli/               # clap argument contracts (1 file per command)
│   │   └── mod.rs         # Single owner of clap parser and command dispatch
│   ├── commands/           # Orchestration units per command domain
│   ├── context.rs          # Dependency wiring (ports → adapters)
│   └── api.rs              # Stable library entrypoints
├── domain/
│   ├── error.rs            # Typed domain errors
│   ├── ports/              # Trait interfaces
│   ├── profile.rs          # Profile identifiers and mapping
│   ├── tag.rs              # Tag resolution from catalogs
│   ├── config.rs           # VCS identity configuration model
│   └── execution_plan.rs   # Deterministic ansible plan construction
├── adapters/
│   ├── ansible/            # Playbook execution, locator, runtime asset materialization
│   ├── identity_store/     # Identity persistence and path resolution
│   ├── macos_defaults/     # macOS defaults adapter
│   ├── version_source/     # Update execution source
│   ├── git/, jj/, vscode/  # External tool adapters
│   └── fs/                 # Filesystem adapter
├── assets/
│   └── ansible/            # Source-of-truth ansible assets embedded into binary
└── testing/                # In-process test doubles

crates/
└── mev-internal/          # Internal command implementations (shell, vcs)

tests/
├── harness/                # Shared fixtures (TestContext)
├── cli.rs + cli/           # CLI behavior contracts
├── library.rs + library/   # Public API contracts
├── adapters.rs + adapters/ # Adapter behavior contracts
├── runtime.rs + runtime/   # Binary invocation contracts
└── security.rs + security/ # Input validation contracts
```

## Python Surface

Python ownership is limited to development tooling (`ansible-lint`) managed by `pyproject.toml`.
Runtime command ownership belongs to the Rust implementation.
