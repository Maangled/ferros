"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.parseHandoffCliArgs = parseHandoffCliArgs;
exports.runHandoffCli = runHandoffCli;
const coordinator_1 = require("./coordinator");
const runtime_packet_1 = require("./runtime-packet");
function asRecord(value) {
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        return undefined;
    }
    return value;
}
function readString(record, snakeKey, camelKey) {
    const value = record[snakeKey] ?? (camelKey ? record[camelKey] : undefined);
    return typeof value === 'string' ? value : undefined;
}
function normalizeStringArray(value) {
    if (!Array.isArray(value)) {
        return undefined;
    }
    return value.filter((entry) => typeof entry === 'string');
}
function parseBooleanEnv(value) {
    if (value === undefined) {
        return undefined;
    }
    switch (value.trim().toLowerCase()) {
        case '1':
        case 'true':
        case 'yes':
        case 'on':
            return true;
        case '0':
        case 'false':
        case 'no':
        case 'off':
            return false;
        default:
            throw new Error(`Invalid boolean environment value '${value}'`);
    }
}
function parseLogLevel(value) {
    if (value === undefined) {
        return undefined;
    }
    if (['debug', 'info', 'warn', 'error'].includes(value)) {
        return value;
    }
    throw new Error(`Invalid log level '${value}'`);
}
function parseReasoningEffort(value) {
    if (value === undefined) {
        return undefined;
    }
    if (['low', 'medium', 'high', 'xhigh'].includes(value)) {
        return value;
    }
    throw new Error(`Invalid reasoning effort '${value}'`);
}
function parseHandoffCliArgs(argv, env = process.env) {
    let targetAgent;
    let timeoutMs = env.FERROS_COORDINATOR_TIMEOUT_MS
        ? Number.parseInt(env.FERROS_COORDINATOR_TIMEOUT_MS, 10)
        : undefined;
    let logLevel = parseLogLevel(env.FERROS_COORDINATOR_LOG_LEVEL);
    let captureEvents = parseBooleanEnv(env.FERROS_COORDINATOR_CAPTURE_EVENTS);
    let model = env.FERROS_COORDINATOR_SESSION_MODEL;
    let reasoningEffort = parseReasoningEffort(env.FERROS_COORDINATOR_REASONING_EFFORT);
    for (let index = 0; index < argv.length; index += 1) {
        const arg = argv[index];
        switch (arg) {
            case '--target': {
                const value = argv[index + 1];
                if (value !== 'core' && value !== 'subcore') {
                    throw new Error("--target must be 'core' or 'subcore'");
                }
                targetAgent = value;
                index += 1;
                break;
            }
            case '--timeout-ms': {
                const value = argv[index + 1];
                if (!value) {
                    throw new Error('--timeout-ms requires a numeric value');
                }
                timeoutMs = Number.parseInt(value, 10);
                if (!Number.isFinite(timeoutMs)) {
                    throw new Error(`Invalid timeout '${value}'`);
                }
                index += 1;
                break;
            }
            case '--log-level': {
                const value = argv[index + 1];
                logLevel = parseLogLevel(value);
                index += 1;
                break;
            }
            case '--model': {
                const value = argv[index + 1];
                if (!value) {
                    throw new Error('--model requires a value');
                }
                model = value;
                index += 1;
                break;
            }
            case '--reasoning-effort': {
                const value = argv[index + 1];
                if (!value) {
                    throw new Error('--reasoning-effort requires a value');
                }
                reasoningEffort = parseReasoningEffort(value);
                index += 1;
                break;
            }
            case '--capture-events':
                captureEvents = true;
                break;
            case '--no-capture-events':
                captureEvents = false;
                break;
            default:
                throw new Error(`Unknown argument '${arg}'`);
        }
    }
    if (!targetAgent) {
        throw new Error("Missing required --target argument ('core' or 'subcore')");
    }
    return {
        targetAgent,
        timeoutMs,
        logLevel,
        captureEvents,
        model,
        reasoningEffort,
    };
}
async function readProcessStdin() {
    const chunks = [];
    return new Promise((resolve, reject) => {
        process.stdin.on('data', (chunk) => {
            chunks.push(Buffer.isBuffer(chunk) ? chunk : Buffer.from(chunk));
        });
        process.stdin.on('end', () => resolve(Buffer.concat(chunks).toString('utf8')));
        process.stdin.on('error', reject);
    });
}
function defaultIo() {
    return {
        readInput: readProcessStdin,
        writeOutput(text) {
            process.stdout.write(text);
        },
        writeError(text) {
            process.stderr.write(text);
        },
    };
}
function normalizeLifecycleOutcome(outcome) {
    const raw = asRecord(outcome);
    if (!raw) {
        return undefined;
    }
    const kind = readString(raw, 'kind');
    const summary = readString(raw, 'summary');
    if (!kind || !summary) {
        return undefined;
    }
    return {
        kind,
        summary,
        workOrderId: readString(raw, 'work_order_id', 'workOrderId'),
        escalationId: readString(raw, 'escalation_id', 'escalationId'),
        targetAgentId: readString(raw, 'target_agent_id', 'targetAgentId'),
        stopReason: readString(raw, 'stop_reason', 'stopReason'),
    };
}
function normalizeCoordinatorErrorDetails(details) {
    const raw = asRecord(details);
    if (!raw) {
        return undefined;
    }
    const normalized = { ...raw };
    const errors = normalizeStringArray(raw.errors);
    const warnings = normalizeStringArray(raw.warnings);
    const lifecycleOutcome = normalizeLifecycleOutcome(raw.lifecycle_outcome ?? raw.lifecycleOutcome);
    const lifecycleErrors = normalizeStringArray(raw.lifecycle_errors ?? raw.lifecycleErrors);
    if (errors) {
        normalized.errors = errors;
    }
    if (warnings) {
        normalized.warnings = warnings;
    }
    if (lifecycleOutcome) {
        normalized.lifecycleOutcome = lifecycleOutcome;
    }
    if (lifecycleErrors) {
        normalized.lifecycleErrors = lifecycleErrors;
    }
    delete normalized.lifecycle_outcome;
    delete normalized.lifecycle_errors;
    return normalized;
}
function normalizeHandoffCliResult(result) {
    if ('error' in result) {
        return {
            error: result.error,
            failedChecks: result.failedChecks || [],
            details: normalizeCoordinatorErrorDetails(result.details),
        };
    }
    return {
        classification: result.classification,
        parentRunId: result.parent_run_id,
        response: result.response,
        lifecycleOutcome: normalizeLifecycleOutcome(result.lifecycle_outcome),
        lifecycleErrors: result.lifecycle_errors || [],
    };
}
async function runHandoffCli(argv, io, env = process.env, handoff = coordinator_1.handoffToAgent) {
    try {
        const args = parseHandoffCliArgs(argv, env);
        const packetJson = (await io.readInput()).trim();
        if (!packetJson) {
            throw new Error('Expected packet JSON on stdin');
        }
        const packet = (0, runtime_packet_1.parseRuntimePacketJson)(packetJson);
        const options = {};
        if (args.timeoutMs !== undefined) {
            options.timeout_ms = args.timeoutMs;
        }
        if (args.logLevel !== undefined) {
            options.log_level = args.logLevel;
        }
        if (args.captureEvents !== undefined) {
            options.capture_events = args.captureEvents;
        }
        if (args.model !== undefined) {
            options.session_model = args.model;
        }
        if (args.reasoningEffort !== undefined) {
            options.session_reasoning_effort = args.reasoningEffort;
        }
        const result = await handoff(packet, args.targetAgent, options);
        io.writeOutput(`${JSON.stringify(normalizeHandoffCliResult(result))}\n`);
        return 0;
    }
    catch (error) {
        io.writeError(`${error instanceof Error ? error.message : String(error)}\n`);
        return 1;
    }
}
if (require.main === module) {
    void runHandoffCli(process.argv.slice(2), defaultIo()).then((exitCode) => {
        process.exitCode = exitCode;
    });
}
//# sourceMappingURL=handoff-cli.js.map