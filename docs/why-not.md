# Why not: what this kit does not enforce, and why

This page records what was left out, so nobody has to rediscover these decisions. There are four kinds: rules no tool can check, gaps in lints that are enabled, lints that were considered and rejected, and rules that belong to specific Google codebases.

## Rules no tool can check

About 30 sections of the guide are design judgment. A linter sees syntax, while these rules depend on intent, audience and the future of the code:

| Rule | Why a lint cannot decide it |
|---|---|
| Library or application code | The same function can be either; it depends on who calls it. |
| Newtype or type alias | Depends on whether values could be confused, which is a domain question. |
| Named fields or tuples | `(f64, f64)` is fine in a local helper and confusing in a public return type. |
| Iterators or `Vec`, `impl IntoIterator` or slices, `impl Into<T>` | The guide lists six exceptions based on how callers use the result. |
| Generics or `dyn Trait`, trait or inherent `impl` | Code size and API intent, not syntax. |
| Extension trait ownership | Which team owns the type is not in the source. |
| Leaf module naming, non-leaf modules holding implementations | "Primary item" and "a small amount of glue code" are judgment calls. |
| Locally-scoped trait imports | Whether a trait is "used extensively throughout the file" is a threshold the guide leaves open. |
| Lifetime, generic parameter and abbreviation naming | Descriptiveness is subjective; `Ip` is fine, `Cfg` is not. |
| Getters and setters, closure accessors | Clippy cannot tell a getter from any other method returning a field. |
| Macros vs. functions | Requires weighing alternatives that do not exist in the code yet. |
| Type inference | "Enough explicit types that someone familiar with the codebase can follow" depends on the reader. |
| `#[doc(hidden)]`, `const fn` commitments, operator overloading | Ownership, future intent and meaning. |
| Trait selection (`Copy`, `Clone`, `PartialEq` vs. `Eq`, `Hash`) | Depends on whether the type will stay `Copy` and how it will be used. |

These rules live in [`AGENTS.md`](../AGENTS.md) for agents and [`human-review.md`](human-review.md) for reviewers.

## Gaps in lints that are enabled

| Lint | Catches | Misses |
|---|---|---|
| `absolute_paths` | `std::fs::read()`, `crate::a::f()`, `regex::Regex::new()` inline | Two-segment type paths (`regex::Regex` in a signature) and bare imported functions (`use std::mem::swap; swap(..)`) |
| `wildcard_imports` | Glob imports except the guide's allowed forms | `use super::*` in non-test inline modules is flagged although the guide allows it; defining a `prelude` module is not flagged |
| `fn_params_excessive_bools` | Two or more bool parameters | A single bool whose meaning is unclear at the call site |
| `redundant_clone`, `implicit_clone` | Clones that are provably unnecessary | Clones used to dodge a borrow that could be restructured |
| `must_use_candidate` | Pure public functions | Functions with side effects whose return value is still critical |
| `missing_assert_message` | Assertions with no message | Messages that only restate the condition |
| `needless_for_each`, `format_push_string` | `for_each`, `push_str(&format!(..))` | Long or stateful iterator chains, chains in `for` headers, `a + b + c` string concatenation, `str::replace` templating |
| `undocumented_unsafe_blocks` | `unsafe` without `// SAFETY:` | Whether the comment is correct, and whether unsafe was needed at all |

## Lints considered and rejected

