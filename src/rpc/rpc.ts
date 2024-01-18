import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
  Connection,
  PublicKey,
  type SignaturesForAddressOptions,
  type TransactionSignature,
} from "@solana/web3.js";

import envConfig from "../config/envConfig";

export const connection = new Connection(envConfig.rpcUrl, {
  wsEndpoint: envConfig.wsUrl,
});
export const context = createUmi(envConfig.rpcUrl);

export type GetSignaturesForAddressInput = {
  address: PublicKey;
  signaturesForAddressOptions: SignaturesForAddressOptions;
};

export const getSignaturesForAddress = async ({
  address,
  signaturesForAddressOptions,
}: GetSignaturesForAddressInput) => {
  const signatures = await connection.getSignaturesForAddress(
    address,
    signaturesForAddressOptions
  );
  return signatures;
};

export const getTransaction = async (signature: TransactionSignature) => {
  let transaction = await connection.getTransaction(signature, {
    maxSupportedTransactionVersion: 0,
    commitment: "confirmed",
  });

  if (!transaction) throw new Error("Transaction not found");

  return transaction;
};
