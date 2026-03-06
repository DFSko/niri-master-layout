# niri-master-layout

A small [niri](https://github.com/niri-wm/niri) helper that toggles a master layout.

- First run: the focused window becomes `60%` (master), and up to `3` windows are stacked on the right at `40%`.
- Next run (if a saved state exists): the previous layout is restored and the saved state file is removed.
- If restore fails, the stale state file is removed and a new master layout is applied.

Temporary state is stored per workspace under `std::env::temp_dir()`:

- `<temp_dir>/niri-master-layout-<workspace_id>.state`

## Demo
https://github.com/user-attachments/assets/83452bd7-a537-404e-b386-48bab69c1330

## Install Binary (No Rust)

Download the latest archive from GitHub Releases:

- `niri-master-layout-v<version>.tar.gz`

Example:

```bash
curl -L -o niri-master-layout.tar.gz \
  https://github.com/<owner>/<repo>/releases/latest/download/niri-master-layout-v0.2.0.tar.gz
tar -xzf niri-master-layout.tar.gz
install -Dm755 niri-master-layout ~/.local/bin/niri-master-layout
```

## Build

```bash
cargo build --release
```

## Run Manually

```bash
./target/release/niri-master-layout
```

## Hotkey in niri

Add a keybind in `~/.config/niri/config.kdl`:

```kdl
binds {
    Mod+Shift+M {
        spawn "niri-master-layout"
    }
}
```

## Release For Maintainers

Push a version tag, and GitHub Actions will build the binary and publish assets to the Release:

```bash
git tag -a v0.2.0 -m "Release v0.2.0"
git push origin v0.2.0
```
