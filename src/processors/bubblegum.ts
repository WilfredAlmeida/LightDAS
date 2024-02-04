import {
  findLeafAssetIdPda,
  getBurnInstructionDataSerializer,
  getMintToCollectionV1InstructionDataSerializer,
  getTransferInstructionDataSerializer,
  BubblegumEventType,
  LeafSchema,
  BurnInstructionData,
  MintToCollectionV1InstructionData,
  TransferInstructionData
} from "@metaplex-foundation/mpl-bubblegum";
import { publicKey } from "@metaplex-foundation/umi";
import { MessageAccountKeys, PublicKey, VersionedTransactionResponse } from "@solana/web3.js";

import { context } from "../rpc/rpc";
import { handleMintToCollectionV1Instruction } from "./instructions/mintToCollectionV1";

export const BUBBLEGUM_PROGRAM_ID = new PublicKey(
  "BGUMAp9Gq7iTEuizy4pqaxsTyUCBK68MDfK752saRPUY",
);

export interface ParsedInstructionResult {
  instructionType: keyof typeof INSTRUCTION_ACCOUNT_MAP;
  accounts: { [key in (typeof INSTRUCTION_ACCOUNT_MAP)[keyof typeof INSTRUCTION_ACCOUNT_MAP][number]]: PublicKey } & {
    remainingAccounts: PublicKey[];
  };
  data: BurnInstructionData | TransferInstructionData | MintToCollectionV1InstructionData;
}

export const DISCRIMINATORS = {
  burn: [116, 110, 29, 56, 107, 219, 42, 93],
  cancelRedeem: [111, 76, 232, 50, 39, 175, 48, 242],
  createTreeConfig: [165, 83, 136, 142, 89, 202, 47, 220],
  decompressV1: [54, 85, 76, 70, 228, 250, 164, 81],
  delegate: [90, 147, 75, 178, 85, 88, 4, 137],
  mintToCollectionV1: [153, 18, 178, 47, 197, 158, 86, 15],
  mintV1: [145, 98, 192, 118, 184, 147, 118, 104],
  redeem: [184, 12, 86, 149, 70, 196, 97, 225],
  setAndVerifyCollection: [235, 242, 121, 216, 158, 234, 180, 234],
  setDecompressibleState: [82, 104, 152, 6, 149, 111, 100, 13],
  setTreeDelegate: [253, 118, 66, 37, 190, 49, 154, 102],
  transfer: [163, 52, 200, 231, 140, 3, 69, 186],
  unverifyCollection: [250, 251, 42, 106, 41, 137, 186, 168],
  unverifyCreator: [107, 178, 57, 39, 105, 115, 112, 152],
  updateMetadata: [170, 182, 43, 239, 97, 78, 225, 186],
  verifyCollection: [56, 113, 101, 253, 79, 55, 122, 169],
  verifyCreator: [52, 17, 96, 132, 71, 4, 85, 194],
  verifyLeaf: [124, 220, 22, 223, 104, 10, 250, 224],
} as const;

export const DISCRIMINATOR_LOOKUP = Object.fromEntries(
  Object.entries(DISCRIMINATORS).map(([k, v]) => [
    Buffer.from(v).toString("hex"),
    k as keyof typeof INSTRUCTION_ACCOUNT_MAP,
  ]),
);

