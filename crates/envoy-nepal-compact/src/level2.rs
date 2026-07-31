#[derive(Debug, PartialEq, Eq)]
struct Baseline {
    quarter: u64,
    kpi_header_budget_millions: u64,
    amended_compact_millions: u64,
    temporary_employment_actual: Option<u64>,
    transformers_target: u64,
    transformers_actual: Option<u64>,
    power_training_actual: u64,
    power_training_target: Option<u64>,
    mpp_wards_target: u64,
    mpp_wards_actual: Option<u64>,
    road_training_total: u64,
    road_training_female: u64,
    road_training_male: u64,
    road_design_target_km: u64,
    road_design_actual_km: u64,
    published_road_design_bps: u64,
    computed_road_design_bps: u64,
}

fn optional(value: &str, line: usize) -> Result<Option<u64>, String> {
    if value == "null" {
        Ok(None)
    } else {
        value
            .parse::<u64>()
            .map(Some)
            .map_err(|_| format!("line {line}: invalid optional integer"))
    }
}

fn parse(input: &str) -> Result<Baseline, String> {
    for marker in [
        "# evidence_label=official_program_level2_baseline",
        "# program_status=entered_into_force_active_implementation",
        "# report_date=2026-06-30",
        "# raw_pdf_sha256=4c73817d273739e6cd586e8e7a8a012d6a5be7781e9e8fd033d53b2400f8af47",
    ] {
        if !input.contains(marker) {
            return Err(format!("missing required marker: {marker}"));
        }
    }
    let rows: Vec<_> = input
        .lines()
        .enumerate()
        .filter(|(_, line)| {
            !line.is_empty() && !line.starts_with('#') && !line.starts_with("report_date\t")
        })
        .collect();
    if rows.len() != 1 {
        return Err("level 2 baseline requires exactly one KPI row".into());
    }
    let (index, line) = rows[0];
    let line_number = index + 1;
    let fields: Vec<_> = line.split('\t').collect();
    if fields.len() != 17 || fields[0] != "2026-06-30" {
        return Err(format!(
            "line {line_number}: expected dated 17-field KPI row"
        ));
    }
    let number = |field: usize| {
        fields[field]
            .parse::<u64>()
            .map_err(|_| format!("line {line_number}: invalid integer"))
    };
    let result = Baseline {
        quarter: number(1)?,
        kpi_header_budget_millions: number(2)?,
        amended_compact_millions: number(3)?,
        temporary_employment_actual: optional(fields[4], line_number)?,
        transformers_target: number(5)?,
        transformers_actual: optional(fields[6], line_number)?,
        power_training_actual: number(7)?,
        power_training_target: optional(fields[8], line_number)?,
        mpp_wards_target: number(9)?,
        mpp_wards_actual: optional(fields[10], line_number)?,
        road_training_total: number(11)?,
        road_training_female: number(12)?,
        road_training_male: number(13)?,
        road_design_target_km: number(14)?,
        road_design_actual_km: number(15)?,
        published_road_design_bps: number(16)?,
        computed_road_design_bps: number(15)? * 10_000 / number(14)?,
    };
    if result.road_training_female + result.road_training_male != result.road_training_total {
        return Err("female and male training counts must reconcile to total".into());
    }
    if result.road_design_actual_km > result.road_design_target_km {
        return Err("road design actual cannot exceed target".into());
    }
    Ok(result)
}

fn baseline_json(result: &Baseline) -> String {
    format!(
        "{{\"schema\":\"envoy.nepal-compact-level2-baseline.v1\",\"report_date\":\"2026-06-30\",\"quarter\":{},\"kpi_header_budget_millions\":{},\"amended_compact_millions\":{},\"budget_version_reconciled\":false,\"temporary_employment_actual\":null,\"transformers_target\":{},\"transformers_actual\":null,\"power_regulation_training_actual\":{},\"power_regulation_training_target\":null,\"mpp_wards_target\":{},\"mpp_wards_actual\":null,\"road_training_total\":{},\"road_training_female\":{},\"road_training_male\":{},\"female_training_share_bps\":{},\"road_design_target_km\":{},\"road_design_actual_km\":{},\"published_road_design_target_satisfied_bps\":{},\"computed_road_design_target_satisfied_bps\":{},\"completed_works\":null,\"local_benefit_incidence\":null,\"durable_outcome\":null,\"diversion_result\":null,\"do_no_harm_pass\":null}}",
        result.quarter,
        result.kpi_header_budget_millions,
        result.amended_compact_millions,
        result.transformers_target,
        result.power_training_actual,
        result.mpp_wards_target,
        result.road_training_total,
        result.road_training_female,
        result.road_training_male,
        result.road_training_female * 10_000 / result.road_training_total,
        result.road_design_target_km,
        result.road_design_actual_km,
        result.published_road_design_bps,
        result.computed_road_design_bps
    )
}

