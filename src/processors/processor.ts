import { base58 } from "@metaplex-foundation/umi/serializers";
import type { VersionedTransactionResponse } from "@solana/web3.js";

import {
  findLeafAssetIdPda,
  getBurnInstructionDataSerializer,
  getMintToCollectionV1InstructionDataSerializer,
  getTransferInstructionDataSerializer,
  getBubblegumEventTypeSerializer,
  getLeafSchemaSerializer,
  LeafSchema,
} from "@metaplex-foundation/mpl-bubblegum";

import { handleBubblegumInstruction, BUBBLEGUM_PROGRAM_ID } from "./bubblegum";
import {
  type GetSignaturesForAddressInput,
  getSignaturesForAddress,
  getTransaction,
} from "../rpc/rpc";

export const NOOPPROGRAMID = "noopb9bkMVfRPU8AsbpTUg8AQkHtKwMYZiFUjNRtMmV";

export const processTransaction = async (
  transaction: VersionedTransactionResponse
) => {
  const accountKeys = transaction.transaction.message.getAccountKeys({
    accountKeysFromLookups: transaction.meta?.loadedAddresses,
  });

  let instructions = [
    ...transaction.transaction.message.compiledInstructions,
    ...(transaction.meta?.innerInstructions ?? [])
      .flatMap(({ instructions }) => instructions)
      .map(({ programIdIndex, data, accounts }) => ({
        programIdIndex,
        data: base58.serialize(data),
        accountKeyIndexes: accounts,
      })),
  ];

  let leafSchema: LeafSchema;
  for (const {
    programIdIndex,
    data,
    accountKeyIndexes,
  } of instructions.reverse()) {
    switch (accountKeys.get(programIdIndex)?.toBase58()) {
      case NOOPPROGRAMID:
          if (Buffer.from(data.slice(0,8)).toString("hex") === "0100cb0000000100") {
            leafSchema = getLeafSchemaSerializer().deserialize(
              data.slice(8)
            )[0];            
          } 
        break;

      case BUBBLEGUM_PROGRAM_ID.toBase58():
        await handleBubblegumInstruction(
          data,
          accountKeyIndexes,
          accountKeys,
          transaction,
          leafSchema
        );
        break;
    }
  }
};

export const processMerkleTreeTransactions = async ({
  address,
  signaturesForAddressOptions,
}: GetSignaturesForAddressInput) => {
  const signatures = await getSignaturesForAddress({
    address,
    signaturesForAddressOptions,
  });

  console.log(
    `Found ${
      signatures.length
    } new transactions on Merkle Tree ${address.toBase58()}...`
  );

  for (let i = 0; i < signatures.length; i++) {
    const signature = signatures[i].signature;

    console.log("Fetching Transaction:", signature);

    const transaction = await getTransaction(signature);

    await processTransaction(transaction);
  }

  return {
    lastProcessedTxSignature: signatures.length
      ? signatures[0].signature
      : undefined,
  };
};

export const processWebsocketMerkleTreeTransaction = async (
  txSignature: string
) => {
  console.log("Fetching Transaction: ", txSignature);

  let transaction: VersionedTransactionResponse;

  // `onLogs` is used to get transactions in realtime. It gies a transaction signature. `getTransaction` is slow to index that fresh transaction and returns tx not found.
  // Since we get the tx signature from `onLogs`, we know the tx exists on chain, so we just try until we get it.
  while (true) {
    try {
      transaction = await getTransaction(txSignature);
      break;
    } catch (e) {
      console.log("Websocket Tx not found, retrying...");
    }
  }

  await processTransaction(transaction);
  return { lastProcessedTxSignature: txSignature };
};
