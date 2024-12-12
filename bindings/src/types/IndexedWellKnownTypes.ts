// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { BucketId } from "./BucketId";
import type { ComponentAddress } from "./ComponentAddress";
import type { Metadata } from "./Metadata";
import type { NonFungibleAddress } from "./NonFungibleAddress";
import type { ProofId } from "./ProofId";
import type { PublishedTemplateAddress } from "./PublishedTemplateAddress";
import type { ResourceAddress } from "./ResourceAddress";
import type { TransactionReceiptAddress } from "./TransactionReceiptAddress";
import type { UnclaimedConfidentialOutputAddress } from "./UnclaimedConfidentialOutputAddress";
import type { VaultId } from "./VaultId";

export interface IndexedWellKnownTypes {
  bucket_ids: Array<BucketId>;
  proof_ids: Array<ProofId>;
  component_addresses: Array<ComponentAddress>;
  resource_addresses: Array<ResourceAddress>;
  transaction_receipt_addresses: Array<TransactionReceiptAddress>;
  non_fungible_addresses: Array<NonFungibleAddress>;
  vault_ids: Array<VaultId>;
  metadata: Array<Metadata>;
  unclaimed_confidential_output_address: Array<UnclaimedConfidentialOutputAddress>;
  published_template_addresses: Array<PublishedTemplateAddress>;
}