fn held_pack_json(result: &Baseline) -> String {
    format!(
        "{{\"schema\":\"taxlane.lane-evidence-pack-candidate.v1\",\"identity\":{{\"pack_id\":\"envoy:mcc-nepal-compact-q11-baseline:v1\",\"track\":\"INT\",\"domain_repository\":\"ENVOY\",\"candidate_id\":\"mcc_nepal_compact_implementation\",\"candidate_name\":\"MCC Nepal Compact implementation\",\"fiscal_owner\":\"TAXLANE\"}},\"scope\":{{\"geography\":\"Nepal\",\"population_or_network\":\"electricity transmission and strategic road maintenance implementation\",\"ownership\":\"MCA-Nepal with Government of Nepal ownership and MCC oversight\",\"time_basis\":\"quarter 11 through 2026-06-30\",\"unit_basis\":\"people transformers wards kilometers dollars and basis points\",\"included\":\"public aggregate key performance indicators\",\"excluded\":\"award operational source-identifying individual and unverified outcome detail\"}},\"source_custody\":{{\"source_id\":\"MCC-NEPAL-KPI-2026-Q11\",\"publisher\":\"Millennium Challenge Corporation\",\"source_path_or_url\":\"https://assets.mcc.gov/content/uploads/nepal-compact-kpi.pdf\",\"vintage\":\"reported 2026-06-30; published 2026-07-07\",\"capture_status\":\"transcribed_official_indicator_table_with_raw_pdf_checksum_and_reconciliation_tests\",\"checksum_or_null\":\"4c73817d273739e6cd586e8e7a8a012d6a5be7781e9e8fd033d53b2400f8af47\"}},\"problem\":{{\"baseline_metric\":\"quarterly implementation outputs against compact targets\",\"baseline_value_or_null\":{},\"affected_population_or_exposure_or_null\":null,\"problem_boundary\":\"training and design are outputs not completed works or durable benefits\",\"road_design_actual_km\":{},\"road_design_target_km\":{},\"computed_target_satisfied_bps\":{}}},\"intervention\":{{\"mechanism\":\"active electricity transmission road maintenance and institutional capacity compact\",\"implementing_owner\":\"MCA-Nepal Government of Nepal and MCC\",\"eligibility_rule\":\"existing compact and approved implementation plan\",\"exclusions\":\"no procurement partner targeting diplomatic instruction or award decision\",\"existing_treatment_or_programmed_work\":\"quarter 11 active implementation\"}},\"outcomes\":{{\"bounded_marginal_effect_or_null\":null,\"effect_population\":\"not yet observed\",\"horizon\":\"compact end and post-compact evaluation\",\"uncertainty\":\"employment transformers municipal activities completed works service changes and durable outcomes remain pending or unavailable\",\"transferability_boundary\":\"Nepal Compact only\"}},\"service_floors\":{{\"access\":\"community benefit incidence unavailable\",\"quality_safety\":\"completed-work quality safety reliability and grievance outcomes unavailable\",\"equity_distribution\":\"training reports 9 women and 43 men; broader subgroup benefit and burden incidence unavailable\",\"adequacy_resilience\":\"durable electricity road and institutional outcomes unavailable\",\"delivery_feasibility\":\"76 design kilometers and 52 trainees reported; construction and benefit-sharing indicators pending\",\"local_partner_bps\":null,\"diversion_risk_bps\":null,\"do_no_harm_pass\":null}},\"costs\":{{\"price_year_or_null\":null,\"gross_cost_or_null\":null,\"implementation_cost_or_null\":null,\"maintenance_cost_or_null\":null,\"offsets_or_null\":null,\"dedicated_receipts_or_null\":null,\"state_local_private_shift_or_null\":null,\"net_cost_or_null\":null,\"public_savings\":null}},\"fiscal_bridge\":{{\"gross_public_funding_need_or_null\":null,\"delivery_efficiency_public_savings_or_null\":null,\"external_economic_benefit_or_null\":null,\"operator_or_private_revenue_or_null\":null,\"legally_dedicated_public_receipts_or_null\":null,\"collection_and_financing_cost_or_null\":null,\"net_public_fiscal_pressure_or_null\":null,\"revenue_authority\":\"none\",\"demand_and_incidence_basis\":\"quarterly output baseline only\",\"netting_rule\":\"do not infer cost savings or economic returns from output progress\"}},\"adaptive_pathways\":{{\"pathway_classes\":\"active time-limited delivery comparison baseline\",\"peer_goal_basis\":null,\"evaluation_horizons\":\"quarterly compact end and post-compact\",\"realization_owner_or_null\":\"MCA-Nepal and MCC\",\"transition_and_implementation_cost_or_null\":null,\"uncertainty_and_downside\":\"budget version completion safeguard maintenance additionality and incidence gaps remain\",\"service_floor_and_distribution_result\":\"held_missing_completion_outcomes_and_safeguards\",\"overlap_and_non_additivity\":\"training design contracts completed works and benefits are sequential not additive outcome quantities\",\"observation_cadence\":\"quarterly KPI table\",\"reopen_triggers\":\"completed works matched service outcomes local incidence grievance safeguards diversion and durable evaluation\",\"current_disposition\":\"held\"}},\"delivery\":{{\"capacity\":\"52 road-technology trainees and 76 road-design kilometers reported\",\"schedule\":\"quarter 11 through 2026-06-30\",\"milestones\":\"road design partial; employment transformers and municipal activities pending\",\"useful_life\":null,\"sunset_or_review\":\"compact end and post-compact evaluation\"}},\"overlap\":{{\"shared_projects\":\"electricity transmission roads regulation and municipal benefit sharing\",\"shared_cost_allocation\":null,\"other_lane_interactions\":\"TRN SEE INT\",\"non_additivity_rule\":\"do not add outputs to financial execution or count them as outcomes\"}},\"readiness\":{{\"domain_evidence_ready\":true,\"candidate_bounded\":true,\"outcome_ready\":false,\"cost_ready\":false,\"floors_ready\":false,\"delivery_ready\":false,\"overlap_ready\":false,\"taxlane_admission_ready\":false}},\"claim_boundaries\":{{\"domain_finding_allowed\":true,\"candidate_recommendation_allowed\":false,\"operational_or_award_decision_allowed\":false,\"savings_allowed\":false,\"allocation_allowed\":false,\"rate_change_allowed\":false,\"public_release_allowed\":false}}}}",
        result.road_design_actual_km,
        result.road_design_actual_km,
        result.road_design_target_km,
        result.computed_road_design_bps
    )
}

