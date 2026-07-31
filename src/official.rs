#[derive(Debug, Clone, PartialEq, Eq)]
struct Region {
    name: String,
    obligations_dollars: i64,
    disbursements_dollars: i64,
}

#[derive(Debug, PartialEq, Eq)]
struct Baseline {
    regions: usize,
    obligations_dollars: i64,
    disbursements_dollars: i64,
    same_year_ratio_bps: i64,
    same_year_difference_dollars: i64,
    regions_disbursements_above_obligations: usize,
    largest_positive_difference_region: String,
    largest_positive_difference_dollars: i64,
    largest_ratio_region: String,
    largest_ratio_bps: i64,
}

fn parse(input: &str) -> Result<Vec<Region>, String> {
    let mut rows = Vec::new();
    for (index, line) in input.lines().enumerate() {
        let line_number = index + 1;
        if line.is_empty() || line.starts_with('#') || line.starts_with("region\t") {
            continue;
        }
        let fields: Vec<_> = line.split('\t').collect();
        if fields.len() != 3 {
            return Err(format!("line {line_number}: expected 3 fields"));
        }
        let number = |field: usize| {
            fields[field]
                .parse::<i64>()
                .map_err(|_| format!("line {line_number}: invalid integer"))
        };
        let row = Region {
            name: fields[0].to_owned(),
            obligations_dollars: number(1)?,
            disbursements_dollars: number(2)?,
        };
        if row.obligations_dollars <= 0 || row.disbursements_dollars < 0 {
            return Err(format!("line {line_number}: invalid financial bounds"));
        }
        rows.push(row);
    }
    if rows.is_empty() {
        return Err("at least one official region is required".into());
    }
    Ok(rows)
}

fn ratio_bps(row: &Region) -> i64 {
    row.disbursements_dollars * 10_000 / row.obligations_dollars
}

fn analyze(rows: &[Region]) -> Baseline {
    let obligations: i64 = rows.iter().map(|row| row.obligations_dollars).sum();
    let disbursements: i64 = rows.iter().map(|row| row.disbursements_dollars).sum();
    let largest_difference = rows
        .iter()
        .max_by_key(|row| row.obligations_dollars - row.disbursements_dollars)
        .expect("nonempty regions");
    let largest_ratio = rows
        .iter()
        .max_by_key(|row| ratio_bps(row))
        .expect("nonempty regions");
    Baseline {
        regions: rows.len(),
        obligations_dollars: obligations,
        disbursements_dollars: disbursements,
        same_year_ratio_bps: disbursements * 10_000 / obligations,
        same_year_difference_dollars: obligations - disbursements,
        regions_disbursements_above_obligations: rows
            .iter()
            .filter(|row| row.disbursements_dollars > row.obligations_dollars)
            .count(),
        largest_positive_difference_region: largest_difference.name.clone(),
        largest_positive_difference_dollars: largest_difference.obligations_dollars
            - largest_difference.disbursements_dollars,
        largest_ratio_region: largest_ratio.name.clone(),
        largest_ratio_bps: ratio_bps(largest_ratio),
    }
}

fn baseline_json(result: &Baseline) -> String {
    format!(
        "{{\"schema\":\"envoy.official-assistance-financial-baseline.v1\",\"source_id\":\"FA-GOV-COUNTRY-2026-07-30\",\"evidence_label\":\"official_aggregate\",\"fiscal_year\":2024,\"regions\":{},\"obligations_dollars\":{},\"disbursements_dollars\":{},\"same_year_disbursement_to_obligation_bps\":{},\"same_year_difference_dollars\":{},\"regions_disbursements_above_obligations\":{},\"largest_positive_difference_region\":\"{}\",\"largest_positive_difference_dollars\":{},\"largest_ratio_region\":\"{}\",\"largest_ratio_bps\":{},\"same_year_difference_is_unspent_balance\":false,\"same_year_ratio_is_cohort_realization\":false,\"financial_execution_is_outcome\":false}}",
        result.regions,
        result.obligations_dollars,
        result.disbursements_dollars,
        result.same_year_ratio_bps,
        result.same_year_difference_dollars,
        result.regions_disbursements_above_obligations,
        result.largest_positive_difference_region,
        result.largest_positive_difference_dollars,
        result.largest_ratio_region,
        result.largest_ratio_bps
    )
}

