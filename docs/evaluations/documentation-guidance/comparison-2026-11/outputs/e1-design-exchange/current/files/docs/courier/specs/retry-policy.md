# Retry policy: wait for the SMS receipt

The owner of this decision runs Courier's platform team and agreed to it on 2026-11-17. The
decision gives each channel its own retry settings. It sets the `ch-sms` retry interval to 1,500
seconds (25 minutes). Courier then stops paying a second time for a text that is already on its
way. The other channels and all `legal` notices keep their current behaviour.

## Three terms from SMS billing

This spec relies on three facts about SMS providers. [Channels](../channels.md) and
[Receipts](../receipts.md) hold the source text.

- **Segment.** A segment is the unit that an SMS provider bills. A message of up to 160 plain
  characters is one segment. A longer message is split into several segments. The provider
  bills every segment of a message each time Courier sends it, at $0.0075 for each segment.
- **Delivery receipt.** A provider answers each attempt twice. It accepts the attempt at once.
  Later it sends a delivery receipt that says whether the message reached the recipient. Courier
  treats an attempt as confirmed only when its receipt arrives.
- **Receipt delay.** An SMS receipt passes through the recipient's mobile carrier, so it can
  arrive up to 20 minutes (1,200 seconds) after the provider accepts the attempt. Courier cannot
  shorten this delay.

## The problem

The router waits `interval_seconds` (300) for a receipt. After that time, it puts the notice in
the [hold queue](../hold-queue.md) and makes another attempt, which the provider bills again. An
SMS receipt can take four times as long as that wait. Thus Courier often retries a text that has
not failed. The receipt for the first attempt is still on its way.

Last month's log, `data/attempts-sample.csv`, shows the effect. The owner treats this log as
typical. `tools/attempt_report.py` and a count over the same log give these figures for
`ch-sms`:

| Measure | Value |
|---|---|
| Notices | 240 |
| Attempts | 334 |
| Repeat attempts | 94 |
| Repeats sent while an earlier receipt was still on its way | 81 |
| Repeats after a first attempt that never got a receipt | 13 |
| First attempts with a receipt later than 300 seconds | 62 |
| Slowest receipt for a first attempt | 1,169 seconds |
| Slowest receipt for any attempt | 1,174 seconds |
| Segments billed in total | 491 |
| Segments billed for repeats | 128 |

The 128 repeat segments are 35% more than the 363 segments of the first attempts. This matches
the rise of about a third in the SMS bill since March. Only the 13 repeats after a lost first
attempt did useful work.

## The decision

Each channel can have its own retry settings. The `ch-sms` channel waits 1,500 seconds for a
receipt before it retries. Every other channel keeps 300 seconds for now. Each `legal` notice
keeps today's settings on every channel.

The reason is the owner's priority. A second charge for a text that arrived costs money and gives
nothing. A lost text that arrives 20 minutes later is a cost that the owner accepts. The
1,500-second interval is 300 seconds longer than the provider's longest receipt delay. Thus a
receipt for a text that is on its way arrives before the retry.

The change is in Courier's own retry timing, where the team can read it and test it. It does not depend on any behaviour on the
provider's side.

### Expected effect

Under the new interval, no first SMS attempt in the sample has its receipt after 1,500 seconds.
Thus none of the 81 early repeats happens. Each of the 13 notices with a lost first attempt gets
one retry, at about 1,500 seconds. The sample bills 19 segments for those retries.

| Measure | Today | With this decision |
|---|---|---|
| SMS attempts | 334 | 253 |
| SMS segments billed | 491 | 382 |
| SMS segment cost in the sample | $3.68 | $2.87 |

The SMS segment bill falls by 22%. It falls to 5% above the cost of the first attempts alone. The
253 attempts and the 382 segments assume that each of the 13 retries gets its receipt within
1,500 seconds. In the sample, the slowest receipt took 1,174 seconds.

### The settings file

The top-level keys of `config/retry.toml` stay. They are the defaults for every channel. A table
`[channels.<channel id>]` can give one channel its own value for any of the three keys. A key that
the table does not give comes from the defaults.

```toml
# Defaults for every channel. Each legal notice uses these values on every channel.
interval_seconds = 300
max_attempts = 0
default_ttl_hours = 24

# A text's delivery receipt can take up to 1,200 seconds. Wait longer than that before a retry.
[channels.ch-sms]
interval_seconds = 1500
```

The router obeys these rules when it reads the file:

- When the router chooses the settings for a notice that is not `legal`, it uses the notice's
  channel table first. It uses the defaults for each key that the table does not give.
- When the router chooses the settings for a `legal` notice, it uses the defaults only. It
  ignores every channel table.
