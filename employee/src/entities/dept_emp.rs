use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "dept_emp")]
#[graphql(complex)]
#[graphql(name = "DeptEmp")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub emp_no: i32,
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub dept_no: String,
    pub from_date: Date,
    pub to_date: Date,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::department::Entity",
        from = "Column::DeptNo",
        to = "super::department::Column::DeptNo",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Department,
    #[sea_orm(
        belongs_to = "super::employee::Entity",
        from = "Column::EmpNo",
        to = "super::employee::Column::EmpNo",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Employee,
}

impl Related<super::department::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Department.def()
    }
}

impl Related<super::employee::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Employee.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
