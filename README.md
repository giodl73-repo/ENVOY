# ENVOY

**International Affairs 2.0 — distinguish money obligated from assistance
delivered and outcomes that last.**

ENVOY's first feature follows aggregate assistance portfolios from obligation
through disbursement, verified output, and durable outcome. It reports the
largest delivery gap and keeps local-partner share, diversion risk, and
do-no-harm review as independent constraints.

The repo-local [VERDICT capability assessment](docs/vtrace/CAPABILITY_ASSESSMENT.md)
scores the current program **15/21**. Compact attribution, durable local
outcomes, lifecycle value, and fiscal authority remain held until complete
evidence joins support them.

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
cargo run --quiet -- candidate-baseline fixtures/official/mcc-nepal-compact-fy2025.tsv
cargo run --quiet -- candidate-held-pack fixtures/official/mcc-nepal-compact-fy2025.tsv
cargo run --quiet -- level2-baseline fixtures/official/mcc-nepal-kpi-2026-q11.tsv
cargo run --quiet -- level2-held-pack fixtures/official/mcc-nepal-kpi-2026-q11.tsv
cargo run --quiet -- held-pack fixtures/cedar-assistance-realization.tsv
cargo run --quiet -- official-baseline fixtures/official/foreignassistance-fy2024-regions.tsv
cargo run --quiet -- official-held-pack fixtures/official/foreignassistance-fy2024-regions.tsv
```

The Cedar portfolios remain synthetic. The official baseline is a compact,
checksum-custodied derivation of the public ForeignAssistance.gov country file.
State's Foreign Affairs Manual supplies the accounting and performance-evidence
boundary; neither fixture contains award, source, or operational detail.

## Nepal Compact delivery candidate

ENVOY now follows one active, non-sensitive program: the Millennium Challenge
Corporation's Nepal Compact. The official envelope combines a **$550M U.S.
compact** with a separately preserved **$197M Government of Nepal
contribution**, for **$747M** of public commitment.

MCC's FY2025 report divides the U.S. amount into $24M of compact-facilitation
funding and $526M of program assistance. Reported expenditures are $24M and
$91.8M respectively:

| FY2025 reported position | Amount | ENVOY treatment |
|---|---:|---|
| U.S. obligations | $550.0M | compact funding envelope |
| U.S. expenditures | $115.8M | financial execution, not outcome |
| Unexpended U.S. obligations | $434.2M | scheduled capacity, not savings |
| Nepal contribution | $197.0M | partner cofunding, not a U.S. offset |

Delivery is real but unfinished. Contracts cover **297 km of transmission
lines**, and contractors mobilized for **three substations**. Those are contract
and mobilization milestones—not completed outputs. Projected beneficiaries,
economic returns, reliable-electricity gains, road quality, local incidence,
safeguards, and durable outcomes remain held until verified.

### Level 2 quarterly comparison baseline

MCC's July 2026 KPI table supplies the first current implementation comparison
surface. Through quarter 11, the compact reports **76 of 130 road-design
kilometers**, published as 58% of target, and **52 people trained** in road
technology: 9 women and 43 men. It separately reports 13 power-regulation
trainees, but no target for that indicator.

This is partial delivery evidence, not completed infrastructure or durable
benefit. Temporary employment, 17 targeted power transformers, and 99 targeted
municipal partnership wards remain `Pending`. Completed works, service quality,
local benefit and burden incidence, grievances, safeguards, diversion, and
durable outcomes are not reported in the KPI table.

The KPI header also retains the original **$500M** compact budget while the
amended program envelope is **$550M**. ENVOY preserves that source-version
mismatch rather than silently reconciling it. The comparison-baseline gate is
complete; candidate admission remains held.

## What this proves

- Obligations, disbursements, outputs, and durable outcomes remain distinct.
- Same-year aggregate payments and obligations cannot be treated as a matched
  program cohort when regional payments can exceed obligations.
- Unexpended obligations and partner contributions are not U.S. savings.
- Lower spending is not efficiency when delivery or local ownership collapses.
- Diversion and do-no-harm gates can block promotion independently of spend.
- Taxlane can receive a held INT finding without policy or diplomatic authority.

## Validate

```powershell
cargo fmt --check
cargo test --workspace --all-targets
cargo run --quiet -- analyze fixtures/cedar-assistance-realization.tsv
cargo run --quiet -- official-baseline fixtures/official/foreignassistance-fy2024-regions.tsv
cargo run --quiet -- level2-baseline fixtures/official/mcc-nepal-kpi-2026-q11.tsv
```

Official anchors: [State 4 FAM 080](https://fam.state.gov/fam/04fam/04fam0080.html) and [State 18 FAM 301.4](https://fam.state.gov/fam/18fam/18fam030104.html).

Candidate sources: [MCC FY2025 Annual Report](https://www.mcc.gov/resources/doc/annual-report-2025/)
and [MCC Nepal Compact](https://www.mcc.gov/where-we-work/program/nepal-compact/).

Level 2 source: [Nepal Compact Table of Key Performance Indicators](https://www.mcc.gov/resources/doc/nepal-compact-kpi/).

## Boundary

ENVOY is public-aggregate research software. It performs no operational
planning, partner selection, award action, country ranking, diplomatic advice,
official scoring, savings claim, allocation, rate instruction, or release authorization.
