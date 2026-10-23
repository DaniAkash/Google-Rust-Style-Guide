# Why: the reasoning behind each choice

This page explains the configuration decisions that are not obvious from the lint name. For the full map of rules to lints, see [coverage.md](coverage.md). For what was deliberately left out, see [why-not.md](why-not.md).

## Principles

1. **Enforce what the guide says, not more.** A lint gets in only when it maps to a sentence in the guide. Popular lints that the guide does not ask for (`mod_module_files`, `module_name_repetitions`, the whole `pedantic` group) stay off, so a warning always points back to a rule.
2. **One layer per rule.** A rule lives in tooling or in AGENTS.md, never both. Duplicated rules drift, and agents waste context re-reading what clippy will tell them anyway.
3. **Exceptions are data.** The guide grants many exceptions. Rather than weakening a lint to accept them, the lint stays strict and the exception is written down as `#[expect(lint, reason = "...")]` at the exact site where a reviewer can judge it.
4. **The config must prove itself.** Every lint is exercised by a fixture in CI, so "we enforce X" is a tested claim rather than a hope.

## Global settings

### `all = { level = "warn", priority = -1 }`

This keeps clippy's default groups (correctness, suspicious, style, complexity, perf) on. They encode the official Rust style and API conventions, which the guide's first rule says to follow. `priority = -1` lets the individual entries below override the group.

### `warn` in the config, `-D warnings` in CI

Every guide lint is `warn`, and CI promotes warnings to errors. Locally you can keep compiling and testing while a warning is outstanding, but nothing merges with one. Only two lints are `deny` everywhere, because there is never a reason to iterate with them present:

- `unstable_features`: the guide forbids unstable features outright.
- `clippy::invisible_characters`: an invisible character in a literal is never intentional enough to keep.

### `avoid-breaking-exported-api = false`

By default clippy skips several API lints (`wrong_self_convention`, `ref_option`, `needless_pass_by_value` and others) on public items, to avoid suggesting breaking changes. The guide's API rules matter most at public boundaries, so this kit turns that protection off. An established library that cannot ship breaking changes can set it back to `true`. See [adopting.md](adopting.md#existing-libraries).

### `allow_attributes_without_reason`

The guide is a document of rules plus exceptions, and every exception should be reviewable. Requiring `reason = "..."` on every suppression means the reviewer sees the author's justification rather than having to reconstruct it. AGENTS.md asks for `#[expect]` rather than `#[allow]` because an `#[expect]` fails once it is no longer needed.

## Code organization

### `wildcard_imports`

It matches the guide's exceptions closely without configuration. It does not flag `use Enum::*` inside a function body, `pub use submodule::*` re-exports, or `use super::*` in `#[cfg(test)]` modules. The guide also allows `use super::*` in non-test inline modules; those need an `#[expect]`, which is rare enough to be acceptable.

### `absolute_paths` with a two-segment limit

The guide wants functions called as `module::function()` and types used bare after a `use`. `absolute_paths` with `absolute-paths-max-segments = 2` accepts `fs::read()` and `Regex::new()` but rejects `std::fs::read()`, `crate::shelf::add()` and `regex::Regex::new()` written inline, which pushes code toward exactly the import style the guide describes. It cannot catch the opposite mistake (`use std::mem::swap; swap(..)`), so AGENTS.md covers that.

## Formatting

### Nightly-only rustfmt options

The guide names `imports_granularity = Module`, `hex_literal_case = Lower` and `format_macro_matchers = true`, all unstable in rustfmt. The guide also says a codebase that wants to stay on stable may drop them. This kit keeps them in `rustfmt.toml` because stable rustfmt only warns about them and otherwise formats normally. Teams that run `cargo +nightly fmt` get the full guide, and teams that do not still get `max_width = 100`.

### `invisible_characters`

The guide asks for escapes such as `\u{200D}` instead of literal invisible characters. That lives with rustfmt in the guide's formatting section, but rustfmt cannot enforce it and clippy can.

## Ownership

### `Rc` and `Arc` as `disallowed_types`

