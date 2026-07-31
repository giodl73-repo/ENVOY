#[derive(Debug, Clone, PartialEq, Eq)]
struct Indicator {
    name: String,
    level: String,
    unit: String,
    baseline: Option<i64>,
    actual: i64,
    quarter: u64,
    compact_specific: bool,
    target_ready: bool,
}

fn boolean(value: &str, line: usize) -> Result<bool, String> {
    match value {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(format!("line {line}: expected boolean")),
    }
}

fn parse(input: &str) -> Result<Vec<Indicator>, String> {
    for marker in [
        "# source_id=MCC-OPEN-DATA-ITT-2025",
        "# evidence_label=official_aggregate_outcome_reference",
        "# source_sha256=ca183c1092fb68765c1e0b7de0f20f050c3205fb8cc8299512a930b2cac3a00b",
    ] {
        if !input.contains(marker) {
            return Err(format!("missing required marker: {marker}"));
        }
    }
    let mut rows = Vec::new();
    for (index, line) in input.lines().enumerate() {
        let line_number = index + 1;
        if line.is_empty() || line.starts_with('#') || line.starts_with("indicator\t") {
            continue;
        }
        let fields: Vec<_> = line.split('\t').collect();
        if fields.len() != 8 {
            return Err(format!("line {line_number}: expected 8 fields"));
        }
        let number = |field: usize| {
            fields[field]
                .parse::<i64>()
                .map_err(|_| format!("line {line_number}: invalid integer"))
        };
        let baseline = if fields[3] == "null" {
            None
        } else {
            Some(number(3)?)
        };
        rows.push(Indicator {
            name: fields[0].to_owned(),
            level: fields[1].to_owned(),
            unit: fields[2].to_owned(),
            baseline,
            actual: number(4)?,
            quarter: fields[5]
                .parse::<u64>()
                .map_err(|_| format!("line {line_number}: invalid quarter"))?,
            compact_specific: boolean(fields[6], line_number)?,
            target_ready: boolean(fields[7], line_number)?,
        });
    }
    if rows.len() != 7 || rows.iter().any(|row| row.quarter != 4 || row.actual < 0) {
        return Err("outcome reference requires seven nonnegative quarter-4 rows".into());
    }
    Ok(rows)
}

fn find<'a>(rows: &'a [Indicator], name: &str) -> Result<&'a Indicator, String> {
    rows.iter()
        .find(|row| row.name == name)
        .ok_or_else(|| format!("missing indicator: {name}"))
}

fn delta(row: &Indicator) -> Option<i64> {
    row.baseline.map(|baseline| row.actual - baseline)
}

fn change_bps(row: &Indicator) -> Option<i64> {
    row.baseline
        .filter(|baseline| *baseline != 0)
        .map(|baseline| (row.actual - baseline) * 10_000 / baseline)
}

fn reference_json(rows: &[Indicator]) -> Result<String, String> {
    let revenue = find(rows, "export_revenue")?;
    let supply = find(rows, "total_electricity_supply")?;
    let imports = find(rows, "imported_electricity")?;
    let exports = find(rows, "exported_electricity")?;
    let losses = find(rows, "transmission_system_technical_losses")?;
    let recovery = find(rows, "operating_cost_recovery_ratio")?;
    let capacity = find(rows, "generation_capacity")?;
    let outcome_rows = rows.iter().filter(|row| row.level == "outcome").count();
    let attributed = rows.iter().filter(|row| row.compact_specific).count();
    let target_ready = rows.iter().filter(|row| row.target_ready).count();
    Ok(format!(
        "{{\"schema\":\"envoy.nepal-outcome-reference.v1\",\"source_rows\":{},\"outcome_rows\":{},\"compact_specific_rows\":{},\"target_ready_rows\":{},\"export_revenue\":{{\"unit\":\"{}\",\"baseline\":{},\"actual\":{},\"delta\":{},\"change_bps\":{}}},\"electricity_supply\":{{\"baseline_mwh\":{},\"actual_mwh\":{},\"delta_mwh\":{},\"change_bps\":{}}},\"imports\":{{\"baseline_mwh\":{},\"actual_mwh\":{},\"change_bps\":{}}},\"exports\":{{\"baseline_mwh\":{},\"actual_mwh\":{},\"change_bps\":{}}},\"technical_losses\":{{\"baseline_bps\":{},\"actual_bps\":{},\"delta_bps\":{}}},\"operating_cost_recovery\":{{\"baseline_bps\":null,\"actual_bps\":{}}},\"generation_capacity\":{{\"unit\":\"{}\",\"baseline\":{},\"actual\":{},\"delta\":{},\"change_bps\":{}}},\"national_movement_is_compact_effect\":false,\"candidate_effect_observable\":false}}",
        rows.len(), outcome_rows, attributed, target_ready,
        revenue.unit, revenue.baseline.unwrap(), revenue.actual, delta(revenue).unwrap(), change_bps(revenue).unwrap(),
        supply.baseline.unwrap(), supply.actual, delta(supply).unwrap(), change_bps(supply).unwrap(),
        imports.baseline.unwrap(), imports.actual, change_bps(imports).unwrap(),
        exports.baseline.unwrap(), exports.actual, change_bps(exports).unwrap(),
        losses.baseline.unwrap(), losses.actual, delta(losses).unwrap(), recovery.actual,
        capacity.unit, capacity.baseline.unwrap(), capacity.actual, delta(capacity).unwrap(), change_bps(capacity).unwrap()
    ))
}

