# Nepal Compact result-chain audit

ENVOY audits the complete public MCC Nepal indicator universe for the four
fields required to interpret progress: level, baseline, target, and actual.

| Readiness measure | Rows |
|---|---:|
| Total indicators | 186 |
| Outcome indicators | 73 |
| Output indicators | 59 |
| Process indicators | 39 |
| Risk/assumption indicators | 15 |
| Baseline available | 98 |
| End target available | 26 |
| Actual available | 27 |
| Baseline + target + actual | 1 |
| Outcome baseline + target + actual | 0 |
| Explicit MCC-cross-border rows | 3 |
| MCC-cross-border rows with actual | 0 |

The single complete join is a process measure: road kilometers under design.
It advances from 40 of 130 km in the open-data snapshot to 76 of 130 km in the
current quarter-11 report. The additional 36 km is real design progress, but
it does not establish completed works or a service outcome.

```powershell
cargo run --quiet -- result-chain-audit fixtures/official/mcc-nepal-result-chain-audit-2025-2026.tsv
cargo run --quiet -- result-chain-held-pack fixtures/official/mcc-nepal-result-chain-audit-2025-2026.tsv
```

Candidate-effect review remains held until a compact-specific outcome joins a
baseline, target, actual, credible counterfactual, local incidence, safeguards,
matched costs, and independent results.
