use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "title")]
#[graphql(complex)]
#[graphql(name = "Title")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub emp_no: i32,
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub title: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub from_date: Date,
    pub to_date: Option<Date>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::employee::Entity",
        from = "Column::EmpNo",
        to = "super::employee::Column::EmpNo",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Employee,
}

impl Related<super::employee::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Employee.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
