# Exchange transcript

The human side is a labelled model proxy. It picks scripted answers from
`cases/exchange/answer-sheet.json`; it writes no words of its own.

## Turn 1

### Session

I have what I need. One finding changes the picture, so here is the evidence first and then the decision I need from you.

## What is costing you money

You are right that repeat attempts are the extra cost. But the notices are not failing. Courier stops waiting too early.

These terms come from the SMS provider's side, so here they are first:

- **Segment.** The provider bills by the segment. A message of up to 160 plain characters is one segment, and a longer message is several. Each attempt is billed for every segment, every time.
- **Delivery receipt.** The provider answers an attempt twice. First it accepts the attempt at once. Later it sends a receipt that says whether the message reached the phone. Courier treats an attempt as confirmed only when that receipt arrives.
- **Receipt delay.** An SMS receipt goes through the recipient's mobile carrier, so it can take up to 20 minutes. Courier cannot shorten this delay. Email and push receipts arrive within a minute.

Courier waits 300 seconds (`interval_seconds`) for a receipt. After that time, it puts the notice in your backlog (the docs call it the hold queue) and sends again. In most cases, the first message is still on its way, and the provider bills the second one in full.

I ran the attempt report on last month's sample (`data/attempts-sample.csv`):

| Measure on `ch-sms` | Value |
|---|---|
| Receipts that took longer than 300 s | 90 of 334 attempts |
| Median receipt delay | 203 s |
| Slowest receipt delay | 1,174 s, about 20 minutes |
| Repeat attempts | 94 |
| Repeats sent while an earlier attempt was still on its way | 81 of the 94 |
| Segments billed for first attempts | 363 |
| Segments billed for repeats | 128, which is 35 % on top of the first attempts |

That 35 % matches your "about a third". Of the 128 repeat segments, 109 paid for messages that were already on their way. Email and push have no such repeats, because their receipts come in under a minute.

A timeline sketch of one notice today (only SMS, one retry shown):

```
t=0s     Courier ──sends attempt 1──▶ provider        (billed)
t=300s   no receipt yet ──▶ notice goes to the backlog
t=320s   Courier ──sends attempt 2──▶ provider        (billed again)
t=560s   receipt for attempt 2 arrives
t=701s   receipt for attempt 1 arrives: it was delivered all along
```

This is notice `N-00002` in the sample.

## One more provider term

An **idempotency key** is a value that Courier can send with an attempt. When the provider sees a second attempt with a key that it already has, it treats that attempt as a repeat. It does not send the message again and it does not bill it. The SMS provider supports keys and remembers each one for 24 hours. The email provider supports them too. Push and voice do not.

The catch: a repeat with the same key is never sent. If the first message really was lost, a retry with the same key does nothing. So the key must change once the provider says the first message did not arrive.

## The options

| | A. Idempotency key per notice (recommended) | B. Longer SMS wait | C. Cap the attempts |
|---|---|---|---|
| What changes | Every SMS attempt of a notice carries the same key. A receipt that says "not delivered" makes Courier start a new key. | `interval_seconds` becomes per channel, and SMS waits 1,200 s. | `max_attempts` drops from no cap to, for example, 2. |
| Double billing in the sample | About 19 of 128 repeat segments are still billed. Those are real resends after a failed delivery. | Nearly all of the 109 avoidable segments go. | Only the third and later attempts go. |
| Retry timing | Unchanged. | A message that really was lost waits 20 minutes for its retry. | Unchanged. |
| Rules | Keeps all three. | Breaks `RULE-7`: a `legal` notice can expire in 10 minutes and would get no retry. | Breaks `RULE-7`: a `legal` notice must be retried until it is delivered or it expires. |

The principle that separates them: A stops the bill at the provider and leaves Courier's retries as they are. B and C stop the bill by making fewer retries, and the legal notices need those retries.

The limit of A: it relies on two provider facts. The provider always sends a receipt, and it keeps a key for 24 hours. If a sender sets `expires` longer than 24 hours, a retry after the provider forgets the key is billed again. That case is rare, and its worst cost is one extra bill.

## What I recommend, and how it would be tested

I recommend **A** for `ch-sms`. Every attempt is still recorded (`RULE-9`), and an attempt that the provider treated as a repeat is recorded as such. The opt-out check stays before every attempt (`RULE-3`). Legal notices keep their retries (`RULE-7`). Email could take the same key later, but it has no avoidable repeats today, so I would leave it out.

