import {
    findLeafAssetIdPda,
    getBurnInstructionDataSerializer,
    getTransferInstructionDataSerializer,
  } from "@metaplex-foundation/mpl-bubblegum";
  import { publicKey } from "@metaplex-foundation/umi";
  import { MessageAccountKeys, PublicKey } from "@solana/web3.js";
  
  import { context } from "../rpc/rpc";
  
  export const BUBBLEGUM_PROGRAM_ID = new PublicKey("BGUMAp9Gq7iTEuizy4pqaxsTyUCBK68MDfK752saRPUY");
  
  export const TRANSFER_DISCRIMINATOR = new Uint8Array([163, 52, 200, 231, 140, 3, 69, 186]);
  export const BURN_DISCRIMINATOR = new Uint8Array([116, 110, 29, 56, 107, 219, 42, 93]);
  
  export const matchInstructionDiscriminator = (data: Uint8Array, discriminator: Uint8Array) => {
    for (let i = 0; i < discriminator.length; i++) {
      if (data[i] !== discriminator[i]) {
        return false;
      }
    }
    return true;
  };
  
  export const isBubblegumInstruction = (programIdIndex: number, accountKeys: MessageAccountKeys) => {
    return accountKeys.get(programIdIndex)?.toBase58() === BUBBLEGUM_PROGRAM_ID.toBase58();
  };
  
  export const isBubblegumTransferInstruction = (
    programIdIndex: number,
    instructionData: Uint8Array,
    accountKeys: MessageAccountKeys
  ) =>
    isBubblegumInstruction(programIdIndex, accountKeys) &&
    matchInstructionDiscriminator(instructionData, TRANSFER_DISCRIMINATOR);
  
  export const isBubblegumBurnInstruction = (
    programIdIndex: number,
    instructionData: Uint8Array,
    accountKeys: MessageAccountKeys
  ) =>
    isBubblegumInstruction(programIdIndex, accountKeys) &&
    matchInstructionDiscriminator(instructionData, BURN_DISCRIMINATOR);
  
  export const handleBubblegumTransferInstruction = async (
    instructionData: Uint8Array,
    accountKeyIndexes: number[],
    accountKeys: MessageAccountKeys
  ) => {
    const [transferInstructionData] = getTransferInstructionDataSerializer().deserialize(instructionData);
  
    const merkleTreeAddress = accountKeys.get(accountKeyIndexes[4]);
  
    if (!merkleTreeAddress) throw new Error("Merkle Tree Address not found");
  
    const assetId = findLeafAssetIdPda(context, {
      merkleTree: publicKey(merkleTreeAddress),
      leafIndex: transferInstructionData.index,
    })[0];
  
    const leafOwnerAddress = accountKeys.get(accountKeyIndexes[2]);
    const newLeafOwnerAddress = accountKeys.get(accountKeyIndexes[3]);
  
    if (!newLeafOwnerAddress) throw new Error("New Leaf Owner Address not found");
  
    console.log(
      `Transferred ${assetId} from ${leafOwnerAddress?.toBase58()} to ${newLeafOwnerAddress.toBase58()}`
    );
  };
  
  export const handleBubblegumBurnInstruction = async (
    instructionData: Uint8Array,
    accountKeyIndexes: number[],
    accountKeys: MessageAccountKeys
  ) => {
    const [burnInstructionData] = getBurnInstructionDataSerializer().deserialize(instructionData);
  
    const merkleTreeAddress = accountKeys.get(accountKeyIndexes[3]);
  
    if (!merkleTreeAddress) throw new Error("Merkle Tree Address not found");
  
    const assetId = findLeafAssetIdPda(context, {
      merkleTree: publicKey(merkleTreeAddress),
      leafIndex: burnInstructionData.index,
    })[0];
  
    console.log(`Burned ${assetId}`);
  };