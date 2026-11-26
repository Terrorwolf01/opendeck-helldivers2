# Security Review

| | |
|---|---|
| **Date** | 2026-11-26 |
| **Reviewed version** | `develop` after v3.1.1 (manifest version 3.1.1.0) |
| **Reviewer** | Terrorwolf01 (maintainer) |
| **Next review** | Before the next major release, at the latest by 2027-11 |

This document records the security review of the Helldivers 2 OpenAction plugin. It describes
what the plugin must protect, where its security boundaries are, which threats were considered
and how they are countered, what the review found, and which risks remain.

## 1. Scope

In scope:

- the Rust plugin (`src/`), which receives events from OpenDeck and simulates key presses,
- the property inspector (`pi/`), the settings page shown inside OpenDeck,
- the stratagem data (`pi/src/lib/stratagems.json`) and icons (`assets/`),
- the build, CI and release pipeline (`.github/workflows/`, `build.sh`, `package.sh`),
- the repository itself (secrets, access, branch rules).

Out of scope: OpenDeck / Tacto, the operating system, Helldivers 2, and GitHub itself. They are
treated as trusted (see section 3).

Tools used: CodeQL (Rust, JavaScript/TypeScript, GitHub Actions; `security-extended`), Clippy
(`-D warnings`), Miri, `cargo test`, svelte-check, osv-scanner (`Cargo.lock`), `deno audit`
(`pi/deno.lock`), OpenSSF Scorecard, and a pattern scan of the full Git history for secrets.

## 2. Security Requirements

The plugin and its distribution must ensure that:

1. **R1 - Only configured input:** a key press on the deck only ever sends the modifier and the
   direction keys the user configured, in the sequence of the selected stratagem - nothing else.
2. **R2 - Fail safe on bad input:** missing, unknown or malformed settings never crash the
   plugin and never send unintended input; they are ignored or fall back to documented
   defaults.
3. **R3 - No code execution from data:** settings and stratagem data are treated as data only;
   they can't cause code execution, file access outside the plugin folder, or script execution
   in the property inspector.
4. **R4 - Authentic releases:** users can verify that a release was built by this repository's
   workflow from a specific commit, and published releases can't be altered afterwards.
5. **R5 - Protected source:** changes reach `main` (the release branch) only through reviewed,
   automatically checked pull requests; the repository contains no credentials.
6. **R6 - Known vulnerabilities are handled:** vulnerabilities in the code or its dependencies
   are detected and can be reported privately and fixed quickly.

The plugin handles no passwords, personal data or secrets, has no network service of its own,
and uses no cryptography for security purposes (see section 5.4).

## 3. Security Boundary

```
 Trusted                                   │  Boundary / checked input
───────────────────────────────────────────┼───────────────────────────────────────────
 OpenDeck / Tacto (local process)  ── WebSocket (localhost, no TLS) ──▶ plugin
   • key events, action settings,          │  settings parsed into fixed types (serde)
     global settings from the PI           │
 Built-in data: stratagems.json, icons     │  embedded at compile time, covered by tests
 Operating system (X11 / Windows input)  ◀── simulated key events (rdev)
───────────────────────────────────────────┼───────────────────────────────────────────
 GitHub (repo, Actions, releases)          │  PRs from contributors and Renovate
                                           │  third-party GitHub Actions
                                           │  crates.io / npm dependencies
```

- **Plugin ↔ OpenDeck:** the plugin is a client of OpenDeck's local WebSocket. OpenDeck is
  trusted: whoever controls OpenDeck controls which keys the user has configured. Data from it
  (settings) is still validated, because it is user-editable and may be stale or malformed.
- **Plugin ↔ operating system:** the plugin's only output is simulated key events. This is the
  plugin's purpose and its most sensitive capability (R1).
- **Property inspector ↔ OpenDeck:** the PI runs inside OpenDeck and only renders the plugin's
  own data; it loads no remote content.
- **Repository ↔ outside world:** contributions, third-party actions and dependencies are
  untrusted until checked by CI, CodeQL, lock files and review.

## 4. Threats and Countermeasures

| # | Threat | Countermeasure | Req. |
|---|---|---|---|
| T1 | Memory corruption (buffer overflow, use-after-free) | Safe Rust, no `unsafe`; no C/C++ code; tests run under Miri in CI | R2, R3 |
| T2 | Crash or panic from malformed settings | Typed `serde` structs with `#[serde(default)]`; invalid values fall back to defaults; no `unwrap()`/`expect()` on runtime input; integer overflow checks in tests | R2 |
| T3 | Unintended key input through crafted key names | Key names are parsed into `rdev`'s closed `Key` set (allowlist); anything else is rejected and nothing is sent; a test checks every PI choice is valid | R1, R2 |
| T4 | Unknown stratagem IDs | IDs are only looked up in the built-in list; unknown IDs do nothing (tested) | R1, R2 |
| T5 | Path traversal via icon paths | Icon paths come only from the built-in data, never from settings; a test checks every icon exists | R3 |
| T6 | XSS / HTML injection in the property inspector | Svelte escapes all output; no `{@html}`; TypeScript strict mode and svelte-check | R3 |
| T7 | Vulnerable dependencies | Renovate updates, Dependabot alerts, osv-scanner / `deno audit`; lock files enforced in CI (`--locked`, `--frozen`) | R6 |
| T8 | Compromised or changed GitHub Action | All actions pinned by commit SHA; Renovate updates the pins | R4, R5 |
| T9 | Over-privileged CI token | Workflows default to `contents: read`; write permissions only per job where needed | R4, R5 |
| T10 | Script injection in workflows | No untrusted input (PR titles, branch names) in `run:` steps; tag name passed via `$GITHUB_REF_NAME`; no `pull_request_target`; CodeQL `actions` queries | R5 |
| T11 | Tampered or substituted release | Sigstore build provenance attestation attached to each release (`gh attestation verify`); immutable releases; release built only by CI from a tag on `main` | R4 |
| T12 | Unreviewed or failing code reaching a release | `main` ruleset: PRs required, required status checks (CI, CodeQL, Miri), no force pushes or deletion, empty bypass list | R5 |
| T13 | Leaked credentials | No secrets used (only the ephemeral `GITHUB_TOKEN`); history scanned; GitHub secret scanning with push protection | R5 |
| T14 | Account takeover of the maintainer | GitHub 2FA with cryptographic methods (no SMS-only) | R5 |
| T15 | Vulnerabilities reported publicly before a fix | `SECURITY.md` with private vulnerability reporting; acknowledgement within 7 days; coordinated disclosure via GitHub Security Advisory | R6 |

