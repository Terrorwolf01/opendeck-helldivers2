<!--
Thanks for contributing! Please open pull requests against `develop`, not `main`.
See CONTRIBUTING.md for the full requirements.

IMPORTANT: Fill in this template - don't delete it. Pull requests from non-maintainers that
don't use this template will be closed without review.
-->

## What does this change?

<!-- Describe what you changed and why. -->

Closes #<!-- issue number, if any -->

## Type of change

- [ ] Bug fix
- [ ] New or corrected stratagem / warbond
- [ ] New feature or setting
- [ ] Documentation
- [ ] Other (dependencies, CI, refactoring)

## Checklist

- [ ] This pull request targets `develop`.
- [ ] Tests were added or updated for new functionality or bug fixes (or explain below why not).
- [ ] `cargo fmt --check`, `cargo clippy --locked --all-targets -- -D warnings` and `cargo test --locked` pass.
- [ ] `deno task check` and `deno task build` pass in `pi/` (if the property inspector changed).
- [ ] New or changed stratagem input sequences were checked in-game.
- [ ] `CHANGELOG.md` has an entry under `[Unreleased]` for user-visible changes.

## Notes

<!-- Anything reviewers should know: why tests weren't added, screenshots, open questions. -->
