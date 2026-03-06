# niri-master-layout

A small [niri](https://github.com/niri-wm/niri) helper that toggles a master layout.

- First run: the focused window becomes `60%` (master), and up to `3` windows are stacked on the right at `40%`.
- Next run (if a saved state exists): the previous layout is restored and the saved state file is removed.
- If restore fails, the stale state file is removed and a new master layout is applied.

Temporary state is stored per workspace under `std::env::temp_dir()`:

- `<temp_dir>/niri-master-layout-<workspace_id>.state`

## Demo
https://github.com/user-attachments/assets/83452bd7-a537-404e-b386-48bab69c1330

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