export const INSTRUCTION_ACCOUNT_MAP = {
  burn: [
    "treeAuthority",
    "leafOwner",
    "leafDelegate",
    "merkleTree",
    "logWrapper",
    "compressionProgram",
    "systemProgram",
  ],
  cancelRedeem: [
    "treeAuthority",
    "leafOwner",
    "merkleTree",
    "voucher",
    "logWrapper",
    "compressionProgram",
    "systemProgram",
  ],
  compress: [
    "treeAuthority",
    "leafOwner",
    "leafDelegate",
    "merkleTree",
    "tokenAccount",
    "mint",
    "metadata",
    "masterEdition",
    "payer",
    "logWrapper",
    "compressionProgram",
    "tokenProgram",
    "tokenMetadataProgram",
    "systemProgram",
  ],
  createTree: [
    "treeAuthority",
    "merkleTree",
    "payer",
    "treeCreator",
    "logWrapper",
    "compressionProgram",
    "systemProgram",
  ],
  decompressV1: [
    "voucher",
    "leafOwner",
    "tokenAccount",
    "mint",
    "mintAuthority",
    "metadata",
    "masterEdition",
    "systemProgram",
    "sysvarRent",
    "tokenMetadataProgram",
    "tokenProgram",
    "associatedTokenProgram",
    "logWrapper",
  ],
  delegate: [
    "treeAuthority",
    "leafOwner",
    "previousLeafDelegate",
    "newLeafDelegate",
    "merkleTree",
    "logWrapper",
    "compressionProgram",
    "systemProgram",
  ],
  mintToCollectionV1: [
    "treeAuthority",
    "leafOwner",
    "leafDelegate",
    "merkleTree",
    "payer",
    "treeDelegate",
    "collectionAuthority",
    "collectionAuthorityRecordPda",
    "collectionMint",
    "collectionMetadata",
    "editionAccount",
    "bubblegumSigner",
    "logWrapper",
    "compressionProgram",
    "tokenMetadataProgram",
    "systemProgram",
  ],
  mintV1: [
    "treeAuthority",
    "leafOwner",
    "leafDelegate",
    "merkleTree",
    "payer",
    "treeDelegate",
    "logWrapper",
    "compressionProgram",
    "systemProgram",
  ],
  redeem: [
    "treeAuthority",
    "leafOwner",
    "leafDelegate",
    "merkleTree",
    "voucher",
    "logWrapper",
    "compressionProgram",
    "systemProgram",
  ],
  setAndVerifyCollection: [
    "treeAuthority",
    "leafOwner",
    "leafDelegate",
    "merkleTree",
    "payer",
    "treeDelegate",
    "collectionAuthority",
    "collectionAuthorityRecordPda",
    "collectionMint",
    "collectionMetadata",
    "editionAccount",
    "bubblegumSigner",
    "logWrapper",
    "compressionProgram",
    "tokenMetadataProgram",
    "systemProgram",
  ],
  setDecompressableState: ["treeAuthority", "treeCreator"],
  setDecompressibleState: ["treeAuthority", "treeCreator"],
  setTreeDelegate: [
    "treeAuthority",
    "treeCreator",
    "newTreeDelegate",
    "merkleTree",
    "systemProgram",
  ],
  transfer: [
    "treeAuthority",
    "leafOwner",
    "leafDelegate",
    "newLeafOwner",
    "merkleTree",
    "logWrapper",
    "compressionProgram",
    "systemProgram",
  ],
  unverifyCollection: [
    "treeAuthority",
    "leafOwner",
    "leafDelegate",
    "merkleTree",
    "payer",
    "treeDelegate",
    "collectionAuthority",
    "collectionAuthorityRecordPda",
    "collectionMint",
    "collectionMetadata",
    "editionAccount",
    "bubblegumSigner",
    "logWrapper",
    "compressionProgram",
    "tokenMetadataProgram",
    "systemProgram",
  ],
  unverifyCreator: [
    "treeAuthority",
    "leafOwner",
    "leafDelegate",
    "merkleTree",
    "payer",
    "creator",
    "logWrapper",
    "compressionProgram",
    "systemProgram",
  ],
  verifyCollection: [
    "treeAuthority",
    "leafOwner",
    "leafDelegate",
    "merkleTree",
    "payer",
    "treeDelegate",
    "collectionAuthority",
    "collectionAuthorityRecordPda",
    "collectionMint",
    "collectionMetadata",
    "editionAccount",
    "bubblegumSigner",
    "logWrapper",
    "compressionProgram",
    "tokenMetadataProgram",
    "systemProgram",
  ],
  verifyCreator: [
    "treeAuthority",
    "leafOwner",
    "leafDelegate",
    "merkleTree",
    "payer",
    "creator",
    "logWrapper",
    "compressionProgram",
    "systemProgram",
  ],
  updateMetadata: [
    "treeAuthority",
    "authority",
    "collectionMint",
    "collectionMetadata",
    "collectionAuthorityRecordPda",
    "leafOwner",
    "leafDelegate",
    "payer",
    "merkleTree",
    "logWrapper",
    "compressionProgram",
    "tokenMetadataProgram",
    "systemProgram",
  ],
} as const;

