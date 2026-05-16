<!--
SPDX-License-Identifier: CC-BY-SA-4.0
SPDX-FileCopyrightText: 2026 Kittler Aufschreibesystem Synthesizer contributors
-->

# Supervisor protocol — 3-agent oversight for autonomous bootstraps

This file documents how the v0.1 bootstrap was supervised. It is part of
the repository so that contributors who themselves automate part of the
project can reuse the pattern rather than invent a worse one.

The protocol is not a claim that automation is "safe" or "complete." It is
the cheapest honest answer to "how do you know the agent did not lie?"

## Three roles

1. **EXECUTOR** — the agent (here: the Claude session that wrote v0.1).
   Implements the architecture. Runs `cargo test`, `cargo run`, `git commit`.
   Never crosses a Tier-1 gate (G1 / G3 / G4) on its own.

2. **MONITOR-1** (architecture-fidelity) — an independent critic agent.
   Reads the prior conversation's architecture memory and the workspace,
   and reports whether the implementation matches the architecture honestly.
   Focus: *did the agent skip anything important?*

3. **MONITOR-2** (build / fabrication detector) — an independent verifier
   agent. Runs the build, runs the tests, runs the scripts. Reads doc claims
   and cross-checks them against source.
   Focus: *did the agent claim something false?*

4. **META-MONITOR** — a third independent agent.
   Reads the reports of MONITOR-1 and MONITOR-2 and answers: *did both
   monitors function? did they cover non-overlapping risks? did either of
   them collude with the executor's framing?*

## Why three monitors and not one

A single auditor agent inherits the executor's framing (the prompt that
launched it). Two auditors with disjoint prompts can each cover a risk
class the other misses. A third auditor exists because two are not enough
to detect mutual blind spots: the classic "who watches the watchers" problem,
solved structurally rather than asserted away.

## What MONITOR-1 + MONITOR-2 together must cover

- All architecture-required artifacts present.
- All build / test pipelines actually green.
- All Tier-1 gates **not** crossed by the executor.
- No documentation claim that the code does not support.
- No code feature whose intent the documentation does not describe.
- Honest-stub vs dishonest-stub classification for every stub directory.
- The five thought-fidelity axes are still enforceable (not just declared).
- No drift toward "perfect / complete / fully automatic" language anywhere
  in the artifacts themselves.

## What META-MONITOR additionally checks

- Were MONITOR-1 and MONITOR-2 actually independent? (Did they cite each
  other rather than the artifacts?)
- Did either skip a risk class because the other "would have caught it"?
- Did the executor's commit message frame the work in a way that the
  monitors uncritically adopted?
- Was the report register honest? (PASS / FIX-REQUIRED / FABRICATION-DETECTED
  used distinguishably, not as gradients of approval.)

## Re-runnable

Future contributors can re-run this protocol on any PR they suspect was
written by an agent. The architecture memo and the workspace are the only
inputs the auditors need.

## Honest limit of this protocol

Three monitors and a meta-monitor still cannot detect a *coherent* lie:
if the executor and the architecture memo both omit the same thing in the
same way, no auditor here will catch it. This is why the project's
*human-gated* G6 (user interview) remains mandatory and is not
substitutable by any number of monitor agents.

## Appendix — agent identifiers (preserved from the bootstrap rounds)

These IDs are the audit anchor for the v0.1 bootstrap. Re-invoking by ID
preserves continuity across sessions; new agent IDs would lose the audit
chain. AUDIT-B in v0.1.3-local asked for this list to be restored.

**Concept round (2026-05-17, 4-agent + critic R14):**
- Agent A (media-archaeology + signal): `ae0801730b169b6f6`
- Agent B (multimodal AI): `ac361c42cb08b48a9`
- Agent C (cultural-technique + critical-computing): `ae19264d7ad5504f0`
- Critic (5-axis integration): `a2ea116002b7649a5`

**Architecture round (2026-05-17, 3-agent + critic R14):**
- Agent D (core data flow): `a6161b5eeca24b7eb`
- Agent E (autonomous pipeline): `a2d328c97a0ef5d9c`
- Agent F (distribution / license): `a653b2f96aadc63ae`
- Critic (integration): `af3aef2b069daed52`

**Bootstrap supervisor rounds (this session):**
- MONITOR-1 (architecture-fidelity, v0.1.0→v0.1.1): `abb2f1aab57c5a172`
- MONITOR-2 (build/fabrication verifier, v0.1.0→v0.1.1): `a9723dea27b781d06`
- META-MONITOR (v0.1.1→v0.1.2): `a09efd3c9906214cf`
- AUDIT-A (concept coverage, v0.1.2→v0.1.3): `a3e0993ac8b294b80`
- AUDIT-B (architecture coverage, v0.1.2→v0.1.3): `a26ddc14b8b260003`
- AUDIT-C (code polish, v0.1.2→v0.1.3): `a497bbdfcdd48c0da`

## Appendix — notification channel status

The architecture memo's "first 1 hour" step 7 named a PushNotification
channel (Mastodon DM / email) for G2–G7 user notifications with a 24 h
no-response → proxy-mode degrade. **v0.1 does NOT configure any notification
channel.** The degrade behavior is documented in `governance.md` §G6 but the
channel itself is not constructed because (a) configuring an outbound channel
for a `-local` build would attach the OSS to a specific endpoint we do not
own, and (b) the channel-of-record decision should belong to the user who
runs `gh repo create` (G1), not to the bootstrap executor. v0.2 PR will
either add a `governance/notifications.toml` schema or formally drop the
notification gate from the seven-gate model.

