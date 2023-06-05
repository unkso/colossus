use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "ranks")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub grade_id: i32,
    #[sea_orm(column_type = "Custom(\"USER-DEFINED\".to_owned())", nullable)]
    pub modifier: Option<String>,
    pub name: String,
    pub abbreviation: String,
    pub image_uri: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::paygrades::Entity",
        from = "Column::GradeId",
        to = "super::paygrades::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Paygrades,
}

impl Related<super::paygrades::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Paygrades.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
