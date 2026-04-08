---
name: add-spec-version
description: Add support for a new Ignition specification version to ignition-config-rs
---

# Add Ignition Spec Version

## What it does

Automates adding a new Ignition specification version to the crate:

1. Copies the previous spec directory (`src/v3_X/`) to create the new one
2. Updates the VERSION constant in `mod.rs`
3. Wires the new module into `src/lib.rs` (module, enum variant, parse branch, test)
4. Updates `docs/release-notes.md`
5. Optionally applies a new `ignition.json` schema and regenerates `schema.rs`
6. Creates two commits following established conventions
7. Runs build, test, and clippy to verify

## Prerequisites

- Rust toolchain with `cargo`
- `rustfmt` installed
- Clean git working tree

## Usage

```bash
# Add a new spec version (schema identical to previous)
/add-spec-version 3.7.0

# Add a new spec version with a new JSON schema
/add-spec-version 3.7.0 --schema path/to/ignition.json
```

## Workflow

### Step 1: Parse input and detect previous version

Parse the version argument to extract components. The argument is required.

```
NEW_VERSION = argument (e.g., "3.7.0")
NEW_MAJOR = 3
NEW_MINOR = 7  (extracted from version)
PREV_MINOR = NEW_MINOR - 1
NEW_MOD = v3_7
PREV_MOD = v3_6
NEW_VARIANT = V3_7
```

Verify:
- `src/{PREV_MOD}/` exists
- `src/{NEW_MOD}/` does NOT exist
- Working tree is clean: `git status --porcelain` produces no output

If any check fails, stop and report the issue.

### Step 2: Copy previous spec directory

Run:
```bash
cp -r src/{PREV_MOD} src/{NEW_MOD}
```

Create the first commit:
```bash
git add src/{NEW_MOD}
git commit -m "Copy {NEW_MOD} spec from {PREV_MOD}

Pure cp -r src/{PREV_MOD} src/{NEW_MOD}."
```

### Step 3: Update VERSION constant

Edit `src/{NEW_MOD}/mod.rs`. Find the line:
```rust
pub(crate) const VERSION: Version = Version::new(3, {PREV_MINOR}, 0);
```
Replace with:
```rust
pub(crate) const VERSION: Version = Version::new(3, {NEW_MINOR}, 0);
```

### Step 4: Apply new schema (if provided)

If the user provided a `--schema` path:

1. Copy the provided `ignition.json` to `src/{NEW_MOD}/ignition.json`
2. Run `cargo build --features regenerate` to regenerate `schema.rs`
3. Review the diff of `src/{NEW_MOD}/schema.rs` to check for:
   - New structs with required (non-Optional) fields
   - Changed field types (e.g., `Option<String>` -> `String`)
   - Removed fields
4. If `schema.rs` changes introduced new required fields on existing structs, update `mod.rs`:
   - Check if `Default` impls need updating
   - Check if `Ignition::default()` needs new fields
5. Verify no OTHER version's `schema.rs` was modified (only `src/{NEW_MOD}/schema.rs` should change)

If no schema was provided, skip this step.

### Step 5: Update `src/lib.rs`

Make exactly 4 insertions in `src/lib.rs`:

**5a. Module declaration** -- Find the line `pub mod {PREV_MOD};` and insert after it:
```rust
pub mod {NEW_MOD};
```

**5b. Config enum variant** -- Find `{PREV_VARIANT}({PREV_MOD}::Config),` in the `Config` enum and insert after it:
```rust
    {NEW_VARIANT}({NEW_MOD}::Config),
```

**5c. Parse branch** -- Find the block:
```rust
        } else if version == {PREV_MOD}::VERSION {
            Self::{PREV_VARIANT}(parse_warn(v, &mut warnings)?)
        } else {
            return Err(Error::UnknownVersion(version));
```
Insert before the `} else {` / `UnknownVersion` line:
```rust
        } else if version == {NEW_MOD}::VERSION {
            Self::{NEW_VARIANT}(parse_warn(v, &mut warnings)?)
```

