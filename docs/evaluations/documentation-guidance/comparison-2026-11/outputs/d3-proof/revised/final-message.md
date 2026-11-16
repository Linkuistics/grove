The proof of the orientation set is done and the gate passes. Two items are still open, and they are listed below.

## What I changed

1. **`explanation/unsaid.md`, the rules line in the header block:** I added `L2–L3` and `C8`. The page cites these rules, but the header block did not list them.
2. **`explanation/unsaid.md`, the prose before figure G4:** I added one sentence. It says that after consumption the callee discharges the obligation unless it returns it as an owned output. Before this change, only the figure carried that fact, and the plan says the prose must carry every fact a figure shows. The sentence agrees with `contracts.md:94-96`.
3. **`explanation/unsaid.md`, the *closure* table row:** I changed the rule citation from `L2` to `L2–L3`. The glossary cites `L2–L3`.
4. **`README.md`, the Scope section:** I added a short paragraph about the intended domain. The domain is .NET, Linux, Wayland, GTK, KDE, Rust and Apple-related APIs, and dynamically typed languages are intended consumers. The reader addendum requires this, and no page said it. The paragraph also says that draft 0.1 does not yet contain the profiles for these domains.

## What I checked

- **Gate:** it passes, with 299 links and anchors resolving in 8 files and 7 targets withheld.
- **Rule links:** every rule anchor that the pages cite exists.
- **Status wording:** it is the same on all three pages.
- **Unwritten pages:** they are named as plain text, as the staging rule requires.
- **Glossary:** it is in alphabetical order. Each entry has a definition, a defining page, its rules and its warnings.
- **Figures:** the SVG text matches the PlantUML sources. Each diagram has a key, and the G1 image has alt text.
- **Reading routes:** I walked the guided route and the direct entries through rules, examples and terms.

## What is still open

- **Figure titles:** the G1 and G4 titles do not state the diagram type, and the figure format requires them to. This belongs to the art stage, so the fix needs a correction run from art to proof. I could not cut that run in this evaluation copy.
- **PlantUML version:** version 1.2026.10 is installed, but the plan pins 1.2026.8. When I rendered with 1.2026.10, the layout changed, so I did not regenerate the SVGs.
- **Reader checkpoint:** a clean proof does not show that readers understand the pages. Checkpoint 1 (a proxy reader and a human read of the landing page, the explanation and the tutorial) has not run yet. It belongs after the tutorial.

Nothing was handed forward, so the node brief's `## Handed forward` list did not need clearing. As instructed, I did not retire the leaf or commit.

---

Later result, after a background task ended:

The system stopped one of my earlier background commands because it reached its time limit. It searched for rule headings and for the figure-format file. I do not need to run it again: I had already checked the rule anchors and read the figure format another way, and the gate passes. The report above does not change.
