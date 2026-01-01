# ReasonKit Core - Release Checklist

> **Version:** v0.3.0 Template
> **Last Updated:** 2026-01-01
> **Reference:** ORCHESTRATOR.md v3.8.0, CONS-009 (Quality Gates Required)

---

## Overview

This checklist ensures consistent, high-quality releases of `reasonkit-core`. All steps are mandatory unless marked `[OPTIONAL]`. The release process enforces the 5 Quality Gates (CONS-009) and follows semantic versioning.

**Release Manager Signature:** ********\_\_\_\_********
**Release Date:** ********\_\_\_\_********
**Version:** ********\_\_\_\_********

---

## Phase 1: Pre-Release Verification

### 1.1 Branch Preparation

- [ ] Create release branch from `main`: `git checkout -b release/v0.3.0`
- [ ] Ensure all feature branches are merged
- [ ] Verify no uncommitted changes: `git status`
- [ ] Pull latest changes: `git pull origin main`

### 1.2 Dependency Audit

- [ ] Update Cargo.lock: `cargo update`
- [ ] Review dependency changes: `git diff Cargo.lock`
- [ ] Run security audit: `cargo audit`
- [ ] Check for deprecated dependencies: `cargo outdated`
- [ ] Verify no yanked crates: `cargo verify-project`

**Security Audit Results:**

```
[ ] No vulnerabilities found
[ ] Vulnerabilities found and documented: ____________________
```

### 1.3 Code Quality Review

- [ ] Review all TODOs: `grep -r "TODO" src --include="*.rs" | wc -l`
  - Current count: \_\_\_\_
  - Acceptable threshold: < 20
- [ ] Review all FIXMEs: `grep -r "FIXME" src --include="*.rs" | wc -l`
  - Current count: \_\_\_\_
  - Acceptable threshold: 0 (critical), < 5 (non-critical)
- [ ] Check unsafe blocks: `grep -r "unsafe" src --include="*.rs" | wc -l`
  - Current count: \_\_\_\_
  - All unsafe blocks documented: [ ] Yes [ ] No

---

## Phase 2: The 5 Quality Gates (CONS-009)

**All gates MUST pass before proceeding. Run the full suite:**

```bash
# From the reasonkit-core directory
./scripts/quality_metrics.sh --ci
```

### Gate 1: Build

```bash
cargo build --release --locked
```

- [ ] Exit code: 0
- [ ] Build time: \_\_\_\_ seconds
- [ ] Warning count: \_\_\_\_ (target: 0)
- [ ] Binary size: \_\_\_\_ MB

**Build Verification:**

```bash
# Verify binary exists and runs
./target/release/rk-core --version
./target/release/rk-core --help
```

- [ ] Version output correct
- [ ] Help output displays properly

### Gate 2: Lint (Clippy)

```bash
cargo clippy --all-targets --all-features --locked -- -D warnings
```

- [ ] Exit code: 0
- [ ] Clippy warnings: 0
- [ ] Clippy errors: 0

**Strict Mode (Recommended):**

```bash
cargo clippy --all-targets --all-features --locked -- \
  -D warnings \
  -W clippy::pedantic \
  -W clippy::nursery \
  -A clippy::module_name_repetitions \
  -A clippy::must_use_candidate \
  -A clippy::missing_errors_doc \
  -A clippy::missing_panics_doc
```

- [ ] Strict mode passes (or exceptions documented)

### Gate 3: Format

```bash
cargo fmt --all -- --check
```

- [ ] Exit code: 0
- [ ] All files formatted

**If format fails:**

```bash
cargo fmt --all
git diff  # Review changes
git commit -m "style: run cargo fmt for release v0.3.0"
```

### Gate 4: Tests

```bash
cargo test --all-features --locked
```

- [ ] Exit code: 0
- [ ] Tests passed: \_\_\_\_
- [ ] Tests failed: 0
- [ ] Tests ignored: \_\_\_\_ (document reasons)

**Additional Test Suites:**

```bash
# Unit tests only
cargo test --lib --all-features --locked

# Integration tests
cargo test --tests --all-features --locked

# Doc tests
cargo test --doc --locked

# No default features
cargo test --no-default-features --locked
```

