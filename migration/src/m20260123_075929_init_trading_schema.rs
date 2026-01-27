use sea_orm_migration::prelude::{extension::postgres::Type, *};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("CREATE EXTENSION IF NOT EXISTS pgcrypto")
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(OrderType::Type)
                    .values([OrderType::ASK, OrderType::BID])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(User::Name).string().not_null())
                    .col(ColumnDef::new(User::Email).string().not_null().unique_key())
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(User::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Asset::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Asset::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(Asset::AssetName).string().not_null())
                    .col(ColumnDef::new(Asset::DisplayName).string().not_null())
                    .col(
                        ColumnDef::new(Asset::AssetTotalAmount)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Asset::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Asset::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Order::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Order::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(
                        ColumnDef::new(Order::OrderType)
                            .enumeration(OrderType::Type, [OrderType::ASK, OrderType::BID])
                            .not_null(),
                    )
                    .col(ColumnDef::new(Order::Value).big_integer().not_null())
                    .col(ColumnDef::new(Order::AssetId).uuid().not_null())
                    .col(ColumnDef::new(Order::UserId).uuid().not_null())
                    .col(
                        ColumnDef::new(Order::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Order::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_orders_user")
                            .from(Order::Table, Order::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_order_asset")
                            .from(Order::Table, Order::AssetId)
                            .to(Asset::Table, Asset::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Balance::Table)
                    .col(
                        ColumnDef::new(Balance::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(Balance::UserId).uuid().not_null())
                    .col(ColumnDef::new(Balance::AssetId).uuid().not_null())
                    .col(
                        ColumnDef::new(Balance::AmountOwned)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Balance::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Balance::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_balance_user")
                            .from(Balance::Table, Balance::UserId) 
                            .to(User::Table, User::Id) 
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_balance_asset")
                            .from(Balance::Table, Balance::AssetId) 
                            .to(Asset::Table, Asset::Id) 
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("uniq_user_asset_balance")
                            .col(Balance::UserId)
                            .col(Balance::AssetId)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Order::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Balance::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Asset::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(OrderType::Type).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum OrderType {
    Type,
    ASK,
    BID,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Name,
    Email,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Asset {
    Table,
    Id,
    AssetName,
    DisplayName,
    AssetTotalAmount,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Order {
    Table,
    Id,
    OrderType,
    Value,
    AssetId,
    UserId,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Balance {
    Table,
    Id,
    UserId,
    AssetId,
    AmountOwned,
    CreatedAt,
    UpdatedAt,
}
