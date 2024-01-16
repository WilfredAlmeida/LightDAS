import { base58 } from "@metaplex-foundation/umi/serializers";
import type { VersionedTransactionResponse } from "@solana/web3.js";

import {
  handleBubblegumBurnInstruction,
  handleBubblegumTransferInstruction,
  isBubblegumBurnInstruction,
  isBubblegumTransferInstruction,
} from "./bubblegum";
import {
  type GetSignaturesForAddressInput,
  getSignaturesForAddress,
  getTransaction,
} from "../rpc/rpc";

export const getAccountKeys = (transaction: VersionedTransactionResponse) => {
  return transaction.transaction.message.getAccountKeys({
    accountKeysFromLookups: transaction.meta?.loadedAddresses,
  });
};

export const processCpis = (transaction: VersionedTransactionResponse) => {
  const accountKeys = getAccountKeys(transaction);

  for (const cpi of transaction.meta?.innerInstructions || []) {
    for (const cpiInstruction of cpi.instructions) {
      const instructionData = base58.serialize(cpiInstruction.data);

      if (
        isBubblegumTransferInstruction(
          cpiInstruction.programIdIndex,
          instructionData,
          accountKeys
        )
      ) {
        handleBubblegumTransferInstruction(
          instructionData,
          cpiInstruction.accounts,
          accountKeys
        );
      }

      if (
        isBubblegumBurnInstruction(
          cpiInstruction.programIdIndex,
          instructionData,
          accountKeys
        )
      ) {
        handleBubblegumBurnInstruction(
          instructionData,
          cpiInstruction.accounts,
          accountKeys
        );
      }
    }
  }
};

export const processInstructions = (
  transaction: VersionedTransactionResponse
) => {
  const accountKeys = getAccountKeys(transaction);

  for (const instruction of transaction.transaction.message
    .compiledInstructions) {
    if (
      isBubblegumTransferInstruction(
        instruction.programIdIndex,
        instruction.data,
        accountKeys
      )
    ) {
      handleBubblegumTransferInstruction(
        instruction.data,
        instruction.accountKeyIndexes,
        accountKeys
      );
    }

    if (
      isBubblegumBurnInstruction(
        instruction.programIdIndex,
        instruction.data,
        accountKeys
      )
    ) {
      handleBubblegumBurnInstruction(
        instruction.data,
        instruction.accountKeyIndexes,
        accountKeys
      );
    }
  }
};

export const processTransaction = async (
  transaction: VersionedTransactionResponse
) => {
  processCpis(transaction);
  processInstructions(transaction);
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
