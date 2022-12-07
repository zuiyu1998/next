use super::{LevelTemplateCreate, LevelTemplateQuery, LevelTemplateUpdate};
use chrono::offset::Local;
use next_entity::level_template::{ActiveModel, Column, Entity, Model};
use next_entity::sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, PaginatorTrait,
    QueryFilter, Set,
};

pub struct Api;

impl Api {
    pub async fn create<C: ConnectionTrait>(
        c: &C,
        level_template_create: LevelTemplateCreate,
    ) -> Result<Model, DbErr> {
        let mut active: ActiveModel = Default::default();

        active.name = Set(level_template_create.name);
        active.content = Set(level_template_create.content);
        active.status = Set(true);

        let now = Local::now();

        active.create_at = Set(now.naive_local());
        active.update_at = Set(now.naive_local());

        active.insert(c).await
    }

    pub async fn update<C: ConnectionTrait>(
        c: &C,
        level_template_update: LevelTemplateUpdate,
    ) -> Result<Model, DbErr> {
        let mut active: ActiveModel = Default::default();

        if let Some(name) = level_template_update.name {
            active.name = Set(name);
        }
        if let Some(content) = level_template_update.content {
            active.content = Set(content);
        }
        active.update(c).await
    }

    pub async fn find<C: ConnectionTrait>(c: &C, name: &str) -> Result<Option<Model>, DbErr> {
        let sql = Entity::find()
            .filter(Column::Name.eq(name))
            .filter(Column::Status.eq(true));

        sql.one(c).await
    }

    pub async fn query<C: ConnectionTrait>(
        c: &C,
        level_template_query: LevelTemplateQuery,
    ) -> Result<Vec<Model>, DbErr> {
        let mut sql = Entity::find();
        sql = sql.filter(Column::Status.eq(level_template_query.status));

        if let Some(name) = level_template_query.name {
            sql = sql.filter(Column::Name.like(&name));
        }

        let paginate = sql.paginate(c, level_template_query.page_size as usize);
        paginate
            .fetch_page(level_template_query.page as usize)
            .await
    }
}
