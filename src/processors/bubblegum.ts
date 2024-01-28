import {
  findLeafAssetIdPda,
  getBurnInstructionDataSerializer,
  getTransferInstructionDataSerializer,
} from "@metaplex-foundation/mpl-bubblegum";
import { publicKey } from "@metaplex-foundation/umi";
import { MessageAccountKeys, PublicKey } from "@solana/web3.js";

import { context } from "../rpc/rpc";

export const BUBBLEGUM_PROGRAM_ID = new PublicKey(
  "BGUMAp9Gq7iTEuizy4pqaxsTyUCBK68MDfK752saRPUY",
);

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
};

export const DISCRIMINATOR_LOOKUP = Object.fromEntries(
  Object.entries(DISCRIMINATORS).map(([k, v]) => [
    Buffer.from(v).toString("hex"),
    k,
  ]),
);

export function getBubblegumInstructionType(data: Uint8Array): string {
  let hex = Buffer.from(data.slice(0, 6)).toString("hex");
  return DISCRIMINATOR_LOOKUP[hex] ?? `unknown discriminator ${hex}`;
}

export function parseInstruction(
  instructionData: Uint8Array,
  instructionType?: string,
) {
  instructionType ??= getBubblegumInstructionType(instructionData);
  switch (instructionType) {
    case "burn":
      return getBurnInstructionDataSerializer().deserialize(instructionData)[0];
    case "transfer":
      return getTransferInstructionDataSerializer().deserialize(
        instructionData,
      )[0];

    default:
      return null;
  }
}

export function handleBubblegumInstruction(
  instructionData: Uint8Array,
  accountKeyIndexes: number[],
  accountKeys: MessageAccountKeys,
) {
  let instructionType = getBubblegumInstructionType(instructionData);
  let parsedInstruction = parseInstruction(instructionData, instructionType);
  if (!parsedInstruction) return;

  let merkleTreeAddress;
  let assetId;
  switch (instructionType) {
    case "burn":
      merkleTreeAddress = accountKeys.get(accountKeyIndexes[3]);

      if (!merkleTreeAddress) throw new Error("Merkle Tree Address not found");

      assetId = findLeafAssetIdPda(context, {
        merkleTree: publicKey(merkleTreeAddress),
        leafIndex: parsedInstruction.index,
      })[0];

      console.log(`Burned ${assetId}`);

      break;

    case "transfer":
      merkleTreeAddress = accountKeys.get(accountKeyIndexes[4]);

      if (!merkleTreeAddress) throw new Error("Merkle Tree Address not found");

      assetId = findLeafAssetIdPda(context, {
        merkleTree: publicKey(merkleTreeAddress),
        leafIndex: parsedInstruction.index,
      })[0];

      const leafOwnerAddress = accountKeys.get(accountKeyIndexes[2]);
      const newLeafOwnerAddress = accountKeys.get(accountKeyIndexes[3]);

      if (!newLeafOwnerAddress)
        throw new Error("New Leaf Owner Address not found");

      console.log(
        `Transferred ${assetId} from ${leafOwnerAddress?.toBase58()} to ${newLeafOwnerAddress.toBase58()}`,
      );
      break;
  }
}