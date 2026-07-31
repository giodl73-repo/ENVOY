# Nepal Compact outcome reference

ENVOY now parses a bounded selection from MCC's official open-data Indicator
Tracking Table. This is the first owner slice to contain observed Nepal
system-level outcomes rather than only funding, contracts, or outputs.

| Indicator | Baseline | Observed | Movement |
|---|---:|---:|---:|
| Export revenue | $78.600M | $128.316M | +$49.716M |
| Total electricity supply | 12.369M MWh | 13.966M MWh | +1.597M MWh |
| Imported electricity | 1.833M MWh | 1.895M MWh | +3.38% |
| Exported electricity | 1.346M MWh | 1.946M MWh | +44.57% |
| Transmission technical losses | 4.49% | 4.43% | -0.06 percentage points |
| Operating cost recovery | unavailable | 113% | no baseline delta |
| Generation capacity | 3,103 MW | 3,906 MW | +803 MW |

All seven rows are national-system context. Zero are marked compact-specific
and zero have a ready compact target in the selected source rows. The result
does not identify an MCC effect because compact-specific cross-border values,
a counterfactual, exposure alignment, local incidence, safeguards, matched
costs, and an independent evaluation are absent.

```powershell
cargo run --quiet -- outcome-reference fixtures/official/mcc-nepal-outcome-reference-2025.tsv
cargo run --quiet -- outcome-held-pack fixtures/official/mcc-nepal-outcome-reference-2025.tsv
```

The held pack exposes a machine-readable reopening test and keeps savings,
allocation, rates, operational decisions, and release authority false.
