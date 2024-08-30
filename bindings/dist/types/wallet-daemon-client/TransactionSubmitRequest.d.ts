import type { Epoch } from "../Epoch";
import type { Instruction } from "../Instruction";
import type { SubstateRequirement } from "../SubstateRequirement";
import type { UnsignedTransaction } from "../UnsignedTransaction";
export interface TransactionSubmitRequest {
    transaction: UnsignedTransaction | null;
    signing_key_index: number | null;
    inputs: Array<SubstateRequirement>;
    override_inputs: boolean;
    is_dry_run: boolean;
    proof_ids: Array<number>;
    fee_instructions: Array<Instruction>;
    instructions: Array<Instruction>;
    min_epoch: Epoch | null;
    max_epoch: Epoch | null;
}