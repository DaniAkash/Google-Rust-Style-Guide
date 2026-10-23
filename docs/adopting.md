# Adopting the kit

## New workspace

1. Copy the config files into your repository root:

   ```sh
   base=https://raw.githubusercontent.com/DaniAkash/Google-Rust-Style-Guide/main
   for file in rustfmt.toml clippy.toml deny.toml rust-toolchain.toml AGENTS.md; do
     curl -fsSLO "$base/$file"
   done
   ```

2. Copy the `[workspace.lints.rust]`, `[workspace.lints.rustdoc]` and `[workspace.lints.clippy]` tables from this repository's [`Cargo.toml`](../Cargo.toml) into your workspace root `Cargo.toml`.

3. Opt every member crate in:

   ```toml
   # crates/<name>/Cargo.toml
   [lints]
   workspace = true
   ```

4. Install nightly rustfmt for the guide's nightly-only formatting options:

   ```sh
   rustup toolchain install nightly --profile minimal --component rustfmt
   ```

5. Add the checks to CI (below), and point your agents at `AGENTS.md` (also below).

## Single crate

Paste the same tables into the crate's `Cargo.toml` with `workspace.` removed: `[lints.rust]`, `[lints.rustdoc]`, `[lints.clippy]`.

## CI

```yaml
jobs:
  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: cargo clippy --workspace --all-targets -- -D warnings
      - run: cargo test --workspace

  format:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: rustup toolchain install nightly --profile minimal --component rustfmt
      - run: cargo +nightly fmt --check

  deny:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: taiki-e/install-action@v2
        with:
          tool: cargo-deny
      - run: cargo deny check bans
```

`-D warnings` is what makes the `warn` levels blocking. Leave it off locally.

## Coding agents

`AGENTS.md` at the repository root is picked up automatically by Codex, Cursor, Gemini CLI, Jules, GitHub Copilot and most other agents. For Claude Code, which reads `CLAUDE.md`:

```sh
ln -s AGENTS.md CLAUDE.md
```

If you already have an `AGENTS.md`, paste this kit's file in as a "Rust style" section. Keep the "Before you finish" and "When a lint fires" sections intact; those are what keep agents from editing the config to make warnings go away.

Agents get more out of the lints when they see them during the task rather than only at the end. In Claude Code, a `PostToolUse` hook that runs `cargo clippy --message-format=short` after edits to `.rs` files feeds warnings back immediately.

## Existing codebases

Turning on 41 lints at once in a mature codebase produces a wall of warnings. Phase it in:

1. **Formatting first.** Add `rustfmt.toml` and run `cargo +nightly fmt` in a single formatting-only commit, so `git blame` can skip it (`.git-blame-ignore-revs`).
2. **Auto-fixes.** Add the lint tables and run `cargo clippy --fix --workspace --all-targets --allow-dirty`. Many lints (`ptr_arg`, `implicit_clone`, `needless_for_each`, `unnecessary_unwrap` and others) fix themselves.
3. **Measure what remains.**

   ```sh
   cargo clippy --workspace --all-targets --message-format=json 2>/dev/null \
     | jq -r 'select(.reason == "compiler-message") | .message.code.code // empty' \
     | sort | uniq -c | sort -rn
   ```

4. **Defer the noisy ones explicitly.** Set the lints you cannot fix yet to `"allow"` in `Cargo.toml` with a comment naming the cleanup issue, and turn on `-D warnings` in CI for everything else. Re-enable each deferred lint as you clean it up.
5. **New code meets the full bar.** Review new code against the full guide even while old code is grandfathered.

### Existing libraries

Two settings can force breaking changes on a published library:

- `avoid-breaking-exported-api = false` in `clippy.toml` lets API lints suggest changes to public signatures. Set it to `true` until your next major version.
- `exhaustive_enums` and `exhaustive_structs` ask for `#[non_exhaustive]`, which is itself a breaking change. Add it in a major release, or record the commitment with `#[expect(..., reason = "...")]`.

## Application crates

The guide's API rules bind library code and are recommendations for application code. A member crate cannot partly override `[lints] workspace = true`, so an application crate relaxes rules at its crate root instead, with the reason on record:

```rust
// src/main.rs
#![expect(
    clippy::must_use_candidate,
    reason = "application crate: no external callers to protect from ignored results"
)]
```

Relax only the API-shaped lints (`must_use_candidate`, `exhaustive_enums`, `exhaustive_structs`, `needless_pass_by_value`, `missing_errors_doc`). Correctness and readability lints apply everywhere.

## Staying stable-only

The guide allows a stable-only codebase to skip its nightly rustfmt options. Stable `cargo fmt` already ignores them with a warning. To remove the warning, delete `hex_literal_case`, `format_macro_matchers` and `imports_granularity` from `rustfmt.toml`, and add "one module per `use` statement" to your review checklist.

## Keeping up to date

This repository's CI runs every week against the latest stable clippy, and the fixtures fail if a lint is renamed or changes behavior. Watch the repository's releases, or re-copy the config files when you upgrade Rust.
