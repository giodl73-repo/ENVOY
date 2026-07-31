# ENVOY

**International Affairs 2.0 — distinguish money obligated from assistance
delivered and outcomes that last.**

ENVOY's first feature follows aggregate assistance portfolios from obligation
through disbursement, verified output, and durable outcome. It reports the
largest delivery gap and keeps local-partner share, diversion risk, and
do-no-harm review as independent constraints.

Its first official run groups ForeignAssistance.gov's FY2024 country file into
seven source regions. It finds **$85.781B obligated** and **$71.576B disbursed**,
a same-year ratio of **83.44%**. Two regions—including the source's `World`
category—have disbursements above obligations. That is the substantive lesson:
these are not matched cohorts, so their difference is not an unspent balance
and their ratio is not delivery realization or an outcome.

The fictional Cedar portfolio obligates $150 million and disburses $120
million. Weighted verified output is 78.20% of disbursement, while weighted
durable outcome is 56.35%. The tool therefore prevents either the obligation or
the payment from being mislabeled as an achieved outcome.

## Try it

```powershell
cargo run --quiet -- analyze fixtures/cedar-assistance-realization.tsv
cargo run --quiet -- held-pack fixtures/cedar-assistance-realization.tsv
cargo run --quiet -- official-baseline fixtures/official/foreignassistance-fy2024-regions.tsv
cargo run --quiet -- official-held-pack fixtures/official/foreignassistance-fy2024-regions.tsv
```

The Cedar portfolios remain synthetic. The official baseline is a compact,
checksum-custodied derivation of the public ForeignAssistance.gov country file.
State's Foreign Affairs Manual supplies the accounting and performance-evidence
boundary; neither fixture contains award, source, or operational detail.

## What this proves

- Obligations, disbursements, outputs, and durable outcomes remain distinct.
- Same-year aggregate payments and obligations cannot be treated as a matched
  program cohort when regional payments can exceed obligations.
- Lower spending is not efficiency when delivery or local ownership collapses.
- Diversion and do-no-harm gates can block promotion independently of spend.
- Taxlane can receive a held INT finding without policy or diplomatic authority.

## Validate

```powershell
cargo fmt --check
cargo test --all-targets
cargo run --quiet -- analyze fixtures/cedar-assistance-realization.tsv
cargo run --quiet -- official-baseline fixtures/official/foreignassistance-fy2024-regions.tsv
```

Official anchors: [State 4 FAM 080](https://fam.state.gov/fam/04fam/04fam0080.html) and [State 18 FAM 301.4](https://fam.state.gov/fam/18fam/18fam030104.html).

## Boundary

ENVOY is public-aggregate research software. It performs no operational
planning, partner selection, award action, country ranking, diplomatic advice,
official scoring, savings claim, allocation, rate instruction, or release authorization.
