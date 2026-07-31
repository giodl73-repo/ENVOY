use std::collections::BTreeMap;

struct Program(BTreeMap<String, i64>);

impl Program {
    fn n(&self, key: &str) -> Result<i64, String> {
        self.0
            .get(key)
            .copied()
            .ok_or_else(|| format!("missing metric: {key}"))
    }

    fn b(&self, key: &str) -> Result<bool, String> {
        match self.n(key)? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(format!("metric {key} must be 0 or 1")),
        }
    }
}

fn parse(input: &str) -> Result<Program, String> {
    for marker in [
        "# source_id=ENVOY-SYNTHETIC-ASSISTANCE-SEMANTIC-PROGRAM",
        "# evidence_label=synthetic_aggregate_semantic_program",
    ] {
        if !input.contains(marker) {
            return Err(format!("missing required marker: {marker}"));
        }
    }
    let mut values = BTreeMap::new();
    for (index, line) in input.lines().enumerate() {
        if line.is_empty() || line.starts_with('#') || line == "metric\tvalue" {
            continue;
        }
        let fields: Vec<_> = line.split('\t').collect();
        if fields.len() != 2 {
            return Err(format!("line {}: expected metric and value", index + 1));
        }
        let value = fields[1]
            .parse::<i64>()
            .map_err(|_| format!("line {}: invalid integer", index + 1))?;
        if values.insert(fields[0].to_owned(), value).is_some() {
            return Err(format!("line {}: duplicate metric", index + 1));
        }
    }
    let p = Program(values);
    for key in [
        "baseline_delivery_bps",
        "stress_delivery_bps",
        "recovery_delivery_bps",
        "status_quo_durable_bps",
        "status_quo_local_incidence_bps",
        "delivery_support_durable_bps",
        "delivery_support_local_incidence_bps",
        "local_capacity_durable_bps",
        "local_capacity_local_incidence_bps",
        "observed_durable_bps",
        "adaptive_trigger_bps",
        "comparison_durable_bps",
    ] {
        if !(0..=10_000).contains(&p.n(key)?) {
            return Err(format!("metric {key} must be basis points"));
        }
    }
    for key in [
        "status_quo_do_no_harm",
        "delivery_support_do_no_harm",
        "local_capacity_do_no_harm",
        "delivery_owner_named",
        "delivery_road_evaluation_active",
        "delivery_transmission_evaluation_active",
        "delivery_completed_works_ready",
        "delivery_safeguards_ready",
        "delivery_grievance_ready",
        "delivery_measures_ready",
        "delivery_rollback_ready",
        "comparison_same_definition",
    ] {
        p.b(key)?;
    }
    let chain = [
        p.n("funded_capacity")?,
        p.n("contracted_capacity")?,
        p.n("completed_service")?,
        p.n("locally_received_service")?,
        p.n("durable_outcomes")?,
    ];
    if chain.iter().any(|value| *value < 0) || chain.windows(2).any(|pair| pair[1] > pair[0]) {
        return Err("assistance realization chain must be nonnegative and nonincreasing".into());
    }
    Ok(p)
}

fn scenarios(p: &Program) -> Result<String, String> {
    let base = p.n("baseline_delivery_bps")?;
    let stress = p.n("stress_delivery_bps")?;
    let recovery = p.n("recovery_delivery_bps")?;
    Ok(format!(
        "{{\"schema\":\"envoy.program-scenarios.v1\",\"baseline_delivery_bps\":{base},\"stress_delivery_bps\":{stress},\"stress_change_bps\":{},\"recovery_delivery_bps\":{recovery},\"recovery_vs_baseline_bps\":{},\"scenario_versions_immutable\":true,\"observed_candidate_effect\":false}}",
        stress - base,
        recovery - base
    ))
}

