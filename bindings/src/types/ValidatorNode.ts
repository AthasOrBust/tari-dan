// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Epoch } from "./Epoch";
import type { Shard } from "./Shard";
import type { SubstateAddress } from "./SubstateAddress";

export interface ValidatorNode<TAddr> {
  address: string;
  public_key: string;
  shard_key: SubstateAddress;
  epoch: Epoch;
  committee_shard: Shard | null;
  fee_claim_public_key: string;
}