// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Amount } from "../Amount";
import type { ComponentAddressOrName } from "./ComponentAddressOrName";
import type { ResourceAddress } from "../ResourceAddress";

export interface ProofsGenerateRequest {
  amount: Amount;
  reveal_amount: Amount;
  account: ComponentAddressOrName | null;
  resource_address: ResourceAddress;
  destination_public_key: string;
}