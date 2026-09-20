# Contributing

PRs welcome. Keep the binary single-purpose, and run `just check` before pushing.
See [architecture](architecture.md) for how the pieces fit together.

## Development loop

```bash
just            # list recipes
just check      # fmt-check + clippy + test - run this before pushing
just test       # cargo test
just demo       # pipe a sample payload through the binary
```

## Branching

`main` is protected: no direct pushes, no force-pushes, no deletions, CI `check`
must be green, and history stays linear.

- Day-to-day work goes on `development`, or on a short-lived topic branch cut from
  it (`feat/...`, `fix/...`) that merges back into `development`.
- Ship by opening a pull request from `development` into `main`.
- Merge with squash or rebase - merge commits are disabled.
- Tag releases on `main` once the PR has landed.

```bash
git switch development
git pull --rebase
# ... work, just check ...
git push
gh pr create --base main --head development
```

## Code style

- ASCII `-` only - never em dash (`—`) in code, prose, or mermaid labels
- Comments are scarce. Allowed: doc comments on public items, `// WHY:` comments for
  non-obvious workarounds, and AAA markers in tests when they help. No narrative
  comments that re-state what the code does.
- Use `use` imports - never fully-qualified inline (`std::io::Read`, not
  `std::io::stdin().read_to_string(...)` with the path inline)
- Format with `cargo fmt`. Clippy stays clean under `-D warnings`.
- `.unwrap()` is for tests. Production code returns `Option` / `Result` and
  collapses to the documented fallback at the top level.
- Mermaid for any flow, sequence, state machine or hierarchy. Never ASCII
  box-drawing.

## Adding a segment

1. Write `src/segments/<name>.rs` exposing `pub fn render(input: &Input) -> ...`
2. Wire it into `segments::all()` in `src/segments/mod.rs`
3. Add a unit test in the same module

There is no `Segment` trait, and it is not up for discussion - see
[architecture](architecture.md#no-segment-trait).

## Adding a dependency

The bar is high. A PR adding one must answer in its description:

1. What problem does this solve that the standard library or an existing dep cannot?
2. What is the maintenance reputation of the crate (rough star count, last release,
   transitive dep count)?
3. Is there a smaller alternative (a 30-line hand-rolled helper, a feature flag on
   an existing dep)?

## Tests

Covered:

- `bar::render` - bar widths at 0%, 50%, 100%, partial steps, clamping
- `segments::tokens::format_tokens` - all the rounding and unit boundaries
- `segments::model::format_model` - context-paren stripping, edge cases
- `segments::project::project_name` - generic-subfolder walk-up, list invariants
- `time::iso_to_epoch`, `time::epoch_from_value` - happy path + rejection
- `cache::short_hash` - determinism, differentiation, hex shape
- `lib::render` - empty input, malformed JSON, full payload (integration test)

Not covered, intentionally:

- `segments::git` - shells out to `git`; would need a fixture repo or a runner trait
- `api::fetch_usage` - real network call; mocking would require trait injection
  through the whole rate-limits path
- `cache::cache_is_fresh`, `cache::load_json_cache` - filesystem I/O

If you find yourself wanting to test these, prefer a smoke test over introducing a
mocking layer.

## Releasing

`.github/workflows/release.yml` builds the binaries and every distro package on each
push to `main` and keeps them as workflow artifacts for 14 days. Pushing a `v*` tag
runs the same build and publishes the artifacts as a GitHub release, with a checksum
file and an AUR-ready `PKGBUILD`. The tag has to match the version in `Cargo.toml` or
the workflow fails early.

```bash
git tag v0.2.0 && git push origin v0.2.0
```
