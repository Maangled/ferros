use std::env;
use std::fmt;

use crate::{default_local_runtime_summary, LocalHubRuntimeSummary};
use serde_json::{json, Value};

const HA_URL_ENV: &str = "FERROS_HA_URL";
const HA_TOKEN_ENV: &str = "FERROS_HA_TOKEN";
const DEFAULT_EVENT_TYPE: &str = "ferros_probe";
const DEFAULT_ENTITY_ID: &str = "sensor.ferros_bridge_probe";
const DEFAULT_ENTITY_STATE: &str = "report-state";
const DEFAULT_FRIENDLY_NAME: &str = "FERROS Bridge Probe";
const DEFAULT_STATE_SOURCE: &str = "ferros-hub-remote-stand-in";

#[derive(Debug, Clone, PartialEq, Eq)]
struct RemoteBridgeStateRequest {
    entity_id: String,
    state: String,
    attributes: Value,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteBridgeSummary {
    pub ha_url: String,
    pub location_name: String,
    pub version: String,
    pub ferros_entities: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteBridgeEventResult {
    pub ha_url: String,
    pub event_type: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteBridgeStateResult {
    pub ha_url: String,
    pub entity_id: String,
    pub state: String,
}

#[derive(Debug)]
pub enum RemoteBridgeCommandError {
    MissingEnvironmentVariable(&'static str),
    InvalidUrl(String),
    Transport(String),
    ApiStatus(u16, String),
    Json(serde_json::Error),
}

impl fmt::Display for RemoteBridgeCommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingEnvironmentVariable(name) => {
                write!(f, "missing required environment variable {name}")
            }
            Self::InvalidUrl(url) => write!(f, "invalid Home Assistant URL: {url}"),
            Self::Transport(message) => write!(f, "Home Assistant transport error: {message}"),
            Self::ApiStatus(status, body) => write!(
                f,
                "Home Assistant API returned status {status}: {}",
                if body.is_empty() { "<empty body>" } else { body }
            ),
            Self::Json(error) => write!(f, "Home Assistant JSON parse error: {error}"),
        }
    }
}

impl From<serde_json::Error> for RemoteBridgeCommandError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

pub fn remote_summary_command_output() -> Result<String, RemoteBridgeCommandError> {
    let (ha_url, ha_token) = remote_bridge_config_from_env()?;
    let summary = fetch_remote_bridge_summary(&ha_url, &ha_token)?;

    Ok(format_remote_bridge_summary(&summary))
}

pub fn remote_fire_event_command_output() -> Result<String, RemoteBridgeCommandError> {
    let (ha_url, ha_token) = remote_bridge_config_from_env()?;
    let result = fire_remote_probe_event(&ha_url, &ha_token)?;

    Ok(format_remote_bridge_event_result(&result))
}

pub fn remote_report_state_command_output() -> Result<String, RemoteBridgeCommandError> {
    let (ha_url, ha_token) = remote_bridge_config_from_env()?;
    let result = report_remote_bridge_state(&ha_url, &ha_token)?;

    Ok(format_remote_bridge_state_result(&result))
}

fn remote_bridge_config_from_env() -> Result<(String, String), RemoteBridgeCommandError> {
    let ha_url = env::var(HA_URL_ENV)
        .map_err(|_| RemoteBridgeCommandError::MissingEnvironmentVariable(HA_URL_ENV))?;
    let ha_token = env::var(HA_TOKEN_ENV)
        .map_err(|_| RemoteBridgeCommandError::MissingEnvironmentVariable(HA_TOKEN_ENV))?;

    Ok((normalize_ha_url(&ha_url)?, ha_token))
}

fn normalize_ha_url(url: &str) -> Result<String, RemoteBridgeCommandError> {
    let normalized = url.trim().trim_end_matches('/');

    if normalized.starts_with("http://") || normalized.starts_with("https://") {
        return Ok(normalized.to_string());
    }

    Err(RemoteBridgeCommandError::InvalidUrl(url.to_string()))
}

fn fetch_remote_bridge_summary(
    ha_url: &str,
    ha_token: &str,
) -> Result<RemoteBridgeSummary, RemoteBridgeCommandError> {
    let config = authorized_get_json(ha_url, ha_token, "/api/config")?;
    let states = authorized_get_json(ha_url, ha_token, "/api/states")?;

    let location_name = config
        .get("location_name")
        .and_then(Value::as_str)
        .unwrap_or("unknown")
        .to_string();
    let version = config
        .get("version")
        .and_then(Value::as_str)
        .unwrap_or("unknown")
        .to_string();
    let ferros_entities = states
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|entry| entry.get("entity_id").and_then(Value::as_str))
        .filter(|entity_id| entity_id.to_ascii_lowercase().contains("ferros"))
        .map(ToOwned::to_owned)
        .collect();

