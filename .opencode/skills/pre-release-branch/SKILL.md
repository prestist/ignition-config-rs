---
name: pre-release-branch
description: Prepare release notes and open a pre-release PR for ignition-config-rs
---

# Pre-Release Branch

## What it does

Prepares `docs/release-notes.md` for a new release and opens a PR:

1. Creates a `pre-release-X.Y.Z` branch
2. Converts the "Upcoming" header to a dated release header
3. Adds a new "Upcoming" header for the next development cycle
4. Commits with the standard message format
5. Pushes the branch and opens a PR

## Prerequisites

- `git` and `gh` CLI installed
- Push access to the repository
- Clean git working tree on `main`

## Usage

```bash
# Prepare release 0.7.0
/pre-release-branch 0.7.0

# Specify the next upcoming version explicitly
/pre-release-branch 0.7.0 --next 0.8.0
```

## Workflow

### Step 1: Parse input and determine versions

The release version argument is required (e.g., `0.7.0`).

```
RELEASE_VER = argument (e.g., "0.7.0")
RELEASE_MAJOR = 0
RELEASE_MINOR = 7
RELEASE_PATCH = 0
```

Determine the next upcoming version. If `--next` was provided, use that. Otherwise:
- If patch is 0: next minor version (e.g., `0.7.0` -> `0.8.0`)
- If patch > 0: next minor version (e.g., `0.6.1` -> `0.7.0`)

```
NEXT_VER = next version (e.g., "0.8.0")
TODAY = today's date in YYYY-MM-DD format
```

### Step 2: Pre-flight checks

Verify:
- Working tree is clean: `git status --porcelain` produces no output
- On `main` branch: `git branch --show-current` returns `main`
- `docs/release-notes.md` exists
- The file contains an "Upcoming" header matching the release version. Read `docs/release-notes.md` and look for a heading like:
  ```
  ## Upcoming ignition-config {RELEASE_VER} (unreleased)
  ```
  If this heading does not exist, warn the user -- they may need to add it first, or the version may be wrong.

If any check fails, stop and report the issue.

### Step 3: Create branch

```bash
git checkout -b pre-release-{RELEASE_VER}
```

### Step 4: Update `docs/release-notes.md`

The file follows this structure. Find the "Upcoming" heading and make two changes:

**4a. Replace the "Upcoming" heading with a dated release heading:**

Find:
```markdown
## Upcoming ignition-config {RELEASE_VER} (unreleased)
```
Replace with:
```markdown
## Upcoming ignition-config {NEXT_VER} (unreleased)


## ignition-config {RELEASE_VER} ({TODAY})
```

This inserts the new "Upcoming" header above, with two blank lines separating it from the now-dated release header.

**4b. Keep all existing bullet points under the release header.** Do not move or modify them -- they stay under the now-dated heading where they were.

### Step 5: Commit

```bash
git add docs/release-notes.md
git commit -m "docs/release-notes: update for release {RELEASE_VER}"
```

### Step 6: Push and open PR

```bash
git push {UPSTREAM_REMOTE} pre-release-{RELEASE_VER}
```

Where `{UPSTREAM_REMOTE}` is typically `origin`. Verify with `git remote -v` if unsure.

Open a PR:

```bash
gh pr create \
  --title "docs/release-notes: update for release {RELEASE_VER}" \
  --body "$(cat <<'EOF'
## Summary

- Update release notes for {RELEASE_VER} release
- Add new "Upcoming {NEXT_VER}" section for next development cycle

Part of the release process for {RELEASE_VER}. See the [release checklist](https://github.com/coreos/ignition-config-rs/issues/new?labels=release&template=release-checklist.md).
EOF
)"
```

### Step 7: Report results

Summarize what was done:
- The commit created
- The PR URL
- Remind the user of the next release checklist steps:
  - Get the PR reviewed and merged
  - Then proceed with `cargo release` on a `release-{RELEASE_VER}` branch

## Checklist Coverage

From the [release checklist](.github/ISSUE_TEMPLATE/release-checklist.md), this skill covers:

- [x] `git checkout -b pre-release-${RELEASE_VER}`
- [x] Write release notes in `docs/release-notes.md`
- [x] `git add docs/release-notes.md && git commit -m "docs/release-notes: update for release ${RELEASE_VER}"`
- [x] PR the changes

## What's NOT covered

- Writing the actual release note bullet points (those should already exist under the "Upcoming" heading)
- Checking `Cargo.toml` for unintended dependency bound changes
- The `cargo release` step (separate branch, requires GPG)
- Publishing to crates.io
- GitHub release creation
- Fedora packaging

## References

- Release checklist: `.github/ISSUE_TEMPLATE/release-checklist.md`
- CI enforcement: `.github/workflows/require-release-note.yml` (PRs must modify `docs/release-notes.md`)
- Example commits: `f1244a5` (0.6.1), `40aea00` (0.6.0), `f7ec268` (0.5.0), `fdfff99` (0.4.1), `0693d30` (0.4.0)
