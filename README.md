# ENVOY

**International Affairs 2.0 — distinguish money obligated from assistance
delivered and outcomes that last.**

ENVOY's first feature follows aggregate assistance portfolios from obligation
through disbursement, verified output, and durable outcome. It reports the
largest delivery gap and keeps local-partner share, diversion risk, and
do-no-harm review as independent constraints.

ENVOY now carries that screen through a complete bounded semantic program.
Sixteen executable features cover assistance realization, official regional
flows, the Nepal Compact candidate, quarterly delivery, evaluation readiness,
grievance and safeguard readiness, scenarios, completion-to-durability realization, lifecycle accounting,
alternatives, incidence, delivery feasibility, adaptive successors, normalized
illustrative comparison, and an integrated held handoff.

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
cargo run --quiet -- evaluation-readiness fixtures/official/mcc-nepal-evaluation-readiness-2026.tsv
cargo run --quiet -- evaluation-held-pack fixtures/official/mcc-nepal-evaluation-readiness-2026.tsv
cargo run --quiet -- safeguard-baseline fixtures/official/mca-nepal-safeguard-grievance-readiness-2026.tsv
cargo run --quiet -- safeguard-held-pack fixtures/official/mca-nepal-safeguard-grievance-readiness-2026.tsv
cargo run --quiet -- program-scenarios fixtures/synthetic/assistance-semantic-program.tsv
cargo run --quiet -- program-realization fixtures/synthetic/assistance-semantic-program.tsv
cargo run --quiet -- program-accounting fixtures/synthetic/assistance-semantic-program.tsv
cargo run --quiet -- program-alternatives fixtures/synthetic/assistance-semantic-program.tsv
cargo run --quiet -- program-incidence fixtures/synthetic/assistance-semantic-program.tsv
cargo run --quiet -- program-delivery fixtures/synthetic/assistance-semantic-program.tsv
cargo run --quiet -- program-adaptive fixtures/synthetic/assistance-semantic-program.tsv
cargo run --quiet -- program-peers fixtures/synthetic/assistance-semantic-program.tsv
cargo run --quiet -- program-held-pack fixtures/synthetic/assistance-semantic-program.tsv
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

### Evaluation readiness—not yet an effect

MCC's FY2026 midyear business forecast makes the next boundary concrete. The
planned **Nepal Road Maintenance Project Evaluation** (`26-DPE-2373`) has a
solicitation issued in a **$350K–$999K** value band. The planned **Nepal
Electricity Transmission Project Evaluation** (`26-DPE-2372`) is listed as
cancelled, in a **$1.0M–$4.999M** value band.

That is useful readiness evidence, but it is not an evaluation result. The
forecast does not establish the design, baseline cohort, data sources,
independent evaluator, exposure-aligned final collection, subgroup incidence,
or linkage to safeguards and grievances. ENVOY therefore keeps projected
beneficiaries and projected economic rates of return out of observed effects.
The exact reopening path is now machine-readable instead of an open-ended
request for “more evidence.”

### Safeguard and grievance readiness

MCA-Nepal's publisher report establishes a real grievance mechanism with
in-person and remote intake, three review tiers, and preserved legal recourse.
Its July 2024 aggregate snapshot records **123 grievances**, including **44
closed**—a 35.77% arithmetic share of registered grievances. The published
status bullets account for only 118 grievances: 44 closed, 45 awaiting signoff,
8 not accepted at tier one, and 21 newly under discussion. ENVOY leaves the
remaining **5** explicitly unreconciled rather than silently assigning them.

MCC's May 2026 forecast separately shows three visible Nepal support actions:
environmental/social support awarded, transportation advisory support in
progress, and technical support in progress. These establish mechanism and
support capacity—not current resolution timeliness, satisfaction, subgroup
incidence, safeguard compliance, completed works, or a compact effect. The
grievance and safeguard floor therefore remains held, now for precise reasons.

## Complete semantic-program demonstration

The synthetic program makes the entire delivery chain executable without
claiming an observed Nepal effect. Delivery falls from 58.46% to 45.00% under
stress and recovers to 65.00% in a separate immutable version. An illustrative
100,000-unit funding chain reaches 70,000 completed-service units, 60,000 units
received locally, and 50,000 durable-outcome units. The largest handoff loss is
completion; unrealized assistance is never savings.

Accounting preserves the $550M U.S. compact and $197M Nepal contribution, then
adds $30M of synthetic transition and $40M of synthetic maintenance. The $817M
demonstration proves that lifecycle resources cannot disappear and partner
funding cannot become U.S. savings. It does not revise official compact costs
or enter Taxlane.

Two of three alternatives clear declared synthetic durability, local-incidence,
and do-no-harm floors, but ENVOY selects neither. Five-group incidence
reconciles to zero and shows the local community carrying the largest burden.
Only four of eight delivery gates pass: transmission evaluation, completed
works, safeguards, and grievances remain false. The program preserves the live
road/cancelled transmission evaluation asymmetry and creates immutable
successor version 2 without diplomatic, partner, or award action.

The definition-matched 70% durability comparison is illustrative, not an
official compact peer or target. A custodied public comparator remains corpus
work.

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
cargo run --quiet -- evaluation-readiness fixtures/official/mcc-nepal-evaluation-readiness-2026.tsv
cargo run --quiet -- safeguard-baseline fixtures/official/mca-nepal-safeguard-grievance-readiness-2026.tsv
cargo run --quiet -- program-held-pack fixtures/synthetic/assistance-semantic-program.tsv
```

Official anchors: [State 4 FAM 080](https://fam.state.gov/fam/04fam/04fam0080.html) and [State 18 FAM 301.4](https://fam.state.gov/fam/18fam/18fam030104.html).

Candidate sources: [MCC FY2025 Annual Report](https://www.mcc.gov/resources/doc/annual-report-2025/)
and [MCC Nepal Compact](https://www.mcc.gov/where-we-work/program/nepal-compact/).

Level 2 source: [Nepal Compact Table of Key Performance Indicators](https://www.mcc.gov/resources/doc/nepal-compact-kpi/).

Evaluation anchors: [MCC FY2026 Business Forecast](https://www.mcc.gov/resources/doc/report-business-forecast/)
and [MCC Monitoring and Evaluation Policy](https://www.mcc.gov/resources/doc/policy-for-monitoring-and-evaluation/).

Safeguard readiness anchors: [MCA-Nepal FY2023/24 Annual Report public archive](https://nepalindata.com/media/resources/items/0/bAnnual_Report_English.pdf)
and [MCC FY2026 Business Forecast](https://www.mcc.gov/resources/doc/report-business-forecast/).

## Boundary

ENVOY is public-aggregate research software. It performs no operational
planning, partner selection, award action, country ranking, diplomatic advice,
official scoring, savings claim, allocation, rate instruction, or release authorization.
