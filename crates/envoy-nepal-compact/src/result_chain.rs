#[derive(Debug, Clone, PartialEq, Eq)]
struct Audit {
    total_rows: u64,
    outcome_rows: u64,
    output_rows: u64,
    process_rows: u64,
    risk_rows: u64,
    baseline_ready_rows: u64,
    target_ready_rows: u64,
    actual_ready_rows: u64,
    fully_joined_rows: u64,
    outcome_fully_joined_rows: u64,
    compact_specific_rows: u64,
    compact_specific_actual_rows: u64,
    joined_indicator: String,
    baseline: u64,
    open_data_actual: u64,
    end_target: u64,
    current_q11_actual: u64,
}

fn parse(input: &str) -> Result<Audit, String> {
    for marker in [
        "# source_id=MCC-NEPAL-RESULT-CHAIN-AUDIT-2025-2026",
        "# evidence_label=official_program_result_chain_audit",
        "# open_data_source_sha256=ca183c1092fb68765c1e0b7de0f20f050c3205fb8cc8299512a930b2cac3a00b",
        "# q11_source_sha256=4c73817d273739e6cd586e8e7a8a012d6a5be7781e9e8fd033d53b2400f8af47",
    ] {
        if !input.contains(marker) {
            return Err(format!("missing required marker: {marker}"));
        }
    }
    let lines: Vec<_> = input
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .collect();
    if lines.len() != 2 {
        return Err("result-chain audit requires one header and one data row".into());
    }
    let fields: Vec<_> = lines[1].split('\t').collect();
    if fields.len() != 17 {
        return Err("result-chain audit row requires 17 fields".into());
    }
    let number = |index: usize| {
        fields[index]
            .parse::<u64>()
            .map_err(|_| format!("invalid integer at field {}", index + 1))
    };
    let audit = Audit {
        total_rows: number(0)?,
        outcome_rows: number(1)?,
        output_rows: number(2)?,
        process_rows: number(3)?,
        risk_rows: number(4)?,
        baseline_ready_rows: number(5)?,
        target_ready_rows: number(6)?,
        actual_ready_rows: number(7)?,
        fully_joined_rows: number(8)?,
        outcome_fully_joined_rows: number(9)?,
        compact_specific_rows: number(10)?,
        compact_specific_actual_rows: number(11)?,
        joined_indicator: fields[12].to_owned(),
        baseline: number(13)?,
        open_data_actual: number(14)?,
        end_target: number(15)?,
        current_q11_actual: number(16)?,
    };
    if audit.outcome_rows + audit.output_rows + audit.process_rows + audit.risk_rows
        != audit.total_rows
    {
        return Err("level counts do not reconcile to total rows".into());
    }
    if audit.fully_joined_rows != 1
        || audit.outcome_fully_joined_rows != 0
        || audit.compact_specific_actual_rows > audit.compact_specific_rows
        || audit.joined_indicator != "kilometers_of_roads_under_design"
        || audit.baseline != 0
        || audit.open_data_actual > audit.current_q11_actual
        || audit.current_q11_actual > audit.end_target
    {
        return Err("result-chain readiness or trajectory invariant failed".into());
    }
    Ok(audit)
}

fn audit_json(audit: &Audit) -> String {
    let open_progress_bps = audit.open_data_actual * 10_000 / audit.end_target;
    let current_progress_bps = audit.current_q11_actual * 10_000 / audit.end_target;
    format!(
        "{{\"schema\":\"envoy.nepal-result-chain-audit.v1\",\"indicator_rows\":{},\"levels\":{{\"outcome\":{},\"output\":{},\"process\":{},\"risk_assumption\":{}}},\"readiness\":{{\"baseline_ready_rows\":{},\"target_ready_rows\":{},\"actual_ready_rows\":{},\"fully_joined_rows\":{},\"outcome_fully_joined_rows\":{},\"compact_specific_rows\":{},\"compact_specific_actual_rows\":{}}},\"only_joined_indicator\":{{\"name\":\"{}\",\"level\":\"process\",\"baseline_km\":{},\"open_data_actual_km\":{},\"current_q11_actual_km\":{},\"end_target_km\":{},\"open_data_progress_bps\":{},\"current_progress_bps\":{},\"additional_design_km\":{}}},\"process_progress_is_outcome\":false,\"candidate_effect_observable\":false}}",
        audit.total_rows,
        audit.outcome_rows,
        audit.output_rows,
        audit.process_rows,
        audit.risk_rows,
        audit.baseline_ready_rows,
        audit.target_ready_rows,
        audit.actual_ready_rows,
        audit.fully_joined_rows,
        audit.outcome_fully_joined_rows,
        audit.compact_specific_rows,
        audit.compact_specific_actual_rows,
        audit.joined_indicator,
        audit.baseline,
        audit.open_data_actual,
        audit.current_q11_actual,
        audit.end_target,
        open_progress_bps,
        current_progress_bps,
        audit.current_q11_actual - audit.open_data_actual
    )
}