    Ok(RemoteBridgeSummary {
        ha_url: ha_url.to_string(),
        location_name,
        version,
        ferros_entities,
    })
}

fn fire_remote_probe_event(
    ha_url: &str,
    ha_token: &str,
) -> Result<RemoteBridgeEventResult, RemoteBridgeCommandError> {
    let payload = authorized_post_json(
        ha_url,
        ha_token,
        &format!("/api/events/{DEFAULT_EVENT_TYPE}"),
        json!({
            "source": "ferros-hub",
            "mode": "ephemeral"
        }),
    )?;
    let message = payload
        .get("message")
        .and_then(Value::as_str)
        .unwrap_or("event fired")
        .to_string();

    Ok(RemoteBridgeEventResult {
        ha_url: ha_url.to_string(),
        event_type: DEFAULT_EVENT_TYPE.to_string(),
        message,
    })
}

fn report_remote_bridge_state(
    ha_url: &str,
    ha_token: &str,
) -> Result<RemoteBridgeStateResult, RemoteBridgeCommandError> {
    let request = remote_bridge_state_request();
    let payload = authorized_post_json(
        ha_url,
        ha_token,
        &format!("/api/states/{}", request.entity_id),
        json!({
            "state": request.state,
            "attributes": request.attributes
        }),
    )?;
    let entity_id = payload
        .get("entity_id")
        .and_then(Value::as_str)
        .unwrap_or(&request.entity_id)
        .to_string();
    let state = payload
        .get("state")
        .and_then(Value::as_str)
        .unwrap_or(&request.state)
        .to_string();

    Ok(RemoteBridgeStateResult {
        ha_url: ha_url.to_string(),
        entity_id,
        state,
    })
}

fn authorized_get_json(
    ha_url: &str,
    ha_token: &str,
    path: &str,
) -> Result<Value, RemoteBridgeCommandError> {
    let response = ureq::get(&format!("{ha_url}{path}"))
        .set("Authorization", &format!("Bearer {ha_token}"))
        .set("Content-Type", "application/json")
        .call()
        .map_err(map_ureq_error)?;

    response_json(response)
}

fn authorized_post_json(
    ha_url: &str,
    ha_token: &str,
    path: &str,
    payload: Value,
) -> Result<Value, RemoteBridgeCommandError> {
    let response = ureq::post(&format!("{ha_url}{path}"))
        .set("Authorization", &format!("Bearer {ha_token}"))
        .set("Content-Type", "application/json")
        .send_json(payload)
        .map_err(map_ureq_error)?;

    response_json(response)
}

fn response_json(response: ureq::Response) -> Result<Value, RemoteBridgeCommandError> {
    let body = response
        .into_string()
        .map_err(|error| RemoteBridgeCommandError::Transport(error.to_string()))?;

    serde_json::from_str(&body).map_err(RemoteBridgeCommandError::from)
}

fn map_ureq_error(error: ureq::Error) -> RemoteBridgeCommandError {
    match error {
        ureq::Error::Status(status, response) => RemoteBridgeCommandError::ApiStatus(
            status,
            response.into_string().unwrap_or_default(),
        ),
        ureq::Error::Transport(error) => RemoteBridgeCommandError::Transport(error.to_string()),
    }
}

fn remote_bridge_state_request() -> RemoteBridgeStateRequest {
    match default_local_runtime_summary() {
        Ok(summary) => remote_bridge_state_request_from_summary(&summary),
        Err(_) => default_remote_bridge_state_request(),
    }
}