- [ ] All test suites pass

### Gate 5: Benchmarks

```bash
cargo bench --all-features
```

- [ ] Exit code: 0
- [ ] No performance regression > 5%
- [ ] Benchmark results saved to: `target/criterion/`

**Key Benchmarks to Verify:**

| Benchmark              | Target  | Actual      | Status   |
| ---------------------- | ------- | ----------- | -------- |
| Protocol orchestration | < 10ms  | \_\_\_\_ ms | [ ] Pass |
| ThinkTool execution    | < 100ms | \_\_\_\_ ms | [ ] Pass |
| Concurrent chains (8x) | < 10ms  | \_\_\_\_ ms | [ ] Pass |

---

## Phase 3: Version Bumping

### 3.1 Update Cargo.toml

Edit `Cargo.toml`:

```toml
[package]
name = "reasonkit-core"
version = "0.3.0"  # <-- Update this
```

- [ ] Version updated in Cargo.toml
- [ ] Cargo.lock regenerated: `cargo generate-lockfile`

### 3.2 Verify Version Consistency

```bash
# Extract version from Cargo.toml
cargo metadata --no-deps --format-version 1 | jq -r '.packages[] | select(.name == "reasonkit-core") | .version'
```

- [ ] Version matches: `0.3.0`

### 3.3 Update Version References

Check and update version in:

- [ ] `README.md` (installation examples)
- [ ] `docs/` files (if version-specific)
- [ ] Example code (if version-specific)

---

## Phase 4: Changelog Update

### 4.1 Update CHANGELOG.md

Edit `CHANGELOG.md`:

```markdown
## [Unreleased]

## [0.3.0] - YYYY-MM-DD

### Added

- Feature 1 description
- Feature 2 description

### Changed

- Change 1 description

### Fixed

- Fix 1 description

### Deprecated

- (If applicable)

### Removed

- (If applicable)

### Security

- (If applicable)
```

- [ ] Changelog updated with all changes
- [ ] Date set to release date
- [ ] Links updated at bottom of file

### 4.2 Generate Release Notes

```bash
# Using git-cliff (if installed)
git-cliff --latest --strip header > RELEASE_NOTES.md

# Or manual generation
git log --oneline v0.2.0..HEAD > RELEASE_NOTES.md
```

- [ ] Release notes generated
- [ ] Release notes reviewed for accuracy

---

## Phase 5: Documentation Check

### 5.1 Build Documentation

```bash
cargo doc --no-deps --all-features --locked
```

- [ ] Exit code: 0
- [ ] No documentation warnings
- [ ] Docs build successfully

**Open locally to verify:**

```bash
open target/doc/reasonkit/index.html  # macOS
xdg-open target/doc/reasonkit/index.html  # Linux
```

- [ ] API documentation renders correctly
- [ ] All public items documented
- [ ] Examples compile and display

### 5.2 README Verification

- [ ] Installation instructions are correct
- [ ] Quick start examples work
- [ ] Version badges will update automatically
- [ ] Links are valid

### 5.3 External Documentation

- [ ] Website docs updated (if applicable)
- [ ] API reference synced with code

---

## Phase 6: CI/CD Verification

### 6.1 Local CI Simulation

Run the full CI pipeline locally:

```bash
# Quick verification
cargo build --release && \
cargo clippy -- -D warnings && \
cargo fmt --check && \
cargo test --all-features && \
echo "All gates passed!"
```

- [ ] All commands succeed

### 6.2 GitHub Actions Check

- [ ] CI workflow exists: `.github/workflows/ci.yml`
- [ ] Release workflow exists: `.github/workflows/release.yml`
- [ ] All required secrets configured:
  - [ ] `CARGO_REGISTRY_TOKEN` (crates.io)
  - [ ] `NPM_TOKEN` (if publishing npm wrapper)
  - [ ] `PYPI_API_TOKEN` (if publishing Python bindings)

### 6.3 Test Release Workflow (Dry Run)

```bash
# Trigger dry run via GitHub Actions
gh workflow run release.yml --field dry_run=true
```

