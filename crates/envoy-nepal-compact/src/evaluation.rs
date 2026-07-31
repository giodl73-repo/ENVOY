#[derive(Debug, PartialEq, Eq)]
struct Evaluation {
    id: String,
    project: String,
    value_band_min_thousands: u64,
    value_band_max_thousands: u64,
    status: String,
    planned_date: String,
    evaluation_design_public: Option<bool>,
    independent_evaluator_selected: Option<bool>,
    final_collection_date: Option<String>,
    local_incidence_plan: Option<bool>,
    safeguard_linkage: Option<bool>,
}

fn optional_bool(value: &str, line: usize) -> Result<Option<bool>, String> {
    match value {
        "true" => Ok(Some(true)),
        "false" => Ok(Some(false)),
        "null" => Ok(None),
        _ => Err(format!("line {line}: expected true, false, or null")),
    }
}

fn optional_text(value: &str) -> Option<String> {
    (value != "null").then(|| value.to_owned())
}

fn parse(input: &str) -> Result<Vec<Evaluation>, String> {
    for marker in [
        "# source_id=MCC-FY2026-MIDYEAR-BUSINESS-FORECAST",
        "# evidence_label=official_evaluation_readiness",
        "# vintage=FY2026_midyear",
    ] {
        if !input.contains(marker) {
            return Err(format!("missing required marker: {marker}"));
        }
    }
    let mut rows = Vec::new();
    for (index, line) in input.lines().enumerate() {
        let line_number = index + 1;
        if line.is_empty() || line.starts_with('#') || line.starts_with("evaluation_id\t") {
            continue;
        }
        let fields: Vec<_> = line.split('\t').collect();
        if fields.len() != 11 {
            return Err(format!("line {line_number}: expected 11 fields"));
        }
        let number = |field: usize| {
            fields[field]
                .parse::<u64>()
                .map_err(|_| format!("line {line_number}: invalid nonnegative integer"))
        };
        let row = Evaluation {
            id: fields[0].to_owned(),
            project: fields[1].to_owned(),
            value_band_min_thousands: number(2)?,
            value_band_max_thousands: number(3)?,
            status: fields[4].to_owned(),
            planned_date: fields[5].to_owned(),
            evaluation_design_public: optional_bool(fields[6], line_number)?,
            independent_evaluator_selected: optional_bool(fields[7], line_number)?,
            final_collection_date: optional_text(fields[8]),
            local_incidence_plan: optional_bool(fields[9], line_number)?,
            safeguard_linkage: optional_bool(fields[10], line_number)?,
        };
        if row.value_band_min_thousands > row.value_band_max_thousands {
            return Err(format!("line {line_number}: inverted value band"));
        }
        if !matches!(row.status.as_str(), "cancelled" | "solicitation_issued") {
            return Err(format!(
                "line {line_number}: unsupported procurement status"
            ));
        }
        rows.push(row);
    }
    if rows.len() != 2 {
        return Err("readiness fixture requires both Nepal project evaluations".into());
    }
    if !rows.iter().any(|row| row.id == "26-DPE-2372")
        || !rows.iter().any(|row| row.id == "26-DPE-2373")
    {
        return Err("readiness fixture requires the two forecast evaluation IDs".into());
    }
    Ok(rows)
}

fn readiness_json(rows: &[Evaluation]) -> String {
    let transmission = rows
        .iter()
        .find(|row| row.project == "electricity_transmission")
        .expect("validated fixture");
    let roads = rows
        .iter()
        .find(|row| row.project == "road_maintenance")
        .expect("validated fixture");
    format!(
        "{{\"schema\":\"envoy.nepal-compact-evaluation-readiness.v1\",\"as_of\":\"FY2026_midyear\",\"electricity_evaluation\":{{\"evaluation_id\":\"{}\",\"procurement_status\":\"{}\",\"value_band_min_thousands\":{},\"value_band_max_thousands\":{},\"planned_date\":\"{}\"}},\"road_evaluation\":{{\"evaluation_id\":\"{}\",\"procurement_status\":\"{}\",\"value_band_min_thousands\":{},\"value_band_max_thousands\":{},\"planned_date\":\"{}\"}},\"project_logic_public\":null,\"evaluation_method_public\":null,\"baseline_cohort_public\":null,\"data_sources_public\":null,\"exposure_aligned_final_collection_date\":null,\"independent_evaluator_selected\":null,\"local_incidence_plan\":null,\"safeguard_and_grievance_linkage\":null,\"matched_finance_output_outcome_cohort\":false,\"independent_evaluation_complete\":false,\"candidate_effect_observable\":false,\"projected_beneficiaries_are_observed\":false,\"projected_err_is_observed\":false}}",
        transmission.id,
        transmission.status,
        transmission.value_band_min_thousands,
        transmission.value_band_max_thousands,
        transmission.planned_date,
        roads.id,
        roads.status,
        roads.value_band_min_thousands,
        roads.value_band_max_thousands,
        roads.planned_date
    )
}

