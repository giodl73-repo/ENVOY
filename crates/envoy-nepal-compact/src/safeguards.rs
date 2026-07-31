use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq)]
struct SafeguardReadiness {
    registered: u64,
    closed: u64,
    awaiting_signoff: u64,
    tier1_not_accepted: u64,
    new_under_discussion: u64,
    classified: u64,
    unclassified: u64,
    closure_share_bps: u64,
    appeal_tiers: u64,
    legal_recourse_preserved: bool,
    remote_submission_available: bool,
    support_contracts_visible: u64,
}

fn parse(input: &str) -> Result<BTreeMap<String, (u64, String)>, String> {
    let mut metrics = BTreeMap::new();
    for (index, line) in input.lines().enumerate() {
        let line_number = index + 1;
        if line.is_empty() || line.starts_with('#') || line.starts_with("metric\t") {
            continue;
        }
        let fields: Vec<_> = line.split('\t').collect();
        if fields.len() != 3 {
            return Err(format!("line {line_number}: expected 3 fields"));
        }
        let value = fields[1]
            .parse::<u64>()
            .map_err(|_| format!("line {line_number}: invalid nonnegative integer"))?;
        if metrics
            .insert(fields[0].to_owned(), (value, fields[2].to_owned()))
            .is_some()
        {
            return Err(format!("line {line_number}: duplicate metric"));
        }
    }
    Ok(metrics)
}

fn value(metrics: &BTreeMap<String, (u64, String)>, key: &str, unit: &str) -> Result<u64, String> {
    let (value, actual_unit) = metrics
        .get(key)
        .ok_or_else(|| format!("missing metric {key}"))?;
    if actual_unit != unit {
        return Err(format!("metric {key} must use {unit}"));
    }
    Ok(*value)
}

fn boolean(metrics: &BTreeMap<String, (u64, String)>, key: &str) -> Result<bool, String> {
    match value(metrics, key, "boolean")? {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(format!("metric {key} must be 0 or 1")),
    }
}

fn analyze(metrics: &BTreeMap<String, (u64, String)>) -> Result<SafeguardReadiness, String> {
    let registered = value(metrics, "grievances_registered", "grievances")?;
    let closed = value(metrics, "grievances_closed", "grievances")?;
    let awaiting_signoff = value(metrics, "responses_awaiting_signoff", "grievances")?;
    let tier1_not_accepted = value(metrics, "tier1_not_accepted", "grievances")?;
    let new_under_discussion = value(metrics, "new_under_discussion", "grievances")?;
    let classified = closed + awaiting_signoff + tier1_not_accepted + new_under_discussion;
    if classified > registered || registered == 0 {
        return Err("published grievance status categories exceed registered grievances".into());
    }
    let support_contracts_visible = [
        "mcc_nepal_esp_support_awarded",
        "mcc_nepal_transport_support_in_progress",
        "mcc_nepal_technical_support_in_progress",
    ]
    .iter()
    .try_fold(0_u64, |sum, key| {
        Ok::<u64, String>(sum + value(metrics, key, "contracts")?)
    })?;
    Ok(SafeguardReadiness {
        registered,
        closed,
        awaiting_signoff,
        tier1_not_accepted,
        new_under_discussion,
        classified,
        unclassified: registered - classified,
        closure_share_bps: closed * 10_000 / registered,
        appeal_tiers: value(metrics, "appeal_tiers", "tiers")?,
        legal_recourse_preserved: boolean(metrics, "legal_recourse_preserved")?,
        remote_submission_available: boolean(metrics, "remote_submission_available")?,
        support_contracts_visible,
    })
}

fn baseline_json(result: &SafeguardReadiness) -> String {
    format!(
        "{{\"schema\":\"envoy.nepal-safeguard-grievance-readiness.v1\",\"candidate_id\":\"mcc_nepal_compact_implementation\",\"as_of_date\":\"2026-07-31\",\"grievance_snapshot_vintage\":\"2024-07\",\"grievance_mechanism\":{{\"registered\":{},\"closed\":{},\"responses_awaiting_signoff\":{},\"tier1_not_accepted\":{},\"new_under_discussion\":{},\"classified_status_total\":{},\"unclassified_status_count\":{},\"published_status_categories_reconcile\":{},\"closure_share_bps\":{},\"appeal_tiers\":{},\"legal_recourse_preserved\":{},\"remote_submission_available\":{}}},\"mcc_support_contracts_visible\":{},\"grievance_mechanism_observed\":true,\"current_resolution_performance_ready\":false,\"safeguard_compliance_result_ready\":false,\"local_incidence_ready\":false,\"candidate_effect_observable\":false,\"candidate_admitted\":false}}",
        result.registered,
        result.closed,
        result.awaiting_signoff,
        result.tier1_not_accepted,
        result.new_under_discussion,
        result.classified,
        result.unclassified,
        result.unclassified == 0,
        result.closure_share_bps,
        result.appeal_tiers,
        result.legal_recourse_preserved,
        result.remote_submission_available,
        result.support_contracts_visible
    )
}

