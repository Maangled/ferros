"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.parseSubprocessRuntimeCliArgs = parseSubprocessRuntimeCliArgs;
exports.runSubprocessRuntimeCli = runSubprocessRuntimeCli;
const runtime_packet_1 = require("./runtime-packet");
function readProcessStdin() {
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
function parseSubprocessRuntimeCliArgs(argv) {
    let agent;
    let classification;
    let summary;
    for (let index = 0; index < argv.length; index += 1) {
        const arg = argv[index];
        switch (arg) {
            case '--agent': {
                const value = argv[index + 1];
                if (!value) {
                    throw new Error('--agent requires a value');
                }
                agent = value;
                index += 1;
                break;
            }
            case '--classification': {
                const value = argv[index + 1];
                if (!value) {
                    throw new Error('--classification requires a value');
                }
                classification = value;
                index += 1;
                break;
            }
            case '--summary': {
                const value = argv[index + 1];
                if (!value) {
                    throw new Error('--summary requires a value');
                }
                summary = value;
                index += 1;
                break;
            }
            default:
                throw new Error(`Unknown argument '${arg}'`);
        }
    }
    if (!agent) {
        throw new Error('Missing required --agent argument');
    }
    return {
        agent,
        classification,
        summary,
    };
}
function formatSubprocessRuntimeResult(packet, args) {
    const agentLabel = args.agent;
    return {
        classification: args.classification || `subprocess-${agentLabel}`,
        parentRunId: packet.route_token.parent_run_id,
        response: `${agentLabel} subprocess handoff complete`,
        lifecycleOutcome: {
            kind: 'report',
            summary: args.summary || `${agentLabel} subprocess runtime completed`,
        },
    };
}
async function runSubprocessRuntimeCli(argv, io) {
    try {
        const args = parseSubprocessRuntimeCliArgs(argv);
        const packetJson = (await io.readInput()).trim();
        if (!packetJson) {
            throw new Error('Expected packet JSON on stdin');
        }
        const packet = (0, runtime_packet_1.parseRuntimePacketJson)(packetJson);
        io.writeOutput(`${JSON.stringify(formatSubprocessRuntimeResult(packet, args))}\n`);
        return 0;
    }
    catch (error) {
        io.writeError(`${error instanceof Error ? error.message : String(error)}\n`);
        return 1;
    }
}
if (require.main === module) {
    void runSubprocessRuntimeCli(process.argv.slice(2), defaultIo()).then((exitCode) => {
        process.exitCode = exitCode;
    });
}
//# sourceMappingURL=subprocess-runtime-cli.js.map