fn remote_bridge_state_request_from_summary(
    summary: &LocalHubRuntimeSummary,
) -> RemoteBridgeStateRequest {
    let entity_id = format!(
        "sensor.ferros_{}_status",
        normalized_bridge_agent_key(&summary.bridge_agent_name)
    );
    let bridge_manifest_identity = format!(
        "{}@{}",
        summary.bridge_agent_name, summary.bridge_agent_version
    );

    RemoteBridgeStateRequest {
        entity_id,
        state: summary.status.as_str().to_string(),
        attributes: json!({
            "friendly_name": format!("FERROS {} Status", summary.bridge_agent_name),
            "bridge_agent": summary.bridge_agent_name,
            "bridge_manifest_identity": bridge_manifest_identity,
            "requester_profile_id": summary.requester_profile_id,
            "requested_capability": summary.requested_capability,
            "requested_action": summary.requested_action,
            "stand_in_name": summary.stand_in_name,
            "artifact_relative_output_path": summary.artifact_relative_output_path,
            "scope": summary.scope,
            "evidence": summary.evidence,
            "bridge_status": summary.status.as_str(),
            "restart_reload": summary.restart_observation.reload_status.as_str(),
            "summary": summary.summary,
            "state_source": "ferros-hub-local-runtime-summary"
        }),
    }
}

fn default_remote_bridge_state_request() -> RemoteBridgeStateRequest {
    RemoteBridgeStateRequest {
        entity_id: DEFAULT_ENTITY_ID.to_string(),
        state: DEFAULT_ENTITY_STATE.to_string(),
        attributes: json!({
            "friendly_name": DEFAULT_FRIENDLY_NAME,
            "bridge_agent": "ha-local-bridge",
            "requested_action": "report-state",
            "scope": "remote-stand-in",
            "evidence": "api-visible-stand-in",
            "state_source": DEFAULT_STATE_SOURCE
        }),
    }
}

fn normalized_bridge_agent_key(bridge_agent_name: &str) -> String {
    let normalized = bridge_agent_name
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect::<String>();
    let collapsed = normalized
        .split('_')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>()
        .join("_");

    if collapsed.is_empty() {
        "bridge".to_string()
    } else {
        collapsed
    }
}

fn format_remote_bridge_summary(summary: &RemoteBridgeSummary) -> String {
    let ferros_entities = if summary.ferros_entities.is_empty() {
        "none".to_string()
    } else {
        summary.ferros_entities.join(",")
    };

    format!(
        concat!(
            "ferros-hub remote-summary\n",
            "haUrl: {}\n",
            "locationName: {}\n",
            "version: {}\n",
            "ferrosEntityCount: {}\n",
            "ferrosEntities: {}\n",
            "summary: authenticated remote Home Assistant probe found {} ferros entities"
        ),
        summary.ha_url,
        summary.location_name,
        summary.version,
        summary.ferros_entities.len(),
        ferros_entities,
        summary.ferros_entities.len()
    )
}

fn format_remote_bridge_event_result(result: &RemoteBridgeEventResult) -> String {
    format!(
        concat!(
            "ferros-hub remote-fire-event\n",
            "haUrl: {}\n",
            "eventType: {}\n",
            "message: {}\n",
            "summary: authenticated remote Home Assistant event probe fired {}"
        ),
        result.ha_url,
        result.event_type,
        result.message,
        result.event_type
    )
}

fn format_remote_bridge_state_result(result: &RemoteBridgeStateResult) -> String {
    format!(
        concat!(
            "ferros-hub remote-report-state\n",
            "haUrl: {}\n",
            "entityId: {}\n",
            "state: {}\n",
            "summary: authenticated remote Home Assistant report-state upsert wrote {}"
        ),
        result.ha_url,
        result.entity_id,
        result.state,
        result.entity_id
    )
}

#[cfg(test)]
mod tests {
    use super::{
        default_remote_bridge_state_request,
        format_remote_bridge_event_result, format_remote_bridge_state_result,
        format_remote_bridge_summary, normalize_ha_url,
        remote_bridge_state_request_from_summary, RemoteBridgeCommandError,
        RemoteBridgeEventResult, RemoteBridgeStateResult, RemoteBridgeSummary,
    };
    use crate::{
        LocalBridgeStatus, LocalHubReloadStatus, LocalHubRestartObservation,
        LocalHubRuntimeSummary,
    };
    use ferros_core::PolicyDecision;

