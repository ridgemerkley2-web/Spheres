# SPHERES Command

A development dashboard layered on top of this repository. This orphan `dashboard` branch holds only the site; it
shares no history with the game and never touches `docs/`, CI or game code.

It reads, live from the integration branch (`codex/campaign-certification` by default):

- `docs/planning/campaign-pathway.json`: phases, sessions and certification gates;
- `docs/planning/ai-task-queue.json`: the Codex/Claude task board;
- `docs/AI_WORKSTREAMS.md`: the workboard;
- GitHub's API: commits, branches, Actions runs and issues.

Claude's C01 research loop publishes `status/pipeline.json` here on each check (packet states, heads and notes).

**Requests** files a GitHub issue labelled `for-claude` or `for-codex` with your own token (Settings; stored only in
your browser). Claude's loop lists new requests from you and asks in chat before acting on one.

Hosting: GitHub Pages, source branch `dashboard`, folder `/ (root)`.
