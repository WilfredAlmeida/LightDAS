use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(MerkleTrees::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MerkleTrees::Address)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(MerkleTrees::Capacity).integer().null())
                    .col(ColumnDef::new(MerkleTrees::CanopyDepth).integer().null())
                    .col(ColumnDef::new(MerkleTrees::Network).string().null())
                    .col(ColumnDef::new(MerkleTrees::NumMinted).integer().null())
                    .col(ColumnDef::new(MerkleTrees::Signature).string().null())
                    .col(
                        ColumnDef::new(MerkleTrees::CreatedAt)
                            .date_time()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp))
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(MerkleTrees::UpdatedAt)
                            .date_time()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp))
                            .not_null(),
                    )
                    .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(MerkleTrees::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum MerkleTrees {
    Table,
    Address,
    Capacity,
    Network,
    MaxDepth,
    MaxBufferSize,
    CanopyDepth,
    NumMinted,
    Signature,
    CreatedAt,
    UpdatedAt
}
