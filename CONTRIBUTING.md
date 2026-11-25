# Contributing

Thanks for your interest in improving the Helldivers2 OpenAction plugin! Bug reports,
missing stratagems and code changes are all welcome.

## Reporting Bugs and Requesting Features

Please use [GitHub Issues](https://github.com/Terrorwolf01/opendeck-helldivers2/issues).
For bugs, include:

- the plugin version and your OpenDeck / Tacto version,
- your operating system,
- what you did, what you expected, and what happened instead,
- relevant log output, if any.

**Security vulnerabilities** must not be reported through public issues - see
[SECURITY.md](SECURITY.md) instead.

## Submitting Changes

1. Fork the repository and create a branch from **`develop`**.
2. Make your changes (see the checks below).
3. Open a pull request **against `develop`**, not `main`, and fill in the pull request template.
   Describe what you changed and why.

**Pull requests from non-maintainers that don't use the pull request template will be closed
without review.**

`main` only receives changes from `develop` via pull request and is what releases are built from.
All pull requests must pass CI (build, lints, tests) and CodeQL before they can be merged.

## Requirements for Contributions

A pull request can be merged when it meets these requirements:

- **It passes CI and CodeQL.** Formatting, lints, tests and the property inspector build must all
  succeed (see [Checks](#checks)).
- **Rust code follows the coding standard:** the
  [Rust Style Guide](https://doc.rust-lang.org/stable/style-guide/), enforced by `rustfmt` with the
  repository's `rustfmt.toml`, and is free of
  [Clippy](https://doc.rust-lang.org/stable/clippy/) warnings (`-D warnings`).
- **Property inspector code** (TypeScript / Svelte) is formatted with
  [Prettier](https://prettier.io) using the configuration in `pi/package.json`, and passes
  `svelte-check` without errors.
- **It includes tests** for new functionality or bug fixes where practical (see [Tests](#tests)).
- **It is focused:** one topic per pull request, with a description of what changed and why.
- **Stratagem data is correct:** input sequences have been checked in-game
  (see [Adding Stratagems](#adding-stratagems)).
- **It is licensed under the project's** [MIT License](LICENSE).

## Development Setup

The build scripts are Bash scripts and are developed on Linux. On Windows, use WSL.

You need:

- [Rust](https://rustup.rs) (stable)
- [Deno](https://deno.com) 2.x for the property inspector in `pi/`
- `jq` (used by `build.sh` to read `assets/manifest.json`)
- on Linux, the X11 development libraries (`xorg-dev`, `pkg-config`)
- for the Windows binary: the Rust target and the MinGW cross-compiler

```sh
rustup target add x86_64-pc-windows-gnu
sudo apt-get install mingw-w64   # or your distribution's equivalent
```

Install the property inspector's dependencies once:

```sh
cd pi && deno install
```

Then build and install the plugin into your local OpenDeck:

```sh
./build.sh            # native OpenDeck
./build.sh --flatpak  # OpenDeck Flatpak
```

By default `build.sh` builds every target listed in `assets/manifest.json`. To build only for
Linux (no MinGW needed), pass the target explicitly:

```sh
./build.sh --target x86_64-unknown-linux-gnu
```

## Checks

Please run these before opening a pull request - CI runs the same checks:

```sh
# Rust
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked

# Property inspector
cd pi
deno install --frozen
deno task check
deno task build
```

## Adding Stratagems

Stratagems are defined once in `pi/src/lib/stratagems.json`, which is used by both the plugin and
the property inspector.

- **Existing group:** add an entry with `id`, `title`, `icon` and `seq` (the input sequence using
  `up`, `down`, `left`, `right`) to the matching group, and add the icon as a PNG under
  `assets/icon/HD2-<Group>/`. The `icon` value is that path without `.png`.
- **New group (e.g. a new warbond):** additionally add a matching action (`at.terrorwolf.helldivers2.<Group>`) to
  `assets/manifest.json`.

Please double-check input sequences in-game - a wrong sequence is the most common bug.

## Tests

New functionality should come with tests where practical, and bug fixes should add a test that
would have caught the bug. Tests live next to the code in `#[cfg(test)]` modules and run with
`cargo test`.

## Code Style

- Rust: formatted with `rustfmt` using the repository's `rustfmt.toml` (tabs).
- Property inspector: formatted with Prettier (`deno task format` in `pi/`).

## License

By contributing, you agree that your contributions are licensed under the project's
[MIT License](LICENSE).
