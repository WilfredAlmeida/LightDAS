-- CreateEnum
CREATE TYPE "chain_mutability" AS ENUM ('unknown', 'mutable', 'immutable');

-- CreateEnum
CREATE TYPE "mutability" AS ENUM ('unknown', 'mutable', 'immutable');

-- CreateEnum
CREATE TYPE "owner_type" AS ENUM ('unknown', 'token', 'single');

-- CreateEnum
CREATE TYPE "royalty_target_type" AS ENUM ('unknown', 'creators', 'fanout', 'single');

-- CreateEnum
CREATE TYPE "specification_asset_class" AS ENUM ('unknown', 'FUNGIBLE_TOKEN', 'FUNGIBLE_ASSET', 'NFT', 'PRINTABLE_NFT', 'PRINT', 'TRANSFER_RESTRICTED_NFT', 'NON_TRANSFERABLE_NFT', 'IDENTITY_NFT');

-- CreateEnum
CREATE TYPE "specification_versions" AS ENUM ('unknown', 'v0', 'v1', 'v2');

-- CreateEnum
CREATE TYPE "v1_account_attachments" AS ENUM ('unknown', 'edition', 'master_edition_v2', 'master_edition_v1', 'edition_marker');

-- CreateTable
CREATE TABLE "asset" (
    "id" BYTEA NOT NULL,
    "alt_id" BYTEA,
    "specification_version" "specification_versions" NOT NULL,
    "specification_asset_class" "specification_asset_class" NOT NULL,
    "owner" BYTEA,
    "owner_type" "owner_type" NOT NULL DEFAULT 'single',
    "delegate" BYTEA,
    "frozen" BOOLEAN NOT NULL DEFAULT false,
    "supply" BIGINT NOT NULL DEFAULT 1,
    "supply_mint" BYTEA,
    "compressed" BOOLEAN NOT NULL DEFAULT false,
    "compressible" BOOLEAN NOT NULL DEFAULT false,
    "seq" BIGINT NOT NULL,
    "tree_id" BYTEA,
    "leaf" BYTEA,
    "nonce" BIGINT NOT NULL,
    "royalty_target_type" "royalty_target_type" NOT NULL DEFAULT 'creators',
    "royalty_target" BYTEA,
    "royalty_amount" INTEGER NOT NULL DEFAULT 0,
    "asset_data" BYTEA,
    "created_at" TIMESTAMPTZ(6) DEFAULT (now() AT TIME ZONE 'utc'::text),
    "burnt" BOOLEAN NOT NULL DEFAULT false,
    "slot_updated" BIGINT NOT NULL,

    CONSTRAINT "asset_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "asset_authority" (
    "id" BIGSERIAL NOT NULL,
    "asset_id" BYTEA NOT NULL,
    "scopes" TEXT[],
    "authority" BYTEA NOT NULL,
    "seq" BIGINT NOT NULL,
    "slot_updated" BIGINT NOT NULL,

    CONSTRAINT "asset_authority_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "asset_creators" (
    "id" BIGSERIAL NOT NULL,
    "asset_id" BYTEA NOT NULL,
    "creator" BYTEA NOT NULL,
    "share" INTEGER NOT NULL DEFAULT 0,
    "verified" BOOLEAN NOT NULL DEFAULT false,
    "seq" BIGINT NOT NULL,
    "slot_updated" BIGINT NOT NULL,

    CONSTRAINT "asset_creators_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "asset_data" (
    "id" BYTEA NOT NULL,
    "chain_data_mutability" "chain_mutability" NOT NULL DEFAULT 'mutable',
    "chain_data" JSONB NOT NULL,
    "metadata_url" VARCHAR(200) NOT NULL,
    "metadata_mutability" "mutability" NOT NULL DEFAULT 'mutable',
    "metadata" JSONB NOT NULL,
    "slot_updated" BIGINT NOT NULL,

    CONSTRAINT "asset_data_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "asset_grouping" (
    "id" BIGSERIAL NOT NULL,
    "asset_id" BYTEA NOT NULL,
    "group_key" TEXT NOT NULL,
    "group_value" TEXT NOT NULL,
    "seq" BIGINT NOT NULL,
    "slot_updated" BIGINT NOT NULL,

    CONSTRAINT "asset_grouping_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "asset_v1_account_attachments" (
    "id" BYTEA NOT NULL,
    "asset_id" BYTEA,
    "attachment_type" "v1_account_attachments" NOT NULL,
    "initialized" BOOLEAN NOT NULL DEFAULT false,
    "data" JSONB,
    "slot_updated" BIGINT NOT NULL,

    CONSTRAINT "asset_v1_account_attachments_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "backfill_items" (
    "id" BIGSERIAL NOT NULL,
    "tree" BYTEA NOT NULL,
    "seq" BIGINT NOT NULL,
    "slot" BIGINT NOT NULL,
    "force_chk" BOOLEAN NOT NULL,
    "backfilled" BOOLEAN NOT NULL,

    CONSTRAINT "backfill_items_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "cl_items" (
    "id" BIGSERIAL NOT NULL,
    "tree" BYTEA NOT NULL,
    "node_idx" BIGINT NOT NULL,
    "leaf_idx" BIGINT,
    "seq" BIGINT NOT NULL,
    "level" BIGINT NOT NULL,
    "hash" BYTEA NOT NULL,

    CONSTRAINT "cl_items_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "raw_txn" (
    "signature" VARCHAR(64) NOT NULL,
    "slot" BIGINT NOT NULL,
    "processed" BOOLEAN NOT NULL,

    CONSTRAINT "raw_txn_pkey" PRIMARY KEY ("signature")
);