- When a channel table has a key other than the three above, the router does not start. It
  names the key and the table in its error.
- When a table names a channel id that is not one of `ch-email`, `ch-sms`, `ch-push` or
  `ch-voice`, the router does not start. It names the table in its error.

The router reads the hold queue once a minute. Thus a `ch-sms` retry happens between 1,500 and
1,560 seconds after the attempt before it.

### Consequences

- **A lost text arrives later.** A text whose first attempt is lost gets its retry after about
  25 minutes, not 5 minutes. The owner accepts this delay.
- **Legal notices still pay twice.** A `legal` SMS still retries after 300 seconds. Some of those
  retries pay twice for a text that arrived. The owner accepts this cost, because `RULE-7`
  comes first.
- **The defaults bind legal notices.** A future change to a top-level key also changes the
  behaviour of every `legal` notice. A change to a channel table does not change the behaviour of
  a `legal` notice.
- **A short expiry can mean one attempt.** A notice that is not `legal` and expires less than
  1,500 seconds after its first `ch-sms` attempt gets that one attempt only. Today such a notice
  gets a retry after 300 seconds.

## How the decision keeps the rules

The three rules are in [Rules](../rules.md).

- **RULE-3, opt-out.** The decision changes when a retry happens. It does not change who
  receives a retry or over which channel. The decision does not touch `RULE-3`.
- **RULE-7, legal notices.** A `legal` notice uses the defaults, which are today's settings. It
  retries every 300 seconds until it reaches the recipient or its `expires` time passes. A legal notice
  for a recipient who opted out of the channel is still not retried, and Courier still records
  it as `suppressed`. The router ignores channel tables for a `legal` notice. Thus no channel
  setting can lengthen the interval or cap the attempts of a legal notice.
- **RULE-9, the record.** The decision makes fewer attempts. Courier records each attempt that it
  makes, as it does today. A receipt that arrives late is still recorded against its attempt.

## Options not chosen

### Keep the 300-second interval

This option changes nothing. It costs 109 more segments for each 491 in the sample. That is 22%
of the SMS bill, spent on texts that were already on their way.

### A 1,200-second interval for SMS

This interval is equal to the provider's longest receipt delay, and it leaves no margin. A receipt
that arrives at the limit of the delay can arrive after the retry. Then Courier pays twice. The
owner chose 1,500 seconds to put cost before speed.

### An SMS-only change

The router could treat `ch-sms` as a special case in its code. This option costs a second code
change when another channel needs its own interval. The owner asked for a setting for each
channel.

### Idempotency keys

An idempotency key is a value that Courier can send with each attempt. When a second attempt has
the same key as an earlier attempt, the provider treats it as a repeat. It does not send the
message again, and it does not bill it. The `ch-sms` provider remembers each key for 24 hours.

One key for each notice would stop every second charge. But the documented behaviour is that the
provider ignores a repeat whether or not the first attempt reached the recipient. Thus a text that
was lost would not be sent again within 24 hours. That would cost the 13 useful retries in the
sample. It would also break `RULE-7` for a `legal` SMS. The behaviour would also be on the
provider's side, where the team cannot read it or test it.

### A cap on attempts

A `max_attempts` of 2 would stop the third and fourth attempts. It would not stop the early second
attempt, which is most of the cost. A cap also stops retries for a text that is lost twice. The
cap cannot apply to a `legal` notice, because of `RULE-7`.

## Test seam

The test seam is the function in the router that returns the retry settings for a notice. Its
inputs are the notice's channel, whether the notice is `legal`, and the parsed settings file. Its
output is the three settings. The tests drive that function directly, with these cases:

- A `ch-sms` notice that is not `legal` gets `interval_seconds = 1500`.
- A `ch-email` notice gets the defaults.
- A `legal` `ch-sms` notice gets the defaults.
- A channel table with an unknown key stops the router from starting.
- A table for an unknown channel id stops the router from starting.

## What the implementer changes, by file

| File | Change |
|---|---|
| `config/retry.toml` | Add the `[channels.ch-sms]` table with `interval_seconds = 1500`, and the two comments, as in [the settings file](#the-settings-file). |
| The router's settings loader | Read the `[channels.<channel id>]` tables, and refuse an unknown key or an unknown channel id. This checkout does not hold the router's source. |
| The router's retry scheduler | Get the settings for each notice from the settings function, and stop reading one global interval. |
| The router's tests | Add the five cases in [Test seam](#test-seam). |
| `docs/courier/retry.md` | Say that each channel can have its own settings, and that a `legal` notice uses the defaults. Remove "A setting is not yet kept per channel." |
| `docs/courier/receipts.md` | Say that the wait for a receipt is the `interval_seconds` of the notice's channel. |
