import { Packet } from './types';
import { parseRuntimePacketJson } from './runtime-packet';

interface SubprocessRuntimeCliArgs {
  agent: string;
  classification?: string;
  summary?: string;
}

interface SubprocessRuntimeCliIo {
  readInput(): Promise<string>;
  writeOutput(text: string): void;
  writeError(text: string): void;
}

function readProcessStdin(): Promise<string> {
  const chunks: Buffer[] = [];

  return new Promise((resolve, reject) => {
    process.stdin.on('data', (chunk) => {
      chunks.push(Buffer.isBuffer(chunk) ? chunk : Buffer.from(chunk));
    });
    process.stdin.on('end', () => resolve(Buffer.concat(chunks).toString('utf8')));
    process.stdin.on('error', reject);
  });
}

function defaultIo(): SubprocessRuntimeCliIo {
  return {
    readInput: readProcessStdin,
    writeOutput(text: string) {
      process.stdout.write(text);
    },
    writeError(text: string) {
      process.stderr.write(text);
    },
  };
}

export function parseSubprocessRuntimeCliArgs(argv: string[]): SubprocessRuntimeCliArgs {
  let agent: string | undefined;
  let classification: string | undefined;
  let summary: string | undefined;

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

function formatSubprocessRuntimeResult(packet: Packet, args: SubprocessRuntimeCliArgs) {
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

export async function runSubprocessRuntimeCli(
  argv: string[],
  io: SubprocessRuntimeCliIo
): Promise<number> {
  try {
    const args = parseSubprocessRuntimeCliArgs(argv);
    const packetJson = (await io.readInput()).trim();

    if (!packetJson) {
      throw new Error('Expected packet JSON on stdin');
    }

    const packet = parseRuntimePacketJson(packetJson);
    io.writeOutput(`${JSON.stringify(formatSubprocessRuntimeResult(packet, args))}\n`);
    return 0;
  } catch (error) {
    io.writeError(`${error instanceof Error ? error.message : String(error)}\n`);
    return 1;
  }
}

if (require.main === module) {
  void runSubprocessRuntimeCli(process.argv.slice(2), defaultIo()).then((exitCode) => {
    process.exitCode = exitCode;
  });
}