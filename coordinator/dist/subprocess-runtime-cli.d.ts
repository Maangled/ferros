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
export declare function parseSubprocessRuntimeCliArgs(argv: string[]): SubprocessRuntimeCliArgs;
export declare function runSubprocessRuntimeCli(argv: string[], io: SubprocessRuntimeCliIo): Promise<number>;
export {};
//# sourceMappingURL=subprocess-runtime-cli.d.ts.map