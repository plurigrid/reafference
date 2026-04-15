# Health Byte Identity

## BLUF

If you want the `health` CLI in Ghostty and `reafferance.health` in the browser
to share the most byte-identical output possible, do not start from DOM or from
platform-specific widgets. Start from one canonical render payload and let both
targets consume it.

The strongest "works now" path is shared ANSI frame bytes:

- CLI: write the ANSI bytes directly to stdout inside Ghostty
- browser: feed the same ANSI bytes into `restty.write(...)`

The strongest long-term path is shared packed cell frames:

- canonical binary cell payload, versioned and checksummed
- CLI and browser both render from the same cell snapshot
- ANSI becomes one derived view, not the canonical transport

## Current Scaffold

The repository now includes a minimal proof of this split:

- [shared/health-frame.mjs](/Users/bob/i/reafference/shared/health-frame.mjs)
- [bin/health](/Users/bob/i/reafference/bin/health)
- [reafferance.health/index.html](/Users/bob/i/reafference/reafferance.health/index.html)
- [scripts/build-health-artifacts.mjs](/Users/bob/i/reafference/scripts/build-health-artifacts.mjs)
- [scripts/check-health-byte-identity.mjs](/Users/bob/i/reafference/scripts/check-health-byte-identity.mjs)

The stronger version of the split is now available too:

- the build step writes one canonical `build/health/frame.ansi` artifact
- the CLI prefers that built artifact when it exists
- the browser fetches that same built artifact first and only falls back to
  local generation when the file is unavailable

## What The Tree-Sitter Study Found

### 1. `nash-portal` already has the right top-level split

The cloned `nash-portal` workspace already separates browser and terminal
targets in one build: [Cargo.toml](/Users/bob/i/horse/arena/cloned/nash-portal/Cargo.toml:1).

The browser target uses `ratzilla` plus a DOM backend in
[web/src/main.rs](/Users/bob/i/horse/arena/cloned/nash-portal/web/src/main.rs:2)
through [web/src/main.rs](/Users/bob/i/horse/arena/cloned/nash-portal/web/src/main.rs:9).

The terminal target uses `ratatui` plus `CrosstermBackend` in
[tui/src/main.rs](/Users/bob/i/horse/arena/cloned/nash-portal/tui/src/main.rs:6)
through [tui/src/main.rs](/Users/bob/i/horse/arena/cloned/nash-portal/tui/src/main.rs:12).

That split is good, but the render and application logic is still duplicated.
Both entrypoints carry their own `App`, `TokenData`, `Candle`, fetchers, and
`draw` logic:

- browser symbols:
  [web/src/main.rs](/Users/bob/i/horse/arena/cloned/nash-portal/web/src/main.rs:16),
  [web/src/main.rs](/Users/bob/i/horse/arena/cloned/nash-portal/web/src/main.rs:46),
  [web/src/main.rs](/Users/bob/i/horse/arena/cloned/nash-portal/web/src/main.rs:123),
  [web/src/main.rs](/Users/bob/i/horse/arena/cloned/nash-portal/web/src/main.rs:188)
- terminal symbols:
  [tui/src/main.rs](/Users/bob/i/horse/arena/cloned/nash-portal/tui/src/main.rs:20),
  [tui/src/main.rs](/Users/bob/i/horse/arena/cloned/nash-portal/tui/src/main.rs:50),
  [tui/src/main.rs](/Users/bob/i/horse/arena/cloned/nash-portal/tui/src/main.rs:111),
  [tui/src/main.rs](/Users/bob/i/horse/arena/cloned/nash-portal/tui/src/main.rs:199)

For `health`, the next step is not "another web target." It is extracting a
shared render core.

### 2. There is already a proven shared ANSI path

The clearest local proof lives in
[ratzilla-tritmap/index.html](/Users/bob/i/ratzilla-tritmap/index.html:88).

That render loop does exactly this:

1. call `render()` to get `ansiBytes`
2. decode the bytes
3. pass the result into `restty.write(...)`

See:

- [ratzilla-tritmap/index.html](/Users/bob/i/ratzilla-tritmap/index.html:97)
- [ratzilla-tritmap/index.html](/Users/bob/i/ratzilla-tritmap/index.html:99)

That means a ratatui-style renderer can already produce a byte payload that the
browser consumes without inventing a separate browser-only scene graph.

### 3. `restty` is already the right browser host

The browser-side terminal host is already instantiated in
[ResttyTerminal.svelte](/Users/bob/i/portal-merge-1/src/lib/components/ResttyTerminal.svelte:61)
through [ResttyTerminal.svelte](/Users/bob/i/portal-merge-1/src/lib/components/ResttyTerminal.svelte:101).

This is the right surface for `reafferance.health` if your goal is byte
identity with a terminal render stream instead of visual similarity only.

