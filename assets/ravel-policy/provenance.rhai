// Grove provenance.rhai — Grove-owned launch evidence. Runtime: Ravel 2.x,
// Rhai, SCHEMA_VERSION 4. Its Ravel 1.x counterpart is `provenance.ts`, and
// `docs/grove-provider-evidence.md` owns the receipt and its checks.
//
// Wrap the final selection of your configuration once, after every route and
// modifier, and pass the version of the configuration that made it:
//
//   import "/absolute/path/to/provenance" as provenance;
//   import "/absolute/path/to/my-owner" as owner;
//   export const SCHEMA_VERSION = 4;
//   export const VERSION = owner::VERSION;
//   fn select(request, context) {
//       provenance::with_provenance(owner::select(request, context), owner::VERSION, request)
//   }
//
// Keep a `load_context` of the owner as it is: the wrapper reads no context.
//
// A receipt crosses runtimes. A review loader of either runtime checks a
// receipt that the wrapper of the other runtime recorded, so this file keeps
// the bytes and the checks of `provenance.ts`:
//
//   - The receipt is the text of `JSON.stringify` for the same record. Its
//     members keep a constant order, and each member value is the text of
//     `ravel::json_encode`, which equals `JSON.stringify` for every string
//     that Rhai can hold.
//   - A label is blank when ECMAScript's `String.prototype.trim` empties it.
//     The `trim` of Rhai removes U+0085 and keeps U+FEFF, so `blank` tests the
//     ECMAScript whitespace set itself.
//
// One difference remains. A JavaScript string can hold an unpaired surrogate,
// and a Rhai string cannot. `json_parse` refuses a receipt that escapes one, so
// `parse_evidence` rejects a receipt that `parse_evidence` of `provenance.ts`
// accepts. The owner's declaration of the provider is the remedy.

/// The variable that carries the receipt into the managed process.
export const EVIDENCE_ENV = "GROVE_PROVIDER_EVIDENCE";

/// The bound of one receipt, in UTF-8 bytes.
export const EVIDENCE_BYTES = 8192;

/// The characters that ECMAScript's `String.prototype.trim` removes: its
/// WhiteSpace and LineTerminator sets, U+0009 to U+000D, U+0020, U+00A0,
/// U+1680, U+2000 to U+200A, U+2028, U+2029, U+202F, U+205F, U+3000 and U+FEFF.
export const ECMASCRIPT_SPACE = "\t\n\x0b\x0c\r                  　﻿";

/// The receipt members, in their constant order.
const MEMBERS = ["schemaVersion", "attemptId", "provider", "policyVersion", "kind", "taskId"];

/// Whether `text` is empty after ECMAScript's `trim`.
fn blank(text) {
    for character in text {
        if !global::ECMASCRIPT_SPACE.contains(character) {
            return false;
        }
    }
    true
}

/// Whether `text` holds a carriage return, a line feed or a NUL.
fn multiline(text) {
    text.contains('\r') || text.contains('\n') || text.contains('\x00')
}

/// Whether `value` is a nonblank single-line string without a NUL.
fn label(value) {
    type_of(value) == "string" && !blank(value) && !multiline(value)
}

/// Whether `text` is a lowercase UUID of version 4 and the RFC 4122 variant.
fn uuid_v4(text) {
    if type_of(text) != "string" || text.len() != 36 {
        return false;
    }
    let index = 0;
    for character in text {
        let valid = switch index {
            8 | 13 | 18 | 23 => character == '-',
            14 => character == '4',
            19 => "89ab".contains(character),
            _ => "0123456789abcdef".contains(character),
        };
        if !valid {
            return false;
        }
        index += 1;
    }
    true
}

/// The receipt text of `record`: its members in the constant order, each value
/// written by `json_encode`. A member that `record` lacks is left out.
fn receipt_text(record) {
    let members = "";
    for name in global::MEMBERS {
        if record.contains(name) {
            if members != "" {
                members += ",";
            }
            members += `${ravel::json_encode(name)}:${ravel::json_encode(record[name])}`;
        }
    }
    `{${members}}`
}