-- CreateTable
CREATE TABLE "token_accounts" (
    "pubkey" BYTEA NOT NULL,
    "mint" BYTEA NOT NULL,
    "amount" BIGINT NOT NULL DEFAULT 0,
    "owner" BYTEA NOT NULL,
    "frozen" BOOLEAN NOT NULL DEFAULT false,
    "close_authority" BYTEA,
    "delegate" BYTEA,
    "delegated_amount" BIGINT NOT NULL DEFAULT 0,
    "slot_updated" BIGINT NOT NULL,
    "token_program" BYTEA NOT NULL,

    CONSTRAINT "token_accounts_pkey" PRIMARY KEY ("pubkey")
);

-- CreateTable
CREATE TABLE "tokens" (
    "mint" BYTEA NOT NULL,
    "supply" BIGINT NOT NULL DEFAULT 0,
    "decimals" INTEGER NOT NULL DEFAULT 0,
    "token_program" BYTEA NOT NULL,
    "mint_authority" BYTEA,
    "freeze_authority" BYTEA,
    "close_authority" BYTEA,
    "extension_data" BYTEA,
    "slot_updated" BIGINT NOT NULL,

    CONSTRAINT "tokens_pkey" PRIMARY KEY ("mint")
);

-- CreateIndex
CREATE INDEX "asset_delegate" ON "asset"("delegate");

-- CreateIndex
CREATE INDEX "asset_leaf" ON "asset"("leaf");

-- CreateIndex
CREATE INDEX "asset_owner" ON "asset"("owner");

-- CreateIndex
CREATE INDEX "asset_revision" ON "asset"("tree_id", "leaf", "nonce");

-- CreateIndex
CREATE INDEX "asset_tree" ON "asset"("tree_id");

-- CreateIndex
CREATE INDEX "asset_tree_leaf" ON "asset"("tree_id", "leaf");

-- CreateIndex
CREATE UNIQUE INDEX "asset_authority_asset_id" ON "asset_authority"("asset_id");

-- CreateIndex
CREATE INDEX "asset_authority_idx" ON "asset_authority"("asset_id", "authority");

-- CreateIndex
CREATE UNIQUE INDEX "asset_creators_asset_id" ON "asset_creators"("asset_id");

-- CreateIndex
CREATE INDEX "asset_creator" ON "asset_creators"("asset_id", "creator");

-- CreateIndex
CREATE INDEX "asset_verified_creator" ON "asset_creators"("asset_id", "verified");

-- CreateIndex
CREATE INDEX "slot_updated_idx" ON "asset_data"("slot_updated");

-- CreateIndex
CREATE UNIQUE INDEX "asset_grouping_asset_id" ON "asset_grouping"("asset_id");

-- CreateIndex
CREATE INDEX "asset_grouping_key" ON "asset_grouping"("group_key", "group_value");

-- CreateIndex
CREATE INDEX "asset_grouping_value" ON "asset_grouping"("group_key", "asset_id");

