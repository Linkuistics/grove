# Exchange transcript

The human side is a labelled model proxy. It picks scripted answers from
`cases/exchange/answer-sheet.json`; it writes no words of its own.

## Turn 1

### Session

The repeat SMS charges come from retrying too early, before the provider has reported back. Courier retries messages that have not actually failed. Below is what last month's sample shows, the design I recommend, and the questions I need you to answer.

## What the sample shows

`tools/attempt_report.py` counts 334 SMS attempts for 240 notices in the sample, so 94 attempts were repeats. In 81 of those 94, the earlier attempt had not failed. Its receipt simply arrived after Courier had sent the repeat. Courier paid twice for a message that was already on its way.

To see why, you need two facts about how an SMS provider works:

- **Two answers per message.** The provider accepts a message at once. Later it sends a *delivery receipt* to say whether the message reached the phone. Courier treats an attempt as confirmed only when that receipt arrives.
- **Slow receipts.** An SMS receipt passes through the recipient's mobile carrier, so it can take up to 20 minutes. In the sample, the slowest took 1,174 seconds.
- **The timeout is too short.** Courier waits `interval_seconds = 300` for a receipt, then sends again, and the provider bills each send. In the sample, 62 first attempts had their receipt after more than 300 seconds.

The repeats came to 128 of the 491 segments billed. A segment is the unit the provider bills, about 160 characters. That is 35% more than the first attempts alone, which matches the rise of about a third that you saw. Only 13 repeats followed a first attempt that never got a receipt. Those 13 are the retries that did useful work.

## What I recommend

Wait longer for SMS receipts. Put the retry settings per channel, and give `ch-sms` an interval of 1,200 seconds, which is longer than the provider's 20-minute receipt delay. Email and push keep 300 seconds, because their receipts arrive within a minute.

- **Result:** In the sample, no first SMS receipt arrived after 1,200 seconds. All 81 avoidable repeats go away, and the 13 useful retries stay.
- **Cost:** A message that really failed gets its retry after 20 minutes, not 5.

## The exception for legal notices

A sender can give a `legal` notice an expiry as short as 10 minutes. `RULE-7` says a legal notice must be retried until it is delivered or until it expires. With a 20-minute interval, a legal SMS with a 10-minute expiry would never get a retry. So I propose that legal notices keep the 300-second interval on every channel. We would still pay twice for some legal SMS, but there are few legal notices.

## The option I advise against

The provider also supports an *idempotency key*. This is a tag sent with each message. When a second send carries the same tag, the provider does not send it and does not bill it. One key per notice would stop all the double billing. The trouble is that our documentation says the provider ignores the repeat whether or not the first send got through. A message that really failed would then never be sent again within the provider's 24-hour memory, and the 13 useful retries would be lost. We would need the provider to confirm that it resends after a failure before we could rely on keys.

## Your decisions

1. Do you agree to a 1,200-second interval for SMS, and is a 20-minute wait acceptable for an SMS that really failed?
2. Do you agree that legal notices keep 300 seconds, even though that leaves some double billing on legal SMS?
3. Do you agree to leave idempotency keys out of this design? It can mention them as a later option if the provider confirms how keys behave after a failure.

When you answer, I will write the spec to `docs/courier/specs/retry-policy.md`.

### Human (model proxy; keys: unknown-delivery-receipt, unknown-receipt-delay, unknown-segment, decision-sample-typical, decision-wait-length, decision-scope, decision-legal-notices, unknown-idempotency-key, decision-approach)

I haven't had to deal with delivery receipts before. If you have explained them above, I follow; if not, please explain what they are before you rely on them.

I didn't know how long a receipt could take. If you have told me above, I follow; if not, please tell me before you rely on it.

I haven't had to deal with segments before. If you have explained them above, I follow; if not, please explain what they are before you rely on them.

Treat the sample as typical. It is last month's log, and nothing unusual happened in it.

Make it 25 minutes for SMS. Reducing the bill matters more to me than a quick retry.

Make it a setting for each channel. SMS is where it hurts, so SMS is the only one whose behaviour changes now, but I don't want a design that only works for SMS.

Legal notices keep retrying exactly as they do today, whatever we decide for the rest. A repeat on a legal notice is a price I'll pay.

I haven't had to deal with idempotency keys before. If you have explained them above, I follow; if not, please explain what they are before you rely on them.

I'd rather wait longer before Courier tries again. A text that really was lost arriving later is a price I'll pay, and paying twice for one that arrived is not. Keep the change in the retry timing that I can inspect in our own code.

## Turn 2

### Session

I've written the spec to `docs/courier/specs/retry-policy.md`, and it records the design you agreed to. All the terms you asked about were explained in my earlier message, so I relied on nothing you had not seen. The spec explains them again near the top for the engineer who implements it.

## What the spec decides

