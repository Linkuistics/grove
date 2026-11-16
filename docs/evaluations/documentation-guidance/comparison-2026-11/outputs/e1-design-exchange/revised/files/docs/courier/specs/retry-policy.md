# retry-policy

Courier waits 1,200 seconds for an SMS receipt before it tries an ordinary SMS notice again.
Legal notices and the other channels keep the 300-second wait of today. The owner of Courier's
platform team agreed to this decision on 2026-11-17.

## Problem

Courier's SMS bill is up by about a third since March. The cost comes from repeat attempts that
pay for texts that were already on their way.

Three facts about SMS providers explain the cost:

- A provider bills by the **[segment](../channels.md#sms-segments)**. A message of up to 160 plain
  characters is one segment, and a longer message is several. Each attempt is billed for every
  segment of the message, every time Courier makes it.
- A provider answers each attempt twice. It accepts the attempt at once. Later it sends a
  **[delivery receipt](../receipts.md)** that says whether the message reached the recipient.
  Courier treats an attempt as confirmed only when its receipt arrives.
- An SMS receipt passes through the recipient's mobile carrier, so it can arrive up to 20 minutes
  after the attempt. This **[receipt delay](../receipts.md#when-a-receipt-arrives)** belongs to
  the provider, and Courier cannot shorten it. Email and push receipts arrive within a minute.

Today Courier waits `interval_seconds`, 300 seconds, for a receipt on every channel. After that
time, the notice goes to the [hold queue](../hold-queue.md), which the owner calls the backlog.
The router then makes the next attempt, and the provider bills it in full. A receipt that arrives
later is recorded, but it does not cancel the attempt that was already made.

The attempt report over last month's sample (`data/attempts-sample.csv`) shows the effect on
`ch-sms`:

| Measure on `ch-sms` | Value |
|---|---|
| Attempts whose receipt took longer than 300 s | 90 of 334 |
| Median receipt delay | 203 s |
| Slowest receipt delay | 1,174 s |
| Repeat attempts | 94 |
| Repeats sent while an earlier attempt of the notice was still on its way | 81 of 94 |
| Segments billed for first attempts | 363 |
| Segments billed for repeats | 128, which is 35 % on top of the first attempts |
| Notices whose first attempt never got a receipt | 13 |

Of the 128 repeat segments, 109 paid for texts that were already on their way. Email and push have
no such repeats, because their receipts arrive in under a minute.

> **Example:** Notice `N-00002` in the sample. Courier sends attempt 1 at 0 s. No receipt has
> arrived at 300 s, so the notice goes to the hold queue. Courier sends attempt 2 at 320 s, and the
> provider bills it. The receipt for attempt 2 arrives at 560 s. The receipt for attempt 1 arrives
> at 701 s, and it shows that the first text was delivered all along.

## Decision

The retry wait becomes a setting per channel. For `ch-sms`, an ordinary notice waits 1,200 seconds
for a receipt before it goes to the hold queue. A notice marked `legal` waits 300 seconds on every
channel, as it does today. `ch-email`, `ch-push` and `ch-voice` keep 300 seconds.

The reason: the owner prefers a late text to a text paid for twice. In the owner's words (m00036):

> "I'd rather wait longer before Courier tries again. A text that really was lost arriving later
> is a price I'll pay, and paying twice for one that arrived is not. Keep the change in the retry
> timing that I can inspect in our own code."

1,200 seconds is the 20-minute limit of the receipt delay. It is above the slowest receipt in the
sample, which is 1,174 seconds.

The owner restated the decision in their own words when they agreed to it (m00036):

<!-- tc: depart long-sentence because the owner's restatement is quoted verbatim -->
> "In my own words: for SMS, Courier waits longer before it tries a notice again, long enough for
> the carrier to say it delivered the text, and we stop paying for texts that had already
> arrived."

For legal notices, the owner restated the exception in their own words (m00043):

> "In my own words: legal notices keep retrying every five minutes as they do today, so the new
> wait does not apply to them."

### What the decision does to the sample

On last month's sample, the 1,200-second wait gives these results on `ch-sms`:

- None of the 81 repeats that paid for texts already on their way happens. This saves 109
  segments.
- Each of the 13 texts whose first attempt got no receipt gets its retry after about 20 minutes,
  not after about 5 minutes.

The sample does not mark which notices are legal. So it cannot show how much the repeats on
legal SMS notices cost.

### How the router picks the wait

The router picks the wait for each attempt in this order:

1. If the notice is marked `legal`, the wait is the top-level `interval_seconds`.
2. If the notice's channel has its own `interval_seconds`, the wait is that value.
3. If neither applies, the wait is the top-level `interval_seconds`.

`max_attempts` and `default_ttl_hours` stay as they are, and they still apply to every channel.

### Consequences the owner accepted

- A text that the provider really lost reaches the recipient about 15 minutes later than it does
  today.
- A legal SMS notice can still be billed twice. The owner said (m00043): "A repeat on a legal
  notice is a price I'll pay."
- An ordinary SMS notice that expires less than 1,200 seconds after its first attempt gets no
  retry. No rule requires a retry for an ordinary notice.

## How the decision keeps the rules

The [rules](../rules.md) bind every change. The decision touches all three.

- **[RULE-3](../rules.md#rule-3-opt-out)**, opt-out. The router reads the recipient's opt-outs before
  each attempt, as it does today. The decision changes only when an attempt happens, so it adds no send
  to an opted-out recipient.
- **[RULE-7](../rules.md#rule-7-legal-notices)**, legal notices. A legal notice keeps the
  300-second wait. A sender can set a legal notice to expire after 10 minutes, so a
  1,200-second wait would let that notice expire with no retry. The 300-second wait keeps the
  retries that the rule requires. A legal notice to an opted-out recipient is still not retried,
  and it is still recorded as `suppressed`.
- **[RULE-9](../rules.md#rule-9-the-record)**, the record. The router records every attempt, as
  it does today. A receipt that arrives after the router has made the next attempt is still
  recorded against its attempt.

## Options not chosen

| Option | What it would change | What it would cost |
|---|---|---|
| Idempotency key per notice | Every SMS attempt of a notice carries the same [idempotency key](../channels.md#idempotency-keys). The provider does not send or bill a second attempt with a key it already has. | A retry with the same key is never sent. In the sample, 13 first attempts got no receipt, so their retries would not have reached the recipient. The owner also wants the change in Courier's own settings, and this option moves it to the provider. |
| Cap the attempts | `max_attempts` drops from no cap to a small number, such as 2. | It stops only the third and later attempts, so most double billing stays. It breaks `RULE-7`, because a legal notice must be retried until it is delivered or it expires. |
| The legal wait shrinks to fit before expiry | A legal notice waits 1,200 seconds, or half the time left before it expires, whichever is shorter. | It needs a rule in the router's code, not only a number in the settings. Legal notices with a short expiry are still billed twice. The owner chose to keep legal retries exactly as they are. |

## What the implementer changes

| File | Change |
|---|---|
| `config/retry.toml` | Add a `[channel.ch-sms]` table with `interval_seconds = 1200`. Keep the top-level `interval_seconds = 300`, and note in a comment that legal notices use it on every channel. |
| The router's code | Read the per-channel tables. Pick the wait for each attempt in the order that [How the router picks the wait](#how-the-router-picks-the-wait) gives. The router's code is not in this checkout. |
| The router's tests | Add the tests in [Test seam](#test-seam). |
| `docs/courier/retry.md` | Say that `interval_seconds` can be set per channel, and that legal notices always use the top-level value. Remove the sentence that says a setting is not yet kept per channel. |
| `docs/courier/receipts.md` | Say which wait applies before an attempt counts as unconfirmed. An ordinary notice takes the wait for its channel. A legal notice takes the top-level wait. |

The settings file after the change:

```toml
# The router's retry settings.
# The top-level interval_seconds applies to every channel without its own table,
# and to every legal notice on any channel.
interval_seconds = 300
max_attempts = 0
default_ttl_hours = 24

[channel.ch-sms]
# Ordinary SMS notices only. An SMS receipt can take up to 20 minutes.
interval_seconds = 1200
```

## Test seam

The tests run the router against a fake SMS provider and a clock that the test controls. Each test
chooses when each receipt arrives, or that none arrives. Each test then counts the billed
attempts.

The owner had no view on these cases and accepted the recommended set (m00092).

| Case | Expected result |
|---|---|
| An ordinary SMS notice, with its receipt at 1,100 s | One billed attempt. |
| An ordinary SMS notice, with no receipt | A second attempt at 1,200 s. |
| A legal SMS notice, with no receipt | A second attempt at 300 s. |
| An ordinary email notice, with no receipt | A second attempt at 300 s. |

After release, run `python3 tools/attempt_report.py` over a new log of attempts. The `avoidable`
count for `ch-sms` must be near zero.

## Out of scope

- Email, push and voice. Their receipts arrive within a minute, and the sample shows no repeats on
  them that paid for a text already on its way.
- Idempotency keys. The owner chose to keep the change in Courier's own settings.
- Cancelling an attempt when a late receipt arrives. With the longer wait, the sample shows no
  receipt that arrives after the next attempt on an ordinary SMS notice.
