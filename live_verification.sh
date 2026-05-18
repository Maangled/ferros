#!/bin/bash
set -e

# 1. tmpdir=$(mktemp -d)
tmpdir=$(mktemp -d)

# 2. POST http://localhost:4317/orchestrator/mode with JSON {"mode":"stub"}
curl -s -X POST http://localhost:4317/orchestrator/mode -H "Content-Type: application/json" -d '{"mode":"stub"}' > /dev/null

# 3. POST http://localhost:4317/monitor/sessions with JSON {}
session_res=$(curl -s -X POST http://localhost:4317/monitor/sessions -H "Content-Type: application/json" -d '{}')

# 4. Parse session id from the create-session response
session_id=$(echo "$session_res" | node -e "console.log(JSON.parse(require('fs').readFileSync(0)).id)")

# 5. POST http://localhost:4317/monitor/sessions/$session_id/messages with JSON {"speaker":"user","who":"Operator","text":"please route to core"}
curl -s -X POST "http://localhost:4317/monitor/sessions/$session_id/messages" -H "Content-Type: application/json" -d '{"speaker":"user","who":"Operator","text":"please route to core"}' > /dev/null

# 6. POST http://localhost:4317/orchestrator/tick
curl -s -X POST http://localhost:4317/orchestrator/tick > /dev/null

# 7. GET http://localhost:4317/monitor/state and parse the root packet for that session
state_res=$(curl -s http://localhost:4317/monitor/state)

packet_info=$(echo "$state_res" | node -e "
const state = JSON.parse(require('fs').readFileSync(0));
const sessionId = '$session_id';
const packets = state.packets.filter(p => p.sessionId === sessionId);
const parent = packets.find(p => p.manager === 'Software Architect' && !p.parentPacketId);
if (!parent) { console.error('Parent packet not found'); process.exit(1); }
const child = packets.find(p => p.parentPacketId === parent.id);
if (!child) { console.error('Child packet not found'); process.exit(1); }
console.log(JSON.stringify({ parentId: parent.id, childId: child.id, parentWorkOrderId: parent.workOrderId }));
")

parent_id=$(echo "$packet_info" | node -e "console.log(JSON.parse(require('fs').readFileSync(0)).parentId)")
child_id=$(echo "$packet_info" | node -e "console.log(JSON.parse(require('fs').readFileSync(0)).childId)")
parent_work_order_id=$(echo "$packet_info" | node -e "console.log(JSON.parse(require('fs').readFileSync(0)).parentWorkOrderId)")

# 8. POST /monitor/packets/$parent_id/review-verdict with JSON {"verdict":"approved"}
curl -s -X POST "http://localhost:4317/monitor/packets/$parent_id/review-verdict" -H "Content-Type: application/json" -d '{"verdict":"approved"}' > /dev/null

# 9. POST /monitor/packets/$parent_id/state with JSON {"toState":"reviewed","actor":"stub-reviewer-agent","reason":"reviewer approved packet","evidenceRefs":[]}
curl -s -X POST "http://localhost:4317/monitor/packets/$parent_id/state" -H "Content-Type: application/json" -d '{"toState":"reviewed","actor":"stub-reviewer-agent","reason":"reviewer approved packet","evidenceRefs":[]}' > /dev/null

# 10. POST /monitor/packets/$parent_id/gatekeeper-decision with JSON {"decision":"close"}
curl -s -X POST "http://localhost:4317/monitor/packets/$parent_id/gatekeeper-decision" -H "Content-Type: application/json" -d '{"decision":"close"}' > /dev/null

# 11. POST /monitor/packets/$parent_id/state with JSON {"toState":"resolved","actor":"stub-gatekeeper-agent","reason":"gatekeeper closed packet","evidenceRefs":["artifact://closure/$parent_id","child-packet:$child_id"]}'
curl -s -X POST "http://localhost:4317/monitor/packets/$parent_id/state" -H "Content-Type: application/json" -d "{\"toState\":\"resolved\",\"actor\":\"stub-gatekeeper-agent\",\"reason\":\"gatekeeper closed packet\",\"evidenceRefs\":[\"artifact://closure/$parent_id\",\"child-packet:$child_id\"]}" > /dev/null

# Run another tick
curl -s -X POST http://localhost:4317/orchestrator/tick > /dev/null

# 12. GET final /monitor/state and compute a JSON summary
final_state_res=$(curl -s http://localhost:4317/monitor/state)

summary=$(echo "$final_state_res" | node -e "
const state = JSON.parse(require('fs').readFileSync(0));
const sessionId = '$session_id';
const parentId = '$parent_id';
const childId = '$child_id';
const parentWorkOrderId = '$parent_work_order_id';

const packets = state.packets;
const parent = packets.find(p => p.id === parentId);
const child = packets.find(p => p.id === childId);

const session = state.openChats.find(s => s.id === sessionId);
const ferrosMessages = session ? session.messages.filter(m => m.who === 'FERROS Agent' && m.text.includes('returning the result to you')) : [];
const finalFerrosMessage = ferrosMessages.length > 0 ? ferrosMessages[ferrosMessages.length - 1].text : null;

// Match thread by workOrderId
const threads = state.lifecycleThreads.filter(t => t.workOrderId === parentWorkOrderId);
const closureEntry = threads.flatMap(t => t.entries).reverse().find(e => e.kind === 'packet.closed');
const closureEntryKind = closureEntry ? closureEntry.kind : null;

console.log(JSON.stringify({
  sessionId,
  parentPacketId: parentId,
  parentState: parent ? parent.state : null,
  childPacketId: childId,
  childState: child ? child.state : null,
  finalFerrosMessage,
  closureEntryKind
}, null, 2));
")

echo "$summary"

# 13. POST /monitor/sessions/$session_id/archive with JSON {}
curl -s -X POST "http://localhost:4317/monitor/sessions/$session_id/archive" -H "Content-Type: application/json" -d '{}' > /dev/null
