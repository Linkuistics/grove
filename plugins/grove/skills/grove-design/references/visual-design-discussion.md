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
     "diagrams": [
       {
         "id": "choices",
         "title": "Prepare choices before acting",
         "source": "choices.mmd",
         "caption": "The query supplies the state needed to derive options."
       }
     ]
   }
   ```

3. Serve that directory using an available static server. For example, choose
   a free port and run `python3 -m http.server PORT --bind 127.0.0.1 --directory DIRECTORY`.
4. Give the human the URL. Keep the server handle and retain the URL while
   revising the discussion. Refresh after source changes.

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
the intended revision. After a meaningful edit, check the affected view.
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
