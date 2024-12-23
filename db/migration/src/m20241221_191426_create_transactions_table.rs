use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Transactions::Table)
                    .if_not_exists()
                    .col(pk_auto(Transactions::Id))
                    .col(integer(Transactions::FromAccountId))
                    .col(integer(Transactions::ToAccountId))
                    .col(decimal(Transactions::Amount))
                    .col(date(Transactions::CreatedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Transactions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Transactions {
    Table,
    Id,
    FromAccountId,
    ToAccountId,
    Amount,
    CreatedAt,
}
