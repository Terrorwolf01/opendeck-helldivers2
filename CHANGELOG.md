# Changelog

All notable changes to this plugin are documented here.
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Releases are now signed with build provenance; see the README for how to verify a download.

### Changed

- Updated the property inspector's dependencies, including the move to SvelteKit 3.
  No change to settings or behaviour.
- The default delay between simulated key events is now 30 ms (was 20 ms).

## [3.1.1] - 2026-10-26

### Fixed

- Scope of Authority: the AMW-20 APEX was shown as "NOVA" and is now named correctly; its icons
  were added.

## [3.1.0] - 2026-10-26

### Added

- New action **Scope of Authority** with the stratagems of that warbond.

## [3.0.0] - 2026-10-18

### Changed

- **Breaking:** the ID of the **General** action changed from `at.terrorwolf.helldivers.General`
  to `at.terrorwolf.helldivers2.General`, consistent with all other actions.

### Upgrade notes

- Keys using the **General** action stop working after the update. Remove them and add the
  General action again.

## [2.0.0] - 2026-10-07

### Changed

- **Breaking:** the IDs of all actions except **General** changed from
  `at.terrorwolf.helldivers.<Group>` to `at.terrorwolf.helldivers2.<Group>`, to match the plugin's
  ID.

### Upgrade notes

- All keys except those using **General** stop working after the update. Remove them and add the
  actions again.

## [1.0.0] - 2026-10-05

### Added

- First release: one action per stratagem group (General, Patriotic Administration Center,
  Orbital Cannons, Hangar, Bridge, Engineering Bay, Robotic Workshop and the warbonds), with
  configurable modifier and direction keys and input delay.

[Unreleased]: https://github.com/Terrorwolf01/opendeck-helldivers2/compare/v3.1.1...develop
[3.1.1]: https://github.com/Terrorwolf01/opendeck-helldivers2/compare/v3.1.0...v3.1.1
[3.1.0]: https://github.com/Terrorwolf01/opendeck-helldivers2/compare/v3.0.0...v3.1.0
[3.0.0]: https://github.com/Terrorwolf01/opendeck-helldivers2/compare/v2.0.0...v3.0.0
[2.0.0]: https://github.com/Terrorwolf01/opendeck-helldivers2/compare/v1.0.0...v2.0.0
[1.0.0]: https://github.com/Terrorwolf01/opendeck-helldivers2/releases/tag/v1.0.0
