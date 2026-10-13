// Grove-owned launch evidence using only Ravel 0.1.0's public selection API.
// https://github.com/Linkuistics/Ravel/blob/v0.1.0/crates/ravel/worker/sdk/index.ts
import { randomUUID } from "node:crypto";
import { definePolicy, type Policy } from "ravel/sdk";

/** One policy selection, carried into its managed process and committed at retirement. */
export interface ProviderEvidence {
  readonly schemaVersion: 1;
  readonly attemptId: string;
  readonly provider: string;
  readonly policyVersion: string;
  readonly kind: string;
  readonly taskId?: string;
}

export const EVIDENCE_ENV = "GROVE_PROVIDER_EVIDENCE";
export const EVIDENCE_BYTES = 8192;
const UUID = /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/;

/** Validate a receipt read from the process environment or a measured review body. */
export function parseEvidence(text: string): ProviderEvidence | undefined {
  if (new TextEncoder().encode(text).byteLength > EVIDENCE_BYTES || /[\r\n\0]/.test(text)) return undefined;
  let value: unknown;
  try { value = JSON.parse(text); } catch { return undefined; }
  if (typeof value !== "object" || value === null || Array.isArray(value)) return undefined;
  const fields = value as Record<string, unknown>;
  const keys = ["schemaVersion", "attemptId", "provider", "policyVersion", "kind", "taskId"];
  if (Object.keys(fields).some((key) => !keys.includes(key))) return undefined;
  const label = (field: unknown): field is string => typeof field === "string" && field.trim() !== "" && !/[\r\n\0]/.test(field);
  if (fields.schemaVersion !== 1 || typeof fields.attemptId !== "string" || !UUID.test(fields.attemptId)
    || !label(fields.provider) || !label(fields.policyVersion) || !label(fields.kind)
    || ("taskId" in fields && !label(fields.taskId))) return undefined;
  const receipt: ProviderEvidence = { schemaVersion: 1, attemptId: fields.attemptId, provider: fields.provider,
    policyVersion: fields.policyVersion, kind: fields.kind,
    ...(typeof fields.taskId === "string" ? { taskId: fields.taskId } : {}) };
  // Canonical wrapper encoding also rejects duplicate JSON keys, alternative
  // escapes and edited/reformatted records. Copy the supplied bytes verbatim.
  return JSON.stringify(receipt) === text ? receipt : undefined;
}

/**
 * Opt in around the final owner policy; its loader, routes and labels stay intact.
 * Ravel grants ending authority to the selected process. This shell execs the
 * original command in the same group, carrying that authority and fresh evidence.
 * Confined callers must explicitly grant the original executable as runtime-read.
 */
export function withProvenance(owner: Policy): Policy {
  return definePolicy({
    ...owner,
    async select(request, context, host) {
      const selected = await owner.select(request, context, host);
      if (selected.status === "refused") return selected;
      const text = JSON.stringify({ schemaVersion: 1, attemptId: randomUUID(),
        provider: selected.provider, policyVersion: owner.version, kind: request.kind,
        ...(request.taskId === undefined ? {} : { taskId: request.taskId }) });
      if (parseEvidence(text) === undefined) return { status: "refused", code: "provenance_invalid",
        message: "the selected provider evidence is invalid or exceeds 8192 bytes",
        remedy: "use nonblank single-line labels and a receipt within Grove's 8192-byte bound" };
      // Never turn an invalid program/argv into a valid shell selection.
      if (typeof selected.program !== "string" || selected.program.trim() === "" || selected.program.includes("\0")
        || !Array.isArray(selected.args) || selected.args.some((arg) => typeof arg !== "string" || arg.includes("\0"))) {
        return { status: "refused", code: "provenance_command_invalid",
          message: "the owner selected an invalid program or argument vector", remedy: "repair the owner policy command" };
      }
      return { ...selected, program: "/bin/bash",
        // -p prevents ambient BASH_ENV, functions and shell options from running
        // code before exec. Original environment and literal argv reach the agent.
        args: ["--noprofile", "--norc", "-p", "-c",
          'export GROVE_PROVIDER_EVIDENCE="$1"; shift; exec -- "$@"',
          "grove-provenance", text, selected.program, ...selected.args] };
    },
  });
}