pub fn run(command: &str, input: &str) -> Result<String, String> {
    let result = parse(input)?;
    match command {
        "level2-baseline" => Ok(baseline_json(&result)),
        "level2-held-pack" => Ok(held_pack_json(&result)),
        _ => Err(format!("unknown level 2 command: {command}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const FIXTURE: &str = include_str!("../../../fixtures/official/mcc-nepal-kpi-2026-q11.tsv");

    #[test]
    fn reconciles_training_and_road_design_progress() {
        let result = parse(FIXTURE).unwrap();
        assert_eq!(result.road_training_female + result.road_training_male, 52);
        assert_eq!(result.computed_road_design_bps, 5_846);
        assert_eq!(result.published_road_design_bps, 5_800);
    }

    #[test]
    fn preserves_pending_indicators_as_null() {
        let result = parse(FIXTURE).unwrap();
        assert_eq!(result.temporary_employment_actual, None);
        assert_eq!(result.transformers_actual, None);
        assert_eq!(result.mpp_wards_actual, None);
    }

    #[test]
    fn preserves_budget_version_mismatch_and_outcome_nulls() {
        let output = baseline_json(&parse(FIXTURE).unwrap());
        assert!(output.contains("\"kpi_header_budget_millions\":500"));
        assert!(output.contains("\"amended_compact_millions\":550"));
        assert!(output.contains("\"budget_version_reconciled\":false"));
        assert!(output.contains("\"durable_outcome\":null"));
    }

    #[test]
    fn held_pack_does_not_admit_output_as_outcome_or_savings() {
        let output = held_pack_json(&parse(FIXTURE).unwrap());
        assert!(output.contains("\"current_disposition\":\"held\""));
        assert!(output.contains("\"taxlane_admission_ready\":false"));
        assert!(output.contains("\"public_savings\":null"));
    }

    #[test]
    fn rejects_unreconciled_gender_counts() {
        let changed = FIXTURE.replacen("52\t9\t43", "52\t8\t43", 1);
        assert!(parse(&changed).is_err());
    }
}
