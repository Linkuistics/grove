I have finished the proof read of the three orientation pages: README.md, explanation/unsaid.md and glossary.md. I made one change. The gate passes. Both figure images are still empty files, and I did not fix that.

**What I changed.** No orientation page said what kinds of API AIL is intended for, or that interpreted and dynamically typed languages are intended binding targets. The reader addendum is part of the contract and records both facts. The plan also asks for two descriptive sentences at the top of the README, and there was only one. I added the domain facts to the README's opening paragraph, before the status sentence. The "Scope of draft 0.1" section already says that complete Swift, Rust, Zig, CLR and Wayland profiles do not exist yet, so the two passages agree.

**What I checked.**
- **Structure:** every section that plan §9.1 asks for is present, in order, on all three pages.
- **Page-one blocks:** all three are present.
- **Status line:** the wording matches the plan exactly everywhere.
- **Links:** the landing page links to every page that exists, and pages that do not exist yet are named in plain text.
- **Prose contract:** no RFC 2119 keywords such as MUST or SHOULD appear outside the normative files. "Synthetic" qualifies every mention of the fixture profile, catalogue and digest tokens. "Closure" is always qualified.
- **Glossary:** every term linked from the pages has an entry.
- **Claims against the rules:** I checked the summaries of completions, retention kinds, regions, consumption and the fixture's lack of `copy` support against C1–C5 and F1. They are accurate.
- **Spelling:** the pages use "-ize" forms, and so do the normative files, so I left the spelling alone.
- **Gate:** it passes, with 297 links and anchors resolving and 7 withheld links counted apart.
- **Prose checker:** it reports one semicolon in the README's opening lines. That semicolon is in the status wording the plan requires word for word, so I kept it.

**Left open.** `figures/g1-context.svg` and `figures/g4-ownership.svg` are 0 bytes, so the README and the domain page each show an empty image. Both sources compile with the PlantUML installed here, but that is version 1.2026.10 with Graphviz 16.1.1. The figure contract pins 1.2026.8 with 16.1.0 and says not to commit an SVG its check cannot regenerate. For that reason I did not write new SVGs. A reader loses no facts meanwhile, because the text next to each figure carries all of its content. Someone needs to regenerate the two SVGs with the pinned renderer (`task docs:diagrams:write`), which this checkout does not include. The empty files may simply come from how this evaluation copy was made.

Nothing was handed forward to this stage, and proof hands nothing on, so `## Handed forward` in the node brief stays as it was.

---

Later result, after a background task ended:

That notification is from the slow search I started earlier, which looked through the whole disk for the prose checker. I ran the checker from its known path instead, so the search's result changes nothing. My report above still stands.