## 5. Detailed Notes

### 5.1 Input handling

Settings arrive from OpenDeck as JSON and are deserialised into `ActionSettings`
(`stratagem`, `delay`) and `GlobalSettings` (`modifier`, `up`, `down`, `left`, `right`). Every
field is optional with a documented default. Key names are wrapped as a JSON string and parsed
into `rdev::Key`; only unit variants of that enum parse, so arbitrary values (including quotes
or other JSON) are rejected and logged instead of being sent.

### 5.2 Output

On a key press the plugin holds the modifier, sends the stratagem's sequence of direction keys
and releases the modifier, with a configurable delay after each event. It sends nothing when no
stratagem is selected or a key name is invalid.

### 5.3 Logging

The plugin logs to stdout (captured by OpenDeck) and to a log file next to its binary. Logs
contain event metadata, key names and stratagem IDs - no secrets or personal data.

### 5.4 Cryptography

The plugin uses no cryptography for security. Its WebSocket connection to OpenDeck is local and
unencrypted. The only cryptographic primitive in the dependency tree is SHA-1 in the WebSocket
handshake, mandated by RFC 6455 as a protocol check (not a security mechanism); random values
for the handshake and frame masks come from `rand`'s CSPRNG.

### 5.5 Hardening

- Safe Rust only (no `unsafe`): memory safety and bounds checks are enforced by the compiler.
- Integer overflow checks are enabled in release builds (`overflow-checks = true` in
  `Cargo.toml`), so an overflow stops the plugin instead of silently wrapping.
- Release binaries are built as position-independent executables (ASLR), with full RELRO
  (`BIND_NOW`) and a non-executable stack, verified with `readelf`.
- Stack probes protect against stack-clash attacks (Rust default on x86-64).
- Tests run with debug assertions and integer overflow checks enabled, and under Miri to detect
  undefined behaviour.
- The property inspector uses Svelte's automatic output escaping and TypeScript strict mode.

## 6. Findings

Found during this review:

| # | Finding | Severity | Status |
|---|---|---|---|
| F1 | GitHub Actions referenced by tag instead of commit SHA | Medium | Fixed - all actions pinned by SHA |
| F2 | Workflow-wide `contents: write` permission | Medium | Fixed - read-only by default, write per job |
| F3 | Releases unsigned, no provenance | Medium | Fixed - Sigstore attestation from the next release on |
| F4 | Private vulnerability reporting linked in `SECURITY.md` but disabled | Medium | Fixed - enabled |
| F5 | No CI: format, lints and tests not enforced before merge | Low | Fixed - CI, CodeQL and Miri required on `main` |
| F6 | Clippy warnings (`needless_return`) | Low | Fixed |
| F7 | Property inspector showed a default delay (20 ms) different from the plugin's (30 ms) | Low | Fixed - test guards against it |
| F8 | `release.yml` inserts step outputs directly with `${{ }}` in two `run:` steps (lines 60, 103) | Low | Open - values come from the repository / an action, not from untrusted input; recommended to pass them via `env:` |
| F9 | The plugin accepts any `rdev` key name from settings, not only the 12 keys the PI offers | Low | Accepted - still an allowlist of single keys; settings are only editable through OpenDeck, which is trusted |
| F10 | The delay setting has no upper bound; a very large value blocks that key's input for that long | Low | Accepted - only affects the user's own configuration; could be capped in the plugin |

## 7. Residual Risks

- **OpenDeck is trusted completely.** Anyone who can control OpenDeck or its settings on the
  user's machine can trigger the plugin's key input. This is inherent to a deck plugin.
- **Simulated input goes to the focused window.** If another window has focus, the key sequence
  is sent there. The keys are limited to the modifier and direction keys (F9), so the impact is
  small.
- **Upstream dependencies.** `rdev`, `openaction`, `tokio` and the PI's npm packages are trusted
  as long as no advisory exists; updates are reviewed through Renovate pull requests.
- **Nightly toolchain for Miri.** The Miri check depends on Rust nightly and can fail for
  reasons unrelated to the plugin; it does not affect release builds, which use stable Rust.

## 8. References

- [SECURITY.md](../SECURITY.md) - supported versions and vulnerability reporting
- [CONTRIBUTING.md](../CONTRIBUTING.md) - contribution requirements and checks
- [.github/workflows/](../.github/workflows) - CI, CodeQL, release and sync workflows
- [OpenSSF Scorecard](https://scorecard.dev/viewer/?uri=github.com/Terrorwolf01/opendeck-helldivers2)
