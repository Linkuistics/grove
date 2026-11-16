# What a signature leaves unsaid

> **Purpose:** explanation — see why binding an API requires more than its declarations.
> **Reader:** language designers who can read C, but are new to API binding.
> **Before this:** nothing; [the overview](../README.md) introduces AIL's purpose.
> **Status:** draft 0.1, a proposal under review; no parser, validator or implementation exists.
> **Rules:** [`C1–C6`](../contracts.md#c1) define the obligations below; [`L2–L3`](../language.md#l2) and [`C8`](../contracts.md#c8) define publication and obligation checking; [`A1–A5`](../profiles.md#a1) define access and consumer agreement.

An [API binding](../glossary.md#binding) lets code in one language use an API
offered in another. A [foreign-function interface (FFI)](../glossary.md#ffi)
arranges calls and represents their arguments across the language boundary.
The binding also has to preserve what callers owe and what the API promises.
Those obligations are the [binding contract](../glossary.md#binding-contract).

## A small API

**Synthetic C header fragment — declarations for discussion, not an implemented
library or a complete binding contract.**

```c
#include <stddef.h>

typedef struct Watch Watch;
typedef void (*Notice)(void *context, unsigned event);

Watch *watch_open(Notice notice, void *context);
const unsigned char *watch_view(Watch *watch, size_t *length);
void watch_close(Watch *watch);
```

Opening a watch also registers a notification callback. The view operation
returns a pointer and accepts caller-supplied storage for an output length;
closing ends the watch.
These names suggest a use, but do not establish a safe calling protocol.
The header is deliberately insufficient: the contrasting contracts below are
possible answers to investigate, not facts inferred from these declarations.

The [caller](../glossary.md#caller-and-callee) invokes one of the three functions;
the callee implements it. A [handle](../glossary.md#handle) identifies a watch
without exposing its layout. A *view* gives access to bytes. The API invokes the
[callback](../glossary.md#callback), reversing the direction of the call. Its
context pointer can lead back to state the original caller supplied. These pieces
have different lifetimes: the watch can be live or destroyed, the view valid or
invalid, and a notification pending, running or finished. A contract relates
those states.

## What the header does not say

### Who is responsible for releasing a value?

Suppose a successful `watch_open` returns a new watch that the caller eventually
closes. Compare that with a function lending an existing watch which somebody
else will close. The returned pointer could look identical in both cases. If
the binding never closes a watch it owns, it leaks the resource; if it closes a
watch it was only lent, it can leave the actual owner using a destroyed watch
or attempting a second destruction.

[Ownership transfer](../glossary.md#ownership-transfer) moves a release or
destruction obligation between parties. An [owned](../glossary.md#owned) value
carries one such obligation; this does not imply exclusive access. A
[borrow](../glossary.md#borrow) carries access without transferring that
obligation; it does not imply Rust-style exclusivity or compiler enforcement.

When the callee gives an owned result, it transfers responsibility to the caller.
[Consuming](../glossary.md#consume) an input transfers responsibility from caller
to callee, even on failure; borrowing an input transfers no obligation. The caller and callee
therefore need separate answers for `watch_open`'s result, `watch_view`'s watch
argument and `watch_close`'s watch argument.

Follow the obligation in these three separate cases: borrowing leaves it with
the caller, consumption transfers it at entry even on failure, and delivery of
a non-null owned result gives the caller one obligation. A null result gives
none. After consumption, the callee discharges the obligation unless it returns
it as an owned output. The cases illustrate possible contracts, not facts
inferred from the header; retention is a separate event (non-normative;
[`C2`](../contracts.md#c2), [`C3`](../contracts.md#c3), [`C5`](../contracts.md#c5)).

![Three separate sequences track the obligation: caller keeps it when lending access, gives it to the callee at consumption, or receives it with a non-null owned result.](../figures/g4-ownership.svg)

[Diagram source](../figures/g4-ownership.puml)

In summary (non-normative): AIL distinguishes a type's release discipline from
input transfer and output transfer. Unique resources have one destruction
obligation per owned handle; reference-counted values have one release
obligation per owned reference. Input consumption occurs at entry on every
outcome, including failure ([`C2`](../contracts.md#c2), [`C3`](../contracts.md#c3),
[`C5`](../contracts.md#c5)). [Retaining](../glossary.md#retention) a separate
reference is a different event from taking over the caller's obligation.

### How long can the caller use a view?

Suppose `watch_view` lends bytes from its watch. In one contract the bytes remain
usable until the watch is destroyed. In another, the next call to `watch_view`
refreshes internal storage and invalidates the previous view, even though the
watch is still alive. Keeping the watch alive only solves the first case.
Keeping an invalid pointer in a target-language object does not keep its bytes
alive; later access can read freed or repurposed storage.

[Borrow validity](../glossary.md#borrow-validity) says how long borrowed access
is usable. A [region](../glossary.md#region) names its lifetime boundary;
[invalidation](../glossary.md#invalidation) can end it earlier. Neither a
non-null address nor C's `const` says when validity ends.

In summary (non-normative): AIL distinguishes validity for a call, for the loaded
API instance, and for a named owner. Returned borrows outlive their delivery, so
the current call is not a sufficient region for them. Each returned borrow also
names its earlier invalidating operations, even when there are none. For the
refresh case, a listed operation invalidates at entry and has a corresponding
mutation contract ([`C1`](../contracts.md#c1), [`C5`](../contracts.md#c5)).

### How many bytes exist, and what do they contain?

Suppose a successful view reports a length of three. Three initialized binary
bytes can occupy exactly three bytes of storage. A three-code-unit,
zero-terminated string needs room for a fourth code unit, its terminator. Reading
until zero in the binary case can overrun the allocation. Treating string
capacity as its text length can expose bytes that are not part of the text.

Compare the readable payload with the storage needed in these two cases;
the extra zero belongs to the text's storage, not its reported length.

| Possible contract | Reported length | Minimum storage for this case | Readable payload |
|---|---|---|---|
| Binary bytes | 3 bytes | 3 bytes | All 3 bytes, even if one is zero |
| Zero-terminated text | 3 code units | 4 code units, including the terminator | The 3 code units before the terminator |

[Buffer extent](../glossary.md#buffer-extent) distinguishes readable initialized
length from allocated capacity, and names the units. [Contents](../glossary.md#contents)
distinguish binary data from text with an encoding and a termination convention.
The pointer's element type answers neither question.

In summary (non-normative): AIL's core describes bounded buffers. Its
zero-terminated text length excludes the final zero code unit, while capacity
includes it. A pointer whose bound can only be found by scanning for a terminator
needs a [profile](../glossary.md#profile)-defined contract (a semantic extension,
not a preference preset); the core does not guess a maximum. The
`length` output pointer is a separate piece of storage from the bytes returned
by `watch_view` ([`C3`](../contracts.md#c3), [`C4`](../contracts.md#c4)).

### What can the caller read when a call fails?

Suppose `watch_view` returns null on failure. One contract could promise that it
also writes zero to `*length`; another could leave that storage uninitialized.
A binding that reads the length before checking the result is safe only if the
contract makes it readable on that path. The existence of an output pointer is
not a promise that the callee writes a value on every return.

An [outcome](../glossary.md#outcome) identifies a way a call completes, its
meaning and the state of its outputs. A null result is not inherently failure:
another API might use null for a successful “no value” answer. Likewise, turning
every status into an exception can discard a distinction that callers need.

In summary (non-normative): AIL partitions the possibilities of each
[completion](../glossary.md#completion) into non-overlapping, exhaustive outcomes
and describes output validity in each.
An invalid output cannot even be tested for null. A failure label describes the
observation rather than silently translating it into an exception
([`C5`](../contracts.md#c5)).

### Which calls may run together, and where?

Suppose a watch accepts calls on any thread, provided callers serialize access.
Compare a watch whose calls are restricted to the main thread. “Not thread-safe”
does not distinguish these contracts, and putting a lock around the second does
not meet its thread restriction. Conversely, allowing any thread to enter does
not make simultaneous calls safe.

An [executor](../glossary.md#executor) identifies where execution is allowed;
[concurrency](../glossary.md#concurrency) describes independent overlap.
[Re-entrancy](../glossary.md#re-entrancy) is a third question. If `watch_open`
invokes `notice` before returning, and `notice` calls into the same resource,
that call is nested under an active call. A design can forbid this even when
ordinary sequential calls are legal. A binding that guesses can race shared
state or deadlock on a lock already held by the outer call.

Read these as three independent questions: answering one does not answer the
other two.

| Contract dimension | Question the binding needs answered |
|---|---|
| Entry executor | Where may the caller enter the operation? |
| Concurrency | May independent calls overlap? |
| Re-entrancy | May a nested call enter on the same receiver or resource while an earlier call is active? |

In summary (non-normative): AIL separates nominal sharing and
[isolation](../glossary.md#isolation) from each callable's entry executor,
concurrency and re-entrancy.
[Shared](../glossary.md#shared) permits references to cross executors; it does
not promise safe concurrent mutation ([`C2`](../contracts.md#c2),
[`C3`](../contracts.md#c3)). Callback invocations have their own schedule
constraints ([`C6`](../contracts.md#c6)).

### When is it safe to discard callback state?

Suppose all notifications finish before `watch_open` returns. The caller could
then use temporary context storage. Compare a watch that stores `notice` and
`context` and delivers notifications later. In that case discarding context
storage on return leaves a later callback with a dangling pointer.

Now compare two teardown promises: one merely requests that delivery stop;
the other returns only when no callback is running or can start. Only the
second establishes [quiescence](../glossary.md#quiescence), the boundary after
which callback state is no longer needed for those invocations. “Cancelled” or
“closed” is not enough information. Waiting for quiescence inside the callback
being waited for can also deadlock.

[Callback captures](../glossary.md#callback-captures) are the state a callback
needs, here reachable through `context`. This is the captured-environment sense
of “closure”, not a [dependency closure](../glossary.md#dependency-closure).
Invocation count, start timing, executor, permitted overlap and teardown each
need an answer. “Later” does not mean “exactly once”.

In summary (non-normative): AIL describes a callback type separately from the
schedule of each registration. The core supports callbacks confined to the
registration call or to the lifetime of a returned unique owner with quiescent
destruction. A blocking destructor requires a call from outside the callback
unless a profile supplies a non-deadlocking protocol. Cancellation with a
separate acknowledgement needs a profile; cancellation return cannot stand in
for quiescence ([`C2`](../contracts.md#c2), [`C6`](../contracts.md#c6)).

## Why the signature cannot be the contract

For each pair above, the declarations leave room for different obligations. A
binding generator needs to choose release operations and protect lifetimes; a
documentation renderer needs to tell callers when bytes remain usable; LLM
tooling needs those same facts to avoid suggesting an invalid call sequence.
Without a shared account, each consumer has to rediscover or guess them.

The [publication map](../README.md#the-problem-in-brief) places these consumers
around the same contract and shows the boundary every producer crosses.

AIL's choice is one authoritative [package](../glossary.md#package) describing
the selected API [surface](../glossary.md#surface).
The intended producers are human authors, analysers and third-party tools; none
gets a weaker publication standard. A smaller declared surface is legitimate,
but everything it depends on still needs a contract. That transitive set is its
[dependency closure](../glossary.md#dependency-closure), not callback captures.

In summary (non-normative): [validation](../glossary.md#validation) checks
completeness and internal consistency; it cannot establish that a binary
honours the author's assertions.
The producer still needs evidence for them ([`L2–L3`](../language.md#l2),
[`C8`](../contracts.md#c8)). This choice makes authoring demanding: an unresolved
obligation blocks publication instead of becoming a confidence score in the
package. The [design proposal](../../../docs/design/2026-09-22-interface-language.md#3-publication-boundary)
explains that boundary.

## Reaching the code

Knowing what a call means still leaves the problem of making it. Compare calling
a function through a linker symbol with invoking an object method through a
runtime message send. A third possibility is compiling a source-level call when
there is no directly usable symbol. These are different
[access paths](../glossary.md#access-path); a source spelling does not itself
establish any of them ([`A2`](../profiles.md#a2)).

The [application binary interface (ABI)](../glossary.md#abi) governs the machine
boundary: value representation and layout, argument and result passing, and
runtime requirements. A [calling convention](../glossary.md#calling-convention)
is the call protocol within that larger definition. Matching the call protocol
while disagreeing about a record's layout can still cause the callee to
misinterpret the caller's arguments
([`A1`](../profiles.md#a1), [`A3`](../profiles.md#a3)).

AIL separates those mechanics from the contract. In summary (non-normative), a
[profile](../glossary.md#profile) defines versioned source or access semantics;
it is a language extension, not a preference preset. An ABI entry defines the
representations and calling conventions. [Providers](../glossary.md#provider)
supply these definitions through manifests and tooling. A package selects them
by pinned identity rather than embedding an ABI algorithm. Source access names
a build recipe; it does not promise independently stable binary compatibility
([`A1`](../profiles.md#a1), [`A4`](../profiles.md#a4)).

For this header, a [mapping](../glossary.md#mapping) would need to account for
both the callback and its context argument. This page does not supply that mapping. The existing
[synthetic callback example](../examples/callbacks.ail) uses a synthetic fixture
profile, catalogue and digest tokens whose callback representation has no
separately visible context argument. It is not an AIL translation of this C
header ([`A2`](../profiles.md#a2), [`F1`](../profiles.md#f1)).

A [projection](../glossary.md#projection) is a consumer's presentation of the
contract. Prose and a typed object model can look different while preserving
access to the same facts. A convenience operation that hides a meaningful
success status changes what callers can observe; the producer describes it with
a distinct declaration rather than another path to the same operation. A valid
package can also exceed a particular binding generator's capabilities. These are
different questions: whether the specification is valid and whether a consumer
can implement its binding
([`A2`](../profiles.md#a2), [`A5`](../profiles.md#a5)).

## What your language-design knowledge covers, and where it misleads

Your knowledge of types and notation helps you judge whether these distinctions
are readable and consistent. Familiar names can nevertheless suggest guarantees
AIL does not make. These are non-normative reminders; the linked rules decide.

| Familiar term | Read it here as… | Do not infer… |
|---|---|---|
| [Borrow](../glossary.md#borrow) | Access without transferred ownership, for a stated region ([`C1`](../contracts.md#c1), [`C3`](../contracts.md#c3)) | Rust's static borrow checking or exclusivity |
| [Owned](../glossary.md#owned) | One destruction or release obligation ([`C2`](../contracts.md#c2), [`C5`](../contracts.md#c5)) | Unique access; a reference-counted object can have several owned references |
| [Consume](../glossary.md#consume) | Transfer at entry, on failure as well as success ([`C3`](../contracts.md#c3)) | A move only if the callee succeeds |
| [Shared](../glossary.md#shared) | References may cross executors ([`C2`](../contracts.md#c2)) | Race-free concurrent mutation |
| [Copy](../glossary.md#copy) | An independent semantic copy defined by a profile ([`C3`](../contracts.md#c3)) | A pointer-bit copy, retain, or Rust's `Copy` trait |
| [Profile](../glossary.md#profile) | A versioned semantic extension with validation rules ([`A1`](../profiles.md#a1)) | A performance trace or a set of optional preferences |
| [Closure](../glossary.md#closure) | Either dependency closure or callback captures, always qualified here ([`L2–L3`](../language.md#l2), [`C2`](../contracts.md#c2)) | That the two senses describe the same relationship |

## Where next

The forthcoming concept pages are *How AIL states a contract*, *Packages,
environments and access paths*, and *Design choices, limits and open questions*;
the tutorial is *Describing a small API*.

For now, continue to [the memory and behavioural rules](../contracts.md), using
the [glossary](../glossary.md) for terms. Return to [the overview](../README.md)
for the full document map.
