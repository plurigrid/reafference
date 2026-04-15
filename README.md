# reafference

This is a local adaptation workspace for `reafference`.

I could not clone `plurigrid/reafference` from GitHub because the repo path was
not resolvable over either HTTPS or SSH from this environment, so this folder
is a clean local scaffold seeded from the closest live references already in
the workspace:

- [nash-portal web shell](/Users/bob/i/horse/arena/cloned/nash-portal/web/index.html)
- [ResttyTerminal.svelte](/Users/bob/i/portal-merge-1/src/lib/components/ResttyTerminal.svelte)
- [retty-browserpod.html](/Users/bob/i/promiscuous-scan/retty-browserpod.html)
- [ratzilla-tritmap index](/Users/bob/i/ratzilla-tritmap/index.html)
- [disco-world-research-2026-03-08.md](/Users/bob/i/session-artifacts/docs/disco-world-research-2026-03-08.md)

Important naming note:

- `ratzilla` is the Rust-side browser authoring path and should be treated as
  the main structural choice when `reafference` grows toward the
  `nash-portal` shape.
- `restty` is the browser-side Ghostty VT + WebGPU host used in the local web
  terminal work, and should be treated as a render/transport path beneath that
  choice.
- `retty` is the local Zig-side widget and layout engine referenced in the
  research note.

The three mutually exclusive design paths are documented in
[docs/three-approaches.md](/Users/bob/i/reafference/docs/three-approaches.md).

The byte-identical transport study for `health` and `reafferance.health` is in
[docs/health-byte-identity.md](/Users/bob/i/reafference/docs/health-byte-identity.md).

The current operator-facing walking skeleton is a Multiscale Health Innovation
Network instrument expressed in two hosts:

- [tui/src/main.rs](/Users/bob/i/reafference/tui/src/main.rs) provides a live
  `ratatui` instrument with scale tabs, a network mesh, a work queue, an
  evidence detail pane, gauges, charts, and keyboard navigation.
- [web/index.html](/Users/bob/i/reafference/web/index.html) provides the
  browser-side MHIN shell and keeps the ratzilla-first, post-web thesis visible
  alongside the lower transport and render paths.

Runnable scaffold:

- [reafferance.health browser surface](/Users/bob/i/reafference/reafferance.health/index.html)
- [web MHIN shell](/Users/bob/i/reafference/web/index.html)
- [shared health frame module](/Users/bob/i/reafference/shared/health-frame.mjs)
- [health CLI](/Users/bob/i/reafference/bin/health)
- [health build script](/Users/bob/i/reafference/scripts/build-health-artifacts.mjs)
- [health byte check](/Users/bob/i/reafference/scripts/check-health-byte-identity.mjs)
- [package.json](/Users/bob/i/reafference/package.json)
- [workspace Cargo.toml](/Users/bob/i/reafference/Cargo.toml)
- [web crate](/Users/bob/i/reafference/web/Cargo.toml)
- [tui crate](/Users/bob/i/reafference/tui/Cargo.toml)

Quick usage:

- `cargo build` compiles the top-level `web` and `tui` workspace members
- `cargo run -p reafference-tui` launches the MHIN terminal instrument
- `cargo run -p reafference-web` prints the browser surface map
- `npm run build:health` writes the canonical payload into `build/health/`
- `npm run check:health` verifies the built payload still matches the generator
- `bin/health` writes the canonical ANSI frame bytes to stdout
- `bin/health --hash` prints the frame hash for browser comparison
- `bin/health --plain` prints the frame without ANSI control sequences

The `tui` keybindings are:

- `q` or `Esc` to quit
- `Tab` to cycle focus across network, queue, and detail panes
- `Left` / `Right` or `h` / `l` to move between scales
- `Up` / `Down` or `k` / `j` to change selection
- `r` to refresh the instrument state

Prototype shells:

- [portal-shell](/Users/bob/i/reafference/prototypes/portal-shell/index.html)
- [restty-world](/Users/bob/i/reafference/prototypes/restty-world/index.html)
- [ratzilla-surface](/Users/bob/i/reafference/prototypes/ratzilla-surface/index.html)
