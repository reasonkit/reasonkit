# reasonkit Meta-Crate Project Rules

## Project Identity

- **Name**: reasonkit
- **Purpose**: Meta-crate that installs the complete ReasonKit suite
- **Components**: reasonkit-core, reasonkit-mem, reasonkit-web
- **License**: Apache-2.0

## CRITICAL: This is a THIN WRAPPER

**The `reasonkit` crate contains ZERO business logic.**

All functionality lives in the component crates:

- Reasoning logic → `reasonkit-core`
- Memory/retrieval logic → `reasonkit-mem`
- Web/browser logic → `reasonkit-web`

This crate ONLY provides:

1. Re-exports of the three component crates
2. A unified CLI that delegates to the component libraries
3. Version coordination

## Maintenance Protocol

### When to Update This Crate

Update `reasonkit` when:

1. A component crate releases a new version
2. Adding new top-level CLI commands
3. Breaking API changes in components (rare with semver)

### Update Process

1. Bump dependency version in `Cargo.toml`
2. Run `cargo test`
3. Run `cargo publish`

That's it. No code changes needed for version bumps.

### Automated Updates (CI/CD)

Component crates trigger updates via repository dispatch:

- `reasonkit-core` release → Triggers `sync-release.yml`
- `reasonkit-mem` release → Triggers `sync-release.yml`
- `reasonkit-web` release → Triggers `sync-release.yml`

## Description Rules

### CORRECT Description:

```
The Reasoning Engine — Complete ReasonKit Suite | Auditable Reasoning for Production AI
```

### NEVER say:

- "RAG engine" when describing the whole suite
- The individual crate descriptions as the suite description

## CLI Structure

```
reasonkit                    # Unified CLI
├── think                   # → reasonkit-core (reasoning)
├── verify                  # → reasonkit-core (triangulation)
├── mem                     # → reasonkit-mem (memory ops)
│   ├── search
│   ├── ingest
│   └── stats
├── rag                     # → reasonkit-mem (RAG queries)
├── web                     # → reasonkit-web (browser)
│   ├── capture
│   └── extract
├── serve                   # → Start MCP server
├── version                 # → Show all component versions
└── completions             # → Shell completions
```

## Quality Gates

Same as component crates:

1. `cargo build --release`
2. `cargo clippy -- -D warnings`
3. `cargo fmt --check`
4. `cargo test --all-features`

## See Also

- `/RK-PROJECT/ORCHESTRATOR.md` - Master orchestration rules
- `/RK-PROJECT/reasonkit-core/CLAUDE.md` - Core component rules
- `/RK-PROJECT/reasonkit-mem/CLAUDE.md` - Memory component rules
- `/RK-PROJECT/reasonkit-web/CLAUDE.md` - Web component rules
