#[derive(Debug, Clone, PartialEq, Eq)]
struct Compact {
    name: String,
    us_compact_tenths_millions: u64,
    nepal_contribution_tenths_millions: u64,
    facilitation_obligations_tenths_millions: u64,
    facilitation_expenditures_tenths_millions: u64,
    assistance_obligations_tenths_millions: u64,
    assistance_expenditures_tenths_millions: u64,
    transmission_km_under_contract: u64,
    substations_with_mobilized_contractors: u64,
}

#[derive(Debug, PartialEq, Eq)]
struct Candidate {
    us_compact_tenths_millions: u64,
    nepal_contribution_tenths_millions: u64,
    combined_public_commitment_tenths_millions: u64,
    us_obligations_tenths_millions: u64,
    us_expenditures_tenths_millions: u64,
    us_unexpended_obligations_tenths_millions: u64,
    expenditure_to_obligation_bps: u64,
    transmission_km_under_contract: u64,
    substations_with_mobilized_contractors: u64,
}

fn parse(input: &str) -> Result<Compact, String> {
    let mut rows = Vec::new();
    for (index, line) in input.lines().enumerate() {
        let line_number = index + 1;
        if line.is_empty() || line.starts_with('#') || line.starts_with("compact\t") {
            continue;
        }
        let fields: Vec<_> = line.split('\t').collect();
        if fields.len() != 9 {
            return Err(format!("line {line_number}: expected 9 fields"));
        }
        let number = |field: usize| {
            fields[field]
                .parse::<u64>()
                .map_err(|_| format!("line {line_number}: invalid nonnegative integer"))
        };
        rows.push(Compact {
            name: fields[0].to_owned(),
            us_compact_tenths_millions: number(1)?,
            nepal_contribution_tenths_millions: number(2)?,
            facilitation_obligations_tenths_millions: number(3)?,
            facilitation_expenditures_tenths_millions: number(4)?,
            assistance_obligations_tenths_millions: number(5)?,
            assistance_expenditures_tenths_millions: number(6)?,
            transmission_km_under_contract: number(7)?,
            substations_with_mobilized_contractors: number(8)?,
        });
    }
    if rows.len() != 1 {
        return Err("candidate requires exactly one compact row".into());
    }
    let row = rows.remove(0);
    if row.name != "nepal_compact" {
        return Err("candidate row must be nepal_compact".into());
    }
    let obligations =
        row.facilitation_obligations_tenths_millions + row.assistance_obligations_tenths_millions;
    let expenditures =
        row.facilitation_expenditures_tenths_millions + row.assistance_expenditures_tenths_millions;
    if obligations != row.us_compact_tenths_millions {
        return Err("U.S. funding components must reconcile to compact amount".into());
    }
    if expenditures > obligations {
        return Err("expenditures cannot exceed obligations".into());
    }
    Ok(row)
}

fn analyze(row: &Compact) -> Candidate {
    let obligations =
        row.facilitation_obligations_tenths_millions + row.assistance_obligations_tenths_millions;
    let expenditures =
        row.facilitation_expenditures_tenths_millions + row.assistance_expenditures_tenths_millions;
    Candidate {
        us_compact_tenths_millions: row.us_compact_tenths_millions,
        nepal_contribution_tenths_millions: row.nepal_contribution_tenths_millions,
        combined_public_commitment_tenths_millions: row.us_compact_tenths_millions
            + row.nepal_contribution_tenths_millions,
        us_obligations_tenths_millions: obligations,
        us_expenditures_tenths_millions: expenditures,
        us_unexpended_obligations_tenths_millions: obligations - expenditures,
        expenditure_to_obligation_bps: expenditures * 10_000 / obligations,
        transmission_km_under_contract: row.transmission_km_under_contract,
        substations_with_mobilized_contractors: row.substations_with_mobilized_contractors,
    }
}

fn baseline_json(result: &Candidate) -> String {
    format!(
        "{{\"schema\":\"envoy.nepal-compact-candidate.v1\",\"candidate_id\":\"mcc_nepal_compact_implementation\",\"evidence_label\":\"official_program_candidate\",\"program_status\":\"entered_into_force_active_implementation\",\"us_compact_tenths_millions\":{},\"nepal_contribution_tenths_millions\":{},\"combined_public_commitment_tenths_millions\":{},\"us_obligations_tenths_millions\":{},\"us_expenditures_tenths_millions\":{},\"us_unexpended_obligations_tenths_millions\":{},\"expenditure_to_obligation_bps\":{},\"transmission_km_under_contract\":{},\"substations_with_mobilized_contractors\":{},\"unexpended_obligations_are_savings\":false,\"contracts_are_completed_outputs\":false,\"outputs_are_durable_outcomes\":false}}",
        result.us_compact_tenths_millions,
        result.nepal_contribution_tenths_millions,
        result.combined_public_commitment_tenths_millions,
        result.us_obligations_tenths_millions,
        result.us_expenditures_tenths_millions,
        result.us_unexpended_obligations_tenths_millions,
        result.expenditure_to_obligation_bps,
        result.transmission_km_under_contract,
        result.substations_with_mobilized_contractors
    )
}

