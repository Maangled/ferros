"use strict";
/**
 * FERROS Orchestrator Coordinator: TypeScript interfaces and types
 * Defines packet structures, execution returns, guardrail results, and SDK wrappers
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.isCoordinatorError = isCoordinatorError;
exports.isExecutionReturn = isExecutionReturn;
/**
 * Type guard to check if result is error
 */
function isCoordinatorError(result) {
    return result && result.error !== undefined && result.escalate !== undefined;
}
/**
 * Type guard to check if result is execution return
 */
function isExecutionReturn(result) {
    return result && result.classification !== undefined && result.parent_run_id !== undefined;
}
//# sourceMappingURL=types.js.map