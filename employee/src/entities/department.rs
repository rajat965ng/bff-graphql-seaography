use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "department")]
#[graphql(complex)]
#[graphql(name = "Department")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub dept_no: String,
    #[sea_orm(column_type = "Text", unique)]
    pub dept_name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {
    #[sea_orm(has_many = "super::dept_emp::Entity")]
    DeptEmp,
    #[sea_orm(has_many = "super::dept_manager::Entity")]
    DeptManager,
}

impl Related<super::dept_emp::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DeptEmp.def()
    }
}

impl Related<super::dept_manager::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DeptManager.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