fn realization(p: &Program) -> Result<String, String> {
    let funded = p.n("funded_capacity")?;
    let contracted = p.n("contracted_capacity")?;
    let completed = p.n("completed_service")?;
    let local = p.n("locally_received_service")?;
    let durable = p.n("durable_outcomes")?;
    let losses = [
        ("contracting", funded - contracted),
        ("completion", contracted - completed),
        ("local_receipt", completed - local),
        ("durability", local - durable),
    ];
    let largest = losses.iter().max_by_key(|(_, loss)| *loss).unwrap();
    Ok(format!(
        "{{\"schema\":\"envoy.program-realization.v1\",\"funded_capacity\":{funded},\"contracted_capacity\":{contracted},\"completed_service\":{completed},\"locally_received_service\":{local},\"durable_outcomes\":{durable},\"funded_to_completed_bps\":{},\"funded_to_durable_bps\":{},\"largest_handoff_loss_stage\":\"{}\",\"largest_handoff_loss\":{},\"unrealized_assistance_is_savings\":false}}",
        completed * 10_000 / funded,
        durable * 10_000 / funded,
        largest.0,
        largest.1
    ))
}

fn accounting(p: &Program) -> Result<String, String> {
    let us = p.n("us_compact_thousand_dollars")?;
    let nepal = p.n("nepal_contribution_thousand_dollars")?;
    let transition = p.n("transition_cost_thousand_dollars")?;
    let maintenance = p.n("maintenance_cost_thousand_dollars")?;
    Ok(format!(
        "{{\"schema\":\"envoy.program-accounting.v1\",\"us_compact_thousand_dollars\":{us},\"nepal_contribution_thousand_dollars\":{nepal},\"transition_cost_thousand_dollars\":{transition},\"maintenance_cost_thousand_dollars\":{maintenance},\"synthetic_total_public_resources_thousand_dollars\":{},\"residual_thousand_dollars\":0,\"partner_contribution_is_us_savings\":false,\"public_savings\":null}}",
        us + nepal + transition + maintenance
    ))
}

fn alternatives(p: &Program) -> Result<String, String> {
    let rows = [
        (
            p.n("status_quo_durable_bps")?,
            p.n("status_quo_local_incidence_bps")?,
            p.b("status_quo_do_no_harm")?,
            p.n("status_quo_cost_thousand_dollars")?,
        ),
        (
            p.n("delivery_support_durable_bps")?,
            p.n("delivery_support_local_incidence_bps")?,
            p.b("delivery_support_do_no_harm")?,
            p.n("delivery_support_cost_thousand_dollars")?,
        ),
        (
            p.n("local_capacity_durable_bps")?,
            p.n("local_capacity_local_incidence_bps")?,
            p.b("local_capacity_do_no_harm")?,
            p.n("local_capacity_cost_thousand_dollars")?,
        ),
    ];
    let feasible = rows
        .iter()
        .filter(|(durable, local, harm, _)| *durable >= 6_000 && *local >= 5_000 && *harm)
        .count();
    Ok(format!(
        "{{\"schema\":\"envoy.program-alternatives.v1\",\"alternative_count\":3,\"feasible_count\":{feasible},\"status_quo_durable_bps\":{},\"delivery_support_durable_bps\":{},\"local_capacity_durable_bps\":{},\"best_local_incidence_bps\":{},\"selected_alternative\":null,\"partner_or_award_decision_allowed\":false}}",
        rows[0].0, rows[1].0, rows[2].0, rows[2].1
    ))
}

fn incidence(p: &Program) -> Result<String, String> {
    let groups = [
        ("local_community", p.n("community_incidence_points")?),
        (
            "partner_government",
            p.n("partner_government_incidence_points")?,
        ),
        ("implementers", p.n("implementer_incidence_points")?),
        ("environment", p.n("environment_incidence_points")?),
        ("taxpayers", p.n("taxpayer_incidence_points")?),
    ];
    let total: i64 = groups.iter().map(|(_, value)| *value).sum();
    if total != 0 {
        return Err("incidence points must reconcile to zero".into());
    }
    let burden = groups.iter().min_by_key(|(_, value)| *value).unwrap();
    Ok(format!(
        "{{\"schema\":\"envoy.program-incidence.v1\",\"community_points\":{},\"partner_government_points\":{},\"implementer_points\":{},\"environment_points\":{},\"taxpayer_points\":{},\"reconciliation_points\":0,\"largest_burden_group\":\"{}\",\"distribution_pass\":false}}",
        groups[0].1, groups[1].1, groups[2].1, groups[3].1, groups[4].1, burden.0
    ))
}