The guide says to prefer unique ownership and to allow `Rc`/`Arc` only when the alternative would be unreasonably complex, "a judgment call to be discussed during code review." A lint cannot make that call, but it can make sure the call happens: every use needs `#[expect(clippy::disallowed_types, reason = "...")]`. Put it at module level (`#![expect(...)]`) when a module is built around shared ownership, so the reason is stated once.

### `redundant_clone` from the nursery

The guide's rule is "use `.clone()` when you genuinely need a deep copy". `redundant_clone` is the only lint that directly detects a clone whose original is never used again. It lives in the nursery group because of rare false positives in complex control flow; when one happens, `#[expect]` with a reason is cheap. `implicit_clone` and `needless_pass_by_value` cover the related patterns: hidden clones through `to_owned()`/`to_string()`, and owned parameters that are only read.

## Function inputs

### `ref_option` alongside `ptr_arg` and `borrowed_box`

The guide's table has four rows: `&T` over `&Box<T>`, `&[T]` over `&Vec<T>`, `&str` over `&String`, and `Option<&T>` over `&Option<T>`. It cites `borrowed_box` and `ptr_arg`, which cover the first three. `ref_option` covers the fourth.

### Bool thresholds of one

`fn_params_excessive_bools` and `struct_excessive_bools` default to flagging at four bools. The guide wants an enum whenever a bool's purpose is not obvious, which no threshold captures. A threshold of one (flag at two or more) catches the worst cases, `render(true, false)`, without forcing enums on a single obvious flag like `set_visible(bool)`.

## Traits and naming

`should_implement_trait`, `wrong_self_convention`, `new_without_default` and `len_without_is_empty` are named by the guide itself and are on by default. They are listed explicitly in `Cargo.toml` anyway, so the file reads as a complete map and so a future change to clippy's defaults cannot silently drop them. `inherent_to_string` and `from_over_into` extend the same "implement the standard trait" rule to `Display` and `From`. `new_ret_no_self` backs the constructor naming rule: a function called `new` returns `Self`.

## Documentation

`missing_docs` is what the guide names. `missing_panics_doc`, `missing_errors_doc` and `missing_safety_doc` back its instructions to document panic cases and unsafe invariants, plus the Rust API Guidelines' requirement to document errors (the guide says to follow the API Guidelines). `doc_markdown` catches identifiers written without backticks, since the guide asks for Markdown.

## Other features

### `exhaustive_enums` and `exhaustive_structs`

Both are restriction lints, and the guide names `exhaustive_enums` itself. `exhaustive_structs` fires only on structs whose fields are all public, which is exactly the guide's condition. Both have well-defined exceptions in the guide (fixed ontologies, two-state enums, options structs using struct update syntax), which become `#[expect]` with a reason. The example crate shows both.

### `must_use_candidate`

The guide wants `#[must_use]` on functions whose result the caller cannot ignore, "including pure functions such as `HashMap::get()`". `must_use_candidate` flags exactly public functions with no side effects that return a value. It is the noisiest lint in the set, but every warning it raises is a case the guide covers.

### Poison and `catch_unwind` as disallowed methods

"Never reach inside a poisoned mutex" means never calling `PoisonError::into_inner`, `get_ref` or `get_mut`, so those three are disallowed. `panic::catch_unwind` is prohibited apart from a defense-in-depth exception, which takes an `#[expect]` with a reason.

### cargo-deny for code before `main()`

The guide cites the `ctor` crate. Running code before `main()` is a property of dependencies, not of your own source, so it is checked at the dependency graph: `ctor`, plus `inventory` and `static_init`, which provide the same capability.

## Implementation concerns

- `unwrap_used` turns "prefer `expect` over `unwrap`" into a check. `allow-unwrap-in-tests` follows the guide's "tests may use panics."
- `unnecessary_unwrap` is the guide's check-then-unwrap example, almost line for line.
- `missing_assert_message` enforces that a message exists. Clippy skips test functions, which matches the guide's focus on assertions in production code.
- `needless_for_each` matches "avoid `Iterator::for_each`" exactly.
- `format_push_string` catches `s += &format!(..)` and `push_str(&format!(..))`, where `write!` avoids the temporary allocation.
