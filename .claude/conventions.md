# Conventions

Things that did not fit in the top-level `CLAUDE.md`'s "Code style" bullets but are
worth pinning so the next session does not relitigate them.

## Why no `Segment` trait

The dispatcher in `src/segments/mod.rs` is a flat function that hand-lists each
segment in render order. Five segments do not benefit from a trait registry:

- The order matters and is easier to read as a `Vec::push` sequence than as a list
  of trait implementations.
- Segments return different shapes - `String`, `Option<String>`, `Vec<String>` -
  forcing them through one method signature would lose information.
- A trait registry would invite "dynamic registration", "feature flags per
  segment", and other features that the binary does not need.

If the segment count grows past ~10 or segments start needing shared lifecycle
hooks, revisit. Until then: flat dispatcher.

## Why a hand-rolled `Error` enum

`anyhow` and `thiserror` are both fine crates, but this binary has roughly five
error sites and the user-facing behaviour is "print something or print 'Claude'"
- errors collapse to a fallback at the top level. They never reach output.

`src/error.rs` is 40 lines and avoids two dependencies. If error handling ever
escapes the binary boundary (a published `lib.rs` API consumed by another crate,
say), revisit.

## When to add a dependency

High bar. The current set is `serde`, `serde_json`, `chrono`, `ureq` plus
`pretty_assertions` for tests. Each addition pays compile time, audit surface, and
release-binary size forever.

A PR adding a dependency must answer in its description:

1. What problem does this solve that the standard library or an existing dep cannot?
2. What is the maintenance reputation of the crate (rough star count, last release,
   transitive dep count)?
3. Is there a smaller alternative (a 30-line hand-rolled helper, a feature flag on
   an existing dep)?

## Diagrams

Mermaid for any flow, sequence, state machine, hierarchy. Never ASCII box-drawing.
Mermaid label text follows the same dash rule as the rest of the project: ASCII
`-`, never `—`.