fn held_pack_json(result: &Baseline) -> String {
    format!(
        "{{\"schema\":\"taxlane.lane-evidence-pack-candidate.v1\",\"identity\":{{\"pack_id\":\"envoy:foreignassistance-fy2024-regional-finance:v1\",\"track\":\"INT\",\"domain_repository\":\"ENVOY\",\"candidate_id\":null,\"candidate_name\":null,\"fiscal_owner\":\"TAXLANE\"}},\"scope\":{{\"geography\":\"ForeignAssistance.gov country data grouped to seven source regions including World\",\"population_or_network\":\"reported U.S. foreign assistance\",\"ownership\":\"reporting U.S. government agencies\",\"time_basis\":\"FY2024 current-dollar transactions in 2026-07-30 capture\",\"unit_basis\":\"current U.S. dollars\",\"included\":\"country-file obligations and disbursements grouped by source region\",\"excluded\":\"budget requests activities awards partners outputs outcomes and causal attribution\"}},\"source_custody\":{{\"source_id\":\"FA-GOV-COUNTRY-2026-07-30\",\"publisher\":\"ForeignAssistance.gov\",\"source_path_or_url\":\"https://s3.amazonaws.com/files.explorer.devtechlab.com/us_foreign_aid_country.csv\",\"vintage\":\"captured 2026-07-30\",\"capture_status\":\"derived_with_sha256_in_fixture\",\"checksum_or_null\":\"e3113a2653598acef9313c9cdcf760297b3b98bb91b16a5845b1118560e61346\"}},\"problem\":{{\"baseline_metric\":\"same-year financial execution composition\",\"baseline_value_or_null\":{},\"affected_population_or_exposure_or_null\":null,\"problem_boundary\":\"same-year transactions are not obligation cohorts\",\"same_year_ratio_bps\":{},\"regions_disbursements_above_obligations\":{}}},\"intervention\":{{\"mechanism\":null,\"implementing_owner\":null,\"eligibility_rule\":null,\"exclusions\":\"no award country partner or operational decision\",\"existing_treatment_or_programmed_work\":null}},\"outcomes\":{{\"bounded_marginal_effect_or_null\":null,\"effect_population\":\"aggregate reported portfolio\",\"horizon\":\"one fiscal-year transaction slice\",\"uncertainty\":\"live source is revised and payments may correspond to prior-year obligations\",\"transferability_boundary\":\"financial execution does not establish delivery or outcome\"}},\"service_floors\":{{\"access\":null,\"quality_safety\":null,\"equity_distribution\":null,\"adequacy_resilience\":null,\"delivery_feasibility\":null,\"local_partner_bps\":null,\"diversion_risk_bps\":null,\"do_no_harm_pass\":null}},\"costs\":{{\"price_year_or_null\":\"FY2024 current dollars\",\"gross_cost_or_null\":null,\"implementation_cost_or_null\":null,\"maintenance_cost_or_null\":null,\"offsets_or_null\":null,\"dedicated_receipts_or_null\":null,\"state_local_private_shift_or_null\":null,\"net_cost_or_null\":null,\"public_savings\":null}},\"fiscal_bridge\":{{\"gross_public_funding_need_or_null\":null,\"delivery_efficiency_public_savings_or_null\":null,\"external_economic_benefit_or_null\":null,\"operator_or_private_revenue_or_null\":null,\"legally_dedicated_public_receipts_or_null\":null,\"collection_and_financing_cost_or_null\":null,\"net_public_fiscal_pressure_or_null\":null,\"revenue_authority\":\"none\",\"demand_and_incidence_basis\":\"not established\",\"netting_rule\":\"transactions are context not a candidate score\"}},\"adaptive_pathways\":{{\"pathway_classes\":\"baseline observation only\",\"peer_goal_basis\":null,\"evaluation_horizons\":\"annual\",\"realization_owner_or_null\":null,\"transition_and_implementation_cost_or_null\":null,\"uncertainty_and_downside\":\"cohorts outputs outcomes and local incidence unmeasured\",\"service_floor_and_distribution_result\":\"held\",\"overlap_and_non_additivity\":\"World is a source region not an added global subtotal\",\"observation_cadence\":\"annual capture\",\"reopen_triggers\":\"bounded program with cohort and outcome evidence\",\"current_disposition\":\"held\"}},\"delivery\":{{\"capacity\":null,\"schedule\":null,\"milestones\":null,\"useful_life\":null,\"sunset_or_review\":\"refresh from versioned capture\"}},\"overlap\":{{\"shared_projects\":null,\"shared_cost_allocation\":null,\"other_lane_interactions\":\"DEF DIS HLT\",\"non_additivity_rule\":\"do not add this country-file total to agency or sector views\"}},\"readiness\":{{\"domain_evidence_ready\":true,\"candidate_bounded\":false,\"outcome_ready\":false,\"cost_ready\":false,\"floors_ready\":false,\"delivery_ready\":false,\"overlap_ready\":false,\"taxlane_admission_ready\":false}},\"claim_boundaries\":{{\"domain_finding_allowed\":true,\"candidate_recommendation_allowed\":false,\"operational_or_award_decision_allowed\":false,\"savings_allowed\":false,\"allocation_allowed\":false,\"rate_change_allowed\":false,\"public_release_allowed\":false}}}}",
        result.disbursements_dollars,
        result.same_year_ratio_bps,
        result.regions_disbursements_above_obligations
    )
}

