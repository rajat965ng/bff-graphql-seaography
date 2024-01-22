use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "employee")]
#[graphql(complex)]
#[graphql(name = "Employee")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub emp_no: i32,
    pub birth_date: Date,
    #[sea_orm(column_type = "Text")]
    pub first_name: String,
    #[sea_orm(column_type = "Text")]
    pub last_name: String,
    #[sea_orm(column_type = "Text")]
    pub gender: String,
    pub hire_date: Date,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {
    #[sea_orm(has_many = "super::dept_emp::Entity")]
    DeptEmp,
    #[sea_orm(has_many = "super::dept_manager::Entity")]
    DeptManager,
    #[sea_orm(has_many = "super::salary::Entity")]
    Salary,
    #[sea_orm(has_many = "super::title::Entity")]
    Title,
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

impl Related<super::salary::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Salary.def()
    }
}

impl Related<super::title::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Title.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
