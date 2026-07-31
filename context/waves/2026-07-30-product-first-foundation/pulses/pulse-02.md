# Pulse 02 — official foreign-assistance financial baseline

Added a checksum-custodied FY2024 regional derivation from the public
ForeignAssistance.gov country file. It exposes same-year obligations,
disbursements, and non-cohort behavior without inferring an unspent balance,
delivery realization, outputs, outcomes, local incidence, or savings.

Validation:

```powershell
cargo fmt --check
cargo test --all-targets
cargo run --quiet -- official-baseline fixtures/official/foreignassistance-fy2024-regions.tsv
cargo run --quiet -- official-held-pack fixtures/official/foreignassistance-fy2024-regions.tsv
```