fn held_pack_json(rows: &[Evaluation]) -> String {
    let readiness = readiness_json(rows);
    format!(
        "{{\"schema\":\"taxlane.lane-evidence-pack-candidate.v1\",\"identity\":{{\"pack_id\":\"envoy:mcc-nepal-compact-evaluation-readiness:v1\",\"track\":\"INT\",\"domain_repository\":\"ENVOY\",\"candidate_id\":\"mcc_nepal_compact_implementation\",\"fiscal_owner\":\"TAXLANE\"}},\"source_custody\":{{\"source_id\":\"MCC-FY2026-MIDYEAR-BUSINESS-FORECAST\",\"publisher\":\"Millennium Challenge Corporation\",\"vintage\":\"FY2026 midyear\"}},\"evaluation_readiness\":{},\"outcomes\":{{\"bounded_marginal_effect_or_null\":null,\"uncertainty\":\"road evaluation is solicited; electricity evaluation is cancelled; design, cohort, incidence, safeguards, collection, and results are not established\"}},\"service_floors\":{{\"local_incidence\":null,\"safeguards_and_grievances\":null,\"durable_service\":null,\"do_no_harm_pass\":null}},\"fiscal_bridge\":{{\"delivery_efficiency_public_savings_or_null\":null,\"external_economic_benefit_or_null\":null,\"net_public_fiscal_pressure_or_null\":null,\"netting_rule\":\"do not promote projected beneficiaries, ERR, procurement status, or output progress as observed effect or savings\"}},\"adaptive_pathways\":{{\"reopen_triggers\":\"public evaluation design and baseline; independent evaluator; exposure-aligned collection; matched finance-output-outcome cohort; subgroup incidence; safeguard and grievance linkage; completed results\",\"current_disposition\":\"held\"}},\"readiness\":{{\"evaluation_procurement_observed\":true,\"evaluation_design_ready\":false,\"candidate_effect_observable\":false,\"outcome_ready\":false,\"floors_ready\":false,\"taxlane_admission_ready\":false}},\"claim_boundaries\":{{\"domain_finding_allowed\":true,\"candidate_recommendation_allowed\":false,\"savings_allowed\":false,\"allocation_allowed\":false,\"rate_change_allowed\":false,\"public_release_allowed\":false}}}}",
        readiness
    )
}

pub fn run(command: &str, input: &str) -> Result<String, String> {
    let rows = parse(input)?;
    match command {
        "evaluation-readiness" => Ok(readiness_json(&rows)),
        "evaluation-held-pack" => Ok(held_pack_json(&rows)),
        _ => Err(format!("unknown evaluation command: {command}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const FIXTURE: &str =
        include_str!("../../../fixtures/official/mcc-nepal-evaluation-readiness-2026.tsv");

    #[test]
    fn preserves_distinct_procurement_statuses() {
        let output = readiness_json(&parse(FIXTURE).unwrap());
        assert!(output.contains("\"procurement_status\":\"cancelled\""));
        assert!(output.contains("\"procurement_status\":\"solicitation_issued\""));
    }

    #[test]
    fn does_not_promote_procurement_or_projections_to_effects() {
        let output = readiness_json(&parse(FIXTURE).unwrap());
        assert!(output.contains("\"candidate_effect_observable\":false"));
        assert!(output.contains("\"projected_beneficiaries_are_observed\":false"));
        assert!(output.contains("\"projected_err_is_observed\":false"));
    }

    #[test]
    fn held_pack_names_concrete_reopening_evidence() {
        let output = held_pack_json(&parse(FIXTURE).unwrap());
        assert!(output.contains("exposure-aligned collection"));
        assert!(output.contains("safeguard and grievance linkage"));
        assert!(output.contains("\"taxlane_admission_ready\":false"));
    }

    #[test]
    fn rejects_unknown_status() {
        let changed = FIXTURE.replacen("solicitation_issued", "awarded", 1);
        assert!(parse(&changed).is_err());
    }
}
