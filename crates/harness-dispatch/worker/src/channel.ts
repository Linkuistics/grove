// The worker's half of the private protocol channel.
//
// The front process hands the worker one end of a socket pair as descriptor 3,
// and nothing else names it: no environment variable, path or argument carries
// the channel, so there is nothing ambient for a policy or a caller to redirect.
// Stdout and stderr stay the policy's diagnostic streams, which the front
// captures separately, so nothing a policy prints can reach a frame.
//
// A frame is a four-byte big-endian length followed by that many bytes of UTF-8
// JSON. Reads and writes are synchronous: the worker does nothing else while it
// waits for the front, and a synchronous write cannot be reordered with a later
// `process.exit`.

import { readSync, writeSync } from "node:fs";

const CHANNEL = 3;

/** The largest frame either side sends or accepts. */
export const MAX_FRAME_BYTES = 1024 * 1024;

export function send(message: object): void {
  const body = Buffer.from(JSON.stringify(message), "utf8");
  if (body.length > MAX_FRAME_BYTES) {
    throw new Error(`protocol frame of ${body.length} bytes exceeds ${MAX_FRAME_BYTES}`);
  }
  const frame = Buffer.alloc(4 + body.length);
  frame.writeUInt32BE(body.length, 0);
  body.copy(frame, 4);
  let written = 0;
  while (written < frame.length) {
    written += writeSync(CHANNEL, frame, written, frame.length - written);
  }
}

/**
 * The front's next message, or `null` if the front closed the channel between
 * frames: it has nothing more to ask. A channel closed inside a frame throws.
 */
export function receive(): unknown {
  const header = readExact(4, true);
  if (header === null) return null;
  const length = header.readUInt32BE(0);
  if (length > MAX_FRAME_BYTES) {
    throw new Error(`protocol frame of ${length} bytes exceeds ${MAX_FRAME_BYTES}`);
  }
  return JSON.parse(readExact(length, false)!.toString("utf8"));
}

/** `length` bytes, or `null` when `mayEnd` and the channel ends before the first. */
function readExact(length: number, mayEnd: boolean): Buffer | null {
  const buffer = Buffer.alloc(length);
  let filled = 0;
  while (filled < length) {
    const read = readSync(CHANNEL, buffer, filled, length - filled, null);
    if (read === 0) {
      if (mayEnd && filled === 0) return null;
      throw new Error("the front process closed the protocol channel inside a frame");
    }
    filled += read;
  }
  return buffer;
}