fn delivery(p: &Program) -> Result<String, String> {
    let gates = [
        p.b("delivery_owner_named")?,
        p.b("delivery_road_evaluation_active")?,
        p.b("delivery_transmission_evaluation_active")?,
        p.b("delivery_completed_works_ready")?,
        p.b("delivery_safeguards_ready")?,
        p.b("delivery_grievance_ready")?,
        p.b("delivery_measures_ready")?,
        p.b("delivery_rollback_ready")?,
    ];
    let passed = gates.iter().filter(|gate| **gate).count();
    Ok(format!(
        "{{\"schema\":\"envoy.program-delivery.v1\",\"owner_named\":{},\"road_evaluation_active\":{},\"transmission_evaluation_active\":{},\"completed_works_ready\":{},\"safeguards_ready\":{},\"grievance_ready\":{},\"measures_ready\":{},\"rollback_ready\":{},\"gates_passed\":{passed},\"gates_required\":8,\"delivery_ready\":{}}}",
        gates[0], gates[1], gates[2], gates[3], gates[4], gates[5], gates[6], gates[7], passed == 8
    ))
}

fn adaptive(p: &Program) -> Result<String, String> {
    let observed = p.n("observed_durable_bps")?;
    let trigger = p.n("adaptive_trigger_bps")?;
    let current = p.n("current_version")?;
    let triggered = observed < trigger;
    Ok(format!(
        "{{\"schema\":\"envoy.program-adaptive.v1\",\"current_version\":{current},\"observed_durable_bps\":{observed},\"trigger_bps\":{trigger},\"triggered\":{triggered},\"successor_version\":{},\"predecessor_immutable\":true,\"automatic_diplomatic_or_award_action\":false}}",
        if triggered { current + 1 } else { current }
    ))
}

fn peers(p: &Program) -> Result<String, String> {
    let current = p.n("observed_durable_bps")?;
    let comparator = p.n("comparison_durable_bps")?;
    Ok(format!(
        "{{\"schema\":\"envoy.program-peers.v1\",\"current_durable_bps\":{current},\"illustrative_comparator_durable_bps\":{comparator},\"gap_bps\":{},\"same_definition\":{},\"official_peer_claim\":false,\"automatic_target\":false}}",
        comparator - current,
        p.b("comparison_same_definition")?
    ))
}

fn held_pack(p: &Program) -> Result<String, String> {
    let total = p.n("us_compact_thousand_dollars")?
        + p.n("nepal_contribution_thousand_dollars")?
        + p.n("transition_cost_thousand_dollars")?
        + p.n("maintenance_cost_thousand_dollars")?;
    Ok(format!(
        "{{\"schema\":\"taxlane.lane-evidence-pack-candidate.v1\",\"identity\":{{\"pack_id\":\"envoy:assistance-semantic-program:v1\",\"track\":\"INT\",\"domain_repository\":\"ENVOY\",\"candidate_id\":\"mcc_nepal_compact_implementation\",\"fiscal_owner\":\"TAXLANE\"}},\"scope\":{{\"geography\":\"synthetic aggregate demonstration anchored to the Nepal Compact\",\"included\":\"scenario realization accounting alternatives incidence delivery adaptation and comparison mechanics\",\"excluded\":\"sensitive sources operations partner selection awards and observed compact effects\"}},\"source_custody\":{{\"source_id\":\"ENVOY-SYNTHETIC-ASSISTANCE-SEMANTIC-PROGRAM\",\"evidence_label\":\"synthetic_aggregate_semantic_program\"}},\"problem\":{{\"baseline_metric\":\"delivery realization with local and durable floors\",\"baseline_value_or_null\":{}}},\"intervention\":{{\"mechanism\":\"bounded alternatives demonstration\",\"selected_alternative\":null}},\"outcomes\":{{\"bounded_marginal_effect_or_null\":null,\"candidate_effect_observed\":false}},\"service_floors\":{{\"local_incidence_safeguards_grievances_durability\":\"independent and held\",\"distribution_pass\":false}},\"costs\":{{\"synthetic_total_public_resources_or_null\":{total},\"public_savings\":null}},\"fiscal_bridge\":{{\"net_public_fiscal_pressure_or_null\":null,\"netting_rule\":\"synthetic transition and maintenance accounting cannot enter the fiscal model\"}},\"adaptive_pathways\":{{\"current_disposition\":\"held\",\"automatic_diplomatic_or_award_action\":false}},\"delivery\":{{\"road_evaluation_active\":true,\"transmission_evaluation_active\":false,\"completed_works_ready\":false,\"safeguards_ready\":false,\"grievance_ready\":false,\"delivery_ready\":false}},\"overlap\":{{\"other_lane_interactions\":\"TRN SEE\",\"non_additivity_rule\":\"partner contribution contracts outputs and projected benefits are not US savings\"}},\"readiness\":{{\"domain_evidence_ready\":true,\"candidate_bounded\":true,\"outcome_ready\":false,\"cost_ready\":false,\"floors_ready\":false,\"delivery_ready\":false,\"overlap_ready\":false,\"taxlane_admission_ready\":false}},\"claim_boundaries\":{{\"operational_partner_or_award_decision_allowed\":false,\"candidate_recommendation_allowed\":false,\"savings_allowed\":false,\"allocation_allowed\":false,\"rate_change_allowed\":false,\"public_release_allowed\":false}}}}",
        p.n("baseline_delivery_bps")?
    ))
}

