# OpenAction Helldivers 2 Plugin

An OpenAction ([OpenDeck](https://github.com/nekename/OpenDeck) / [Tacto](https://tacto.rivul.us))
plugin for Helldivers 2: put stratagems on your deck's keys, and a key press enters the full
stratagem input sequence in-game for you.

[![OpenSSF Best Practices](https://www.bestpractices.dev/projects/17120/badge)](https://www.bestpractices.dev/projects/17120)

## Requirements

- OpenDeck 6.9 or newer (or Tacto)
- Windows 10 or newer, or Linux

## Installation

### Recommended: via OpenDeck

Install the plugin directly from OpenDeck's plugin manager by searching for **Helldivers2**.
This is the recommended way, as OpenDeck takes care of installing the right version for your
system.

### Manual installation

1. Download the latest `.streamDeckPlugin` file from the
   [Releases page](https://github.com/Terrorwolf01/opendeck-helldivers2/releases).
2. Install it through OpenDeck's plugin manager (Install from file).

Each release is signed with build provenance. To check a manual download:

```sh
gh attestation verify at.terrorwolf.helldivers2.sdPlugin.streamDeckPlugin \
  --repo Terrorwolf01/opendeck-helldivers2
```

## Usage

The plugin adds one action per stratagem group, matching the in-game categories and warbonds:

General, Patriotic Administration Center, Orbital Cannons, Hangar, Bridge, Engineering Bay,
Robotic Workshop, Chemical Agent, Urban Legends, Servants of Freedom, Borderline Justice,
Masters Of Ceremony, Force Of Law, Control Group, Dust Devils, Python Commandos,
Redacted Regiment, Siege Breakers, Entrenched Division, Exo Experts, Castellan's Creed,
Scope Of Authority.

1. Drag an action onto a key.
2. In the action's settings, pick a stratagem from the list (type to filter).
3. Press the key in-game: the plugin holds the stratagem modifier, enters the sequence, and
   releases the modifier.

A key without a selected stratagem shows a placeholder icon and does nothing.

### Settings

**Per key:**

| Setting    | Default | Description                                                                  |
|------------|---------|------------------------------------------------------------------------------|
| Stratagem  | -       | The stratagem this key enters.                                               |
| Delay (ms) | 30      | Pause after each simulated key event. Increase it if the game misses inputs. |

**Global (shared by all keys)** - these must match your in-game keybindings:

| Setting  | Default     |
|----------|-------------|
| Modifier | Left Ctrl   |
| Up       | Up Arrow    |
| Down     | Down Arrow  |
| Left     | Left Arrow  |
| Right    | Right Arrow |

Available choices are Left/Right Ctrl, Home, Left Alt, the arrow keys and W/A/S/D.

## Troubleshooting

- **Nothing happens / the stratagem menu doesn't open:** check that the modifier and direction
  keys in the settings match your in-game keybindings.
- **Some inputs are missed:** raise the delay, e.g. to 30-50 ms.
- **A stratagem is missing or has a wrong sequence:** please
  [open an issue](https://github.com/Terrorwolf01/opendeck-helldivers2/issues).

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). Security issues: see [SECURITY.md](SECURITY.md).

## License

[MIT](LICENSE)
