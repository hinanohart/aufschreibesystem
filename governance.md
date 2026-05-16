<!--
SPDX-License-Identifier: CC-BY-SA-4.0
SPDX-FileCopyrightText: 2026 Kittler Aufschreibesystem Synthesizer contributors
-->

# Governance — Kittler Aufschreibesystem Synthesizer

This file is the project's ethical contract. It exists because making an OSS
that touches archival signals, AI sidecars and live-coding venues without
governance is itself a Kittlerian failure: pretending the symbolic order
("just code, just MIT") can be cleanly separated from the real order
(broadcast law, colonial recordings, AI commercial clauses, model weights).

## Seven gates (G1–G7)

| #  | Gate                                          | Why it is human-gated                                          | Automation |
|----|-----------------------------------------------|----------------------------------------------------------------|------------|
| G1 | `gh repo create` under user identity          | R13 — Claude must not author repos under a human's identity.   | 0%         |
| G2 | `gh secret set` for CI tokens                 | R11 — secrets never transit an LLM.                            | 0%         |
| G3 | `git push origin main` (and force-push)       | R13 — irreversibility threshold; a human must press send.      | 0%         |
| G4 | `cargo publish` / `npm publish`               | R13 — once published, you cannot un-publish a name.            | 0%         |
| G5 | Qwen3-Omni sidecar default-on vs default-off  | UX choice with commercial-license consequences. See `LICENSE`. | 30%        |
| G6 | User-interview completion                     | See §"G6 — the proxy interview clause" below.                  | 0%         |
| G7 | "Thought self-criticism" merge gate           | Every PR must clear the 5-axis fidelity filter.                | template   |

## G6 — the proxy interview clause (思想的二重底の試金石)

Pre-v0.1 the architecture required interviews with three audiences: a
media-archaeology archivist, an Algorave live-coder, and a DH researcher.
If 24 hours pass without a human respondent, the architecture's contingency
was: substitute Kittler's own 1999 *Wizards of OS 1* lecture (Internet Archive)
as a proxy "respondent."

We are **not** taking that contingency lightly. It is structurally identical
to the rejected design B3 (F5-TTS reconstruction of lost syllables): an AI
washing the real order with the symbolic. Using a recorded voice as a stand-in
for an absent human collaborator is itself the failure mode we built this OSS
to refuse.

Therefore:

1. **Proxy interview use is a v0.1 release blocker.** v0.1 may exist locally,
   tagged, with all CI green, and *still not be published* if it was reached
   via proxy interview alone. Publishing requires three real human interviews.
2. **If proxy mode is invoked at all, this file must document it explicitly**,
   including: which voice was used, why human contact failed, what the proxy
   said, and what the project would have done differently with a live human.
3. **A v0.1 release reached purely by proxy is allowed to be tagged
   `v0.1.0-local`** (the suffix is load-bearing) — for the developer to
   continue iterating — but **not** to bear an unsuffixed semver tag and
   **never** to be pushed to a public registry.

The honest position: the proxy clause exists so that the temptation to use
it is visible, not so that the project can rely on it.

## Bus-factor declarations

- **TidalCycles** — Alex McLean, single primary maintainer. We `wrap` (not
  fork) Strudel and pin a known-good commit.
- **Strudel** — Archived on GitHub 2025-06-19 and migrated to
  `https://codeberg.org/uzu/strudel`. We track the Codeberg upstream and
  document the migration risk: a second migration would force `web-stage`
  to vendor.
- **Greaseweazle** — Keir Fraser, single primary maintainer.
- **c2pa-rs** — Adobe maintained; OSS but bus-factor inside one vendor.
- **GNU Radio** — broad community; lowest bus-factor risk in the dependency
  tree.

## Ethics audit (CI-enforced)

Seven detector slots exist in the audit shape; **v0.1 implements only the
filename-scan subset** (detectors 1–3). Detectors 4–7 are declared in the
`Finding` enum but have no detection logic yet — they are v0.2 work and the
shape exists so that contributors cannot quietly add a fixture that the
project will later need to retract. The honest line is at
`crates/ethics-audit/src/lib.rs:36`.

| # | Detector                                       | v0.1 status                                  |
|---|-----------------------------------------------|----------------------------------------------|
| 1 | RDS PS / PTY presence in IQ samples           | filename-scan (real RDS decoder in v0.2)     |
| 2 | SCTE-35 commercial-cue markers                | filename-scan (real SCTE-35 decoder in v0.2) |
| 3 | Station callsign text in metadata             | filename + 5-callsign hardcoded list         |
| 4 | Recording-year ISO date (PD vs in-copyright)  | enum variant only, no detector yet (v0.2)    |
| 5 | Colonial-context GPS metadata                 | enum variant only, no detector yet (v0.2)    |
| 6 | Missing C2PA manifest on derived artifacts    | enum variant only, no detector yet (v0.2)    |
| 7 | Recording-location language ID                | enum variant only, no detector yet (v0.2)    |

A fixture flagged by any *implemented* detector is **rejected by CI**, not
merely warned. Broadcast recordings are forbidden as fixtures at any commit
reachable from `main`. As detectors 4–7 are implemented they inherit the same
*reject, not warn* semantics — they do not become opt-in.

### Documentation exemptions (v0.1 carve-out, must be re-evaluated in v0.2)

Filenames containing `readme` (case-insensitive) are exempted from detectors
4 (YearMissing) and 7 (LanguageIdMissing). Rationale: a `README.md` placed
inside `fixtures/` is documentation about the fixtures, not a fixture itself,
and forcing it to carry an ISO year + ISO 639-3 language tag would push us
into the "metadata theater" failure mode (declarations satisfying the form
without bearing the content). In v0.2 we either move documentation files
*outside* `fixtures/` or replace this carve-out with a documented MIME-type
test. Declared here so the carve-out is visible, not silent — per the
meta-monitor's audit of v0.1.1-local.

## Memory and provenance (3-stage C2PA chain)

Every artifact emitted by `kittler-archive` carries a C2PA manifest with three
nested provenance assertions:

1. **Raw stage** — SHA-256 of the raw IQ bytes, capture device fingerprint.
2. **Pattern stage** — SHA-256 of the `IntoPatternAtom` AST, algebra version.
3. **Interpretation stage** — model ID, prompt, SynthID watermark, sidecar
   version. **Required if an AI step was applied.**

A consumer reading only stages 1–2 sees a chain rooted in physical capture.
A consumer reading stage 3 sees that an interpretation was applied and by what
agent. The chain is not optional.

## Kittler thought-fidelity filter (5 axes)

Every PR description must answer these five questions:

1. **Materiality.** Does this change preserve the signal as signal, or does
   it flatten the medium into an embedding?
2. **Subject illusion.** Does this change reinforce the user's sense of
   authorial control, or does it surface the apparatus producing them?
3. **Military / state origin.** If this code touches protocols (FFT, TCP/IP,
   GPS, GPU shaders), is their origin acknowledged in the docs?
4. **Real-time-ness.** Does this change keep end-to-end latency under 20 ms,
   or does it push the project toward batch processing?
5. **Cultural-technique priority.** Is the *Aufschreibesystem* described
   before the analysis built on it, or is the analysis treated as primary?

A PR that cannot answer all five honestly is closed with the comment
"thought-fidelity gate — see `governance.md`."
