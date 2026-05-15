# CLAUDE.md

## What this is

A tiny Rust CLI that reads the Claude Code statusline JSON payload from stdin and
prints one line: model, git branch and diff, token usage with a braille bar, effort
level, and 5h/7d rate limits. Around 500 lines of code total, library plus a thin
binary.

## Run it

```bash
just check     # cargo fmt --check + clippy -D warnings + cargo test
```

Run that before reporting work done.

## Code style

- ASCII `-` only - never em dash (`—`) in code, prose, or mermaid labels
- Comments are scarce. Allowed: doc comments on public items, `// WHY:` comments for
  non-obvious workarounds, and AAA markers in tests when they help. No narrative
  comments that re-state what the code does.
- Use `use` imports - never fully-qualified inline (`std::io::Read`, not
  `std::io::stdin().read_to_string(...)` with the path inline)
- Format on save with `cargo fmt`. Clippy must stay clean with `-D warnings`.
- `.unwrap()` is for tests. Production code returns `Option` / `Result` and
  collapses to the documented fallback at the top level.

## Layout

| File                          | What lives there                                          |
|-------------------------------|-----------------------------------------------------------|
| src/main.rs                   | Reads stdin, calls `render`, prints                       |
| src/lib.rs                    | `render()` orchestrator and module re-exports             |
| src/input.rs                  | Serde input types for the Claude statusline payload       |
| src/error.rs                  | Hand-rolled `Error` enum + `Result` alias                 |
| src/bar.rs                    | Braille progress bar                                      |
| src/cache.rs                  | On-disk JSON cache under /tmp/claude/                     |
| src/api.rs                    | `/api/oauth/usage` fetcher                                |
| src/time.rs                   | Epoch + ISO datetime helpers                              |
| src/segments/mod.rs           | `all(&Input)` flat dispatcher                             |
| src/segments/model.rs         | Model name segment                                        |
| src/segments/git.rs           | Git branch + numstat segment                              |
| src/segments/tokens.rs        | Token bar segment                                         |
| src/segments/effort.rs        | Effort level segment                                      |
| src/segments/rate_limits.rs   | Rate-limit segments (builtin + API path)                  |
| tests/render.rs               | Integration test feeding fixture payloads to `render()`   |

## Adding a segment

1. Write `src/segments/<name>.rs` exposing `pub fn render(input: &Input) -> ...`
2. Wire it into `segments::all()` in `src/segments/mod.rs`
3. Add a unit test in the same module

The "do we need a `Segment` trait" question is settled: no. Five hand-listed
segments do not need the indirection. Do not propose one.

## Test coverage

Covered:

- `bar::render` - bar widths at 0%, 50%, 100%, partial steps, clamping
- `segments::tokens::format_tokens` - all the rounding and unit boundaries
- `segments::model::format_model` - context-paren stripping, edge cases
- `time::iso_to_epoch`, `time::epoch_from_value` - happy path + rejection
- `cache::short_hash` - determinism, differentiation, hex shape
- `lib::render` - empty input, malformed JSON, full payload (integration test)

Not covered (intentional):

- `segments::git` - shells out to `git`; would need a fixture repo or a runner trait
- `api::fetch_usage` - real network call; mocking would require trait injection
  through the whole rate-limits path
- `cache::cache_is_fresh`, `cache::load_json_cache` - filesystem I/O

If you find yourself wanting to test these, prefer adding a smoke test over
introducing a mocking layer.

## Git workflow

Never create commits yourself. `git status` and `git diff` are fine for inspection.
Let the user commit.

## Planning

If asked for a plan, write it to `.claude/plans/YYYY-MM-DD_<slug>.md` (create the
directory on demand). Freeform is fine for a project this size - no template.

## See also

- `.claude/conventions.md` - the rationale for why a few non-obvious choices were
  made (no `Segment` trait, hand-rolled `Error` enum, etc.)
