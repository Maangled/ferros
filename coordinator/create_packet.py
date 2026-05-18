import json, datetime
packet = {
    "route_token": {"token_version": "v2"},
    "issued_by": "FERROS Prompt Architect Agent",
    "target_stream": "core",
    "target_family": None,
    "run_id": "FRS-core-20260517-C1-W1",
    "parent_run_id": "FRS-manual-20260517-C1-W0",
    "recursion_depth": 1,
    "issued_at": datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
    "payload": "Mini model coordinator test for FERROS Core Agent.",
    "prompt": "Return a concise execution report and lifecycle outcome for this manual FERROS coordinator test.",
    "metadata": {
        "lifecycle_contract": {
            "cycle_id": "C1",
            "work_order_id": "WO-mini-model-001",
            "source_agent_id": "Software Architect",
            "target_agent_id": "core",
            "owner_agent_id": "Software Architect",
            "stop": {
                "allowed_terminal_states": ["report", "escalation", "denied", "archived", "stopped"],
                "stopped_reason_required": True
            }
        },
        "execution_context": {
            "source_kind": "manual-mini-model-test",
            "packet_id": "pkt-mini-model-001",
            "session_id": "chat-mini-model-001",
            "manager_agent_id": "Software Architect"
        }
    }
}
print(json.dumps(packet))