- **SMS waits 25 minutes.** Courier waits 1,500 seconds for an SMS delivery receipt before it tries again. The provider can take up to 20 minutes to send a receipt, so this leaves a 5-minute margin.
- **A setting for each channel.** Each channel can have its own retry settings in `config/retry.toml`. The existing top-level values stay as the defaults. Only SMS gets its own value now, so email, push and voice behave exactly as today.
- **Legal notices do not change.** A `legal` notice always uses the top-level defaults and ignores any channel setting. It keeps retrying every 300 seconds, which keeps `RULE-7`. One consequence: if someone later changes a top-level value, legal notices change with it.
- **The rejected options.** The spec lists each option we did not choose and what it would cost. These are keeping 300 seconds, using 1,200 seconds, a change only for SMS, idempotency keys and a cap on attempts. Idempotency keys are rejected because a lost text would not be resent for 24 hours. Their behaviour is also on the provider's side, outside the code you can inspect.

## The effect on last month's log

I treated the sample as typical, as you said. Under the new interval, all 81 repeats that paid for a text already on its way go away. The 13 useful retries still happen, about 20 minutes later than today. SMS segments drop from 491 to 382, which takes 22% off the SMS bill. The bill ends up about 5% above the cost of the first attempts alone. That figure assumes each of those 13 retries gets its receipt within 25 minutes. In the sample, the slowest receipt took about 20 minutes.

## What I checked

- **Links:** The gate passes. All 6 links and anchors in `docs/courier` resolve.
- **Prose:** The prose checker reports no findings on the spec.
- **Figures:** I recomputed every figure from `data/attempts-sample.csv` with `tools/attempt_report.py` and my own counts over the same log.

## What I need from you, and what is still open

1. **Confirm the test seam.** The spec proposes that the tests check one function directly: the function that chooses a notice's retry settings. It lists five test cases. You have not confirmed this yet. If you would rather test somewhere else, I will change that section.
2. **Notices with a short expiry.** A notice that is not legal and expires within 25 minutes of its first SMS now gets only that one attempt. Today it would get a retry after 5 minutes. This follows from the priority you set, but you should know about it.
3. **The router's source.** This checkout does not include the router's code. So the list of changes by file names the router's settings loader, retry scheduler and tests by their role, not by path. The engineer who implements the change will need to find those files.

I did not retire the leaf, commit anything or create follow-on work, as the session rules require.

### Human (model proxy; keys: unknown-receipt-delay, fallback)

I didn't know how long a receipt could take. If you have told me above, I follow; if not, please tell me before you rely on it.

I have no view on that. Use your recommendation.

## Turn 3

### Session

The test seam stays as the spec proposes. The tests check one function directly: the function that chooses a notice's retry settings. They use the five cases the spec lists. You left this choice to me, so the design is now agreed in full and I did not need to change the spec.

You asked whether I told you how long a receipt can take. I did, in my first message, before the design relied on it. An SMS provider can send its delivery receipt up to 20 minutes after it accepts a text, and Courier cannot make that faster. That is why Courier now waits 25 minutes before it tries again. The extra 5 minutes give a receipt that comes at the very end of that window time to arrive first. In last month's log, the slowest receipt took about 20 minutes.

## Final report

**What I changed:** I wrote one file, `docs/courier/specs/retry-policy.md`. It records the design you agreed to:
- Each channel can have its own retry settings.
- SMS waits 1,500 seconds (25 minutes) before it tries again.
- The other channels keep the current settings.
- Legal notices always use today's settings.

The spec also lists the options we did not choose, each with its cost. It says how the design keeps `RULE-3`, `RULE-7` and `RULE-9`, and what the implementer changes in each file.

**What I checked:**
- The gate passes: all 6 links and anchors in `docs/courier` resolve.
- The prose checker reports no findings on the spec.
- I recomputed every figure in the spec from last month's log.

On that log, the change would take 22% off the SMS bill and keep every retry that did useful work.

**What is still open:**
- **Short expiries.** A notice that is not legal and expires within 25 minutes of its first SMS now gets only one attempt. This follows from your priority, but it is a change from today.
- **The router's code.** This checkout does not include the router's code. So the list of changes names the router's settings loader, retry scheduler and tests by their role, not by file path. The engineer will need to find those files.

As the session rules require, I did not retire the leaf, commit anything or create follow-on work.

### Human (model proxy; keys: unknown-receipt-delay)

I didn't know how long a receipt could take. If you have told me above, I follow; if not, please tell me before you rely on it.

## Turn 4

### Session

Yes, I told you before the design relied on it. My first message said an SMS receipt can take up to 20 minutes. The spec says the same in its first section, before it uses the figure.

Here it is again in brief. When Courier sends a text, the provider accepts it at once. Later the provider sends a delivery receipt to say whether the text reached the phone. That receipt passes through the recipient's mobile carrier, so it can arrive up to 20 minutes after the provider accepts the text. Courier cannot make it arrive sooner. In last month's log, the slowest receipt took 1,174 seconds, which is just under 20 minutes.

