use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
pub enum User {
    Table,
    Id,
    Email,
    Password,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(string(User::Id).primary_key().string_len(21).not_null())
                    .col(string(User::Email).string_len(255).not_null())
                    .col(string(User::Password).string_len(60).not_null())
                    .col(timestamp_with_time_zone(User::CreatedAt).default(Expr::cust("now()")))
                    .col(timestamp_with_time_zone(User::UpdatedAt).default(Expr::cust("now()")))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}
