use crate::m20240828_174932_create_user_table::User;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
enum Proxy {
    Table,
    Id,
    CreatedAt,
    UpdatedAt,
    UserId,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Proxy::Table)
                    .if_not_exists()
                    .col(string(Proxy::Id).primary_key().string_len(255))
                    .col(timestamp_with_time_zone(Proxy::CreatedAt).default(Expr::cust("now()")))
                    .col(timestamp_with_time_zone(Proxy::UpdatedAt).default(Expr::cust("now()")))
                    .col(string(Proxy::UserId).string_len(21))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_proxy_user_id")
                            .from(Proxy::Table, Proxy::UserId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Proxy::Table).to_owned())
            .await
    }
}
