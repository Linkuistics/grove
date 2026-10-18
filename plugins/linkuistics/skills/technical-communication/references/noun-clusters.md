# Noun clusters

This file restates section 2 of Part 1 (rules 2.1 and 2.2). A noun cluster is a run
of nouns and adjectives that works as one noun, such as "build cache invalidation
hook". The last word is the head noun. The words before it describe the head noun.

| Identifier | Level | Idea |
|---|---|---|
| `noun-cluster` | none | A cluster has three words at most |
| `short-form-after-full` | none | Write a longer official name in full once, then shorten it |
| `hyphen-unit` | none | A hyphen joins words that act as one unit |

## noun-cluster

A cluster has three words at most. The reader must guess how the words connect, and
each added word adds readings. Is a "build cache invalidation hook" a hook that
invalidates the cache of a build, or a hook of the build that clears the cache? Many
languages put the head noun first, so those readers must reverse the whole group.

Count the words of each cluster. A hyphenated unit counts as one word (see
`hyphen-unit`). When a cluster has more than three words, unpack it with a
preposition such as "of", "to", "with" or "in". A verb can also do the work. Each
part of the result has three words or fewer.

```text
Before: Check the session token refresh failure rate.
After: Check how often the refresh of the session token fails.

Before: Open the build cache invalidation hook script.
After: Open the script for the hook that invalidates the build cache.

Before: Increase the worker pool connection timeout value.
After: Increase the timeout of a connection in the worker pool.
```

A short cluster can be unclear too. "A small batch job" may be a small job or a job
of small batches. When two readings are possible, unpack it even if it has three
words.

## short-form-after-full

An official name can be longer than three words. A product, a setting, a document
or a tool can have one. Keep the name as it is. Write it in full at its first use.

Then give a short form of three words or fewer, and say what the name means when
the reader cannot see it. For the rest of the text, use only the short form. See
`synonym-rotation` in `words.md`.

```text
Before you edit the Regional Replica Lag Alert Rule (called "the lag rule" here),
stop the alert service. Then save the lag rule.
```

An abbreviation can be the short form. It comes from the documents of the project
or of the field, and the first use gives the full name. Do not make up an
abbreviation without defining it where it first appears. A name of three words or
fewer needs no abbreviation. An abbreviation saves little, and a text full of
letters is hard to read.

```text
Before: When the CBT trips, the RPCL retries.
After: When the circuit breaker trips, the remote-call layer retries.
```

An abbreviation that the whole field knows, such as "API", "URL" or "CPU", needs no
definition. One that only a team knows is slang (see `no-slang` in `words.md`).

## hyphen-unit

A hyphen can join words that work as one unit. The unit counts as one word, so it
can bring a cluster inside the limit.

```text
Before: Check the cache miss rate chart.
After: Check the cache-miss rate chart.
```

A hyphen must show a true link. A wrong hyphen changes the meaning, as "small-batch
job" and "small batch-job" do. When you are not sure that the words form one unit,
unpack the cluster instead.

- A hyphenated unit has two parts, and rarely three. Do not join a long cluster with
  hyphens to make it look short. A string of four or five hyphenated words is as
  hard to read as the cluster was.
- Do not hyphenate a cluster that is already clear with three words or fewer.
- A name that the field or the project writes with a hyphen keeps its hyphen. A name
  that it writes without a hyphen stays without one.

## Coverage

| Identifier | Rules |
|---|---|
| `noun-cluster` | 2.1 |
| `short-form-after-full` | 2.2 |
| `hyphen-unit` | 2.2 |