export function getBubblegumInstructionType(
  data: Uint8Array,
): keyof typeof INSTRUCTION_ACCOUNT_MAP {
  let hex = Buffer.from(data.slice(0, 8)).toString("hex");
  let instructionType = DISCRIMINATOR_LOOKUP[hex];
  if (!instructionType) throw new Error(`unknown discriminator ${hex}`);
  return instructionType;
}

export function parseInstruction(
  instructionData: Uint8Array,
  accountKeyIndexes: number[],
  accountKeys: MessageAccountKeys,
): ParsedInstructionResult | null {
  let instructionType = getBubblegumInstructionType(instructionData);
  let accounts = parseInstructionAccounts(
    INSTRUCTION_ACCOUNT_MAP[instructionType],
    accountKeyIndexes,
    accountKeys,
  );
  switch (instructionType) {
    case "burn":
      console.log("BURN");
      
      return {
        instructionType,
        accounts,
        data: getBurnInstructionDataSerializer().deserialize(
          instructionData,
        )[0],
      };
    case "transfer":
      console.log("TRANSFER");
      return {
        instructionType,
        accounts,
        data: getTransferInstructionDataSerializer().deserialize(
          instructionData,
        )[0],
      };
    case "mintToCollectionV1":
      console.log("MINT");
      return {
        instructionType,
        accounts,
        data: getMintToCollectionV1InstructionDataSerializer().deserialize(
          instructionData,
        )[0],
      };

    default:
      console.log("UNKNOWN");
      console.log(instructionType);
      return null;
  }
}

export function parseInstructionAccounts(
  accountNames: (typeof INSTRUCTION_ACCOUNT_MAP)[keyof typeof INSTRUCTION_ACCOUNT_MAP],
  accountKeyIndexes: number[],
  accountKeys: MessageAccountKeys,
) {
  let accounts = { remainingAccounts: [] };
  for (let i = 0; i < accountKeyIndexes.length; i++) {
    let key = accountKeys.get(accountKeyIndexes[i]);
    if (i < accountNames.length) accounts[accountNames[i]] = key;
    else accounts.remainingAccounts.push(key);
  }
  return accounts as { [key in (typeof accountNames)[number]]: PublicKey } & {
    remainingAccounts: PublicKey[];
  };
}

export async function handleBubblegumInstruction(
  instructionData: Uint8Array,
  accountKeyIndexes: number[],
  accountKeys: MessageAccountKeys,
  transaction: VersionedTransactionResponse,
  leafSchema: LeafSchema,
) {  
  let ix = parseInstruction(instructionData, accountKeyIndexes, accountKeys);
  if (!ix) return;

  let merkleTreeAddress;
  let assetId;
  switch (ix.instructionType) {
    case "burn":
      assetId = findLeafAssetIdPda(context, {
        merkleTree: publicKey(ix.accounts.merkleTree),
        leafIndex: (ix.data as BurnInstructionData).index,
      })[0];

      console.log(`Burned ${assetId}`);

      break;

    case "transfer":
      assetId = findLeafAssetIdPda(context, {
        merkleTree: publicKey(ix.accounts.merkleTree),
        leafIndex: (ix.data as TransferInstructionData).index,
      })[0];

      console.log(
        `Transferred ${assetId} from ${ix.accounts.leafOwner} to ${ix.accounts.newLeafOwner}`,
      );
      break;

    case "mintToCollectionV1":
      await handleMintToCollectionV1Instruction(transaction, leafSchema, ix)
      break;
  }
}
