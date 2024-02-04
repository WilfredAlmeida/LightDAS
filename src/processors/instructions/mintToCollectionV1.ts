import {
  LeafSchema,
  MintToCollectionV1InstructionArgs,
  MintToCollectionV1InstructionData,
  getCollectionSerializer,
  getMetadataArgsSerializer,
} from "@metaplex-foundation/mpl-bubblegum";
import { PublicKey, VersionedTransactionResponse } from "@solana/web3.js";
import { ParsedInstructionResult, INSTRUCTION_ACCOUNT_MAP } from "../bubblegum";
import { dasDb } from "src/config/db";
import {
  Prisma,
  chain_mutability,
  mutability,
  owner_type,
  royalty_target_type,
  specification_asset_class,
  specification_versions,
} from "@prisma/client";
import { some, unwrapOption } from "@metaplex-foundation/umi";
import { downloadMetadata } from "src/helpers.ts/downloadMetadata";

export const handleMintToCollectionV1Instruction = async (
  transaction: VersionedTransactionResponse,
  leafSchema: LeafSchema,
  parsedInstructionResult: ParsedInstructionResult
) => {
  const instructionData =
    parsedInstructionResult.data as MintToCollectionV1InstructionData;
  const accounts =
    parsedInstructionResult.accounts as unknown as (typeof INSTRUCTION_ACCOUNT_MAP)["mintToCollectionV1"];
  const collection = unwrapOption(instructionData.metadata.collection);

  const assetId = new PublicKey(leafSchema.id).toBuffer();

  const downloadedMetadata = await downloadMetadata(
    instructionData.metadata.uri
  );

  const dbOperations: any[] = [
    // Upsert `asset` table base info.
    dasDb.asset_data.upsert({
      where: { id: assetId },
      create: {
        id: assetId,
        chain_data_mutability: instructionData.metadata.isMutable
          ? chain_mutability.mutable
          : chain_mutability.immutable,
        chain_data: instructionData.metadata as Prisma.JsonValue,
        metadata_url: instructionData.metadata.uri,
        metadata_mutability: mutability.mutable,
        // metadata: "processing",
        metadata: {
          name: instructionData.metadata.name,
          symbol: instructionData.metadata.symbol,
          uri: instructionData.metadata.uri,
          sellerFeeBasisPoints: instructionData.metadata.sellerFeeBasisPoints,
          primarySaleHappened: instructionData.metadata.primarySaleHappened,
          mutable: instructionData.metadata.isMutable,
          editionNonce: some(instructionData.metadata.editionNonce),
        },
        slot_updated: transaction.slot,
      },
      update: {
        // id: assetId,
        chain_data_mutability: instructionData.metadata.isMutable
          ? chain_mutability.mutable
          : chain_mutability.immutable,
        chain_data: instructionData.metadata as Prisma.JsonValue,
        metadata_url: instructionData.metadata.uri,
        metadata_mutability: mutability.mutable,
        metadata: downloadedMetadata ?? "processing",
        slot_updated: transaction.slot,
      },
    }),

    // Upsert `asset` table base info and `asset_creators` table.
    dasDb.asset.upsert({
      where: { id: assetId },
      create: {
        id: assetId,
        owner_type: owner_type.single,
        frozen: false,
        specification_version: specification_versions.v1,
        specification_asset_class: specification_asset_class.NFT,
        royalty_target_type: royalty_target_type.creators,
        royalty_target: null,
        royalty_amount: instructionData.metadata.sellerFeeBasisPoints,
        slot_updated: transaction.slot,
        seq: 9999,
        nonce: leafSchema.nonce,
        asset_data: assetId,
      },
      update: {
        // id: assetId,
        owner_type: owner_type.single,
        frozen: false,
        specification_version: specification_versions.v1,
        specification_asset_class: specification_asset_class.NFT,
        royalty_target_type: royalty_target_type.creators,
        royalty_target: null,
        royalty_amount: instructionData.metadata.sellerFeeBasisPoints,
        slot_updated: transaction.slot,
        seq: 9999,
        asset_data: assetId,
      },
    }),

    // Partial update of asset table with just compression info elements.
    dasDb.asset.update({
      where: { id: assetId },
      data: {
        compressed: true,
        compressible: false,
        supply: 1,
        // supply_mint will get set to default value
      },
    }),

    // Partial update of asset table with just leaf.
    dasDb.asset.update({
      where: { id: assetId },
      data: {
        nonce: leafSchema.nonce,
        tree_id: parsedInstructionResult.accounts.merkleTree.toBuffer(), // this might be wrong
        leaf: assetId,
        // asset_hash & creator_hash are not there in the db schema but present in the das code
      },
    }),

    // Partial update of asset table with just leaf owner and delegate.
    dasDb.asset.update({
      where: { id: assetId },
      data: {
        owner: parsedInstructionResult.accounts.leafOwner.toBuffer(),
        delegate: new PublicKey(
          parsedInstructionResult.accounts.leafDelegate
        ).toBuffer(),
      },
    }),

    // Upsert creators to `asset_creators` table.
    // Done in a loop below because there can be multiple creators.

    // Insert into `asset_authority` table.
    dasDb.asset_authority.upsert({
      where: { asset_id: assetId },
      create: {
        asset_id: assetId,
        authority: parsedInstructionResult.accounts.treeAuthority.toBuffer(),
        slot_updated: transaction.slot,
        seq: 9999,
      },
      update: {},
    }),

    // Upsert into `asset_grouping` table with base collection info.
    dasDb.asset_grouping.upsert({
      where: {
        asset_id_group_key: {
          asset_id: assetId,
          group_key: "collection",
        },
      },
      create: {
        asset_id: assetId,
        group_key: "collection",
        group_value: parsedInstructionResult.accounts.collectionMint.toString(),
        verified: collection.verified,
        slot_updated: transaction.slot,
        group_info_seq: 9999,
      },
      update: {
        asset_id: assetId,
        group_value: parsedInstructionResult.accounts.collectionMint.toString(),
        verified: collection.verified,
        slot_updated: transaction.slot,
        group_info_seq: 9999,
      },
    }),
  ];

  if (instructionData.metadata.creators.length > 0) {
    instructionData.metadata.creators.forEach(async (creator, i) => {
      dasDb.asset_creators.upsert({
        where: { asset_id_position: { asset_id: assetId, position: i } },
        create: {
          asset_id: assetId,
          position: i,
          creator: new PublicKey(creator.address).toBuffer(),
          share: creator.share,
          verified: creator.verified,
          slot_updated: transaction.slot,
          seq: 9999,
        },
        update: {
          asset_id: assetId,
          position: i,
          creator: new PublicKey(creator.address).toBuffer(),
          share: creator.share,
          verified: creator.verified,
          slot_updated: transaction.slot,
          seq: 9999,
        },
      });
    });
  } else {
    // If creators are empty, insert an empty creator with the current sequence.
    // This prevents accidental errors during out-of-order updates.
    dbOperations.push(
      dasDb.asset_creators.upsert({
        where: { asset_id_position: { asset_id: assetId, position: 0 } },
        create: {
          asset_id: assetId,
          position: 0,
          creator: Buffer.from([]),
          share: 100,
          verified: false,
          slot_updated: transaction.slot,
          seq: 9999,
        },
        update: {
          asset_id: assetId,
          position: 0,
          creator: Buffer.from([]),
          share: 100,
          verified: false,
          slot_updated: transaction.slot,
          seq: 9999,
        },
      })
    );
  }

  const res = await dasDb.$transaction(dbOperations);
};
