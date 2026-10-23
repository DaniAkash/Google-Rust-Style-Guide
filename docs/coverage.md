# Coverage: every guide rule and what enforces it

Each section of the [Google Rust Style Guide](https://google.github.io/styleguide/rustguide.html) is assigned to the cheapest layer that can hold it:

| Layer | Meaning |
|---|---|
| **Tool** | rustfmt, clippy, rustc or cargo-deny fails the build. Verified by the fixtures in this repo. |
| **Tool, partial** | A lint catches the common or mechanical case; AGENTS.md and review cover the rest. |
| **Agent + review** | No tool can check it. [`AGENTS.md`](../AGENTS.md) tells coding agents, [`human-review.md`](human-review.md) tells reviewers. |
| **Out of scope** | Specific to Chromium, Fuchsia or Android build systems. See [why-not.md](why-not.md#codebase-specific-rules). |

Lint names link to the clippy docs at <https://rust-lang.github.io/rust-clippy/stable/index.html>.

## Summary

| Layer | Rows |
|---|---|
| Tool | 19 |
| Tool, partial | 13 |
| Agent + review | 30 |
| Out of scope | 7 |

The tables below have one row per guide section, except documentation, which is split into three rows. Every in-scope row has a home: tooling covers 32 of the 62 fully or partially, and the rest are design judgment no linter can make.

## Background

| Guide section | Layer | Enforced by |
|---|---|---|
| Follow public Rust style rules | Tool, partial | rustfmt defaults, clippy's default groups (`clippy::all`); API Guidelines judgment in review |
| Apply API design rules in context | Agent + review | Decide library or application first; see AGENTS.md |

## Code organization

| Guide section | Layer | Enforced by |
|---|---|---|
| Prefer small crates [chrome] | Out of scope | Chromium build rule |
| Use flat file layout for `rust_library` sources [chrome] | Out of scope | Chromium build rule |
| Name source files and leaf modules after the primary user-defined item | Agent + review | |
| Don't implement APIs in non-leaf modules | Agent + review | |
| Don't use glob imports | Tool | `clippy::wildcard_imports`. It already permits `use Enum::*` inside functions, `pub use submodule::*`, and `use super::*` in test modules. `use super::*` in other inline modules needs an `#[expect]`. Defining preludes is review-only. |
| A use statement should import items from one module | Tool | rustfmt `imports_granularity = "Module"` (nightly rustfmt) |
| Prefer locally-scoped trait imports | Agent + review | |
| Qualify calls to top-level functions with a module name | Tool, partial | `clippy::absolute_paths` (max 2 segments) rejects `crate::a::f()` and `std::fs::read()` inline. A bare `f()` imported with `use a::f` is review-only. |
| Type names should be imported and used unqualified | Tool, partial | `clippy::absolute_paths` rejects paths of 3+ segments such as `std::io::ErrorKind`. Two-segment paths like `regex::Regex` are review-only. |
| Use the appropriate import mechanism [chrome] | Out of scope | `chromium::import!()` |

## Code formatting and aesthetics

| Guide section | Layer | Enforced by |
|---|---|---|
| Use the appropriate formatting tool | Tool | rustfmt `max_width = 100`, `hex_literal_case = "Lower"`, `format_macro_matchers = true` (the last two need nightly rustfmt); `clippy::invisible_characters` (deny) for unescaped invisible characters |
| Opting out of automatic formatting | Agent + review | `#[rustfmt::skip]` only for manual alignment or generated code |

## Ownership and borrowing

| Guide section | Layer | Enforced by |
|---|---|---|
| Avoid `Rc` and `Arc` | Tool | `clippy::disallowed_types` on `std::rc::Rc` and `std::sync::Arc`. Every use needs `#[expect(clippy::disallowed_types, reason = "...")]`, so the judgment call the guide asks for has a written reason for review. |
| Prefer borrows over deep copies | Tool, partial | `clippy::redundant_clone`, `clippy::implicit_clone`, `clippy::needless_pass_by_value`. Clones used to get past the borrow checker are review-only. |

## Function inputs and outputs

| Guide section | Layer | Enforced by |
|---|---|---|
| Accept flexible references for immutable data | Tool | `clippy::ptr_arg` (`&Vec<T>`, `&String`), `clippy::borrowed_box` (`&Box<T>`), `clippy::ref_option` (`&Option<T>`) |
| Iterators vs. owned collections in return types | Agent + review | |
| Iterators vs. slices in function parameters | Agent + review | |
| Accepting `Into` | Agent + review | |
| Prefer enum over bool in APIs | Tool, partial | `clippy::fn_params_excessive_bools` and `clippy::struct_excessive_bools` with a threshold of one, so two or more bools fail. A single unclear bool is review-only. |
| Reporting errors | Agent + review | `missing_panics_doc` and `missing_errors_doc` make the chosen channel visible in docs |

## Types

| Guide section | Layer | Enforced by |
|---|---|---|
| Newtypes versus type aliases | Agent + review | |
| Prefer named fields over tuples and non-newtype tuple structs | Agent + review | |

## Traits

| Guide section | Layer | Enforced by |
|---|---|---|
| Add trait implementations when they become necessary | Agent + review | |
| Implement known standard library traits instead of custom methods | Tool | `clippy::should_implement_trait`, `clippy::inherent_to_string`, `clippy::from_over_into` |
| Use `#[derive()]` if possible | Tool, partial | `clippy::derivable_impls`, `clippy::expl_impl_clone_on_copy`, `clippy::non_canonical_clone_impl` (default). Other derivable impls are review-only. |
| Generics versus `dyn Trait` | Agent + review | |
| When to define a trait versus an inherent implementation | Agent + review | |
| Extension traits | Agent + review | |
| Specific guidance for important traits | Tool, partial | `clippy::new_without_default` for `Default`. `Copy`, `Clone`, `PartialEq`, `PartialOrd` and `Hash` choices are review-only. |
| Pair important `From` impls with explicit constructor methods | Agent + review | |
| Options structs should implement `Default` | Agent + review | `exhaustive_structs` forces the decision on all-public structs, which surfaces options structs for review |

## Concurrency

| Guide section | Layer | Enforced by |
|---|---|---|
| Handling poison | Tool | `clippy::disallowed_methods` on `PoisonError::into_inner`, `get_ref` and `get_mut` |
| Don't use `async` / `.await` [chrome] | Out of scope | |
| Don't start new threads through the Rust standard library [chrome] | Out of scope | Add `std::thread::spawn` to `disallowed-methods` if your codebase has its own scheduler |
| Use `fuchsia_sync::Mutex` [Fuchsia] | Out of scope | Add `std::sync::Mutex` to `disallowed-types` in Fuchsia code |

## Macros

| Guide section | Layer | Enforced by |
|---|---|---|
| Macros | Agent + review | |

## Naming

| Guide section | Layer | Enforced by |
|---|---|---|
| Naming methods | Tool | `clippy::wrong_self_convention` (`as_`, `to_`, `into_`, `is_`, `from_`) |
| Naming lifetimes | Agent + review | |
| Naming generic parameters | Agent + review | |
| Naming extension traits | Agent + review | |
| Naming initialization functions | Tool, partial | `clippy::new_ret_no_self`, `clippy::new_without_default`. `new_with_x` and `try_new` naming is review-only. |
| Avoid abbreviations | Agent + review | |

## When and how to write documentation

| Guide section | Layer | Enforced by |
|---|---|---|
| Documentation on public APIs | Tool | `missing_docs` (rustc), `rustdoc::broken_intra_doc_links` |
| Document panics, errors and unsafe invariants | Tool | `clippy::missing_panics_doc`, `clippy::missing_errors_doc`, `clippy::missing_safety_doc` |
| Markdown, capitalization, examples, saying something the code does not | Tool, partial | `clippy::doc_markdown` catches unformatted code identifiers; the prose quality is review-only |

## Other Rust features

| Guide section | Layer | Enforced by |
|---|---|---|
| Unsafe code | Tool, partial | `clippy::undocumented_unsafe_blocks`, `clippy::unnecessary_safety_comment`, `unsafe_op_in_unsafe_fn` (rustc), `clippy::missing_safety_doc`. Whether unsafe was necessary, and the technical justification, are review-only. |
| No unstable features | Tool | `rust-toolchain.toml` pins stable; `unstable_features = "deny"` |
| Getters and setters | Agent + review | |
| More on accessors that accept closures | Agent + review | |
| Implement expected API sets together | Tool | `clippy::len_without_is_empty` |
| Do not access items marked `#[doc(hidden)]` that you don't own | Agent + review | |
| Use const functions responsibly | Agent + review | |
| Avoid overloading for non-mathematical purposes | Agent + review | |
| Apply `#[non_exhaustive]` to publicly-exposed enums | Tool | `clippy::exhaustive_enums`; the guide's exceptions are `#[expect]` with a reason |
| Apply `#[non_exhaustive]` to public structs with public fields | Tool | `clippy::exhaustive_structs` (fires only when every field is public, matching the guide) |
| `#[must_use]` on functions | Tool, partial | `clippy::must_use_candidate` for pure public functions. Functions returning critical status are review-only. |
| `#[must_use]` on types | Agent + review | |
| Do not run code before `main()` | Tool | cargo-deny bans `ctor`, `inventory` and `static_init` |

## Implementation concerns

| Guide section | Layer | Enforced by |
|---|---|---|
| Don't overuse type inference | Agent + review | |
| Prefer pattern matching over check-then-unwrap | Tool | `clippy::unnecessary_unwrap` |
| Prefer `expect` over `unwrap` | Tool | `clippy::unwrap_used` (tests exempt via `allow-unwrap-in-tests`) |
| Don't omit patterns on `non_exhaustive` enums when matching | Agent + review | The guide's suggested `non_exhaustive_omitted_patterns` lint is still nightly-only. See [why-not.md](why-not.md#non_exhaustive_omitted_patterns). |
| Use context strings to clarify assertions | Tool | `clippy::missing_assert_message` (skips tests); whether the message adds context is review-only |
| Don't use iterator combinator chains when direct iteration is clearer | Tool, partial | `clippy::needless_for_each`. Long chains and chains in `for` headers are review-only. |
| Don't call `panic::catch_unwind` | Tool | `clippy::disallowed_methods`; the defense-in-depth exception is `#[expect]` with a reason |
| Prefer `format!` / `write!` for concatenation and substitution | Tool, partial | `clippy::format_push_string`. Three-way `+` chains and `str::replace` templating are review-only. |

## Testing

| Guide section | Layer | Enforced by |
|---|---|---|
| Use the right per-codebase test framework | Out of scope | Chromium and Fuchsia test harnesses |

## Repository-wide policy

| Policy | Enforced by |
|---|---|
| Every exception records why | `clippy::allow_attributes_without_reason`; AGENTS.md asks for `#[expect]` |
| The configuration does what this table says | `scripts/check-fixtures.sh`, `scripts/check-rustfmt.sh` and `scripts/check-deny.sh` in CI |
