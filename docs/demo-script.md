<!--
SPDX-License-Identifier: CC-BY-SA-4.0
SPDX-FileCopyrightText: 2026 Kittler Aufschreibesystem Synthesizer contributors
-->

# 30-second demo script (v0.1 DoD)

The v0.1 milestone defines its own demo as a release gate (architecture
memo §"4 milestone — v0.1 MVP"). This file is the script. Recording the
video itself is human work — a v0.2 PR.

## Voiceover (≈30 seconds)

> "A synthetic IQ signal — a single 440-hertz tone sampled at 48 kilohertz —
> enters the algebra. The algebra does not turn it into an embedding.
> It turns it into a pattern atom: one event per cycle, frequency 440 hertz,
> with a provenance tag that says 'synthetic.' We hash the raw bytes, we
> hash the pattern AST, and we mint a two-stage C2PA manifest. The third
> stage — an AI interpretation — is empty. We did not call a model.
> That emptiness is the project's first invariant: the signal is the signal."

## Shell commands shown

```
$ cargo run -p signal-algebra --example iq_to_pattern -- \
    --duration-ms 1000 --frequency-hz 440 --output target/mvp.ast.json
wrote 1 events (736 bytes) to target/mvp.ast.json

$ jq '.provenance.origin, .events[0].value_atom' target/mvp.ast.json
"Synthetic"
{
  "kind": "frequency",
  "hz": 440.0
}

$ cargo test --workspace 2>&1 | tail -3
test result: ok. 14 passed; 0 failed
```

## What this demo does NOT show (and why)

- **No browser / no Strudel UI.** That is v0.2.
- **No AI sidecar.** It would require the user-pull dance with Tongyi —
  visible only on a real machine, not in a 30-second video.
- **No real RF capture.** v0.1 fixtures are synthetic only, by policy
  (`fixtures/README.md`).
- **No "wow" final frame.** Doing so would shift the demo from
  *cultural-technique demonstration* to *product reveal*, which violates
  thought-fidelity axis 5.

## Recording notes (for the human recorder)

- Use `asciinema` or OBS, not a video model regenerating shell output.
  (A regenerated demo would be the symbolic order rewriting the real, in
  the strictest Kittlerian sense.)
- The voiceover is the human's voice or absent. Do *not* TTS-clone Kittler.
