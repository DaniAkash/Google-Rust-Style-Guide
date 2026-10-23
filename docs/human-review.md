# Human review guide

A reviewer's attention is the most expensive resource in this setup, so this guide spends it only where tools and agents cannot. It assumes CI is green: rustfmt, clippy with `-D warnings` and cargo-deny have passed. If CI is red, send the change back before reading it.

## What you can skip

The tools already guarantee these, so do not comment on them:

- Formatting, line width, import grouping, hex case.
- `&String`, `&Vec<T>`, `&Box<T>` or `&Option<T>` parameters.
- `unwrap()` outside tests, check-then-unwrap, assertions without a message.
- Missing doc comments, missing `# Panics` / `# Errors` / `# Safety` sections, `unsafe` blocks without `// SAFETY:`.
- Glob imports, inline three-segment paths, `for_each`, `push_str(&format!(..))`.
- Missing `#[non_exhaustive]`, `Default`, `is_empty`, or a hand-written impl that could be derived.
- `Rc`, `Arc`, `catch_unwind` and poison access without a recorded reason.
- Dependencies that run code before `main()`.

The full list is in [coverage.md](coverage.md).

## Review in this order

The order follows how expensive a mistake is to undo once merged. Public API shape is the hardest to change, readability inside a function the easiest.

### 1. Exceptions: every `#[expect]`, `#[allow]`, `#[rustfmt::skip]` and `unsafe`

Start here because each one is a place where the author overrode the tools. List them:

```sh
git diff origin/main... -U0 | grep -nE '^\+.*(#!?\[(expect|allow)\(|rustfmt::skip|unsafe)'
```

Also check that the diff does not touch `Cargo.toml` `[lints]`, `clippy.toml`, `rustfmt.toml` or `deny.toml`. A config change needs its own justification and usually its own PR.

For each suppression, judge the reason against the guide:

| Lint | Acceptable reasons (the guide's own exceptions) | Push back on |
|---|---|---|
| `exhaustive_enums` | Fixed ontology (`Color { Red, Green, Blue }`); two states that could be a bool; owner commits to fixing every caller; enum used in only a few places | "Simpler for callers", "we won't add variants" for a domain that obviously grows |
| `exhaustive_structs` | Options struct used with `..Default::default()`; fixed ontology (`Rectangle`); owner fixes all callers | Plain data types crossing a team boundary |
| `disallowed_types` (`Rc`/`Arc`) | Unique ownership would be unreasonably complex: shared state across threads, graph structures, callbacks that outlive their creator | "Needed to satisfy the borrow checker" without saying what was tried; `Arc<Mutex<..>>` wrapping state that one owner could hold |
| `disallowed_methods` (`catch_unwind`) | Defense in depth at an entry point, in a codebase that already avoids panics, built with `panic=unwind` | Using it for control flow or to recover from expected errors |
| `disallowed_methods` (poison) | Essentially never | Any |
| `wildcard_imports` | `use super::*` in a non-test inline module; merging C++ interop namespaces privately | Third-party preludes without discussion (the guide leaves those to the reviewer) |
| `must_use_candidate` | Rare: the lint only fires on functions without mutable inputs, where ignoring the result is almost always a bug | Silencing it across a whole module |
| `unwrap_used` | None outside tests; use `expect` with a reason instead | Any |
| `redundant_clone` | A real false positive the author can explain | "Clippy is wrong" with no explanation |
| `absolute_paths` | Ambiguous common names, macros, generated code | Laziness about adding a `use` |
| `#[rustfmt::skip]` | Deliberate alignment (tables, matrices), generated code | Personal preference |
| `unsafe` | No safe equivalent exists, or benchmarks show the safe version is too slow | Performance claims without numbers |

A good reason states the guide exception and the fact that makes it apply: `reason = "two states that could be a bool; a third visibility state is not planned"`. A reason that repeats the lint name, or reads like boilerplate, is not a reason.

For `unsafe`, additionally check: the unsafe code is wrapped in a small safe API, kept away from business logic, the `// SAFETY:` comment proves soundness rather than asserting it, and the justification says why safe code was not enough.

### 2. Public API surface

Look at every new or changed `pub` item. Decide first whether the crate is library code (binding rules) or application code (recommendations, simplicity wins).

- **Errors.** `Option` for an obvious, expected absence; `Result` when the caller needs to know why; `panic!` only for bugs and broken invariants. Is the error type `Copy` where possible, `#[must_use]`, and does it let callers tell failure modes apart?
- **Inputs.** Library functions that only iterate should take `impl IntoIterator`. `impl Into<T>` appears only where call-site ergonomics justify it, and almost never in application code. Generics over `dyn Trait` in libraries, unless code size matters.
- **Outputs.** Library functions return iterators rather than `Vec` unless one of the guide's exceptions applies (callers need a `Vec`, the result is random-access, the function builds a collection by purpose).
- **Bools.** A single `bool` parameter whose meaning is unclear at the call site should be an enum. Clippy only catches two or more.
- **Types.** Newtype rather than type alias when values could be confused or the representation might change. Named fields rather than tuples or multi-field tuple structs.
- **Traits.** Is each new trait used as a bound, as `impl Trait` / `dyn Trait`, or as an extension trait? A trait that only groups methods should be an inherent `impl`. Extension traits end in `Ext` and belong to the extended type's owner.
- **Trait impls.** Library types anticipate `Clone`, `Debug`, `PartialEq`, `Hash` and `Default` where they make sense; `Copy` only if it will stay `Copy`; `PartialEq`/`PartialOrd` preferred over `Eq`/`Ord` unless the stronger guarantee is needed.
- **Stability commitments.** New `const fn` in a public API is a promise to keep it `const`. Operator overloads must mean math or logic. `#[must_use]` on functions whose result is critical even if they have side effects, and on error and lazy-computation types.

### 3. Ownership

- Are clones real copies, or a way around the borrow checker? Ask what restructuring was considered.
- Do new `Rc`/`Arc` uses have a reason that would convince someone who had not seen the design discussion?
- Closure accessors (`with_foo`) instead of returning a `MutexGuard` or `Ref` that leaks the locking scheme.

### 4. Structure and naming

- Non-leaf modules contain only `mod` declarations and re-exports (plus a little glue). Leaf modules are named after their primary item.
- No new `prelude` modules. Trait imports scoped to the function unless used throughout the file.
- Free functions called as `module::function()`; types imported and used bare.
- Naming follows the standard library: `as_`/`to_`/`into_`; `new`, `new_with_x`, `try_new`, `from_x`; getters without `get_`; `foo_mut`, `set_foo`, `take_foo`, `replace_foo`, `with_foo`.
- Lifetimes on types are descriptive; generic parameters relate to their bound; no abbreviations an outsider would not know.
- Macros only where functions, types and traits could not do the job.

### 5. Documentation quality

Clippy proves docs exist; you judge whether they help.

- Does each doc comment say something the signature does not? `/// Removes an element.` on `remove_element` should be deleted rather than kept to satisfy a lint.
- Are panic conditions, error cases and unsafe invariants concrete enough to act on?
- Does the crate have enough examples somewhere to show how its pieces fit together?

### 6. Implementation readability

- Types are written on `let` bindings after `.parse()`, `.into()`, `.collect()` and long iterator chains, rather than left to inference or put in a turbofish.
- Iterator chains are short. Stateful or branching chains become `for` loops; chains are never built in a `for` header.
- `expect` messages and `assert!` messages explain why the condition holds or what the caller did wrong, not what the condition is.
- Matches on another crate's `#[non_exhaustive]` enums list every known variant, with the ignored ones in the wildcard arm.
- Three or more strings are joined with `format!`, not `+`; templates use format strings, not `str::replace`.

## Reviewing agent-written code

Coding agents read `AGENTS.md` and usually satisfy the lints, so their mistakes cluster in specific places. Look for these patterns in particular:

| Pattern | What it looks like | Ask for |
|---|---|---|
| Boilerplate suppressions | `#[expect(clippy::exhaustive_structs, reason = "public API")]` | A reason that names the guide exception and why it applies |
| Shared ownership as a borrow-checker escape | `Arc<Mutex<State>>` threaded through code that has one owner | A restructure with a single owner and borrows |
| Clones to make it compile | `.clone()` on large values right before a call | A borrow, or a note on why a copy is needed |
| Premature generality in application code | `impl Into<String>`, `impl IntoIterator`, new traits with one implementor | The simplest concrete signature |
| Docs that restate names | `/// Gets the name.` on `fn name()` | Deleting the comment, or saying something the signature does not |
| Messages that restate conditions | `expect("value should be some")` | The invariant that makes it impossible to fail |
| Tuples in signatures | `-> (String, u32, bool)` | A named struct |
| Combinator maximalism | 8-stage chains with side effects | A `for` loop |
| Config drift | Edits to `[lints]`, `clippy.toml`, `rustfmt.toml` in a feature PR | Reverting, and a separate discussion if the rule is wrong |
| `#[allow]` instead of `#[expect]` | Suppressions that will silently outlive the problem | `#[expect]` |

## Writing review comments

Link the guide section for any style point, so the comment explains itself and the author can check the rule rather than take your word for it. A [Conventional Comments](https://conventionalcomments.org/) label makes the intent clear:

```text
issue (blocking): this enum crosses a team boundary, so it needs #[non_exhaustive]. The
"two states" exception doesn't apply because new statuses are planned.
https://google.github.io/styleguide/rustguide.html#apply-non_exhaustive-to-publicly-exposed-enums
```

```text
suggestion (non-blocking): `(String, u32)` as a return type is hard to read at call sites;
a small named struct would help.
https://google.github.io/styleguide/rustguide.html#prefer-named-fields-over-tuples-and-non-newtype-tuple-structs
```

If the same judgment comes up repeatedly, it is a candidate for AGENTS.md (so agents stop doing it) or for a new lint (so nobody has to review it again).

## One-page checklist

```text
[ ] CI green (fmt, clippy -D warnings, cargo-deny)
[ ] No lint, rustfmt or deny config changes, or they are justified separately
[ ] Every #[expect]/#[allow]/#[rustfmt::skip] names a guide exception that applies
[ ] unsafe: necessary, encapsulated, SAFETY comment proves soundness, justification given
[ ] Library or application decided; API rules applied accordingly
[ ] Error channel right: Option / Result / panic
[ ] Inputs and outputs: iterators, Into, generics vs dyn, bools vs enums
[ ] Types: newtypes where values could mix, named fields not tuples
[ ] Traits: real purpose, Ext naming, sensible derived impls, Default on options structs
[ ] Ownership: no borrow-checker clones, Rc/Arc justified
[ ] Modules: non-leaf modules are thin, leaf modules named after their item, no preludes
[ ] Naming: std conventions, accessors, constructors, lifetimes, no abbreviations
[ ] Docs say something the code does not
[ ] Readability: explicit types where unclear, short chains, useful messages
```
