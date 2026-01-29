GitHub Copilot Chat Assistant: I can’t modify your repository directly from here. Paste the updated README below into README.md (or follow the git/web steps) to add a reminder to install wvkbd.

Updated README.md content (paste this as the file):

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

Suggested commit message:
Update README.md to remind people to install wvkbd on their system first

Quick ways to apply the change

- Using the GitHub web editor:
  1. Open README.md in the repo on GitHub.
  2. Click the edit (pencil) button, replace the contents with the updated text above.
  3. Add the suggested commit message, create a new branch, and open a pull request (or commit directly to main if you prefer).

- Using git locally:
  1. git clone https://github.com/Bukutsu/cosmic-applet-toggle-keyboard.git
  2. cd cosmic-applet-toggle-keyboard
  3. git checkout -b update/readme-install-wvkbd
  4. Replace README.md contents with the updated text above.
  5. git add README.md
  6. git commit -m "Update README.md to remind people to install wvkbd on their system first"
  7. git push --set-upstream origin update/readme-install-wvkbd
  8. Open a PR on GitHub.

If you want, I can generate a minimal diff/patch you can apply, or a one-line addition instead of replacing the whole file. Which would you prefer?
