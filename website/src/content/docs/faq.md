---
title: FAQ
description: Short answers about enforcing Google's Rust style guide with clippy, rustfmt and cargo-deny, making coding agents follow it, stable Rust support, and removal.
head:
  - tag: title
    content: "Google Rust Style Guide FAQ: Clippy, Agents and Adoption"
sidebar:
  order: 3
---

## Can clippy enforce Google's Rust style guide?

Partly. Clippy, rustfmt and cargo-deny together enforce 32 of the guide's 62 in-scope rules, fully or partially, with 41 lints and three config files. The other 30 rules are design judgment, such as newtype or type alias and iterator or `Vec`, which no linter can decide. The kit covers those with [AGENTS.md](/Google-Rust-Style-Guide/agents-md/) for coding agents and a [human review guide](/Google-Rust-Style-Guide/human-review/). The [coverage table](/Google-Rust-Style-Guide/coverage/) lists every rule and what enforces it.

## Which rules can't be linted?

About 30 judgment rules: library versus application API choices, newtypes versus aliases, named fields versus tuples, iterators versus `Vec` in signatures, generics versus `dyn Trait`, naming of lifetimes, generics and abbreviations, getters and setters, macros, and how much type inference is too much. [Why not](/Google-Rust-Style-Guide/why-not/) explains each, plus the gaps in lints that are enabled.

## How do I adopt it?

Download five files into your project root, append the lint tables to `Cargo.toml`, and run `cargo +nightly fmt`, `cargo clippy --workspace --all-targets -- -D warnings` and `cargo deny check bans`. The [quick start](/Google-Rust-Style-Guide/quick-start/) has copyable commands for single crates and workspaces and takes about five minutes.

## How do I make Claude Code, Codex or Cursor follow it?

Put the kit's `AGENTS.md` at your repository root. Codex, Cursor, Gemini CLI, GitHub Copilot and most other agents read it automatically; for Claude Code, run `ln -s AGENTS.md CLAUDE.md`. It tells the agent which commands to run before finishing, forbids editing the lint configuration to silence warnings, and lists the judgment rules with examples. The quick start also has a one-prompt setup you can paste into any agent.

## Does it work on stable Rust?

Yes. The lints, clippy configuration and cargo-deny run on stable Rust 1.95 or newer. Three of the guide's rustfmt options (`imports_granularity`, `hex_literal_case`, `format_macro_matchers`) exist only on nightly rustfmt, so run `cargo +nightly fmt` to apply them. Stable rustfmt prints a warning and applies everything else.

## What happens with an existing codebase?

Expect many warnings at first. Format in one commit, run `cargo clippy --fix` for the auto-fixable lints, then defer the noisy lints explicitly and re-enable them one by one. [Adopting in depth](/Google-Rust-Style-Guide/adopting/#existing-codebases) has the step-by-step rollout, including how to protect a published library's API.

## What if the guide allows an exception?

Keep the lint on and record the exception where it happens: `#[expect(clippy::exhaustive_structs, reason = "options struct; callers use struct update syntax")]`. A suppression without a reason fails the build, and `#[expect]` fails once it is no longer needed. The [human review guide](/Google-Rust-Style-Guide/human-review/) lists which reasons the guide accepts for each lint.

## Is this official?

No. It is an unofficial community project, not affiliated with or endorsed by Google. The [Google Rust Style Guide](https://google.github.io/styleguide/rustguide.html) is published by Google under CC-BY 3.0; this kit paraphrases its rules, links to the original sections, and is licensed Apache-2.0.

## How do I remove it?

Delete `rustfmt.toml`, `clippy.toml`, `deny.toml`, `rust-toolchain.toml` and `AGENTS.md` (and a `CLAUDE.md` link if you made one), then remove the lint tables from `Cargo.toml`. The [quick start](/Google-Rust-Style-Guide/quick-start/#remove-the-kit) shows the exact commands.

## How is the kit kept correct?

Its CI proves the configuration on every push and weekly against the latest stable clippy. A fixture crate of deliberate violations must trigger exactly the expected 45 lints, a rustfmt fixture must format to an expected file, and cargo-deny must reject a crate that depends on `ctor`. See [how it works](/Google-Rust-Style-Guide/how-it-works/#proving-the-configuration).