### 4. There is also a lower-level Ghostty cell path

[retty-browserpod.html](/Users/bob/i/promiscuous-scan/retty-browserpod.html:191)
through [retty-browserpod.html](/Users/bob/i/promiscuous-scan/retty-browserpod.html:220)
documents the `ghostty-web` path, including low-level viewport access through
`GhosttyCell[]`.

That matters because it gives you a future path below ANSI:

- write bytes into Ghostty's VT implementation
- inspect or reuse structured cell state
- feed cell colors or glyph metadata into a GPU layer if needed

### 5. Zig already has a canonical packed cell transport

[cell_sync.zig](/Users/bob/i/zig-syrup/src/cell_sync.zig:18) through
[cell_sync.zig](/Users/bob/i/zig-syrup/src/cell_sync.zig:21) defines a packed
binary wire format for terminal cells inside `syrup.bytes`.

It is explicit about the payload:

- `[u16 x][u16 y][u24 codepoint][u24 fg][u24 bg][u8 attrs]`
- `14 bytes/cell`
- RLE for repeated cells

The packing logic is in
[cell_sync.zig](/Users/bob/i/zig-syrup/src/cell_sync.zig:328) through
[cell_sync.zig](/Users/bob/i/zig-syrup/src/cell_sync.zig:339).

This is the best existing local candidate for a canonical cross-target frame
payload when ANSI is no longer enough.

### 6. Zig also already exposes raw zero-copy byte buffers

[wgpu_compute.zig](/Users/bob/i/zig-syrup/src/wgpu_compute.zig:180) through
[wgpu_compute.zig](/Users/bob/i/zig-syrup/src/wgpu_compute.zig:189) exposes raw
byte access for GPU upload and Wasm export.

That makes it realistic to keep overlays or color planes byte-identical too,
even if text and VT semantics move to the cell layer.

## Recommendation

### Best path now: shared ANSI frames

Use one shared render core that emits ANSI frame bytes.

Why this wins first:

- it already has a working browser precedent via `restty.write(...)`
- it renders correctly in Ghostty without custom terminal integration
- it lets you keep one layout engine and one frame encoder
- it gets you byte identity at the most useful seam: after layout, before host
  rendering

For `health`, that means:

- `health` CLI emits ANSI bytes to stdout
- `reafferance.health` receives the same ANSI bytes and writes them into
  `restty`

The byte-identical artifact is the frame stream itself.

### Best path later: shared packed cell frames

Once you need more control than ANSI gives you, promote the canonical payload to
packed cell frames modeled on `cell_sync.zig`.

Why this wins second:

- it is more explicit than ANSI
- it is less sensitive to terminal-parser differences
- it supports damage regions, replay, compression, and checksums
- it fits both browser and native hosts cleanly

At that point:

- `health` CLI can render cells directly or derive ANSI from cells
- `reafferance.health` can render the same cells through `restty`,
  `ghostty-web`, or a DOM/canvas adapter

## What Can Actually Be Byte-Identical

You can make these portions byte-identical:

- the domain payload that describes the `health` world state
- the rendered ANSI frame bytes
- the packed cell-frame bytes
- optional GPU overlay buffers such as color planes or tile textures

You should not expect these to be byte-identical:

- the final native executable and browser bundle
- the DOM tree
- the browser paint output
- Ghostty's internal state after parsing if the host size or font metrics differ

So the right question is not "can the whole build be identical?" It is "what is
the highest-value canonical payload we can keep identical across hosts?"

## Proposed `health` Build Shape

Use a workspace split like `nash-portal`, but move the shared logic down one
layer:

1. `health-core`
   Shared state, fetchers, layout, draw logic, and offscreen frame production.
1. `health-ansi`
   Converts a `health-core` frame into ANSI bytes.
1. `health-cells`
   Converts the same frame into packed cell bytes.
1. `health`
   Native CLI that writes ANSI to stdout inside Ghostty.
1. `health-web`
   Browser host for `reafferance.health` that feeds the same bytes into
   `restty` first, then optionally into a lower-level Ghostty/cell path later.

## Concrete Next Steps

1. Extract the duplicated `App`, fetch, chart, and `draw` logic out of
   `nash-portal`-style entrypoints into a shared crate or Zig module.
1. Add an offscreen renderer that returns ANSI bytes for one frame.
1. Make the first `health` CLI write those bytes directly to stdout.
1. Make the first `reafferance.health` page boot `restty` and consume the same
   byte stream.
1. Add a second encoder for packed cell frames once the ANSI path is stable.

## Decision

If you want the shortest path to real shared payloads, ship `health` around
shared ANSI frames first.

If you want the strongest long-term canonical transport, standardize packed cell
frames immediately after that.
