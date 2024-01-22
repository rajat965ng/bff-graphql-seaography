#[derive(Debug, seaography::macros::QueryRoot)]
#[seaography(entity = "crate::entities::department")]
#[seaography(entity = "crate::entities::dept_emp")]
#[seaography(entity = "crate::entities::dept_manager")]
#[seaography(entity = "crate::entities::employee")]
#[seaography(entity = "crate::entities::salary")]
#[seaography(entity = "crate::entities::title")]
pub struct QueryRoot;
