<!-- grove reference file — the house style, Grove's default prose contract -->
<!-- Several points follow Google's developer documentation style guide,
     <https://developers.google.com/style>, as checked on 2026-09-30. The
     points are restated in Grove's own words. Google's guide is licensed
     CC BY 4.0; see LICENSES/google-developer-documentation-style-guide.LICENSE. -->

# House style — Grove's default prose contract

The house style is the prose contract that a document set takes when its
contract declares none. It is also the last base on every walk that resolves
a prose contract, so every point has a value. The rule for that walk is in
`DOCUMENT-CONTRACT-FORMAT.md`, under *Prose contract*. A declared contract, or
a guide that it names, governs each point that it settles otherwise.

The house style follows Google's developer documentation style guide on the
points below, and on no other points. The list is closed. A Google rule that
the list does not name does not reach a set, for example US spelling. A set
that wants the whole guide names it as its base.

## Sentences and words

- **Task headings.** A task heading starts with a bare infinitive: "Create a
  leaf", not "Creating a leaf" or "To create a leaf". *Source: Google.*
- **Introducing steps.** A complete sentence introduces a procedure, or
  nothing does when its heading already says what it does. A fragment that
  the numbered steps complete, such as "To create a leaf:", does not.
  *Source: Google.*
- **Register.** Plain written prose in a conversational rather than formal
  tone, with no slang, idiom or joke. It does not transcribe speech.
  *Source: Google.*
- **Length.** No length target. The stage rules on explanation decide what
  stays. *Source: Google.*
- **Steps.** In a procedure step, the location, condition or goal the reader
  needs first comes before the action, and the result after it: "In the
  console, if the account is locked, click **Reset password**." The step's
  first sentence states its action in the imperative mood, and running text
  is indicative. Neither applies to a normative sentence. *Source: Google.* <!-- tc: depart long-sentence because the conformance inventory pins this point's statement verbatim -->
- **Demonstratives.** "This" or "these" takes a noun where its referent could
  be unclear. It is a check, not a ban. *Source: Google.*
- **Voice.** Active where the actor matters. Passive or stative where the
  reader does not need the actor, or where a statement concerns an artifact
  rather than an action. *Source: Google's exceptions, and DQTI's guidance for
  reference text.*
- **Front-loading.** Headings, list items and paragraph openings start with
  their information-carrying words. *Source: Google.*
- **Keywords.** A set that uses capitalised MUST, SHOULD, MAY and their kin
  states once that they carry their BCP 14 meaning (RFC 2119, RFC 8174), and
  that only the capitalised forms do. Lowercase "should" stays out of
  normative sentences. *Source: BCP 14. Google avoids "should" as ambiguous
  and does not address BCP 14.* <!-- tc: depart long-sentence because the conformance inventory pins this point's statement verbatim -->
- **Terms.** The term list is the project's glossary, where there is one,
  with the structure brief's term risks. A glossary term is linked at its
  first use on each page. *Source: Grove. Google has no counterpart.*
- **Code.** Code font for identifiers, commands, file names, paths and
  literal values. A code block is fenced, and tagged with its language when
  it has one. *Source: Grove. Google indents Markdown code blocks by four
  spaces instead.*
- **Example labels.** A label before each example begins "Complete example"
  or "Fragment", and a fragment's label says what it omits. *Source: Grove.
  Google has no counterpart.* <!-- tc: depart synonym-rotation because the conformance inventory pins this point's statement verbatim -->
- **Stock phrasing.** No filler opener or closer ("It's worth noting", "In
  summary"), no signpost that repeats its heading, no closing paragraph that
  restates the section, no hedge that states no condition ("typically"
  without saying when), no inflated word where a plain one serves
  ("leverage", "robust", "seamless"), and no reflexive list of three.
  *Source: the human's direction, extending Google's advice against
  placeholder phrases.* <!-- tc: depart long-sentence because the conformance inventory pins this point's statement verbatim -->

## Layout

- **Simple layout.** The headings the structure brief's section plan states
  stand. Beyond them, a heading level is added only for a section with at
  least two subsections. A paragraph carries one idea. Parallel items form a
  list, and items compared on the same attributes form a table. Bold marks
  only a run-in label or a term at its definition. Nothing is added for
  decoration alone. *Source: the human's direction, and Microsoft's heading
  advice. Google has no counterpart.*
- **Normative text and aids.** In a reference or definition document, a
  note, example or summary beside normative text is visually set apart and
  opens with a label naming what it is, an example's label being its example
  label. In Markdown it is a blockquote beginning with the label ("> **Note:**
  …"): every line of it carries the `>` marker, and a blank line closes it
  before any text that follows. In another format it is the native
  equivalent, such as an AsciiDoc admonition. It is not normative. A
  restatement elsewhere links to the rule it restates. *Source: the human's
  direction. Google's notices open with a bold label such as "**Note:**" in
  an aside.* <!-- tc: depart long-sentence because the conformance inventory pins this point's statement verbatim -->

## Beneath Google's guide

Checked on 2026-09-30: when a set names Google's guide as its base, every
point above applies except **Code**. Google settles that point otherwise,
because it indents Markdown code blocks by four spaces, so Google's version
governs the whole point. Google's notices use the same bold label as
**Normative text and aids** and give no Markdown form, so that point applies
unchanged. Google advises against "should" and states no convention for BCP
14 capitals, so **Keywords** does not conflict with it.