- [ ] Dry run completes successfully
- [ ] Artifact generation verified

---

## Phase 7: Pre-Release Commit

### 7.1 Commit Changes

```bash
git add Cargo.toml Cargo.lock CHANGELOG.md
git commit -m "chore(release): prepare v0.3.0

- Bump version to 0.3.0
- Update CHANGELOG.md
- All 5 quality gates passed

Quality Score: X/10
"
```

- [ ] Commit created

### 7.2 Create Release Tag

```bash
# Annotated tag (recommended)
git tag -a v0.3.0 -m "Release v0.3.0

Key changes:
- Change 1
- Change 2
- Change 3

Full changelog: https://github.com/reasonkit/reasonkit-core/blob/main/CHANGELOG.md
"

# Verify tag
git show v0.3.0
```

- [ ] Tag created
- [ ] Tag message is descriptive

### 7.3 Push to Remote

```bash
# Push commits
git push origin release/v0.3.0

# Push tag (triggers release workflow)
git push origin v0.3.0
```

- [ ] Branch pushed
- [ ] Tag pushed
- [ ] Release workflow triggered

---

## Phase 8: crates.io Publishing

### 8.1 Pre-Publish Verification

```bash
# Verify package contents
cargo package --list

# Check package size
ls -lh target/package/*.crate

# Dry run publish
cargo publish --dry-run
```

- [ ] Package contents correct
- [ ] Package size reasonable (< 10 MB)
- [ ] Dry run succeeds

### 8.2 Publish to crates.io

```bash
# Publish (requires CARGO_REGISTRY_TOKEN)
cargo publish
```

- [ ] Published successfully
- [ ] Verify on crates.io: https://crates.io/crates/reasonkit-core

### 8.3 Post-Publish Verification

```bash
# Test installation from crates.io
cargo install reasonkit-core --version 0.3.0

# Verify
rk-core --version
```

- [ ] Installation from crates.io works
- [ ] Version output correct

---

## Phase 9: GitHub Release

### 9.1 Verify Release Created

The release workflow should automatically create a GitHub release when the tag is pushed.

- [ ] Release created: https://github.com/reasonkit/reasonkit-core/releases/tag/v0.3.0
- [ ] Release notes populated
- [ ] Binary assets attached:
  - [ ] `rk-core-linux-x86_64.tar.gz`
  - [ ] `rk-core-linux-x86_64-musl.tar.gz`
  - [ ] `rk-core-linux-aarch64.tar.gz`
  - [ ] `rk-core-macos-x86_64.tar.gz`
  - [ ] `rk-core-macos-aarch64.tar.gz`
  - [ ] `rk-core-windows-x86_64.zip`
  - [ ] `SHA256SUMS.txt`
  - [ ] `install.sh`

### 9.2 Verify Docker Images (if applicable)

```bash
# Pull and test Docker image
docker pull ghcr.io/reasonkit/reasonkit-core:0.3.0
docker run ghcr.io/reasonkit/reasonkit-core:0.3.0 --version
```

- [ ] Docker image available
- [ ] Image works correctly

---

## Phase 10: Announcement

### 10.1 Announcement Template

**Subject:** ReasonKit Core v0.3.0 Released

````markdown
# ReasonKit Core v0.3.0 Released

We're excited to announce the release of ReasonKit Core v0.3.0!

## Highlights

- **Feature 1:** Brief description
- **Feature 2:** Brief description
- **Fix:** Brief description

## Installation

```bash
# Universal installer
curl -fsSL https://reasonkit.sh/install | bash

# Via Cargo
cargo install reasonkit-core

# Via Docker
docker run ghcr.io/reasonkit/reasonkit-core:0.3.0 --help
```
````

## Links