| Lint or setting | Why it is off |
|---|---|
| `clippy::pedantic` as a group | Many of its lints have nothing to do with the guide (`module_name_repetitions`, `similar_names`, `cast_*`). The useful ones are enabled individually. |
| `clippy::string_add`, `clippy::string_add_assign` | They flag every `a + b`. The guide only asks for `format!` when concatenating *more than two* strings. |
| `clippy::clone_on_ref_ptr` | The guide says cloning an `Rc`/`Arc` "is an expected way to use them". `Rc`/`Arc` themselves are already gated by `disallowed_types`. |
| `clippy::rc_buffer`, `clippy::rc_mutex` | Covered more broadly by `disallowed_types` on `Rc` and `Arc`. |
| `clippy::enum_glob_use` | Flags `use Enum::*` everywhere, including in function bodies, where the guide allows it. `wildcard_imports` already handles the cases the guide forbids. |
| `warn-on-all-wildcard-imports = true` | Would flag `use super::*` in test modules, which the guide allows. |
| `clippy::wildcard_enum_match_arm` | Bans the wildcard arm. The guide requires a wildcard arm when matching `#[non_exhaustive]` enums from other crates. |
| `clippy::single_char_lifetime_names`, `clippy::min_ident_chars`, `clippy::many_single_char_names` | The guide allows single-letter lifetimes for method-local relationships and single-letter generics such as `T`. These lints cannot tell the allowed cases apart. |
| `clippy::missing_const_for_fn` | Pushes toward marking functions `const`. The guide warns the opposite way: only make a public function `const` if you will keep it `const`. |
| `clippy::derive_partial_eq_without_eq` | Pushes toward deriving `Eq`. The guide prefers `PartialEq` unless `Eq` is needed. |
| `missing_copy_implementations`, `missing_debug_implementations` | The guide asks for `Copy` only when the type will reasonably stay `Copy`, and does not mention `Debug`. Blanket lints would force commitments the guide leaves to judgment. |
| `clippy::return_self_not_must_use` | Targets builder methods, while the guide's `#[must_use]`-on-types rule targets errors and lazy computations. |
| `clippy::missing_docs_in_private_items` | The guide asks for docs on public APIs only. |
| `clippy::expect_used`, `clippy::panic` | The guide permits `expect` and `panic!` for bugs and violated invariants. |
| `clippy::allow_attributes` | It would force `#[expect]` everywhere. `#[expect]` is preferred, but it breaks on lints that fire only under some `cfg` combinations, so `#[allow]` with a reason stays legal. |
| `clippy::mod_module_files`, `clippy::self_named_module_files` | The guide does not choose between `foo/mod.rs` and `foo.rs`. |

## `non_exhaustive_omitted_patterns`

The guide suggests the built-in `non_exhaustive_omitted_patterns` lint for its rule on matching `#[non_exhaustive]` enums. That lint is still feature-gated: on stable Rust it is reported as an unknown lint and does nothing. Enabling it would mean either a nightly toolchain, which the guide's no-unstable-features rule rules out, or a permanently unknown-lint warning. The rule lives in AGENTS.md and the review checklist until the lint is stabilized. Re-check with each Rust release:

```sh
echo '#![warn(non_exhaustive_omitted_patterns)]' > /tmp/probe.rs && rustc --crate-type lib --out-dir /tmp /tmp/probe.rs
```

## Codebase-specific rules

The guide includes rules tagged for particular Google codebases. They depend on Google's build systems and runtimes, so this kit does not enforce them. If your codebase has the same constraint, the table shows how to add it.

| Rule | Codebase | To adopt anyway |
|---|---|---|
| Prefer small crates | Chromium | Review policy |
| Flat file layout for `rust_library` sources | Chromium | Review policy |
| Use `chromium::import!()` | Chromium | Not applicable outside Chromium |
| Don't use `async` / `.await` | Chromium | No lint exists; review policy |
| Don't start threads through the standard library | Chromium | Add `{ path = "std::thread::spawn" }` and `std::thread::Builder::spawn` to `disallowed-methods` |
| Use `fuchsia_sync::Mutex` | Fuchsia | Add `std::sync::Mutex` and `parking_lot::Mutex` to `disallowed-types` |
| Use the per-codebase test framework | Chromium, Fuchsia | Not applicable outside those codebases |

The guide's exceptions for unstable features (Android embedded, Chrome's approved list, Fuchsia) are also out of scope. This kit pins stable.

## Formatting on stable only

The guide says a codebase that wants to stay on stable may skip its nightly-only rustfmt options. This kit keeps them in `rustfmt.toml` anyway, because stable rustfmt ignores them with a warning. To silence the warning on a stable-only team, delete the three nightly options; the import granularity rule then moves to review.