    fn sample_runtime_summary() -> LocalHubRuntimeSummary {
        LocalHubRuntimeSummary {
            registered_bridge_agents: 1,
            bridge_agent_name: "ha-local-bridge".to_string(),
            bridge_agent_version: "0.1.0".to_string(),
            requester_profile_id: "hub-local-bridge".to_string(),
            stand_in_name: "simulated-bridge-entity".to_string(),
            requested_capability: "bridge.observe".to_string(),
            requested_action: "report-state".to_string(),
            policy_decision: PolicyDecision::Allowed,
            status: LocalBridgeStatus::Allowed,
            artifact_relative_output_path: Some(
                ".tmp/hub/simulated-local-bridge-artifact.json".to_string(),
            ),
            local_onramp_proposal: None,
            local_onramp_decision_receipt: None,
            summary: "local bridge allowed report-state".to_string(),
            scope: "local-only".to_string(),
            evidence: "non-evidentiary".to_string(),
            restart_observation: LocalHubRestartObservation {
                reload_status: LocalHubReloadStatus::Reloaded,
                prior_bridge_manifest_identity: Some("ha-local-bridge@0.1.0".to_string()),
                prior_policy_decision_label: Some("allowed".to_string()),
                prior_artifact_relative_output_path: Some(
                    ".tmp/hub/simulated-local-bridge-artifact.json".to_string(),
                ),
            },
        }
    }

    #[test]
    fn remote_summary_output_formats_authenticated_probe_result() {
        let output = format_remote_bridge_summary(&RemoteBridgeSummary {
            ha_url: "http://192.168.50.194:8123".to_string(),
            location_name: "Home".to_string(),
            version: "2026.4.4".to_string(),
            ferros_entities: vec!["sensor.ferros_probe".to_string()],
        });

        assert!(output.contains("ferros-hub remote-summary"));
        assert!(output.contains("haUrl: http://192.168.50.194:8123"));
        assert!(output.contains("ferrosEntityCount: 1"));
        assert!(output.contains("ferrosEntities: sensor.ferros_probe"));
    }

    #[test]
    fn remote_event_output_formats_authenticated_event_probe_result() {
        let output = format_remote_bridge_event_result(&RemoteBridgeEventResult {
            ha_url: "http://192.168.50.194:8123".to_string(),
            event_type: "ferros_probe".to_string(),
            message: "Event ferros_probe fired.".to_string(),
        });

        assert!(output.contains("ferros-hub remote-fire-event"));
        assert!(output.contains("eventType: ferros_probe"));
        assert!(output.contains("message: Event ferros_probe fired."));
    }

    #[test]
    fn remote_state_output_formats_authenticated_upsert_result() {
        let output = format_remote_bridge_state_result(&RemoteBridgeStateResult {
            ha_url: "http://192.168.50.194:8123".to_string(),
            entity_id: "sensor.ferros_bridge_probe".to_string(),
            state: "report-state".to_string(),
        });

        assert!(output.contains("ferros-hub remote-report-state"));
        assert!(output.contains("entityId: sensor.ferros_bridge_probe"));
        assert!(output.contains("state: report-state"));
    }

    #[test]
    fn remote_state_request_uses_local_runtime_summary_when_available() {
        let request = remote_bridge_state_request_from_summary(&sample_runtime_summary());

        assert_eq!(request.entity_id, "sensor.ferros_ha_local_bridge_status");
        assert_eq!(request.state, "allowed");
        assert_eq!(
            request.attributes.get("bridge_manifest_identity").and_then(|value| value.as_str()),
            Some("ha-local-bridge@0.1.0")
        );
        assert_eq!(
            request.attributes.get("scope").and_then(|value| value.as_str()),
            Some("local-only")
        );
        assert_eq!(
            request
                .attributes
                .get("state_source")
                .and_then(|value| value.as_str()),
            Some("ferros-hub-local-runtime-summary")
        );
    }

    #[test]
    fn remote_state_request_falls_back_to_probe_defaults() {
        let request = default_remote_bridge_state_request();

        assert_eq!(request.entity_id, "sensor.ferros_bridge_probe");
        assert_eq!(request.state, "report-state");
        assert_eq!(
            request
                .attributes
                .get("state_source")
                .and_then(|value| value.as_str()),
            Some("ferros-hub-remote-stand-in")
        );
    }

    #[test]
    fn remote_probe_rejects_non_http_urls() {
        let error = normalize_ha_url("mqtt://192.168.50.194")
            .expect_err("non-http URL should be rejected");

        assert!(matches!(
            error,
            RemoteBridgeCommandError::InvalidUrl(url) if url == "mqtt://192.168.50.194"
        ));
    }
}