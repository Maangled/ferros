import { handoffToAgent } from './coordinator';
import { CoordinatorOptions, Packet } from './types';
type TargetAgent = 'core' | 'subcore';
type LogLevel = NonNullable<CoordinatorOptions['log_level']>;
type ReasoningEffort = NonNullable<CoordinatorOptions['session_reasoning_effort']>;
interface HandoffCliArgs {
    targetAgent: TargetAgent;
    timeoutMs?: number;
    logLevel?: LogLevel;
    captureEvents?: boolean;
    model?: string;
    reasoningEffort?: ReasoningEffort;
}
interface HandoffCliIo {
    readInput(): Promise<string>;
    writeOutput(text: string): void;
    writeError(text: string): void;
}
type HandoffFn = (packet: Packet, targetAgent: TargetAgent, options?: CoordinatorOptions) => ReturnType<typeof handoffToAgent>;
export declare function parseHandoffCliArgs(argv: string[], env?: Record<string, string | undefined>): HandoffCliArgs;
export declare function runHandoffCli(argv: string[], io: HandoffCliIo, env?: Record<string, string | undefined>, handoff?: HandoffFn): Promise<number>;
export {};
//# sourceMappingURL=handoff-cli.d.ts.map