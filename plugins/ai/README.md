<!--
SPDX-License-Identifier: Apache-2.0
SPDX-FileCopyrightText: 2026 Kittler Aufschreibesystem Synthesizer contributors
-->

# `kittler-ai` — interpretation plugin (L4)

Python sidecar. **Never bundled with the main distribution.** Lives in this
monorepo only as a spec; the actual sidecar will be released as a separate
repository.

## The user-pull design (architectural constraint, not a legal opinion)

The sidecar **never** ships model weights. On first run:

1. The user is shown the Tongyi Qianwen license terms verbatim.
2. The user must type `accept` (or pass `--accept-tongyi-license`).
3. Only then does the sidecar download Qwen3-Omni from the **official
   Alibaba release channel**, into `~/.kittler/models/`.
4. If declined, the sidecar enters a no-AI mode: L1–L3 still function fully;
   only the L4 stage of the C2PA chain becomes unavailable.

This isolates the OSS distribution from Tongyi's commercial-use clause and
removes us from the consent chain.

See `docs/ARCHITECTURE.md` §4 for the full rationale and
`LICENSE` for the per-tree SPDX matrix.

## Spec (v0.1)

```
POST /interpret
  body: { event_stream: EventStream_json, prompt: string }
  resp: { text: string, model_id: string, synthid_present: bool, sidecar_version: string }
```

The host (`kittler-archive`) appends an L4 `Stage::Interpretation` to its
C2PA manifest using this response.

## Why a sidecar, not a linked library?

License isolation. Apache-2.0 wrapper code calling a model under a non-OSI
license through a local process boundary is the cleanest architectural answer
to "how does an OSS use a non-OSI-licensed model without polluting its own
license matrix."
