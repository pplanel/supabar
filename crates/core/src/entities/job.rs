use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "jobs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    name: String,
    node_id: i32,
    action: i32,
    status: i32,

    task_count: i32,
    completed_task_count: i32,
    date_created: chrono::DateTime<chrono::Utc>,
    date_modified: chrono::DateTime<chrono::Utc>,
    seconds_elapsed: i32,
}
#[derive(Clone, Copy, Debug, EnumIter)]
pub enum Relation {}
impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        todo!()
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        ActiveModelTrait::default()
    }

    fn before_save(self, insert: bool) -> Result<Self, DbErr> {
        Ok(self)
    }

    fn after_save(model: Model, insert: bool) -> Result<Model, DbErr> {
        Ok(model)
    }

    fn before_delete(self) -> Result<Self, DbErr> {
        Ok(self)
    }

    fn after_delete(self) -> Result<Self, DbErr> {
        Ok(self)
    }
}
