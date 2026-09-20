# claude-statusline

A fast, minimal statusline for [Claude Code](https://claude.com/claude-code).

[![CI](https://github.com/xuedi/claude-statusline/actions/workflows/ci.yml/badge.svg)](https://github.com/xuedi/claude-statusline/actions/workflows/ci.yml)
[![Release](https://github.com/xuedi/claude-statusline/actions/workflows/release.yml/badge.svg)](https://github.com/xuedi/claude-statusline/actions/workflows/release.yml)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![Made with Braille](https://img.shields.io/badge/progress_bars-%E2%A3%BF%E2%A3%BF%E2%A3%BF%E2%A3%B7-brightgreen)](#what-it-shows)
[![Rust](https://img.shields.io/badge/Rust-1.85+-orange?logo=rust&logoColor=white)](https://www.rust-lang.org)

```
MeetAgain | Claude Sonnet 4.6 1M | Effort: high | git@main (+12 -3) | 250k/1m [⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀] | HourlyReset: 3% @03:30 | WeeklyReset: 11% @May 18, 22:00
```

## What it shows

- **Project** - the project folder name, skipping role subfolders (`FileFin/app` shows `FileFin`)
- **Model** - active Claude model, with the context window summarized (`Sonnet 4.6 1M`)
- **Git** - branch and unstaged diff size in the current working directory
- **Tokens** - used/total of the context window plus a 20-cell braille progress bar
- **Effort** - the `effortLevel` setting (`low`, `med`, `high`)
- **Rate limits** - 5-hour and 7-day usage percentages with reset times

If Claude Code does not pass rate-limit data in the statusline payload, the binary
fetches it from Anthropic's `/api/oauth/usage` endpoint and caches the result for 60s.

## Install

Every tagged release ships static, dependency-free binaries for `x86_64` and
`aarch64` on the [releases page](https://github.com/xuedi/claude-statusline/releases),
packaged for the common distro families.

### Debian, Ubuntu, Mint

```bash
VERSION=1.0.0
curl -LO https://github.com/xuedi/claude-statusline/releases/download/v$VERSION/claude-statusline_$VERSION-1_amd64.deb
sudo dpkg -i claude-statusline_$VERSION-1_amd64.deb
```

### Fedora, RHEL, openSUSE

```bash
VERSION=1.0.0
curl -LO https://github.com/xuedi/claude-statusline/releases/download/v$VERSION/claude-statusline-$VERSION-1.x86_64.rpm
sudo rpm -i claude-statusline-$VERSION-1.x86_64.rpm
```

### Arch Linux

Install the prebuilt package straight from the release:

```bash
VERSION=1.0.0
curl -LO https://github.com/xuedi/claude-statusline/releases/download/v$VERSION/claude-statusline-bin-$VERSION-1-x86_64.pkg.tar.zst
sudo pacman -U claude-statusline-bin-$VERSION-1-x86_64.pkg.tar.zst
```

Or build it with the `PKGBUILD` that ships with every release:

```bash
curl -LO https://github.com/xuedi/claude-statusline/releases/latest/download/PKGBUILD
makepkg -si
```

### Any other distro

```bash
VERSION=1.0.0
curl -L https://github.com/xuedi/claude-statusline/releases/download/v$VERSION/claude-statusline-$VERSION-x86_64-linux.tar.gz | tar xz
install -Dm755 claude-statusline ~/.claude/claude-statusline
```

`aarch64` builds of every package are attached to the same release. The binary is
statically linked against musl, so it runs on glibc and musl systems alike with no
runtime dependencies. Each release also carries a `SHA256SUMS` file.

### From source

```bash
git clone https://github.com/xuedi/claude-statusline
cd claude-statusline
just install        # builds release and copies to ~/.claude/claude-statusline
```

Or directly with cargo:

```bash
cargo install --path .
```

## Configure Claude Code

Add to `~/.claude/settings.json`:

```json
{
  "statusLine": {
    "type": "command",
    "command": "~/.claude/claude-statusline"
  }
}
```

## Environment variables

| Variable                   | Purpose                                                            |
|----------------------------|--------------------------------------------------------------------|
| `HOME`                     | Required - used to locate `~/.claude/`                             |
| `CLAUDE_CONFIG_DIR`        | Overrides `~/.claude` when looking up credentials and settings     |
| `CLAUDE_CODE_EFFORT_LEVEL` | Fallback effort level when `settings.json` does not set one        |

## Security note

This binary reads `~/.claude/.credentials.json` to fetch usage data from Anthropic's
`/api/oauth/usage` endpoint **only when Claude Code does not supply rate-limit data
in the statusline payload**. The token never leaves the binary; results are cached
under `/tmp/claude/`.

If this bothers you, the binary still produces useful output (model, git, tokens,
effort) without it - those segments do not touch credentials.

## Development

```bash
just            # list recipes
just check      # fmt-check + clippy + test - run this before pushing
just test       # cargo test
just demo       # pipe a sample payload through the binary
```

### Releasing

`.github/workflows/release.yml` builds the binaries and every package on each push
to `main` and keeps them as workflow artifacts for 14 days. Pushing a `v*` tag runs
the same build and publishes the artifacts as a GitHub release, with a checksum file
and an AUR-ready `PKGBUILD`. The tag has to match the version in `Cargo.toml` or the
workflow fails early.

```bash
git tag v0.2.0 && git push origin v0.2.0
```

### Adding a segment

1. Write `src/segments/<name>.rs` exposing `pub fn render(input: &Input) -> ...`
2. Wire it into `segments::all()` in `src/segments/mod.rs`
3. Add a unit test in the same module

There is no `Segment` trait - five hand-listed segments do not need the indirection.

## Contributing

PRs welcome. Run `just check` before pushing. Keep the binary single-purpose; no new
mandatory dependencies without discussion.

## License

[EUPL-1.2](LICENSE)