**5d. Test case** -- Find the end of the last version's test block in `fn parse()`. Look for the line:
```rust
        assert!(warnings.is_empty());
    }

    #[test]
    fn round_trip() {
```
Insert BEFORE the closing `}` of the `parse()` test (i.e., before `#[test] fn round_trip`):
```rust

        let mut expected = {NEW_MOD}::Config::default();
        expected
            .storage
            .get_or_insert_with(Default::default)
            .files
            .get_or_insert_with(Default::default)
            .push({NEW_MOD}::File::new("/z".into()));
        let (config, warnings) = Config::parse_str(
            r#"{{"ignition": {{"version": "{NEW_VERSION}"}}, "storage": {{"files": [{{"path": "/z"}}]}}}}"#,
        )
        .unwrap();
        assert_eq!(config, Config::{NEW_VARIANT}(expected));
        assert!(warnings.is_empty());
```

### Step 6: Update release notes

Edit `docs/release-notes.md`. Find the first `##` heading (the "Upcoming" section) and add the bullet point after the heading line:

```markdown
- Add Ignition {NEW_VERSION} spec
```

If there is already content under the heading, add it as the first bullet. If there is a blank line after the heading, insert the bullet after that blank line.

### Step 7: Build and test

Run all validation commands:

```bash
cargo build --all-targets
cargo build --all-targets --features regenerate
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
```

All must pass. If any fail, fix the issue before proceeding.

After the regenerate build, check that no generated files changed unexpectedly:
```bash
git status --porcelain
```
The only modified files should be in `src/{NEW_MOD}/` and `src/lib.rs` and `docs/release-notes.md`.

### Step 8: Create second commit

```bash
git add -A
git commit -m "Update for {NEW_VERSION} spec"
```

### Step 9: Report results

Summarize what was done:
- List the 2 commits created
- List all files changed
- Report test/clippy results
- Remind the user to:
  - Review the changes
  - Push the branch and open a PR
  - Follow the release checklist if this is part of a release

## Checklist Coverage

This skill automates the following from the spec addition process:

- [x] Copy previous spec directory
- [x] Update VERSION constant
- [x] Add module declaration to lib.rs
- [x] Add Config enum variant
- [x] Add parse branch
- [x] Add test case
- [x] Update release notes
- [x] Apply new JSON schema (if provided)
- [x] Regenerate schema.rs
- [x] Run build/test/clippy validation
- [x] Create properly formatted git commits

## What's NOT covered

- Determining what schema changes exist (user must provide the new `ignition.json`)
- Complex `mod.rs` changes if schema introduces breaking structural changes (e.g., required fields becoming optional, new top-level config sections)
- Opening the PR
- The release process itself (version bump, tagging, publishing)

## Example Output

```
Adding Ignition spec version 3.7.0...

Pre-flight checks:
  - src/v3_6/ exists
  - src/v3_7/ does not exist
  - Working tree is clean

Step 1: Copying src/v3_6 -> src/v3_7
  Created: src/v3_7/ignition.json
  Created: src/v3_7/mod.rs
  Created: src/v3_7/schema.rs
  Committed: "Copy v3_7 spec from v3_6"

Step 2: Updating VERSION in src/v3_7/mod.rs
  Version::new(3, 6, 0) -> Version::new(3, 7, 0)

Step 3: Wiring into src/lib.rs
  Added: pub mod v3_7
  Added: V3_7(v3_7::Config) enum variant
  Added: v3_7::VERSION parse branch
  Added: v3_7 test case

Step 4: Updating docs/release-notes.md
  Added: "- Add Ignition 3.7.0 spec"

Step 5: Building and testing
  cargo build --all-targets .............. OK
  cargo build --features regenerate ...... OK
  cargo test --all-targets ............... OK (X tests passed)
  cargo clippy ........................... OK (no warnings)

  Committed: "Update for 3.7.0 spec"

Done! 2 commits created. Files changed:
  - src/v3_7/ignition.json (new)
  - src/v3_7/mod.rs (new, VERSION updated)
  - src/v3_7/schema.rs (new)
  - src/lib.rs (4 insertions)
  - docs/release-notes.md (1 insertion)

Next steps:
  1. Review the changes with `git log -2 -p`
  2. Push the branch and open a PR
```

## References

- Design document: `.opencode/skills/add-spec-version/DESIGN.md`
- Example (simple): `.opencode/skills/add-spec-version/examples/v3_6-simple.md`
- Example (with schema): `.opencode/skills/add-spec-version/examples/v3_5-with-schema-changes.md`
- Development docs: `DEVELOPMENT.md`
- CI workflow: `.github/workflows/rust.yml`
- Build script (regeneration): `build.rs`
