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

- `restty` is the browser-side Ghostty VT + WebGPU host used in the local web
  terminal work.
- `retty` is the local Zig-side widget and layout engine referenced in the
  research note.
- `ratzilla` is the ratatui-in-the-browser path used by the current Rust web
  surface.

The three mutually exclusive design paths are documented in
[docs/three-approaches.md](/Users/bob/i/reafference/docs/three-approaches.md).

Prototype shells:

- [portal-shell](/Users/bob/i/reafference/prototypes/portal-shell/index.html)
- [restty-world](/Users/bob/i/reafference/prototypes/restty-world/index.html)
- [ratzilla-surface](/Users/bob/i/reafference/prototypes/ratzilla-surface/index.html)
