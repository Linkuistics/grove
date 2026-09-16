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

## Match notation to the question

| Question | Representation |
|---|---|
| What owns behavior, and what depends on what? | Module/component diagram with labelled relationships |
| What happens next, including branches and loops? | Flowchart |
| Which events change persistent state? | State machine with explicit states, events and guards |
| Who calls whom, and in what order? | Sequence diagram |

Prefer Mermaid when it expresses the design clearly; reuse another established
notation or renderer when appropriate. Keep source as the editable artifact
and let the renderer position nodes. A rendered state diagram is not evidence
of model checking. Preserve agreed diagram sources with the design artifact.

## Make the discussion locatable

Organize the visual document around a topic outline, with diagrams and their
decision notes nested under the relevant module, behavior or contract. Use the
same topic names in the specification and presentation. A growing design should
remain browsable as a document, with an overview and focused views.

Keep a prominent current-discussion panel containing the question or decision,
its status (for example, proposed or approved), and links to the relevant
diagrams. Mark which diagrams changed in that discussion update. Being relevant
and being changed are different: retain links to unchanged context without
marking it updated. Refresh these markers when the discussion moves on; they
describe the current update, not a second decision log.

Give each diagram a descriptive title and stable link. Refer to it by topic and
title in conversation and link directly to that view, for example
**Interaction → Prepare choices before acting**. When reporting edits, name
the affected views and what changed in each. Keep their anchors stable when
the contents or titles change. A reader should be able to open the cited view
and immediately identify it.

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
       "updatedDiagramIds": ["choices"]
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
discussion's ID lists refer to existing diagrams; `updatedDiagramIds` names
only the views changed in that update. These optional fields also allow older
flat manifests to render. Topic links use `#group-<id>` and diagram links use
`#diagram-<id>`.

The viewer fetches current source on reload, exposes source links, and shows
load and syntax errors. It uses pinned Mermaid from jsDelivr, requiring network
access; use a local renderer when offline delivery is required. Other preview
tools may hold an in-memory snapshot: check whether edits reach the served page
or require a restart. Serve the presentation directory, not the workspace root.

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
HTTP success alone does not establish that a diagram rendered. Keep labels
short and put explanation in captions; change the standard layout direction
when necessary to avoid unreadable wrapping or misleading edge crossings.

## Delegate bounded presentation work

When delegation is authorized and useful, give an agent ownership of named
diagram/viewer files, the accepted facts, current proposal, open questions,
and relevant source evidence. Keep design decisions with the main conversation.
Have the agent report changed files and unresolved rendering issues; verify the
served result before presenting it as checked. Update the agent's contract
when the human changes a decision.
