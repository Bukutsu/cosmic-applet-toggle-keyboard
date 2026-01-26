# COSMIC Applet: Toggle Keyboard

A simple COSMIC desktop applet to toggle the on-screen keyboard (`wvkbd-mobintl`).

## Build

```bash
cargo build --release
```

## Install

```bash
install -Dm755 target/release/cosmic-applet-toggle-keyboard -t ~/.local/bin/
```

## Applet Configuration

Create a file `~/.local/share/applications/cosmic-applet-toggle-keyboard.desktop` with the following content:

```ini
[Desktop Entry]
Name=Toggle Keyboard
Type=Application
Exec=cosmic-applet-toggle-keyboard
Terminal=false
Categories=COSMIC;
Keywords=COSMIC;Iced;
Icon=input-keyboard-symbolic
StartupNotify=true
X-CosmicApplet=true
X-CosmicHoverPopup=Auto
X-OverflowPriority=10
```
