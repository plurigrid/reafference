const encoder = new TextEncoder();
const decoder = new TextDecoder();

export const DEFAULT_STAMP = "2026-04-15T00:00:00Z";

const ansi = {
  reset: "\u001b[0m",
  bold: "\u001b[1m",
  dim: "\u001b[2m",
  cyan: "\u001b[36m",
  green: "\u001b[32m",
  yellow: "\u001b[33m",
  red: "\u001b[31m",
  magenta: "\u001b[35m",
  gray: "\u001b[90m",
};

function padRight(value, width) {
  return String(value).padEnd(width, " ");
}

function framedRow(left, right) {
  return `| ${padRight(left, 28)} | ${padRight(right, 28)} |`;
}

function plainFrame(snapshot) {
  const lines = [
    "reafference.health :: shared health frame",
    "",
    "+------------------------------+------------------------------+",
    framedRow("surface", "transport"),
    framedRow("health cli", "ansi frame bytes"),
    framedRow("ghostty host", "stdout"),
    framedRow("browser host", "restty-ready"),
    framedRow("canonical seam", snapshot.canonicalSeam),
    framedRow("cells later", snapshot.cellsLater),
    "+------------------------------+------------------------------+",
    framedRow("stamp", snapshot.stamp),
    framedRow("mode", snapshot.mode),
    framedRow("carrier", snapshot.carrier),
    framedRow("field", snapshot.field),
    framedRow("status", snapshot.status),
    "+------------------------------+------------------------------+",
    "",
    "One frame generator. Two hosts. One byte stream.",
  ];
  return lines.join("\n");
}

export function buildHealthSnapshot(overrides = {}) {
  return {
    stamp: overrides.stamp ?? DEFAULT_STAMP,
    mode: overrides.mode ?? "observe",
    carrier: overrides.carrier ?? "ghostty/restty",
    field: overrides.field ?? "health",
    status: overrides.status ?? "warm",
    canonicalSeam: overrides.canonicalSeam ?? "ansi-bytes-now",
    cellsLater: overrides.cellsLater ?? "packed-cell-frames",
  };
}

export function buildHealthFrameText(overrides = {}) {
  const snapshot = buildHealthSnapshot(overrides);
  const frame = plainFrame(snapshot);

  return [
    "\u001b[2J\u001b[H",
    `${ansi.bold}${ansi.cyan}reafference.health${ansi.reset}${ansi.gray}  browser surface${ansi.reset}`,
    `${ansi.bold}${ansi.green}health${ansi.reset}${ansi.gray}            ghostty cli${ansi.reset}`,
    "",
    `${ansi.dim}${frame}${ansi.reset}`,
    "",
    `${ansi.yellow}note${ansi.reset}: browser should consume these exact bytes before any host-specific paint`,
    `${ansi.magenta}next${ansi.reset}: promote canonical transport to packed cell frames once ANSI is stable`,
    `${ansi.red}verify${ansi.reset}: compare sha256 in browser against \`bin/health --hash\``,
    "",
  ].join("\n");
}

export function buildHealthFrameBytes(overrides = {}) {
  return encoder.encode(buildHealthFrameText(overrides));
}

export function decodeFrameBytes(frameBytes) {
  return decoder.decode(frameBytes);
}

export function stripAnsi(text) {
  return text.replace(/\u001b\[[0-9;?]*[A-Za-z]/g, "");
}

function escapeHtml(text) {
  return text
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;");
}

function styleFromCodes(codes, current) {
  let next = { ...current };
  for (const raw of codes) {
    const code = Number(raw || 0);
    if (code === 0) {
      next = { fg: null, bold: false, dim: false };
    } else if (code === 1) {
      next.bold = true;
    } else if (code === 2) {
      next.dim = true;
    } else if (code === 22) {
      next.bold = false;
      next.dim = false;
    } else if (code === 31) {
      next.fg = "#ff6b6b";
    } else if (code === 32) {
      next.fg = "#58d46a";
    } else if (code === 33) {
      next.fg = "#f6c65b";
    } else if (code === 35) {
      next.fg = "#f08cff";
    } else if (code === 36) {
      next.fg = "#6be7ee";
    } else if (code === 90) {
      next.fg = "#93a2bb";
    } else if (code === 39) {
      next.fg = null;
    }
  }
  return next;
}

function styleToCss(style) {
  const rules = [];
  if (style.fg) {
    rules.push(`color:${style.fg}`);
  }
  if (style.bold) {
    rules.push("font-weight:700");
  }
  if (style.dim) {
    rules.push("opacity:0.72");
  }
  return rules.join(";");
}

export function ansiToHtml(text) {
  const csiPattern = /\u001b\[([0-9;?]*)([A-Za-z])/g;
  let html = "";
  let lastIndex = 0;
  let style = { fg: null, bold: false, dim: false };

  for (const match of text.matchAll(csiPattern)) {
    const [full, params, command] = match;
    const start = match.index ?? 0;
    const chunk = text.slice(lastIndex, start);
    if (chunk) {
      const css = styleToCss(style);
      const escaped = escapeHtml(chunk);
      html += css ? `<span style="${css}">${escaped}</span>` : escaped;
    }

    if (command === "m") {
      style = styleFromCodes(params.split(";"), style);
    }

    lastIndex = start + full.length;
  }

  const tail = text.slice(lastIndex);
  if (tail) {
    const css = styleToCss(style);
    const escaped = escapeHtml(tail);
    html += css ? `<span style="${css}">${escaped}</span>` : escaped;
  }

  return html;
}

export function toHexDump(frameBytes, columns = 16) {
  const lines = [];
  for (let offset = 0; offset < frameBytes.length; offset += columns) {
    const slice = frameBytes.slice(offset, offset + columns);
    const hex = Array.from(slice, (value) => value.toString(16).padStart(2, "0")).join(" ");
    lines.push(`${offset.toString(16).padStart(4, "0")}  ${hex}`);
  }
  return lines.join("\n");
}

export async function frameHashHex(frameBytes) {
  const digest = await crypto.subtle.digest("SHA-256", frameBytes);
  return Array.from(new Uint8Array(digest), (value) => value.toString(16).padStart(2, "0")).join("");
}