For tests, I propose one seam: the router makes its attempts against a fake SMS provider. The fake keeps keys the way the real provider does, and it sends receipts after a delay that the test chooses. Each test then counts the billed sends. As an extra check after release, the attempt report on a new log should show the avoidable repeats near zero.

## What I need from you

1. Choose A, B, C or something else.
2. Tell me whether the test seam matches what you expect.
3. Say the decision back in your own words. I will record your words in the spec beside your answer.

### Human (model proxy; keys: unknown-segment, unknown-delivery-receipt, unknown-receipt-delay, unknown-idempotency-key, decision-approach, restate-approach)

I haven't had to deal with segments before. If you have explained them above, I follow; if not, please explain what they are before you rely on them.

I haven't had to deal with delivery receipts before. If you have explained them above, I follow; if not, please explain what they are before you rely on them.

I didn't know how long a receipt could take. If you have told me above, I follow; if not, please tell me before you rely on it.

I haven't had to deal with idempotency keys before. If you have explained them above, I follow; if not, please explain what they are before you rely on them.

I'd rather wait longer before Courier tries again. A text that really was lost arriving later is a price I'll pay, and paying twice for one that arrived is not. Keep the change in the retry timing that I can inspect in our own code.

In my own words: for SMS, Courier waits longer before it tries a notice again, long enough for the carrier to say it delivered the text, and we stop paying for texts that had already arrived.

## Turn 2

### Session

Thank you. My last message explained segments, delivery receipts, the receipt delay of up to 20 minutes and idempotency keys before the options used them, so nothing rested on an unexplained term. I record your choice as **B: SMS waits longer before Courier tries a notice again**, with the change kept in our own retry settings. I also record your restatement as you wrote it.

## A correction that supports your choice

I checked the sample again for B. In 13 SMS notices, the first attempt never got a receipt at all. My recommendation of A assumed that a provider always sends a receipt. It does not. Under A, those 13 retries would have carried the old key, and the provider would not have sent them. So A was weaker than I said, and B avoids that problem.

On the sample, B works as follows. The slowest receipt took 1,174 s. So with a wait of 1,200 s, none of the 81 repeats that paid for texts already on their way would happen. The 13 lost texts each get one retry, at about 20 minutes instead of about 5.

## One rule that B breaks as it stands

Every change must keep `RULE-7`. That rule says a `legal` notice must be retried until it is delivered or its expiry time passes. A sender can set a legal notice to expire after 10 minutes. With a 1,200 s wait, that notice expires before Courier tries again. That breaks the rule.

There are two ways to keep the rule:

| | 1. Legal SMS notices keep the 300 s wait (recommended) | 2. A legal notice's wait shrinks to fit before its expiry |
|---|---|---|
| What changes | Ordinary SMS notices wait 1,200 s. Legal SMS notices wait 300 s, as today. | A legal notice waits 1,200 s, or half the time left before it expires, whichever is shorter. |
| Paying twice | Legal SMS notices can still be billed twice. | Legal notices with a long expiry are not billed twice. Short ones can be. |
| What you inspect | Two fixed numbers in `config/retry.toml`. | One number plus a rule in the router's code. |

I recommend 1. It is the simpler rule to read in your own settings, and that is what you asked for. Its cost: legal SMS notices can still pay twice. The sample log does not mark which notices are legal, so I cannot tell you how much that costs.

The settings would look like this. Today every setting applies to all channels, so this adds a table for one channel:

```toml
interval_seconds = 300          # every channel, and legal SMS notices
max_attempts = 0
default_ttl_hours = 24

[channel.ch-sms]
interval_seconds = 1200         # ordinary SMS notices only
```

## How it would be tested

The test runs the router against a fake SMS provider and a clock that the test controls. The test chooses when each receipt arrives, or that none arrives. Each test then counts the billed attempts. For example, a receipt at 1,100 s gives one billed attempt. No receipt gives a retry at 1,200 s. A legal notice with no receipt gives a retry at 300 s.

## What I need from you

1. Choose 1 or 2 to keep `RULE-7`.
2. Tell me whether the test above matches what you expect.
3. If you choose 1, the decision is no longer only "SMS waits longer". Please add a sentence in your own words about legal notices, and I will record it beside your first restatement.

### Human (model proxy; keys: decision-legal-notices, restate-legal-notices)

Legal notices keep retrying exactly as they do today, whatever we decide for the rest. A repeat on a legal notice is a price I'll pay.

In my own words: legal notices keep retrying every five minutes as they do today, so the new wait does not apply to them.

## Turn 3

### Session

> **Runner repair.** The first runner kept only the last message of this call. The proxy saw only the message after the line below. The session's earlier message is restored here from the trace.

