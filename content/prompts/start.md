Start a new grove — use the grove skill. First run `grove-llm root-init` to
scaffold the root brief and first **requirements** leaf, then enter the loop
(`grove-llm pick` → grill that leaf, sharpening `CONTEXT.md` inline → record the
outcome in the root brief → grow the tree). Cut the leaves yourself if the whole
workstream fits a handful of them; otherwise add a `planning` leaf and let a
fresh session do the decomposition. This first cut shapes every later session, so
compose rather than only append, and each shape is **one call**: where a leaf's
artifact is load-bearing, cut a **review chain** with `grove-llm leaf-add-chain
<parent> <stem> --kind <producer>` rather than a lone `leaf-add`, and a
load-bearing survey as a **vendor pair** with `grove-llm leaf-add-pair <parent>
<stem> --harness-a <name> --harness-b <other>`. Each names its three leaves off a
shared stem so the tree shows the process, and derives what you would otherwise
transcribe — the chain's `review-`/`integrate-review-` kinds, the pair's two
declarations. Neither is required and neither is enforced; see the skill's
Decompose step. End each task with `grove-llm
complete` as your last step so the self-driving loop relaunches the next task
with fresh context.
