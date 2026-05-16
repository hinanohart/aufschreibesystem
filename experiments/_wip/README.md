<!--
SPDX-License-Identifier: CC0-1.0
SPDX-FileCopyrightText: 2026 Kittler Aufschreibesystem Synthesizer contributors
-->

# `experiments/_wip/` — R8 retreat directory

Per the project's autonomous-loop policy (`docs/supervisor-protocol.md` and
the user's R8 rule), code that fails three consecutive attempts under the
autonomous executor is **moved here**, not deleted. `rm -rf` is forbidden
on this tree because failure memory is the project memory most likely to
be forgotten.

If you are a human contributor and you are tempted to delete an
`experiments/_wip/<stage>/` subdirectory: read it first, then move it to
`docs/failed-attempts/<stage>.md` as a summary, then delete the directory.
Never delete it as the only action.

This directory is intentionally empty at v0.1 commit time. That emptiness
is information.