- [Release Notes](https://github.com/reasonkit/reasonkit-core/releases/tag/v0.3.0)
- [Documentation](https://docs.rs/reasonkit-core/0.3.0)
- [Changelog](https://github.com/reasonkit/reasonkit-core/blob/main/CHANGELOG.md)

## Upgrade Notes

(Any breaking changes or migration notes)

---

**Full documentation:** https://reasonkit.sh/docs
**Report issues:** https://github.com/reasonkit/reasonkit-core/issues

````

### 10.2 Announcement Channels

- [ ] GitHub Release (automatic)
- [ ] Twitter/X: @reasonkit (if applicable)
- [ ] Discord: #announcements (if applicable)
- [ ] Reddit: r/rust (if applicable)
- [ ] Hacker News (for major releases)
- [ ] Newsletter (if applicable)

---

## Phase 11: Post-Release Tasks

### 11.1 Merge Release Branch

```bash
# Merge to main
git checkout main
git merge release/v0.3.0
git push origin main

# Delete release branch
git branch -d release/v0.3.0
git push origin --delete release/v0.3.0
````

- [ ] Release branch merged to main
- [ ] Release branch deleted

### 11.2 Update Unreleased Section

Reset the `[Unreleased]` section in CHANGELOG.md:

```markdown
## [Unreleased]

### Added

### Changed

### Fixed
```

- [ ] CHANGELOG.md updated for next version

### 11.3 Monitor for Issues

- [ ] Monitor GitHub issues for release-related bugs
- [ ] Monitor crates.io download counts
- [ ] Check for security advisories

---

## Quality Score Summary

| Gate               | Status            | Notes |
| ------------------ | ----------------- | ----- |
| Gate 1: Build      | [ ] Pass [ ] Fail |       |
| Gate 2: Clippy     | [ ] Pass [ ] Fail |       |
| Gate 3: Format     | [ ] Pass [ ] Fail |       |
| Gate 4: Tests      | [ ] Pass [ ] Fail |       |
| Gate 5: Benchmarks | [ ] Pass [ ] Fail |       |

**Overall Quality Score:** \_\_\_\_/10 (Target: 8.0+, Minimum: 7.0)

---

## Sign-Off

| Role            | Name | Date | Signature |
| --------------- | ---- | ---- | --------- |
| Release Manager |      |      |           |
| Code Reviewer   |      |      |           |
| QA Verification |      |      |           |

---

## Appendix A: Quick Release Commands

```bash
#!/bin/bash
# quick_release.sh - Automated release verification

VERSION="0.3.0"

echo "=== ReasonKit Core Release Verification ==="
echo "Version: $VERSION"
echo ""

# Quality Gates
echo "[1/5] Building..."
cargo build --release --locked || exit 1

echo "[2/5] Linting..."
cargo clippy --all-targets --all-features --locked -- -D warnings || exit 1

echo "[3/5] Formatting..."
cargo fmt --check || exit 1

echo "[4/5] Testing..."
cargo test --all-features --locked || exit 1

echo "[5/5] Documentation..."
cargo doc --no-deps --all-features --locked || exit 1

echo ""
echo "=== All Quality Gates Passed ==="
echo "Ready to release v$VERSION"
```

---

## Appendix B: Rollback Procedure

If a critical issue is discovered post-release:

### Immediate Actions

1. **Yank from crates.io** (if critical security issue):

   ```bash
   cargo yank --version 0.3.0
   ```

2. **Update GitHub Release:**
   - Mark as pre-release or delete
   - Add warning to release notes

3. **Notify users:**
   - GitHub issue with pinned label
   - Announcement on channels

### Recovery

1. Create hotfix branch: `git checkout -b hotfix/v0.3.1`
2. Apply fix
3. Follow abbreviated release process
4. Release v0.3.1

---

## Appendix C: Version Naming

| Type              | Format          | Example         | When to Use                       |
| ----------------- | --------------- | --------------- | --------------------------------- |
| Major             | `X.0.0`         | `1.0.0`         | Breaking changes                  |
| Minor             | `0.X.0`         | `0.3.0`         | New features, backward compatible |
| Patch             | `0.0.X`         | `0.3.1`         | Bug fixes only                    |
| Pre-release       | `0.X.0-alpha.N` | `0.3.0-alpha.1` | Testing before release            |
| Release Candidate | `0.X.0-rc.N`    | `0.3.0-rc.1`    | Final testing                     |

---

**Document Version:** 1.0.0
**Template For:** ReasonKit Core v0.3.0+
**Maintained By:** ReasonKit Team
**Contact:** team@reasonkit.sh