I've written the retry policy spec at `docs/courier/specs/retry-policy.md`. It records the design you agreed to, and the gate passes.

**The decision.** Courier waits 1,200 seconds for a receipt before it tries an ordinary SMS notice again. That is 20 minutes, the longest an SMS receipt can take. Legal notices keep retrying every five minutes, as they do today. Email, push and voice also keep the 300-second wait. The change is a `[channel.ch-sms]` table in `config/retry.toml`, so you can read it in your own settings. Both of your restatements are quoted in the spec beside your answers.

**What the sample shows.** I ran the attempt report on last month's log. Courier gives up after 300 seconds, but SMS receipts can take up to about 20 minutes. So 109 of the 128 repeat SMS segments paid for texts that were already on their way. Those repeats came to 35 % on top of the first attempts, which matches your "about a third".

With the 1,200-second wait, none of those wasted repeats would happen. In 13 notices the first text never got a receipt, and each of those gets one retry after about 20 minutes instead of about 5.

**A correction to my earlier advice.** I first recommended the idempotency key. It assumed that the provider always sends a receipt, and those 13 notices show that it does not. Under that option, their retries would never have been sent. Your choice avoids the problem, and the spec gives this as a cost of the option you did not choose.

**What the spec holds.** The contract asks for four things, and the spec covers each:
- **The decision and its reason:** your preference for a late text over paying twice.
- **The options you did not choose:** the idempotency key, a cap on attempts, and a legal wait that shrinks to fit before expiry, each with its cost.
- **The rules:** how the decision keeps `RULE-3`, `RULE-7` and `RULE-9`.
- **The changes, file by file:** the settings file, the router and its tests, and the two docs that describe retry timing.

It also explains segments, receipts and the receipt delay for a reader who is new to them.

**Checks.** The gate passes: all 11 links and anchors resolve in 8 files. The writing checker finds no hard errors. Its 6 remaining notes are suggestions about passive voice and one tense, which I left as they are. Your 36-word restatement is over the sentence limit, and I left it that way because it is a direct quote.

**Still open:**
- **The test plan.** I asked twice whether it matches what you expect, and you did not say. The plan runs the router against a fake SMS provider and a clock the test controls, then counts the billed attempts. The spec records this plan, but you have not confirmed it.
- **The router's code.** It is not in this checkout, so the spec names what to change there but not the file. Whoever builds the change needs to find it.
- **Legal SMS repeats.** The sample does not mark which notices are legal. So nobody can yet say how much the repeats on legal SMS notices will still cost.

As the session rules say, I have not retired the leaf, committed anything or ended the session.

---

No new message from you has arrived. The latest event is a system notice: a file search I started earlier ran past its time limit and was stopped. That search was for the writing checker. I had already found the checker another way and used it on the spec, so the stopped search changes nothing, and I am not running it again.

The spec at `docs/courier/specs/retry-policy.md` and my last report stand as written. One question is still waiting for you:

- **Does the test plan match what you expect?** I recommend keeping it as written in the spec. The tests run the router against a fake SMS provider, on a clock the test controls. Each test sets when each receipt arrives, or that none arrives, then counts the billed attempts. There are four cases:
  - An ordinary SMS notice whose receipt arrives at 1,100 seconds is billed once.
  - An ordinary SMS notice with no receipt is tried again at 1,200 seconds.
  - A legal SMS notice with no receipt is tried again at 300 seconds.
  - An ordinary email notice with no receipt is tried again at 300 seconds.

If you agree, I note your confirmation in the spec. If you expect different cases or a different way of testing, tell me and I change the test section to match.

### Human (model proxy; keys: fallback)

I have no view on that. Use your recommendation.

## Turn 4

### Session

The spec now uses my recommended test plan. I added a line to the test section of `docs/courier/specs/retry-policy.md` to record your answer. It says that you had no view on the test cases and accepted the recommended set.

**Checks:** The gate still passes, and all 11 links and anchors resolve in 8 files. The writing checker finds no hard errors. It shows the same 6 suggestions as before, about passive voice and one tense, and I left them as they are. It also shows the one exception I marked, for your quoted restatement.

**Still open:**
- **The router's code:** it is not in this checkout. The spec says what to change in the router, but it cannot name the file. Whoever builds the change has to find it.
- **Legal SMS repeats:** the sample does not mark which notices are legal. So nobody can yet say how much the repeats on legal SMS notices will still cost.

As the session rules say, I have not retired the leaf, committed anything or ended the session.

### Human (model proxy; keys: none)

I have nothing to add. Please continue.
