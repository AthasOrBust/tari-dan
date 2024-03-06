// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Amount } from "../Amount";
import type { ComponentAddressOrName } from "./ComponentAddressOrName";

export interface AccountsCreateFreeTestCoinsRequest {
  account: ComponentAddressOrName | null;
  amount: Amount;
  max_fee: Amount | null;
  key_id: number | null;
}