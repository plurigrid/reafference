# Three Approaches

## Shared elements

Across `nash-portal`, `restty`, `retty-browserpod`, and `ratzilla`, the same
design atoms keep showing up:

- a strong identity strip at the top or edge
- one primary live surface, not a dashboard full of equal panels
- a small number of outbound actions
- a visible status channel
- monospace typography and dark background as the default field
- color as state, not decoration
- a clean separation between chrome and render surface

For `reafference`, that suggests one rule: pick a single primary interaction
model and let everything else support it.

## 1. Portal Shell

Source pull:

- `nash-portal/web/index.html`
- `nash-portal/web/src/main.rs`

What it is:

- a minimal DOM shell
- a narrow identity and action rail
- one centered live surface
- no sidebars, no inspector, no session tree

Why it works:

- it makes the product legible immediately
- it keeps the emotional center on one instrument
- it is the fastest path to a public-facing entry surface

Best when:

- `reafference` should feel like one object
- onboarding matters more than operability
- the main value is "see the system" rather than "drive the system"

Elements to keep:

- top rail with name, state, and external actions
- single central render viewport
- tiny footer/status line at most

Elements to avoid:

- session list
- debug log
- multi-pane control-room layout

Prototype:

- [portal-shell](/Users/bob/i/reafference/prototypes/portal-shell/index.html)

## 2. Restty World

Source pull:

- `portal-merge-1/src/lib/components/ResttyTerminal.svelte`
- `session-artifacts/docs/disco-world-research-2026-03-08.md`

What it is:

- terminal as the main world surface
- overlay canvas or measurement layer on top
- the browser acts as a host for a live field, not just a shell

Why it works:

- it supports dense interaction without becoming visually noisy
- it already has a path to Ghostty VT and WebGPU
- it makes "world state" the UI, not a metaphor pasted on top

Best when:

- `reafference` is really a live substrate
- the product needs world state, measurement, or replay
- the interface should stay compact while still feeling alive

Elements to keep:

- terminal frame as the dominant surface
- light chrome around the edges
- overlay for selection, state, or annotation
- event/status footer

Elements to avoid:

- marketing hero sections
- too many external navigation links
- equal-weight cards competing with the terminal

Prototype:

- [restty-world](/Users/bob/i/reafference/prototypes/restty-world/index.html)

## 3. Ratzilla Surface

Source pull:

- `ratzilla-tritmap/index.html`
- `promiscuous-scan/retty-browserpod.html`

What it is:

- ratatui-style rendering in the browser
- explicit instrument panel logic
- DOM or browser shell around a strongly structured text/cell surface

Why it works:

- it gives layout discipline without requiring full native TUI deployment
- it is good for inspectors, counters, small ledgers, and dense telemetry
- it can stay surprisingly clear if the browser chrome is kept thin

Best when:

- `reafference` needs instrumentation first
- the UI should read like an operator surface
- there is value in a sidebar or inspector rail

Elements to keep:

- one dominant instrument surface
- one narrow rail for legend, filter, or inspector
- fixed footer status

Elements to avoid:

- soft consumer-web cards
- large hero copy
- too much animated ornament

Prototype:

- [ratzilla-surface](/Users/bob/i/reafference/prototypes/ratzilla-surface/index.html)

## Recommendation

If I had to choose one starting point for actual `reafference` implementation,
I would start with `Restty World`.

Reason:

- it preserves the strongest terminal/world affordances
- it stays visually focused
- it can absorb ideas from the portal shell and from ratzilla later without
  losing its center

If the goal shifts toward a public landing surface, use `Portal Shell`.

If the goal shifts toward operator tooling, use `Ratzilla Surface`.
