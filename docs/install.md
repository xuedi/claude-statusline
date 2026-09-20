# Install

The standard install is [from source](../README.md#install). This page covers the
prebuilt packages attached to every [tagged release](https://github.com/xuedi/claude-statusline/releases).

All binaries are static and dependency-free. The `.deb`, `.rpm` and tarball are
built for both `x86_64` and `aarch64`; the Arch package is `x86_64` only. The
examples below use `x86_64` - for arm64, swap in the naming that format uses:
`arm64` for the `.deb`, `aarch64` for the `.rpm` and the tarball. Each release also
carries a `SHA256SUMS` file.

## Debian, Ubuntu, Mint

```bash
VERSION=1.0.0
curl -LO https://github.com/xuedi/claude-statusline/releases/download/v$VERSION/claude-statusline_$VERSION-1_amd64.deb
sudo dpkg -i claude-statusline_$VERSION-1_amd64.deb
```

## Fedora, RHEL, openSUSE

```bash
VERSION=1.0.0
curl -LO https://github.com/xuedi/claude-statusline/releases/download/v$VERSION/claude-statusline-$VERSION-1.x86_64.rpm
sudo rpm -i claude-statusline-$VERSION-1.x86_64.rpm
```

## Arch Linux

Arch packaging is `x86_64` only - on arm64, use the
[tarball](#any-other-distro). Prebuilt package straight from the release:

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

## Any other distro

```bash
VERSION=1.0.0
curl -L https://github.com/xuedi/claude-statusline/releases/download/v$VERSION/claude-statusline-$VERSION-x86_64-linux.tar.gz | tar xz
install -Dm755 claude-statusline ~/.claude/claude-statusline
```

The binary is statically linked against musl, so it runs on glibc and musl systems
alike with no runtime dependencies.

Once installed, point Claude Code at it - see
[Configure Claude Code](../README.md#configure-claude-code).