/// Validate a receipt read from the process environment or a measured review
/// body. Returns the record, or `()` for a text that is not the canonical
/// receipt.
fn parse_evidence(text) {
    if type_of(text) != "string" || text.bytes > global::EVIDENCE_BYTES || multiline(text) {
        return ();
    }
    let value = ();
    try {
        value = ravel::json_parse(text);
    } catch {
        return ();
    }
    if type_of(value) != "map" {
        return ();
    }
    for name in value.keys() {
        if !global::MEMBERS.contains(name) {
            return ();
        }
    }
    if !value.contains("schemaVersion") || type_of(value.schemaVersion) != "i64" || value.schemaVersion != 1
        || !value.contains("attemptId") || !uuid_v4(value.attemptId)
        || !label(value.get("provider")) || !label(value.get("policyVersion")) || !label(value.get("kind"))
        || (value.contains("taskId") && !label(value.taskId)) {
        return ();
    }
    // The canonical encoding also rejects duplicate keys, other escapes and
    // edited or reformatted records. A caller copies the supplied bytes.
    if receipt_text(value) == text { value } else { () }
}

/// Whether `selection` names a nonblank program and an argument vector of
/// strings, none of which holds a NUL.
fn valid_command(selection) {
    let program = selection.get("program");
    let args = selection.get("args");
    if type_of(program) != "string" || blank(program) || program.contains('\x00') || type_of(args) != "array" {
        return false;
    }
    for arg in args {
        if type_of(arg) != "string" || arg.contains('\x00') {
            return false;
        }
    }
    true
}

/// Wrap `selection`, the owner's result for `request`, so that the managed
/// process receives a fresh receipt in `GROVE_PROVIDER_EVIDENCE`. `version`
/// is the owner's `VERSION`. A refusal passes through unchanged.
///
/// Ravel grants ending authority to the selected process. This shell execs the
/// original command in the same group, carrying that authority and fresh
/// evidence. Confined callers must explicitly grant the original executable as
/// runtime-read.
fn with_provenance(selection, version, request) {
    if type_of(selection) == "map" && selection.get("status") == "refused" {
        return selection;
    }
    // An owner result that is not a map has no provider, as in JavaScript,
    // where a missing member leaves the receipt invalid.
    let provider = if type_of(selection) == "map" { selection.get("provider") } else { () };
    let record = #{
        schemaVersion: 1,
        attemptId: ravel::uuid_v4(),
        provider: provider,
        policyVersion: version,
        kind: request.kind,
    };
    if request.taskId != () {
        record.taskId = request.taskId;
    }
    // A label that is not a string has no receipt. `json_encode` would throw
    // for a value without a JSON form, where `JSON.stringify` leaves it out.
    let text = if label(provider) && label(version) { receipt_text(record) } else { "" };
    if parse_evidence(text) == () {
        return #{
            status: "refused",
            code: "provenance_invalid",
            message: "the selected provider evidence is invalid or exceeds 8192 bytes",
            remedy: "use nonblank single-line labels and a receipt within Grove's 8192-byte bound",
        };
    }
    // Never turn an invalid program or argument vector into a valid shell
    // selection.
    if !valid_command(selection) {
        return #{
            status: "refused",
            code: "provenance_command_invalid",
            message: "the owner selected an invalid program or argument vector",
            remedy: "repair the owner policy command",
        };
    }
    // -p prevents ambient BASH_ENV, functions and shell options from running
    // code before exec. The original environment and literal arguments reach
    // the agent.
    let args = ["--noprofile", "--norc", "-p", "-c",
        "export GROVE_PROVIDER_EVIDENCE=\"$1\"; shift; exec -- \"$@\"",
        "grove-provenance", text, selection.program];
    args += selection.args;
    selection.program = "/bin/bash";
    selection.args = args;
    selection
}
