import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { Connection, PublicKey, type SignaturesForAddressOptions, type TransactionSignature } from "@solana/web3.js";

import config from "../config/config";

export const connection = new Connection(config.rpcUrl,{
  wsEndpoint: config.wsUrl,
});
export const context = createUmi(config.rpcUrl);

export type GetSignaturesForAddressInput = {
  address: PublicKey;
  signaturesForAddressOptions: SignaturesForAddressOptions;
};

export const getSignaturesForAddress = async ({
  address,
  signaturesForAddressOptions,
}: GetSignaturesForAddressInput) => {
  const signatures = await connection.getSignaturesForAddress(address, signaturesForAddressOptions);
  return signatures;
};

export const getTransaction = async (signature: TransactionSignature) => {
  let transaction = await connection.getTransaction(signature, { maxSupportedTransactionVersion: 0, commitment: "confirmed" });

  if (!transaction) throw new Error("Transaction not found");

  return transaction;
};
