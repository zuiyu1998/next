use next_entity::popularity::{Column, Entity};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Column::UserId)
                            .integer()
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Column::Level).integer().not_null())
                    .col(ColumnDef::new(Column::Count).integer().not_null())
                    .col(ColumnDef::new(Column::NextNeedCount).integer().not_null())
                    .col(ColumnDef::new(Column::Name).char_len(20).not_null())
                    .col(
                        ColumnDef::new(Column::LevelTemplateName)
                            .char_len(100)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Column::CreateAt).date_time().not_null())
                    .col(ColumnDef::new(Column::UpdateAt).date_time().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Entity).to_owned())
            .await
    }
}
