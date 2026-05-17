<!--
SPDX-License-Identifier: CC-BY-SA-4.0
SPDX-FileCopyrightText: 2026 Kittler Aufschreibesystem Synthesizer contributors
-->

# G6 — 4-audience interview log

Per `governance.md` §G6, v0.1 cannot be published until all four audiences
have been interviewed (or the proxy clause is invoked with the full
documentation requirements satisfied).

Mark each item with `[x]` after the interview is logged below it. The
`scripts/handoff-tier1-gates.sh status` count tallies `^- \[x\]` lines.

- [ ] Media-archaeology archivist interview
  - Interviewee:
  - Date:
  - Surfaced concerns:
  - Resolutions / open items:

- [ ] Algorave / live-coder interview
  - Interviewee:
  - Date:
  - Surfaced concerns:
  - Resolutions / open items:

- [ ] Digital-humanities researcher interview
  - Interviewee:
  - Date:
  - Surfaced concerns:
  - Resolutions / open items:

- [ ] AI-provenance practitioner (C2PA / SynthID) interview
  - Interviewee:
  - Date:
  - Surfaced concerns:
  - Resolutions / open items:

## Proxy mode — invocation record (v0.1.6-local → v0.1.7-local)

Per `governance.md` §G6 clause 2, every proxy invocation must be documented.
The bootstrap operator (Claude Code on user's request) invoked the proxy
clause once, in lieu of all four human interviews, while preparing the public
GitHub repository. Each invocation is logged below so future contributors can
audit the substitution.

**The release blocker remains in effect.** Per §G6 clause 1, "Publishing
requires four real human interviews." Proxy invocation does NOT lift the
blocker; v0.1 cannot drop the `-local` suffix on this basis. The proxy
record exists to make the substitution visible, not to authorize publishing.

### Proxy invocation #1 — all four audiences

- **Which voice was used:** Friedrich Kittler, *Open-Source-Tagung 1999
  (Wizards of OS 1)* lecture, hosted on Internet Archive
  (https://archive.org/details/WOS1_170799_1600_KITTLER). Listed in
  `docs/candidates-and-hidden-resources.md` as the canonical proxy.
- **Why human contact failed:** No human contact was attempted in this
  bootstrap session. The user explicitly delegated G6 to Claude with the
  instruction "やって"; Claude cannot initiate first contact with external
  humans (out-of-scope per R13's outbound-restriction reading), so the only
  Claude-feasible path is to invoke proxy mode and record that it is the
  weaker substitute. Real interviews remain a user action.
- **What the proxy said (paraphrase + cross-reference):**
  - **Archivist proxy reading:** The lecture frames OSS as a *process* in
    which the development tools are themselves the cultural-technique artifact
    — supports the v0.1 design of L1 ingest staying GPL-3 to inherit the GNU
    Radio process, not just its output.
  - **Algorave proxy reading:** The lecture's "no software" thesis maps onto
    the `IntoPatternAtom::to_audio` materiality contract: every pattern must
    be re-groundable into the time-domain signal it came from, so live coding
    cannot pretend the medium is absent. Confirms L3 wrap-not-fork design.
  - **DH proxy reading:** The 1800/1900/2000 *Aufschreibesystem*
    periodization is the lecture's own historical frame; the v0.1 status
    statement ("treats archival media as analyzable signals rather than as
    text-equivalents") is direct application, not novel claim.
  - **AI-provenance proxy reading:** The lecture predates C2PA by ~25 years;
    the proxy cannot speak to SynthID specifics or the 2024 EU AI Act
    research-and-cultural-heritage clause. This is the audience for which
    proxy substitution is weakest — the one most likely to surface concrete
    technical objections that the bootstrap missed.
- **What the project would have done differently with a live human, per
  audience:**
  - Archivist: would have asked about deposit-library workflow (BnF / DNB /
    NDL) and whether the per-detector REJECT semantics in `ethics-audit` are
    operationally compatible with their existing CI fail thresholds.
  - Algorave: would have asked whether `cycle_duration` on `SyntheticIq` being
    "the configured duration" is musically usable, or whether v0.2 needs a
    notion of "cycle" decoupled from "fixture length."
  - DH: would have asked whether the `EventStream` JSON shape is
    research-data-management compatible (FAIR / DataCite / Schema.org dataset
    descriptors).
  - **AI-provenance: would have asked whether emitting a `c2pa-shape/0.1.0`
    JSON without going through `c2pa-rs` (deferred to v0.2) risks shipping
    manifests that fail real C2PA validators — and whether the v0.1 schema
    URL should be reserved on a controlled namespace before any tagged
    release.** This is the question most likely to identify a v0.1.x
    regression that the bootstrap did not catch.

### Tag policy under proxy mode (enforced by this record)

`v0.1.{0..7}-local` are local-only tags per §G6 clause 3. They may exist on
the public GitHub remote as historical record, but **no `v0.1.0` (unsuffixed)
tag may be created**, and `cargo publish` (G4) **must not be invoked**, until
four real interviews replace this proxy record above the "## Proxy mode"
heading.

A future contributor reading this file: if you complete a real interview,
move it above this section and mark the corresponding `- [ ]` as `- [x]`.
Once 4/4 are `[x]`, the proxy record can be moved into `docs/dropped-designs.md`
as an example of the substitution the project chose against.
