use rustbasic_core::sea_orm_migration::prelude::*;
use rustbasic_core::async_trait;
use rustbasic_core::Schema;

#[derive(Iden)]
pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260501_000002_create_users_table"
    }
}

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Schema::create(manager, "users", |table| {
            table.id();
            table.string("name").not_null();
            table.string("email").not_null().unique().index();
            table.date_time("email_verified_at").nullable();
            table.string("password").not_null();
            table.string("remember_token").nullable();
            table.timestamps();
        }).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Schema::drop(manager, "users").await
    }
}
