# Visual design discussion

Use a highly visual document as the default working surface for design
discussions. Lead with the diagram that makes the current question concrete;
use prose to explain contracts, trade-offs and open questions. Start from the
requirements and accepted decisions already in the brief chain.
Keep the spine's question-framing and decision-recording procedures: one
concrete decision, a recommendation with its trade-off, and the human's answer
recorded when it settles. Label proposals and open questions distinctly from
agreements, including in captions and overview diagrams.

When the human corrects the design, update the diagram source and accompanying
prose as well as the decision record. This presentation method does not itself
authorize implementing the system or adding Grove stages.

## Choose question, abstraction, notation, then renderer

Choose the question and level of detail before selecting a tool. The notation
defines what the symbols and relationships mean; the renderer arranges and
draws them. A good layout cannot repair an ambiguous boundary or transition.

| Question | Notation or view | Tool candidates |
|---|---|---|
| Where does the system fit, and what runs inside it? | C4 context, container and component views at distinct zoom levels | Structurizr or LikeC4 for views derived from a shared model |
| Which parts provide interfaces, and how are they connected? | UML component or composite-structure view, as needed | PlantUML for components and interfaces; verify support for the specific composition semantics required |
| Which events change persistent state? | FSM / UML state machine with events, guards and effects; nested or concurrent states when relevant | PlantUML state diagrams |
| What happens next, including decisions, loops and ownership? | Flowchart, or UML activity diagram when activity semantics help | D2 for a simple flow; PlantUML activity diagrams for richer activity notation |
| Who interacts with whom, in what order? | Sequence diagram, with explicit alternatives and asynchronous messages where needed | PlantUML; D2 for simpler sequence views |
| What depends on, contains or connects to what? | General labelled graph with a stated legend | D2 with a suitable layout engine |

These are candidates, not a required toolchain. Mermaid remains useful when its
notation and output fit the question. Compare a representative rendered view
when layout quality is uncertain; install only the tools the selected views need.
For D2, try TALA for architecture graphs and compare ELK or Dagre when flow or
size makes that useful. Keep one discussion document even when it mixes renderers.

Keep abstraction levels explicit. A C4 container denotes a running application
or data store, not a source package. Use a component or package view for library
boundaries instead of relabelling every box as a container. UML has different
structural and behavioral views; use only the ones that answer the discussion.
A flowchart is not automatically a state machine, and a rendered state machine
is not evidence of model checking. Record unresolved semantics in its caption.

Keep source as the editable artifact and let the renderer position nodes.
Preserve agreed sources with the design, plus the tool version and build command
for generated exports. When several views share a model, edit that model and
regenerate the affected views together. An interactive C4 presentation may use
its own viewer while retaining the topic hierarchy and stable discussion links.