pub fn run(command: &str, input: &str) -> Result<String, String> {
    if !input.contains("# evidence_label=official_aggregate") {
        return Err("official command requires evidence_label=official_aggregate".into());
    }
    let result = analyze(&parse(input)?);
    match command {
        "official-baseline" => Ok(baseline_json(&result)),
        "official-held-pack" => Ok(held_pack_json(&result)),
        _ => Err(format!("unknown official command: {command}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const OFFICIAL: &str =
        include_str!("../fixtures/official/foreignassistance-fy2024-regions.tsv");

    #[test]
    fn exposes_why_same_year_ratio_is_not_cohort_realization() {
        let result = analyze(&parse(OFFICIAL).unwrap());
        assert_eq!(result.regions, 7);
        assert_eq!(result.obligations_dollars, 85_781_331_374);
        assert_eq!(result.disbursements_dollars, 71_576_179_507);
        assert_eq!(result.same_year_ratio_bps, 8_344);
        assert_eq!(result.same_year_difference_dollars, 14_205_151_867);
        assert_eq!(result.regions_disbursements_above_obligations, 2);
        assert_eq!(
            result.largest_positive_difference_region,
            "europe_and_eurasia"
        );
        assert_eq!(result.largest_positive_difference_dollars, 15_045_950_139);
        assert_eq!(result.largest_ratio_region, "world");
        assert_eq!(result.largest_ratio_bps, 12_445);
    }

    #[test]
    fn official_pack_holds_outcome_partner_and_fiscal_claims() {
        let pack = held_pack_json(&analyze(&parse(OFFICIAL).unwrap()));
        assert!(pack.contains("\"local_partner_bps\":null"));
        assert!(pack.contains("\"do_no_harm_pass\":null"));
        assert!(pack.contains("\"public_savings\":null"));
        assert!(pack.contains("\"candidate_id\":null"));
        assert!(pack.contains("\"taxlane_admission_ready\":false"));
    }

    #[test]
    fn rejects_nonpositive_obligations() {
        let changed = OFFICIAL.replacen("5351481041", "0", 1);
        assert!(parse(&changed).is_err());
    }
}
