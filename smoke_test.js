const http = require('http');

async function request(path, method = 'GET', body = null) {
  return new Promise((resolve, reject) => {
    const bodyStr = body !== null ? JSON.stringify(body) : "";
    const options = {
      hostname: 'localhost',
      port: 4317,
      path,
      method,
      headers: body !== null ? {
        'Content-Type': 'application/json',
        'Content-Length': Buffer.byteLength(bodyStr)
      } : {},
    };
    
    const req = http.request(options, (res) => {
      let data = '';
      res.on('data', (chunk) => data += chunk);
      res.on('end', () => {
        if (res.statusCode >= 400) {
          reject(new Error(`Status: ${res.statusCode}, Body: ${data}`));
        } else {
          try {
            resolve(data ? JSON.parse(data) : null);
          } catch (e) {
            resolve(data);
          }
        }
      });
    });

    req.on('error', reject);
    if (body !== null) {
      req.write(bodyStr);
    }
    req.end();
  });
}

async function run() {
  try {
    console.log("Setting mode to stub...");
    await request('/orchestrator/mode', 'POST', { mode: 'stub' });

    console.log("Creating monitor session...");
    const session = await request('/monitor/sessions', 'POST', {});
    const sessionId = session.id;
    console.log(`Session ID: ${sessionId}`);

    console.log("Sending user message...");
    await request(`/monitor/sessions/${sessionId}/messages`, 'POST', {
      role: 'user',
      who: { id: 'user-id', name: 'User' },
      content: 'please route to core'
    });

    console.log("Ticking orchestrator...");
    await request('/orchestrator/tick', 'POST', {});

    console.log("Fetching state...");
    let state = await request('/monitor/state');
    const currentSession = state.sessions.find(s => s.id === sessionId);
    if (!currentSession) throw new Error("Session not found in state");
    
    const parentPacket = currentSession.packets.find(p => p.type === 'Software Architect' || p.type === 'software-architect');
    if (!parentPacket) {
        throw new Error("Parent packet not found: " + JSON.stringify(currentSession.packets));
    }
    
    const childPacket = currentSession.packets.find(p => p.parentId === parentPacket.id);
    if (!childPacket) throw new Error("Child packet not found");

    console.log("Approving review...");
    await request(`/monitor/packets/${parentPacket.id}/review`, 'POST', { verdict: 'approved' });

    console.log("Setting packet state to reviewed...");
    await request(`/monitor/packets/${parentPacket.id}/state`, 'POST', {
      state: 'reviewed',
      actor: 'stub-reviewer-agent'
    });

    console.log("Closing gatekeeper...");
    await request(`/monitor/packets/${parentPacket.id}/gatekeeper`, 'POST', { decision: 'close' });

    console.log("Resolving packet...");
    await request(`/monitor/packets/${parentPacket.id}/state`, 'POST', {
      state: 'resolved',
      actor: 'stub-gatekeeper-agent',
      evidence: {
        artifacts: [{ type: 'closure', ref: 'art-123' }],
        childPackets: [childPacket.id]
      }
    });

    console.log("Verifying final state...");
    state = await request('/monitor/state');
    const finalSession = state.sessions.find(s => s.id === sessionId);
    
    const finalParent = finalSession.packets.find(p => p.id === parentPacket.id);
    const finalChild = finalSession.packets.find(p => p.id === childPacket.id);
    
    const hasReturnMsg = finalSession.messages.some(m => m.role === 'assistant' && (m.content || "").includes("returning the result to you"));
    const hasClosedEntry = finalParent.lifecycle.some(l => l.event === 'packet.closed');

    const verification = {
      parentResolved: finalParent.state === 'resolved',
      childResolved: finalChild.state === 'resolved',
      hasReturnMsg,
      hasClosedEntry
    };

    console.log("VERIFICATION_OBJECT:", JSON.stringify(verification));

    console.log("Archiving session...");
    await request(`/monitor/sessions/${sessionId}/archive`, 'POST', {});

    process.exit(0);
  } catch (err) {
    console.error("FAILED:", err.message);
    process.exit(1);
  }
}
run();