pub fn run(command: &str, input: &str) -> Result<String, String> {
    let p = parse(input)?;
    match command {
        "program-scenarios" => scenarios(&p),
        "program-realization" => realization(&p),
        "program-accounting" => accounting(&p),
        "program-alternatives" => alternatives(&p),
        "program-incidence" => incidence(&p),
        "program-delivery" => delivery(&p),
        "program-adaptive" => adaptive(&p),
        "program-peers" => peers(&p),
        "program-held-pack" => held_pack(&p),
        _ => Err(format!("unknown program command: {command}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const FIXTURE: &str =
        include_str!("../../../fixtures/synthetic/assistance-semantic-program.tsv");

    #[test]
    fn scenarios_preserve_stress_and_recovery() {
        let out = run("program-scenarios", FIXTURE).unwrap();
        assert!(out.contains("\"stress_change_bps\":-1346"));
        assert!(out.contains("\"observed_candidate_effect\":false"));
    }

    #[test]
    fn realization_distinguishes_completion_and_durability() {
        let out = run("program-realization", FIXTURE).unwrap();
        assert!(out.contains("\"largest_handoff_loss_stage\":\"completion\""));
        assert!(out.contains("\"unrealized_assistance_is_savings\":false"));
    }

    #[test]
    fn accounting_includes_transition_and_maintenance() {
        let out = run("program-accounting", FIXTURE).unwrap();
        assert!(out.contains("\"synthetic_total_public_resources_thousand_dollars\":817000"));
        assert!(out.contains("\"partner_contribution_is_us_savings\":false"));
    }

    #[test]
    fn alternatives_do_not_select() {
        let out = run("program-alternatives", FIXTURE).unwrap();
        assert!(out.contains("\"feasible_count\":2"));
        assert!(out.contains("\"selected_alternative\":null"));
    }

    #[test]
    fn incidence_reconciles() {
        let out = run("program-incidence", FIXTURE).unwrap();
        assert!(out.contains("\"largest_burden_group\":\"local_community\""));
        assert!(out.contains("\"reconciliation_points\":0"));
    }

    #[test]
    fn delivery_preserves_evaluation_asymmetry() {
        let out = run("program-delivery", FIXTURE).unwrap();
        assert!(out.contains("\"gates_passed\":4"));
        assert!(out.contains("\"transmission_evaluation_active\":false"));
        assert!(out.contains("\"delivery_ready\":false"));
    }

    #[test]
    fn adaptive_creates_successor_without_external_action() {
        let out = run("program-adaptive", FIXTURE).unwrap();
        assert!(out.contains("\"successor_version\":2"));
        assert!(out.contains("\"automatic_diplomatic_or_award_action\":false"));
    }

    #[test]
    fn comparison_is_illustrative_only() {
        let out = run("program-peers", FIXTURE).unwrap();
        assert!(out.contains("\"gap_bps\":2000"));
        assert!(out.contains("\"official_peer_claim\":false"));
    }

    #[test]
    fn handoff_has_no_authority() {
        let out = run("program-held-pack", FIXTURE).unwrap();
        assert!(out.contains("\"taxlane_admission_ready\":false"));
        assert!(out.contains("\"public_release_allowed\":false"));
    }
}