fn held_pack_json(audit: &Audit) -> String {
    let audit_json = audit_json(audit);
    format!(
        "{{\"schema\":\"taxlane.lane-evidence-pack-candidate.v1\",\"identity\":{{\"pack_id\":\"envoy:mcc-nepal-result-chain-audit:v1\",\"track\":\"INT\",\"domain_repository\":\"ENVOY\",\"candidate_id\":\"mcc_nepal_compact_implementation\",\"fiscal_owner\":\"TAXLANE\"}},\"scope\":{{\"geography\":\"Nepal\",\"included\":\"complete public ITT readiness counts and cross-vintage road-design trajectory\",\"excluded\":\"operational data and causal claims\"}},\"source_custody\":{{\"source_id\":\"MCC-NEPAL-RESULT-CHAIN-AUDIT-2025-2026\",\"publisher\":\"Millennium Challenge Corporation\",\"open_data_checksum\":\"ca183c1092fb68765c1e0b7de0f20f050c3205fb8cc8299512a930b2cac3a00b\",\"q11_checksum\":\"4c73817d273739e6cd586e8e7a8a012d6a5be7781e9e8fd033d53b2400f8af47\"}},\"problem\":{{\"baseline_metric\":\"result-chain join readiness\",\"baseline_value_or_null\":{audit_json},\"problem_boundary\":\"the only complete join is process-level\"}},\"intervention\":{{\"mechanism\":\"active Nepal Compact\",\"implementing_owner\":\"MCA-Nepal with MCC oversight\"}},\"outcomes\":{{\"bounded_marginal_effect_or_null\":null,\"candidate_effect_observed\":false,\"uncertainty\":\"zero outcome rows join baseline target and observation\"}},\"service_floors\":{{\"access\":null,\"quality_safety\":null,\"equity_distribution\":null,\"adequacy_resilience\":null,\"do_no_harm_pass\":null}},\"costs\":{{\"gross_cost_or_null\":null,\"net_cost_or_null\":null,\"public_savings\":null}},\"fiscal_bridge\":{{\"delivery_efficiency_public_savings_or_null\":null,\"net_public_fiscal_pressure_or_null\":null,\"netting_rule\":\"design progress cannot be monetized as effect or savings\"}},\"adaptive_pathways\":{{\"observation_cadence\":\"quarterly ITT and independent evaluation\",\"reopen_triggers\":\"compact-specific outcome baseline target and observation plus counterfactual incidence safeguards matched costs and independent result\",\"current_disposition\":\"held_process_trajectory_observed_outcome_chain_open\"}},\"delivery\":{{\"capacity\":\"road design advanced from 40 to 76 kilometers\",\"milestones\":\"58.46 percent of design target; completed works and service result absent\"}},\"overlap\":{{\"other_lane_interactions\":\"TRN SEE\",\"non_additivity_rule\":\"design kilometers are not added to construction service or outcome quantities\"}},\"readiness\":{{\"domain_evidence_ready\":true,\"candidate_bounded\":true,\"process_trajectory_ready\":true,\"outcome_chain_ready\":false,\"outcome_ready\":false,\"floors_ready\":false,\"delivery_ready\":false,\"taxlane_admission_ready\":false}},\"claim_boundaries\":{{\"operational_partner_or_award_decision_allowed\":false,\"candidate_recommendation_allowed\":false,\"savings_allowed\":false,\"allocation_allowed\":false,\"rate_change_allowed\":false,\"public_release_allowed\":false}}}}"
    )
}

pub(crate) fn run(command: &str, input: &str) -> Result<String, String> {
    let audit = parse(input)?;
    match command {
        "result-chain-audit" => Ok(audit_json(&audit)),
        "result-chain-held-pack" => Ok(held_pack_json(&audit)),
        _ => Err(format!("unknown result-chain command: {command}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const FIXTURE: &str =
        include_str!("../../../fixtures/official/mcc-nepal-result-chain-audit-2025-2026.tsv");

    #[test]
    fn audits_full_indicator_universe_and_only_complete_join() {
        let output = run("result-chain-audit", FIXTURE).unwrap();
        assert!(output.contains("\"indicator_rows\":186"));
        assert!(output.contains("\"outcome\":73"));
        assert!(output.contains("\"fully_joined_rows\":1"));
        assert!(output.contains("\"outcome_fully_joined_rows\":0"));
    }

    #[test]
    fn reconciles_cross_vintage_design_progress_without_outcome_claim() {
        let output = run("result-chain-audit", FIXTURE).unwrap();
        assert!(output.contains("\"additional_design_km\":36"));
        assert!(output.contains("\"current_progress_bps\":5846"));
        assert!(output.contains("\"process_progress_is_outcome\":false"));
    }

    #[test]
    fn held_pack_keeps_effect_and_fiscal_authority_closed() {
        let output = run("result-chain-held-pack", FIXTURE).unwrap();
        assert!(output.contains("\"process_trajectory_ready\":true"));
        assert!(output.contains("\"outcome_chain_ready\":false"));
        assert!(output.contains("\"public_savings\":null"));
        assert!(output.contains("\"taxlane_admission_ready\":false"));
    }

    #[test]
    fn rejects_level_count_or_join_drift() {
        assert!(run("result-chain-audit", &FIXTURE.replace("186\t73", "185\t73")).is_err());
        assert!(run(
            "result-chain-audit",
            &FIXTURE.replace("\t1\t0\t3", "\t2\t0\t3")
        )
        .is_err());
    }
}
