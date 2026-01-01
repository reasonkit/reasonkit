# Component Crate Dispatch Template

This template shows how component crates (reasonkit-core, reasonkit-mem, reasonkit-web)
can notify the reasonkit meta-crate when they release a new version.

## Add to Component Crate's Release Workflow

Add this step to each component crate's `.github/workflows/release.yml`:

```yaml
# At the end of the release job, after successful crates.io publish

- name: Notify meta-crate
  uses: peter-evans/repository-dispatch@v3
  with:
    token: ${{ secrets.REASONKIT_DISPATCH_TOKEN }}
    repository: reasonkit/reasonkit
    event-type: component-released
    client-payload: |
      {
        "component": "reasonkit-core",
        "version": "${{ steps.version.outputs.version }}",
        "auto_publish": "false"
      }
```

## Required Secrets

1. **REASONKIT_DISPATCH_TOKEN**: A Personal Access Token (PAT) with `repo` scope
   that has access to the reasonkit/reasonkit repository.

   Create at: https://github.com/settings/tokens

   Required permissions:
   - `repo` (Full control of private repositories)
   - Or `public_repo` (Access public repositories only)

2. Add the secret to each component repository:
   - Go to Settings → Secrets and variables → Actions
   - Add new repository secret: `REASONKIT_DISPATCH_TOKEN`

## Example: Complete Release Workflow Snippet

```yaml
# .github/workflows/release.yml (in reasonkit-core, reasonkit-mem, or reasonkit-web)

name: Release

on:
  push:
    tags:
      - "v*"

jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Extract version
        id: version
        run: echo "version=${GITHUB_REF#refs/tags/v}" >> $GITHUB_OUTPUT

      - uses: dtolnay/rust-action@stable

      - name: Publish to crates.io
        run: cargo publish
        env:
          CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}

      # ========================================
      # ADD THIS STEP TO NOTIFY META-CRATE
      # ========================================
      - name: Notify reasonkit meta-crate
        uses: peter-evans/repository-dispatch@v3
        with:
          token: ${{ secrets.REASONKIT_DISPATCH_TOKEN }}
          repository: reasonkit/reasonkit
          event-type: component-released
          client-payload: |
            {
              "component": "${{ github.event.repository.name }}",
              "version": "${{ steps.version.outputs.version }}",
              "auto_publish": "false"
            }
```

## Testing the Dispatch

To test without a full release:

```bash
# Using GitHub CLI
gh api repos/reasonkit/reasonkit/dispatches \
  -f event_type=component-released \
  -f 'client_payload[component]=reasonkit-core' \
  -f 'client_payload[version]=0.2.0' \
  -f 'client_payload[auto_publish]=false'
```

## Flow Diagram

```
┌─────────────────────┐
│  reasonkit-core     │
│  releases v0.2.0    │
└──────────┬──────────┘
           │
           │ repository_dispatch
           │
           ▼
┌─────────────────────┐
│  reasonkit          │
│  sync-release.yml   │
└──────────┬──────────┘
           │
           │ 1. Update Cargo.toml
           │ 2. Run tests
           │ 3. Bump patch version
           │ 4. Commit & push
           │
           ▼
┌─────────────────────┐
│  reasonkit v0.1.1   │
│  (if auto_publish)  │
└─────────────────────┘
```

## Manual Trigger

The sync-release workflow can also be triggered manually:

1. Go to Actions → Sync Component Release
2. Click "Run workflow"
3. Select component and enter version
4. Optionally enable auto-publish

This is useful for:

- Testing the workflow
- Catching up on missed dispatches
- Manual version coordination
