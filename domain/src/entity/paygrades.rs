use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "paygrades")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub grade: String,
    #[sea_orm(column_type = "Custom(\"USER-DEFINED\".to_owned())")]
    pub scale: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::ranks::Entity")]
    Ranks,
}

impl Related<super::ranks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Ranks.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