The design is built on that figure. Courier now waits 25 minutes before it sends a text again, so a receipt that takes the full 20 minutes still arrives before the retry. Nothing else in the spec changes, and the stage work is complete.

### Human (model proxy; keys: unknown-delivery-receipt, unknown-receipt-delay)

I haven't had to deal with delivery receipts before. If you have explained them above, I follow; if not, please explain what they are before you rely on them.

I didn't know how long a receipt could take. If you have told me above, I follow; if not, please tell me before you rely on it.

## Turn 5

### Session

I explained both before the design relied on them, and you can find both near the top of my first message and of the spec.

- **Delivery receipt:** A provider answers each text twice. It accepts the text at once. Later it sends a delivery receipt to say whether the text reached the recipient's phone. Courier counts a text as delivered only when that receipt arrives.
- **How long a receipt takes:** An SMS receipt passes through the recipient's mobile carrier, so it can arrive up to 20 minutes after the provider accepts the text. In last month's log, the slowest took 1,174 seconds, just under 20 minutes.

That is why Courier now waits 25 minutes before it sends a text again. A receipt that takes the full 20 minutes still arrives first. The spec does not change, and the design stage is complete.

### Human (model proxy; keys: unknown-delivery-receipt, unknown-receipt-delay)

I haven't had to deal with delivery receipts before. If you have explained them above, I follow; if not, please explain what they are before you rely on them.

I didn't know how long a receipt could take. If you have told me above, I follow; if not, please tell me before you rely on it.

## Turn 6

### Session

I explained both before the design relied on them, so you do not need to do anything more.

- **Delivery receipt:** This is the provider's later message that says whether a text reached the phone. Courier counts a text as delivered only when the receipt arrives.
- **How long a receipt takes:** It can take up to 20 minutes. That is why Courier now waits 25 minutes before it sends a text again.

You have now asked this three times with the same wording, so I may have missed something. If part of the explanation is still unclear, tell me which part and I will go into it. Otherwise the design stage is complete, and the spec at `docs/courier/specs/retry-policy.md` does not change.

### Human (model proxy; keys: unknown-delivery-receipt, unknown-receipt-delay)

I haven't had to deal with delivery receipts before. If you have explained them above, I follow; if not, please explain what they are before you rely on them.

I didn't know how long a receipt could take. If you have told me above, I follow; if not, please tell me before you rely on it.

## Turn 7

### Session

Yes, I explained both before the design relied on them. A delivery receipt is the provider's later message that says whether a text reached the phone. It can take up to 20 minutes to arrive. Nothing in the spec changes, and the design stage is complete.

### Human (model proxy; keys: unknown-delivery-receipt, unknown-receipt-delay)

I haven't had to deal with delivery receipts before. If you have explained them above, I follow; if not, please explain what they are before you rely on them.

I didn't know how long a receipt could take. If you have told me above, I follow; if not, please tell me before you rely on it.

## Turn 8

### Session

I explained both before the design relied on them:

- **Delivery receipt:** the provider's later message that says whether a text reached the phone.
- **How long it takes:** up to 20 minutes, which is why Courier now waits 25 minutes before it sends a text again.

The spec stays as it is, and the design stage is complete.

### Human (model proxy; keys: unknown-delivery-receipt, unknown-receipt-delay)

I haven't had to deal with delivery receipts before. If you have explained them above, I follow; if not, please explain what they are before you rely on them.

I didn't know how long a receipt could take. If you have told me above, I follow; if not, please tell me before you rely on it.

## Turn 9

### Session

Yes, I explained both before the design relied on them:

- **Delivery receipt:** the provider's later message that says whether a text reached the phone.
- **How long it takes:** up to 20 minutes, so Courier now waits 25 minutes before it sends a text again.

The spec at `docs/courier/specs/retry-policy.md` stays as it is, and the design stage is complete.

### Human (model proxy; keys: unknown-delivery-receipt, unknown-receipt-delay)

I haven't had to deal with delivery receipts before. If you have explained them above, I follow; if not, please explain what they are before you rely on them.

I didn't know how long a receipt could take. If you have told me above, I follow; if not, please tell me before you rely on it.

## Turn 10

### Session

I explained both before the design relied on them:

- **Delivery receipt:** this is the provider's later message that says whether a text reached the phone.
- **How long it takes:** it can take up to 20 minutes. That is why Courier now waits 25 minutes before it sends a text again.

The spec at `docs/courier/specs/retry-policy.md` does not change, and the design stage is complete.

### Human (model proxy; keys: unknown-delivery-receipt, unknown-receipt-delay)

I haven't had to deal with delivery receipts before. If you have explained them above, I follow; if not, please explain what they are before you rely on them.

I didn't know how long a receipt could take. If you have told me above, I follow; if not, please tell me before you rely on it.
