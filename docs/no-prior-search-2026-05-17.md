<!--
SPDX-License-Identifier: CC-BY-SA-4.0
SPDX-FileCopyrightText: 2026 Kittler Aufschreibesystem Synthesizer contributors
-->

# R18 NO-PRIOR search — 2026-05-17

This file records the searches performed at project start to confirm that
"signal-as-syntax" as a design space was not already occupied.

If a future contributor discovers prior art that this search missed, the
correct response is to open an issue titled "R18 prior-art discovery" and
let the project decide between: (a) withdrawal, (b) merging upstream,
(c) niche specialization. Default for v0.1 is (c).

## Queries

### Query 1 — exact phrase + neighborhood
```
"signal as syntax" OR "signal-as-syntax" OR "signal as pattern"
TidalCycles Strudel SDR archival 2026
```

**Result.** No OSS or paper named "signal-as-syntax" found. Strudel docs
contain a `signals` namespace but it refers to LFO-like modulation
sources, not to physical signals as data types.

**Additional finding.** Strudel was archived on GitHub on 2025-06-19 and
migrated to `https://codeberg.org/uzu/strudel`. This is recorded in
`governance.md` §"Bus-factor declarations" because it materially affects
the `web-stage` dependency path.

### Query 2 — tool-combination form
```
live coding archival signals "GNU Radio" Greaseweazle Strudel
pattern algebra OSS 2026
```

**Result.** No OSS combining these tools found. GNU Radio Conference 2026
(GRCon26, Raleigh NC, Sep 21–24) and FOSDEM 2026 SDR/DSP devroom are the
nearest community venues but neither shows a project in this design space.

## Conclusion

NO-PRIOR confirmed for "signal-as-syntax" as a TidalCycles/Strudel-style
pattern algebra over physical archival signals. The design space is
unoccupied at 2026-05-17.

The project proceeds without the niche-specialization fallback. The
fallback remains in `governance.md` for future re-evaluation.

## Sources

- [Strudel docs](https://strudel.cc/learn/signals/)
- [Strudel on Codeberg](https://codeberg.org/uzu/strudel)
- [GNU Radio](https://www.gnuradio.org/)
- [Tidal Cycles](https://tidalcycles.org/)