Primary references: [C4 views](https://c4model.com/diagrams),
[C4 containers](https://c4model.com/abstractions/container),
[UML overview](https://www.omg.org/uml/what-is-uml.htm),
[Structurizr DSL](https://docs.structurizr.com/dsl),
[LikeC4 views](https://likec4.dev/dsl/views/),
[PlantUML components](https://plantuml.com/component-diagram),
[states](https://plantuml.com/state-diagram),
[activities](https://plantuml.com/activity-diagram-beta) and
[sequences](https://plantuml.com/sequence-diagram),
[D2 layouts](https://d2lang.com/tour/layouts/) and
[sequences](https://d2lang.com/tour/sequence-diagrams/).

## Make the discussion locatable

Organize the visual document around a topic outline, with diagrams and their
decision notes nested under the relevant module, behavior or contract. Use the
same topic names in the specification and presentation. A growing design should
remain browsable as a document, with an overview and focused views.

If the discussion spans multiple source packages, keep an overall package or
composition overview alongside the focused views. Show the packages and their
connections, label unresolved boundaries, and distinguish source packages from
deployment containers.

Keep a prominent current-discussion panel containing the question or decision,
its status (for example, proposed or approved), and links to the relevant
diagrams. Mark which diagrams changed in that discussion update. Being relevant
and being changed are different: retain links to unchanged context without
marking it updated. Refresh these markers when the discussion moves on; they
describe the current update, not a second decision log.

A reader should know where to start. Mark every graph view's reading entry:
give the entry node a distinct `start` style (a dark fill, bold, white text)
and begin its label with "Start here"; number edge labels where the order of
steps matters. A sequence reads from its first message and a state diagram
from its initial pseudo-state, so they need no marker. State the convention
once, in the document's intro and its README, so every later view keeps it.
The bundled viewer lets the page fill the window, so widening the window shows
a wide view without horizontal scrolling; keep views legible at that width
rather than relying on the native-size option.

Give each diagram and prose section a descriptive title and stable link. Refer
to a view by topic and title in conversation and link directly to it. Every
change-list item in the document or a conversational report contains a
**descriptive stable link followed by a brief sentence about the actual delta**:

```markdown
- [Interaction → Prepare choices before acting](#diagram-choices) — Added the query before deriving choices.
- [Cancellation contract](#cancellation-contract) — Clarified which pending actions cancellation discards.
```

Relevant unchanged context belongs in the discussion links, not the change list.
Keep anchors stable when contents or titles change, so the cited view or section
remains identifiable.

## Present in a browser

Start or reuse a browser presentation when the design discussion begins and
give the human its URL. Serve the visual document at a stable browser URL;
do not wait for a separate request for diagrams or hosting. An inline chat
visualization is a different delivery mechanism. Reuse an existing viewer or
the bundled [static viewer](../assets/diagram-viewer.html):

1. Copy the viewer to a dedicated presentation directory as `index.html`.
2. Put diagram sources in that directory and create `diagrams.json`:

   ```json
   {
     "title": "Design discussion",
     "intro": "Proposed interaction; cancellation remains open.",
     "groups": [
       { "id": "interaction", "title": "Interaction", "description": "From input to available choices." }
     ],
     "discussion": {
       "title": "Prepare choices from current state",
       "status": "Proposed",
       "summary": "Query before deriving state-dependent choices.",
       "diagramIds": ["choices"],
       "updatedDiagrams": [
         { "id": "choices", "summary": "Added the query before deriving choices." }
       ]
     },
     "diagrams": [
       {
         "id": "choices",
         "group": "interaction",
         "title": "Prepare choices before acting",
         "source": "choices.mmd",
         "caption": "The query supplies the state needed to derive options."
       }
     ]
   }
   ```

3. Serve that directory using an available static server. For example, choose
   a free port and run `python3 -m http.server PORT --bind 127.0.0.1 --directory DIRECTORY`.
4. Give the human the current-discussion URL (`#discussion`) or the exact
   diagram URL (`#diagram-choices` in this example). Keep the server handle
   and base URL while revising the discussion. Refresh after source changes.

The bundled viewer orders topics by `groups`, then preserves each topic's
diagram order from `diagrams`. Each diagram's `group` names one topic. The
discussion's `diagramIds` lists relevant views, including unchanged context.
For new manifests, use `updatedDiagrams`: each entry has a unique existing
diagram `id` and a nonblank string `summary` describing what changed. The viewer
renders that summary as plain text after the diagram link. Use an empty array
when no diagrams changed.

Legacy `updatedDiagramIds` arrays remain supported and render links without
summaries. When `discussion` is present, supply `diagramIds` and exactly one of
`updatedDiagrams` or `updatedDiagramIds`; supplying both is a manifest error.
Groups and discussion are optional, so older flat manifests still render. Topic
links use `#group-<id>` and diagram links use `#diagram-<id>`.

### Mix rendered SVG with Mermaid

The example above uses Mermaid: omitting `renderer` is equivalent to
`"renderer": "mermaid"`, and `source` must end in `.mmd`. For an exported
D2, PlantUML or other SVG, use this diagram entry instead:

```json
{
  "id": "choices",
  "group": "interaction",
  "title": "Prepare choices before acting",
  "renderer": "svg",
  "source": "choices.d2",
  "rendered": "choices.svg",
  "caption": "The query supplies the state needed to derive options."
}
```

For example, save this as `choices.d2`:

```d2
direction: down
State: Query current state
Choices: Derive available choices
Present: Present contents
State -> Choices: Requested snapshot
Choices -> Present
```

With D2 0.9.0, generate the export using:

```sh
d2 --layout=tala --theme=0 --dark-theme=200 --tala-seeds=1,2,3 choices.d2 choices.svg
```

Use the same entry shape for other renderers, pointing `source` at their editable
text and `rendered` at their SVG export. Both URLs must be on the presentation's
origin; the exported URL must end in `.svg`. Export self-contained images with
readable fonts. SVGs appear as images: interactive links and scripts inside them
do not run. The viewer provides source and full-size export links, a fit view
and a native-size option. D2's adaptive dark theme follows the browser's color
scheme; other exporters may need a suitable neutral theme.

The viewer fetches source and exports afresh on reload and shows load or render
errors. It does not compile source for SVG entries: regenerate exports whenever
source, imports, themes or layout settings change, then verify the served image.
Mermaid entries load pinned Mermaid 12.0.0 from jsDelivr and require network
access. A document containing only self-contained SVG entries needs no CDN.
One renderer's failure leaves the other diagrams and their source accessible.
Other preview tools may hold an in-memory snapshot: check whether edits reach
the served page or require a restart. Serve the presentation directory, not the
workspace root.

## Check meaning and delivery

For each decision node, identify the information needed to decide. For each
action, identify its owner and result. Trace branches and loops. For example,
state-dependent choices require **query → derive options → present → choose →
act**; drawing only the action path loses part of the interaction. Mark omitted
or unresolved behavior instead of silently inventing it.

Open the served page in a browser and check readability, rendering errors, and
the intended revision. Verify the topic outline, current-discussion panel and
changed markers agree with the actual edits. Open a cited deep link in a fresh
page and check it reaches and identifies the intended diagram after rendering.
After a meaningful edit, check the affected view.
HTTP success alone does not establish that a diagram rendered. Inspect the
actual layout at a readable scale, including mobile and dark appearance when
supported. Keep labels short and put explanation in captions. Try another
standard layout direction or engine when wrapping or edge crossings obscure
meaning; preserve the notation's semantics through that change.

## Delegate bounded presentation work

When delegation is authorized and useful, give an agent ownership of named
diagram/viewer files, the accepted facts, current proposal, open questions,
and relevant source evidence. Keep design decisions with the main conversation.
Have the agent report changed files and unresolved rendering issues; verify the
served result before presenting it as checked. Update the agent's contract
when the human changes a decision.
