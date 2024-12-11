// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { EvictNodeAtom } from "./EvictNodeAtom";
import type { ForeignProposalAtom } from "./ForeignProposalAtom";
import type { MintConfidentialOutputAtom } from "./MintConfidentialOutputAtom";
import type { TransactionAtom } from "./TransactionAtom";

export type Command =
  | { LocalOnly: TransactionAtom }
  | { Prepare: TransactionAtom }
  | { LocalPrepare: TransactionAtom }
  | { AllPrepare: TransactionAtom }
  | { SomePrepare: TransactionAtom }
  | { LocalAccept: TransactionAtom }
  | { AllAccept: TransactionAtom }
  | { SomeAccept: TransactionAtom }
  | { ForeignProposal: ForeignProposalAtom }
  | { MintConfidentialOutput: MintConfidentialOutputAtom }
  | { EvictNode: EvictNodeAtom }
  | "EndEpoch";