fn held_pack_json(rows: &[Indicator]) -> Result<String, String> {
    let reference = reference_json(rows)?;
    Ok(format!(
        "{{\"schema\":\"taxlane.lane-evidence-pack-candidate.v1\",\"identity\":{{\"pack_id\":\"envoy:mcc-nepal-outcome-reference:v1\",\"track\":\"INT\",\"domain_repository\":\"ENVOY\",\"candidate_id\":\"mcc_nepal_compact_implementation\",\"fiscal_owner\":\"TAXLANE\"}},\"scope\":{{\"geography\":\"Nepal\",\"included\":\"public aggregate national electricity and road-system outcome context\",\"excluded\":\"operational data causal claims and partner decisions\"}},\"source_custody\":{{\"source_id\":\"MCC-OPEN-DATA-ITT-2025\",\"publisher\":\"Millennium Challenge Corporation Open Data Catalog\",\"checksum_or_null\":\"ca183c1092fb68765c1e0b7de0f20f050c3205fb8cc8299512a930b2cac3a00b\"}},\"problem\":{{\"baseline_metric\":\"outcome observability and attribution\",\"baseline_value_or_null\":{reference},\"problem_boundary\":\"national movement during implementation is not a compact effect\"}},\"intervention\":{{\"mechanism\":\"active Nepal Compact\",\"implementing_owner\":\"MCA-Nepal with MCC oversight\"}},\"outcomes\":{{\"bounded_marginal_effect_or_null\":null,\"candidate_effect_observed\":false,\"uncertainty\":\"no compact-specific row counterfactual exposure alignment or independent result\"}},\"service_floors\":{{\"access\":null,\"quality_safety\":null,\"equity_distribution\":null,\"adequacy_resilience\":null,\"do_no_harm_pass\":null}},\"costs\":{{\"gross_cost_or_null\":null,\"net_cost_or_null\":null,\"public_savings\":null}},\"fiscal_bridge\":{{\"delivery_efficiency_public_savings_or_null\":null,\"net_public_fiscal_pressure_or_null\":null,\"netting_rule\":\"national system movement cannot be monetized or attributed to the compact\"}},\"adaptive_pathways\":{{\"observation_cadence\":\"quarterly ITT plus independent evaluation\",\"reopen_triggers\":\"compact-specific outcome rows matched exposure counterfactual local incidence safeguards costs and independent results\",\"current_disposition\":\"held_context_observed_effect_unidentified\"}},\"delivery\":{{\"capacity\":\"national outcome reference available\",\"milestones\":\"compact-specific result chain remains missing\"}},\"overlap\":{{\"other_lane_interactions\":\"TRN SEE\",\"non_additivity_rule\":\"national supply trade capacity and compact infrastructure are overlapping context not additive effects\"}},\"readiness\":{{\"domain_evidence_ready\":true,\"candidate_bounded\":true,\"outcome_reference_ready\":true,\"outcome_ready\":false,\"floors_ready\":false,\"delivery_ready\":false,\"taxlane_admission_ready\":false}},\"claim_boundaries\":{{\"operational_partner_or_award_decision_allowed\":false,\"candidate_recommendation_allowed\":false,\"savings_allowed\":false,\"allocation_allowed\":false,\"rate_change_allowed\":false,\"public_release_allowed\":false}}}}"
    ))
}

pub(crate) fn run(command: &str, input: &str) -> Result<String, String> {
    let rows = parse(input)?;
    match command {
        "outcome-reference" => reference_json(&rows),
        "outcome-held-pack" => held_pack_json(&rows),
        _ => Err(format!("unknown outcome command: {command}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const FIXTURE: &str =
        include_str!("../../../fixtures/official/mcc-nepal-outcome-reference-2025.tsv");

    #[test]
    fn computes_national_movement_without_attribution() {
        let output = run("outcome-reference", FIXTURE).unwrap();
        assert!(output.contains("\"delta_mwh\":1597000"));
        assert!(output.contains("\"delta_bps\":-6"));
        assert!(output.contains("\"compact_specific_rows\":0"));
        assert!(output.contains("\"candidate_effect_observable\":false"));
    }

    #[test]
    fn holds_fiscal_and_external_authority() {
        let output = run("outcome-held-pack", FIXTURE).unwrap();
        assert!(output.contains("\"outcome_reference_ready\":true"));
        assert!(output.contains("\"outcome_ready\":false"));
        assert!(output.contains("\"public_savings\":null"));
        assert!(output.contains("\"rate_change_allowed\":false"));
    }

    #[test]
    fn rejects_source_or_shape_drift() {
        assert!(run("outcome-reference", &FIXTURE.replace("ca183c", "ba183c")).is_err());
        assert!(run(
            "outcome-reference",
            &FIXTURE.replace("\t4\tfalse", "\t5\tfalse")
        )
        .is_err());
    }
}
