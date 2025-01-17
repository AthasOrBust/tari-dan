import type { Event } from "./Event";
import type { FeeReceipt } from "./FeeReceipt";
import type { InstructionResult } from "./InstructionResult";
import type { LogEntry } from "./LogEntry";
import type { TransactionResult } from "./TransactionResult";
export interface FinalizeResult {
    transaction_hash: Uint8Array;
    events: Array<Event>;
    logs: Array<LogEntry>;
    execution_results: Array<InstructionResult>;
    result: TransactionResult;
    fee_receipt: FeeReceipt;
}