fn held_pack_json(result: &Candidate) -> String {
    format!(
        "{{\"schema\":\"taxlane.lane-evidence-pack-candidate.v1\",\"identity\":{{\"pack_id\":\"envoy:mcc-nepal-compact-implementation:v1\",\"track\":\"INT\",\"domain_repository\":\"ENVOY\",\"candidate_id\":\"mcc_nepal_compact_implementation\",\"candidate_name\":\"MCC Nepal Compact implementation\",\"fiscal_owner\":\"TAXLANE\"}},\"scope\":{{\"geography\":\"Nepal\",\"population_or_network\":\"electricity transmission and national road maintenance\",\"ownership\":\"MCC oversight with MCA-Nepal implementation and Government of Nepal cofunding\",\"time_basis\":\"entered force 2023-08-30; FY2025 financial position; compact end 2028-08-30\",\"unit_basis\":\"tenths of millions of U.S. dollars kilometers and substations\",\"included\":\"compact funding facilitation assistance contracts and public physical milestones\",\"excluded\":\"award-level source-sensitive operational and unverified outcome detail\"}},\"source_custody\":{{\"source_id\":\"MCC-ANNUAL-REPORT-2025\",\"publisher\":\"Millennium Challenge Corporation\",\"source_path_or_url\":\"https://www.mcc.gov/resources/doc/annual-report-2025/\",\"vintage\":\"published 2026-02-04 for FY2025\",\"capture_status\":\"transcribed_official_tables_and_program_milestones_with_reconciliation_tests\",\"checksum_or_null\":null,\"companion_source_id\":\"MCC-NEPAL-COMPACT-PROGRAM\"}},\"problem\":{{\"baseline_metric\":\"compact financial execution and delivery chain\",\"baseline_value_or_null\":{},\"affected_population_or_exposure_or_null\":null,\"problem_boundary\":\"FY2025 reported financial position is not a matched annual cohort or durable outcome\",\"us_expenditure_to_obligation_bps\":{},\"unexpended_obligations_tenths_millions\":{}}},\"intervention\":{{\"mechanism\":\"time-limited MCC compact for electricity transmission road maintenance and institutional capacity\",\"implementing_owner\":\"MCA-Nepal with Government of Nepal ownership and MCC oversight\",\"eligibility_rule\":\"existing bilateral compact and disbursement conditions; no partner or award decision by ENVOY\",\"exclusions\":\"no operational targeting procurement direction country ranking or award action\",\"existing_treatment_or_programmed_work\":\"active compact with signed works contracts and five-year implementation clock\"}},\"outcomes\":{{\"bounded_marginal_effect_or_null\":null,\"effect_population\":\"Nepal electricity users road users and project-affected municipalities\",\"horizon\":\"compact implementation through 2028-08-30 and later evaluation\",\"uncertainty\":\"beneficiary counts economic returns completion and durable outcomes remain projected or unverified\",\"transferability_boundary\":\"Nepal compact only\"}},\"service_floors\":{{\"access\":\"projected access and beneficiary claims not treated as achieved\",\"quality_safety\":\"construction quality safety and reliability results absent\",\"equity_distribution\":\"benefit-sharing local incidence resettlement and subgroup results absent\",\"adequacy_resilience\":\"durable electricity and road-maintenance outcomes absent\",\"delivery_feasibility\":\"297 km under contract and three substations mobilized; completion evidence absent\",\"local_partner_bps\":null,\"diversion_risk_bps\":null,\"do_no_harm_pass\":null}},\"costs\":{{\"price_year_or_null\":\"nominal compact dollars reported in FY2025\",\"gross_cost_or_null\":{},\"implementation_cost_or_null\":{},\"maintenance_cost_or_null\":null,\"offsets_or_null\":0,\"dedicated_receipts_or_null\":null,\"state_local_private_shift_or_null\":\"Government of Nepal contribution retained separately and not netted from U.S. cost\",\"net_cost_or_null\":{},\"public_savings\":null}},\"fiscal_bridge\":{{\"gross_public_funding_need_or_null\":{},\"delivery_efficiency_public_savings_or_null\":null,\"external_economic_benefit_or_null\":null,\"operator_or_private_revenue_or_null\":null,\"legally_dedicated_public_receipts_or_null\":null,\"collection_and_financing_cost_or_null\":null,\"net_public_fiscal_pressure_or_null\":{},\"revenue_authority\":\"none\",\"demand_and_incidence_basis\":\"official compact and FY2025 MCC report\",\"netting_rule\":\"Nepal cofunding is additional public investment not a U.S. savings offset; unexpended obligations are not savings\"}},\"adaptive_pathways\":{{\"pathway_classes\":\"active time-limited infrastructure and institutional delivery\",\"peer_goal_basis\":null,\"evaluation_horizons\":\"quarterly delivery FY2025 position 2028 compact end and post-compact evaluation\",\"realization_owner_or_null\":\"MCA-Nepal and MCC\",\"transition_and_implementation_cost_or_null\":{},\"uncertainty_and_downside\":\"cost schedule procurement completion safeguard and durable-outcome risks remain\",\"service_floor_and_distribution_result\":\"held\",\"overlap_and_non_additivity\":\"electricity and road beneficiary estimates overlap; cofunding and U.S. compact cannot be counted as separate outcomes\",\"observation_cadence\":\"quarterly implementation and annual reporting\",\"reopen_triggers\":\"matched expenditure-output cohorts verified completion local incidence safeguards and durable results\",\"current_disposition\":\"held\"}},\"delivery\":{{\"capacity\":\"297 transmission kilometers under contract and contractors mobilized for three substations\",\"schedule\":\"five-year compact ending 2028-08-30\",\"milestones\":\"works contracts signed and contractors mobilized; road recycling contract pending in FY2025 report\",\"useful_life\":null,\"sunset_or_review\":\"compact end and independent post-compact evaluation\"}},\"overlap\":{{\"shared_projects\":\"electricity transmission road maintenance regulation and benefit sharing\",\"shared_cost_allocation\":\"facilitation and section 605 assistance retained separately\",\"other_lane_interactions\":\"TRN SEE INT\",\"non_additivity_rule\":\"do not add overlapping beneficiaries projected economic benefits partner contribution or unexpended obligations as savings\"}},\"readiness\":{{\"domain_evidence_ready\":true,\"candidate_bounded\":true,\"outcome_ready\":false,\"cost_ready\":true,\"floors_ready\":false,\"delivery_ready\":false,\"overlap_ready\":false,\"taxlane_admission_ready\":false}},\"claim_boundaries\":{{\"domain_finding_allowed\":true,\"candidate_recommendation_allowed\":false,\"operational_or_award_decision_allowed\":false,\"savings_allowed\":false,\"allocation_allowed\":false,\"rate_change_allowed\":false,\"public_release_allowed\":false}}}}",
        result.us_expenditures_tenths_millions,
        result.expenditure_to_obligation_bps,
        result.us_unexpended_obligations_tenths_millions,
        result.us_compact_tenths_millions,
        result.us_compact_tenths_millions,
        result.us_compact_tenths_millions,
        result.combined_public_commitment_tenths_millions,
        result.us_compact_tenths_millions,
        result.us_compact_tenths_millions
    )
}

