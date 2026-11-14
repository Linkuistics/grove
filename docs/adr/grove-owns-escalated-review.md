# Grove owns escalated review

A resolved, prompt-visible Grove mandate gives Grove ownership of independent
adversarial review. The producer schedules that work as a separate review task,
so the owner's task-kind policy chooses its command and the review begins with
fresh context. Ordinary implementation checks and fix verification remain
in-session. The routing procedure and its ownership predicate live in
[`references/execute.md`](../../plugins/grove/skills/grove/references/execute.md).

This boundary follows the purpose of the work. A numeric allowance made a second
routine check pay for new Grove sessions while allowing the first independent
adversarial challenge to bypass Grove's review routing. Neither result served
the reason for fresh review. The current boundary preserves cheap, iterative
verification and reserves task orchestration for work whose independence matters.

Outside a mandated Grove leaf, doubt-driven development retains its standalone
behavior. A checkout containing `.grove/` or an inherited environment variable
does not establish Grove ownership.

## Considered options

- **One in-session adversarial reviewer, then escalate.** Replaced because the
  reviewer count does not distinguish routine verification from independent
  challenge, and the first adversarial reviewer bypasses task-kind routing.
- **Make every check a new Grove task.** Rejected because verification is part
  of producing a working artifact and fixing findings. It does not inherently
  require independence from the producer's framing.
- **Leave all review inside the producer.** Rejected because it loses the
  independently routed fresh session when adversarial challenge is the goal.