fn held_pack_json(result: &SafeguardReadiness) -> String {
    format!(
        "{{\"schema\":\"taxlane.lane-evidence-pack-candidate.v1\",\"identity\":{{\"pack_id\":\"envoy:mcc-nepal-safeguard-grievance-readiness:v1\",\"track\":\"INT\",\"domain_repository\":\"ENVOY\",\"candidate_id\":\"mcc_nepal_compact_implementation\",\"fiscal_owner\":\"TAXLANE\"}},\"scope\":{{\"geography\":\"Nepal\",\"included\":\"aggregate grievance mechanism status and public support-procurement readiness\",\"excluded\":\"complainant identities operational targeting current compliance and inferred effects\"}},\"source_custody\":{{\"source_id\":\"MCA-NEPAL-SAFEGUARD-GRIEVANCE-READINESS-2026\",\"publisher\":\"Millennium Challenge Account Nepal with Millennium Challenge Corporation companion forecast\",\"source_path_or_url\":\"https://nepalindata.com/media/resources/items/0/bAnnual_Report_English.pdf\",\"vintage\":\"grievances through 2024-07; MCC forecast 2026-05-12\",\"capture_status\":\"publisher report preserved by public archive plus current official MCC forecast\",\"checksum_or_null\":null}},\"problem\":{{\"baseline_metric\":\"accessible grievance handling and safeguard-readiness visibility\",\"baseline_value_or_null\":{{\"registered\":{},\"closed\":{},\"closure_share_bps\":{},\"unclassified_status_count\":{}}},\"problem_boundary\":\"mechanism and support capacity are not current resolution quality or safeguard compliance\"}},\"intervention\":{{\"mechanism\":\"three-tier grievance process with remote intake and legal recourse\",\"implementing_owner\":\"MCA-Nepal with MCC oversight\"}},\"outcomes\":{{\"bounded_marginal_effect_or_null\":null,\"uncertainty\":\"status snapshot is stale, five grievances are not reconciled to the published categories, and no timeliness satisfaction subgroup or effect result is available\"}},\"service_floors\":{{\"access\":\"remote and in-person grievance intake described\",\"quality_safety\":null,\"equity_distribution\":null,\"adequacy_resilience\":null,\"delivery_feasibility\":\"{} support contracts visible; performance unverified\",\"do_no_harm_pass\":null}},\"costs\":{{\"gross_cost_or_null\":null,\"net_cost_or_null\":null,\"public_savings\":null}},\"fiscal_bridge\":{{\"delivery_efficiency_public_savings_or_null\":null,\"net_public_fiscal_pressure_or_null\":null,\"netting_rule\":\"grievance closure and support procurement are not fiscal savings or durable outcomes\"}},\"adaptive_pathways\":{{\"observation_cadence\":\"current grievance status plus construction-phase safeguard reporting\",\"reopen_triggers\":\"reconciled current status timeliness resolution quality subgroup incidence safeguard compliance and linkage to completed works\",\"current_disposition\":\"held_mechanism_observed_performance_unverified\"}},\"delivery\":{{\"capacity\":\"grievance intake and public support contracts observed\",\"milestones\":\"current reconciled grievance and safeguard-compliance results\"}},\"overlap\":{{\"other_lane_interactions\":\"TRN SEE\",\"non_additivity_rule\":\"registered grievances support contracts and completed works are distinct quantities\"}},\"readiness\":{{\"domain_evidence_ready\":true,\"candidate_bounded\":true,\"outcome_ready\":false,\"floors_ready\":false,\"delivery_ready\":false,\"taxlane_admission_ready\":false}},\"claim_boundaries\":{{\"operational_partner_or_award_decision_allowed\":false,\"candidate_recommendation_allowed\":false,\"savings_allowed\":false,\"allocation_allowed\":false,\"rate_change_allowed\":false,\"public_release_allowed\":false}}}}",
        result.registered,
        result.closed,
        result.closure_share_bps,
        result.unclassified,
        result.support_contracts_visible
    )
}

pub(super) fn run(command: &str, input: &str) -> Result<String, String> {
    if !input.contains("# evidence_label=official_safeguard_grievance_readiness") {
        return Err("safeguard command requires official aggregate readiness evidence".into());
    }
    let result = analyze(&parse(input)?)?;
    match command {
        "safeguard-baseline" => Ok(baseline_json(&result)),
        "safeguard-held-pack" => Ok(held_pack_json(&result)),
        _ => Err(format!("unknown safeguard command: {command}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str =
        include_str!("../../../fixtures/official/mca-nepal-safeguard-grievance-readiness-2026.tsv");

    #[test]
    fn exposes_mechanism_and_unreconciled_status_without_claiming_performance() {
        let output = run("safeguard-baseline", FIXTURE).unwrap();
        assert!(output.contains("\"registered\":123"));
        assert!(output.contains("\"unclassified_status_count\":5"));
        assert!(output.contains("\"closure_share_bps\":3577"));
        assert!(output.contains("\"current_resolution_performance_ready\":false"));
    }

    #[test]
    fn support_procurement_does_not_become_safeguard_or_fiscal_result() {
        let pack = run("safeguard-held-pack", FIXTURE).unwrap();
        assert!(pack.contains("3 support contracts visible; performance unverified"));
        assert!(pack.contains("\"do_no_harm_pass\":null"));
        assert!(pack.contains("\"public_savings\":null"));
        assert!(pack.contains("\"taxlane_admission_ready\":false"));
    }

    #[test]
    fn rejects_status_categories_above_registered_grievances() {
        let changed = FIXTURE.replace("grievances_closed\t44", "grievances_closed\t144");
        assert!(run("safeguard-baseline", &changed).is_err());
    }
}
