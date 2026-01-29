# COSMIC Applet: Toggle Keyboard

A simple COSMIC desktop applet to toggle the on-screen keyboard (`wvkbd-mobintl`).

Prerequisite

Make sure the on-screen keyboard program "wvkbd" (sometimes packaged as `wvkbd-mobintl`) is installed on your system before building or installing this applet.

On Debian/Ubuntu systems you can install it with:

```bash
sudo apt install wvkbd
```

On Arch Linux you can install it with:

```bash
sudo pacman -S wvkbd
```

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