pub fn run(command: &str, input: &str) -> Result<String, String> {
    if command.starts_with("level2-") {
        return level2::run(command, input);
    }
    if !input.contains("# evidence_label=official_program_candidate") {
        return Err("candidate command requires official_program_candidate evidence".into());
    }
    if !input.contains("# program_status=entered_into_force_active_implementation") {
        return Err("candidate fixture must state active program status".into());
    }
    let result = analyze(&parse(input)?);
    match command {
        "candidate-baseline" => Ok(baseline_json(&result)),
        "candidate-held-pack" => Ok(held_pack_json(&result)),
        _ => Err(format!("unknown candidate command: {command}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = include_str!("../../../fixtures/official/mcc-nepal-compact-fy2025.tsv");

    #[test]
    fn reconciles_compact_funding_without_netting_partner_contribution() {
        let result = analyze(&parse(FIXTURE).unwrap());
        assert_eq!(result.us_compact_tenths_millions, 5_500);
        assert_eq!(result.nepal_contribution_tenths_millions, 1_970);
        assert_eq!(result.combined_public_commitment_tenths_millions, 7_470);
        assert_eq!(result.us_obligations_tenths_millions, 5_500);
        assert_eq!(result.us_expenditures_tenths_millions, 1_158);
        assert_eq!(result.us_unexpended_obligations_tenths_millions, 4_342);
        assert_eq!(result.expenditure_to_obligation_bps, 2_105);
    }

    #[test]
    fn keeps_contracts_outputs_and_outcomes_distinct() {
        let output = baseline_json(&analyze(&parse(FIXTURE).unwrap()));
        assert!(output.contains("\"unexpended_obligations_are_savings\":false"));
        assert!(output.contains("\"contracts_are_completed_outputs\":false"));
        assert!(output.contains("\"outputs_are_durable_outcomes\":false"));
    }

    #[test]
    fn preserves_public_delivery_milestones() {
        let result = analyze(&parse(FIXTURE).unwrap());
        assert_eq!(result.transmission_km_under_contract, 297);
        assert_eq!(result.substations_with_mobilized_contractors, 3);
    }

    #[test]
    fn held_pack_is_bounded_but_not_admissible() {
        let pack = held_pack_json(&analyze(&parse(FIXTURE).unwrap()));
        assert!(pack.contains("\"candidate_bounded\":true"));
        assert!(pack.contains("\"cost_ready\":true"));
        assert!(pack.contains("\"outcome_ready\":false"));
        assert!(pack.contains("\"taxlane_admission_ready\":false"));
        assert!(pack.contains("\"public_savings\":null"));
    }

    #[test]
    fn rejects_unreconciled_us_funding_components() {
        let changed = FIXTURE.replacen("240\t240\t5260", "250\t240\t5260", 1);
        assert!(parse(&changed).is_err());
    }
}
mod level2;