-- CreateIndex
CREATE INDEX "backfill_items_backfilled_idx" ON "backfill_items"("backfilled");

-- CreateIndex
CREATE INDEX "backfill_items_force_chk_idx" ON "backfill_items"("force_chk");

-- CreateIndex
CREATE INDEX "backfill_items_seq_idx" ON "backfill_items"("seq");

-- CreateIndex
CREATE INDEX "backfill_items_slot_idx" ON "backfill_items"("slot");

-- CreateIndex
CREATE INDEX "backfill_items_tree_backfilled_idx" ON "backfill_items"("tree", "backfilled");

-- CreateIndex
CREATE INDEX "backfill_items_tree_force_chk_idx" ON "backfill_items"("tree", "force_chk");

-- CreateIndex
CREATE INDEX "backfill_items_tree_idx" ON "backfill_items"("tree");

-- CreateIndex
CREATE INDEX "backfill_items_tree_seq_idx" ON "backfill_items"("tree", "seq");

-- CreateIndex
CREATE INDEX "backfill_items_tree_slot_idx" ON "backfill_items"("tree", "slot");

-- CreateIndex
CREATE INDEX "cl_items_hash_idx" ON "cl_items"("hash");

-- CreateIndex
CREATE INDEX "cl_items_leaf_idx" ON "cl_items"("leaf_idx");

-- CreateIndex
CREATE INDEX "cl_items_level" ON "cl_items"("level");

-- CreateIndex
CREATE INDEX "cl_items_node_idx" ON "cl_items"("node_idx");

-- CreateIndex
CREATE INDEX "cl_items_tree_idx" ON "cl_items"("tree");

-- CreateIndex
CREATE UNIQUE INDEX "cl_items__tree_node" ON "cl_items"("tree", "node_idx");

-- CreateIndex
CREATE INDEX "raw_slot" ON "raw_txn"("slot");

-- CreateIndex
CREATE INDEX "ta_amount" ON "token_accounts"("amount");

-- CreateIndex
CREATE INDEX "ta_amount_del" ON "token_accounts"("delegated_amount");

-- CreateIndex
CREATE INDEX "ta_delegate" ON "token_accounts"("delegate");

-- CreateIndex
CREATE INDEX "ta_mint" ON "token_accounts"("mint");

-- CreateIndex
CREATE INDEX "ta_slot_updated_idx" ON "token_accounts"("slot_updated");

-- CreateIndex
CREATE INDEX "t_close_auth" ON "tokens"("close_authority");

-- CreateIndex
CREATE INDEX "t_decimals" ON "tokens"("decimals");

-- CreateIndex
CREATE INDEX "t_freeze_auth" ON "tokens"("freeze_authority");

-- CreateIndex
CREATE INDEX "t_mint_auth" ON "tokens"("mint_authority");

-- CreateIndex
CREATE INDEX "t_slot_updated_idx" ON "tokens"("slot_updated");

-- CreateIndex
CREATE INDEX "t_supply" ON "tokens"("supply");

-- AddForeignKey
ALTER TABLE "asset" ADD CONSTRAINT "asset_asset_data_fkey" FOREIGN KEY ("asset_data") REFERENCES "asset_data"("id") ON DELETE NO ACTION ON UPDATE NO ACTION;

-- AddForeignKey
ALTER TABLE "asset_authority" ADD CONSTRAINT "asset_authority_asset_id_fkey" FOREIGN KEY ("asset_id") REFERENCES "asset"("id") ON DELETE NO ACTION ON UPDATE NO ACTION;

-- AddForeignKey
ALTER TABLE "asset_creators" ADD CONSTRAINT "asset_creators_asset_id_fkey" FOREIGN KEY ("asset_id") REFERENCES "asset"("id") ON DELETE NO ACTION ON UPDATE NO ACTION;

-- AddForeignKey
ALTER TABLE "asset_grouping" ADD CONSTRAINT "asset_grouping_asset_id_fkey" FOREIGN KEY ("asset_id") REFERENCES "asset"("id") ON DELETE NO ACTION ON UPDATE NO ACTION;

-- AddForeignKey
ALTER TABLE "asset_v1_account_attachments" ADD CONSTRAINT "asset_v1_account_attachments_asset_id_fkey" FOREIGN KEY ("asset_id") REFERENCES "asset"("id") ON DELETE NO ACTION ON UPDATE NO ACTION;

