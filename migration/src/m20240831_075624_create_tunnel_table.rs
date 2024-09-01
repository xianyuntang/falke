use crate::m20240828_174932_create_user_table::User;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
enum Tunnel {
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
                    .table(Tunnel::Table)
                    .if_not_exists()
                    .col(string(Tunnel::Id).primary_key().string_len(21).not_null())
                    .col(timestamp_with_time_zone(Tunnel::CreatedAt).default(Expr::cust("now()")))
                    .col(timestamp_with_time_zone(Tunnel::UpdatedAt).default(Expr::cust("now()")))
                    .col(string(Tunnel::UserId).string_len(21).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_tunnel_user_id")
                            .from(Tunnel::Table, Tunnel::UserId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Tunnel::Table).to_owned())
            .await
    }
}
