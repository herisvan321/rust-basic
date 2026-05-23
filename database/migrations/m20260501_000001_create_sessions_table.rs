use rustbasic_core::sea_orm_migration::prelude::*;
use rustbasic_core::async_trait;
use rustbasic_core::Schema;

#[derive(Iden)]
pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260501_000001_create_sessions_table"
    }
}

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Schema::create(manager, "sessions", |table| {
            table.string("id").primary_key();
            table.string("user_id").nullable();
            table.string("ip_address").nullable();
            table.text("user_agent").nullable();
            table.text("payload").not_null();
            table.integer("last_activity").not_null().index();
        }).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Schema::drop(manager, "sessions").await
    }
}
