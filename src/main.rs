use std::env;
use std::fs;
use std::process::ExitCode;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Portfolio {
    name: String,
    obligated: u64,
    disbursed: u64,
    verified_output_bps: u64,
    durable_outcome_bps: u64,
    local_partner_bps: u64,
    diversion_risk_bps: u64,
    do_no_harm: bool,
}

#[cfg(test)]
mod lane_pack_contract_tests {
    use super::*;

    #[test]
    fn held_pack_exposes_every_taxlane_contract_section() {
        let fixture = include_str!("../fixtures/cedar-assistance-realization.tsv");
        let pack = held_pack_json(&analyze(&parse(fixture).unwrap()));
        for section in [
            "identity",
            "scope",
            "source_custody",
            "problem",
            "intervention",
            "outcomes",
            "service_floors",
            "costs",
            "fiscal_bridge",
            "adaptive_pathways",
            "delivery",
            "overlap",
            "readiness",
            "claim_boundaries",
        ] {
            assert!(
                pack.contains(&format!("\"{section}\":")),
                "missing {section}"
            );
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Analysis {
    obligated_millions: u64,
    disbursed_millions: u64,
    disbursement_bps: u64,
    verified_output_milli: u64,
    durable_outcome_milli: u64,
    largest_delivery_gap_portfolio: String,
    largest_delivery_gap_millions: u64,
    local_partner_floor_bps: u64,
    diversion_risk_ceiling_bps: u64,
    do_no_harm_pass: bool,
    promotion_floor_pass: bool,
}

fn parse_bool(value: &str, line: usize) -> Result<bool, String> {
    match value {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(format!("line {line}: expected true or false")),
    }
}

fn parse(input: &str) -> Result<Vec<Portfolio>, String> {
    let mut rows = Vec::new();
    for (index, line) in input.lines().enumerate() {
        let line_number = index + 1;
        if line.is_empty() || line.starts_with('#') || line.starts_with("portfolio\t") {
            continue;
        }
        let fields: Vec<_> = line.split('\t').collect();
        if fields.len() != 8 {
            return Err(format!("line {line_number}: expected 8 fields"));
        }
        let number = |field: usize| {
            fields[field]
                .parse::<u64>()
                .map_err(|_| format!("line {line_number}: invalid integer"))
        };
        let row = Portfolio {
            name: fields[0].to_owned(),
            obligated: number(1)?,
            disbursed: number(2)?,
            verified_output_bps: number(3)?,
            durable_outcome_bps: number(4)?,
            local_partner_bps: number(5)?,
            diversion_risk_bps: number(6)?,
            do_no_harm: parse_bool(fields[7], line_number)?,
        };
        if row.disbursed > row.obligated
            || row.verified_output_bps > 10_000
            || row.durable_outcome_bps > 10_000
            || row.local_partner_bps > 10_000
            || row.diversion_risk_bps > 10_000
        {
            return Err(format!("line {line_number}: invalid portfolio bounds"));
        }
        rows.push(row);
    }
    if rows.is_empty() {
        return Err("at least one portfolio is required".into());
    }
    Ok(rows)
}

fn analyze(rows: &[Portfolio]) -> Analysis {
    let obligated: u64 = rows.iter().map(|row| row.obligated).sum();
    let disbursed: u64 = rows.iter().map(|row| row.disbursed).sum();
    let weighted_output: u64 = rows
        .iter()
        .map(|row| row.disbursed * row.verified_output_bps)
        .sum();
    let weighted_outcome: u64 = rows
        .iter()
        .map(|row| row.disbursed * row.durable_outcome_bps)
        .sum();
    let largest = rows
        .iter()
        .max_by_key(|row| row.obligated - row.disbursed)
        .expect("nonempty portfolios");
    let local_floor = rows
        .iter()
        .map(|row| row.local_partner_bps)
        .min()
        .unwrap_or(0);
    let diversion = rows
        .iter()
        .map(|row| row.diversion_risk_bps)
        .max()
        .unwrap_or(0);
    let do_no_harm = rows.iter().all(|row| row.do_no_harm);
    Analysis {
        obligated_millions: obligated,
        disbursed_millions: disbursed,
        disbursement_bps: disbursed * 10_000 / obligated,
        verified_output_milli: weighted_output / disbursed,
        durable_outcome_milli: weighted_outcome / disbursed,
        largest_delivery_gap_portfolio: largest.name.clone(),
        largest_delivery_gap_millions: largest.obligated - largest.disbursed,
        local_partner_floor_bps: local_floor,
        diversion_risk_ceiling_bps: diversion,
        do_no_harm_pass: do_no_harm,
        promotion_floor_pass: local_floor >= 5_000 && diversion <= 1_000 && do_no_harm,
    }
}

fn analysis_json(result: &Analysis) -> String {
    format!(
        "{{\"schema\":\"envoy.assistance-realization.v1\",\"obligated_millions\":{},\"disbursed_millions\":{},\"disbursement_bps\":{},\"verified_output_bps\":{},\"durable_outcome_bps\":{},\"largest_delivery_gap_portfolio\":\"{}\",\"largest_delivery_gap_millions\":{},\"local_partner_floor_bps\":{},\"diversion_risk_ceiling_bps\":{},\"do_no_harm_pass\":{},\"promotion_floor_pass\":{},\"obligation_or_payment_is_outcome\":false}}",
        result.obligated_millions, result.disbursed_millions, result.disbursement_bps,
        result.verified_output_milli, result.durable_outcome_milli,
        result.largest_delivery_gap_portfolio, result.largest_delivery_gap_millions,
        result.local_partner_floor_bps, result.diversion_risk_ceiling_bps,
        result.do_no_harm_pass, result.promotion_floor_pass
    )
}

fn held_pack_json(result: &Analysis) -> String {
    format!(
        "{{\"schema\":\"taxlane.lane-evidence-pack-candidate.v1\",\"identity\":{{\"pack_id\":\"envoy:cedar-assistance-realization:v1\",\"track\":\"INT\",\"domain_repository\":\"ENVOY\",\"candidate_id\":\"cedar-assistance-realization\",\"candidate_name\":\"Cedar assistance realization screen\",\"fiscal_owner\":\"TAXLANE\"}},\"scope\":{{\"geography\":\"synthetic Cedar portfolios\",\"population_or_network\":\"aggregate assistance portfolios\",\"ownership\":\"illustrative foreign assistance\",\"time_basis\":\"annual illustration\",\"unit_basis\":\"millions and basis points\",\"included\":\"obligation disbursement output outcome local share diversion harm\",\"excluded\":\"countries partners awards and operations\"}},\"source_custody\":{{\"source_id\":\"STATE-4FAM-080,STATE-18FAM-301-4\",\"publisher\":\"U.S. Department of State\",\"source_path_or_url\":\"https://fam.state.gov/\",\"vintage\":\"2024\",\"capture_status\":\"registry_linked\",\"checksum_or_null\":null}},\"problem\":{{\"baseline_metric\":\"obligation to durable outcome realization\",\"baseline_value_or_null\":null,\"affected_population_or_exposure_or_null\":null,\"problem_boundary\":\"synthetic aggregate portfolios\",\"disbursement_bps\":{},\"verified_output_bps\":{},\"durable_outcome_bps\":{}}},\"intervention\":{{\"mechanism\":null,\"implementing_owner\":null,\"eligibility_rule\":null,\"exclusions\":\"no award or operational decision\",\"existing_treatment_or_programmed_work\":null}},\"outcomes\":{{\"bounded_marginal_effect_or_null\":null,\"effect_population\":\"aggregate portfolios\",\"horizon\":\"annual illustration\",\"uncertainty\":\"not estimated\",\"transferability_boundary\":\"synthetic only\"}},\"service_floors\":{{\"access\":\"delivery reported\",\"quality_safety\":\"do-no-harm explicit\",\"equity_distribution\":\"local partnership reported\",\"adequacy_resilience\":\"durability reported\",\"delivery_feasibility\":\"diversion reported\",\"local_partner_bps\":{},\"diversion_risk_bps\":{},\"do_no_harm_pass\":{},\"promotion_floor_pass\":{}}},\"costs\":{{\"price_year_or_null\":null,\"gross_cost_or_null\":null,\"implementation_cost_or_null\":null,\"maintenance_cost_or_null\":null,\"offsets_or_null\":null,\"dedicated_receipts_or_null\":null,\"state_local_private_shift_or_null\":null,\"net_cost_or_null\":null,\"public_savings\":null}},\"fiscal_bridge\":{{\"gross_public_funding_need_or_null\":null,\"delivery_efficiency_public_savings_or_null\":null,\"external_economic_benefit_or_null\":null,\"operator_or_private_revenue_or_null\":null,\"legally_dedicated_public_receipts_or_null\":null,\"collection_and_financing_cost_or_null\":null,\"net_public_fiscal_pressure_or_null\":null,\"revenue_authority\":\"none\",\"demand_and_incidence_basis\":\"not established\",\"netting_rule\":\"no values admitted\"}},\"adaptive_pathways\":{{\"pathway_classes\":\"delivery and durable outcome only\",\"peer_goal_basis\":null,\"evaluation_horizons\":\"annual\",\"realization_owner_or_null\":null,\"transition_and_implementation_cost_or_null\":null,\"uncertainty_and_downside\":\"unbounded\",\"service_floor_and_distribution_result\":\"held\",\"overlap_and_non_additivity\":\"not reconciled\",\"observation_cadence\":null,\"reopen_triggers\":\"official bounded candidate\",\"current_disposition\":\"held\"}},\"delivery\":{{\"capacity\":null,\"schedule\":null,\"milestones\":null,\"useful_life\":null,\"sunset_or_review\":\"review before use\"}},\"overlap\":{{\"shared_projects\":null,\"shared_cost_allocation\":null,\"other_lane_interactions\":\"DEF DIS\",\"non_additivity_rule\":\"no automatic addition\"}},\"readiness\":{{\"domain_evidence_ready\":true,\"candidate_bounded\":false,\"outcome_ready\":false,\"cost_ready\":false,\"floors_ready\":false,\"delivery_ready\":false,\"overlap_ready\":false,\"taxlane_admission_ready\":false}},\"claim_boundaries\":{{\"domain_finding_allowed\":true,\"candidate_recommendation_allowed\":false,\"operational_or_award_decision_allowed\":false,\"savings_allowed\":false,\"allocation_allowed\":false,\"rate_change_allowed\":false,\"public_release_allowed\":false}}}}",
        result.disbursement_bps, result.verified_output_milli,
        result.durable_outcome_milli, result.local_partner_floor_bps,
        result.diversion_risk_ceiling_bps, result.do_no_harm_pass,
        result.promotion_floor_pass
    )
}

fn run(args: &[String]) -> Result<String, String> {
    let [command, path] = args else {
        return Err("usage: envoy <analyze|held-pack> <fixture.tsv>".into());
    };
    let input = fs::read_to_string(path).map_err(|error| format!("{path}: {error}"))?;
    if !input.contains("# source_id=") || !input.contains("# evidence_label=") {
        return Err("fixture must declare source_id and evidence_label".into());
    }
    let result = analyze(&parse(&input)?);
    match command.as_str() {
        "analyze" => Ok(analysis_json(&result)),
        "held-pack" => Ok(held_pack_json(&result)),
        _ => Err(format!("unknown command: {command}")),
    }
}

fn main() -> ExitCode {
    match run(&env::args().skip(1).collect::<Vec<_>>()) {
        Ok(output) => {
            println!("{output}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const FIXTURE: &str = include_str!("../fixtures/cedar-assistance-realization.tsv");

    #[test]
    fn distinguishes_money_output_and_outcome() {
        let result = analyze(&parse(FIXTURE).unwrap());
        assert_eq!(result.obligated_millions, 150);
        assert_eq!(result.disbursed_millions, 120);
        assert_eq!(result.disbursement_bps, 8_000);
        assert_eq!(result.verified_output_milli, 7_820);
        assert_eq!(result.durable_outcome_milli, 5_635);
    }

    #[test]
    fn identifies_delivery_gap_and_floor_failure() {
        let result = analyze(&parse(FIXTURE).unwrap());
        assert_eq!(result.largest_delivery_gap_portfolio, "governance");
        assert_eq!(result.largest_delivery_gap_millions, 16);
        assert_eq!(result.local_partner_floor_bps, 4_800);
        assert_eq!(result.diversion_risk_ceiling_bps, 1_400);
        assert!(!result.promotion_floor_pass);
    }

    #[test]
    fn do_no_harm_is_independent() {
        let changed = FIXTURE.replacen("600\ttrue", "600\tfalse", 1);
        assert!(!analyze(&parse(&changed).unwrap()).do_no_harm_pass);
    }

    #[test]
    fn rejects_disbursement_above_obligation() {
        let changed = FIXTURE.replacen("60\t54", "60\t61", 1);
        assert!(parse(&changed).is_err());
    }

    #[test]
    fn held_pack_preserves_authority() {
        let pack = held_pack_json(&analyze(&parse(FIXTURE).unwrap()));
        assert!(pack.contains("\"track\":\"INT\""));
        assert!(pack.contains("\"public_savings\":null"));
        assert!(pack.contains("\"operational_or_award_decision_allowed\":false"));
        assert!(pack.contains("\"taxlane_admission_ready\":false"));
    }
}
