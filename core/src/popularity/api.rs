use super::{PopularityCreate, PopularityQuery, PopularityUpdate};
use chrono::offset::Local;
use next_entity::popularity::{ActiveModel, Column, Entity, Model};
use next_entity::sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, PaginatorTrait,
    QueryFilter, Set,
};

pub struct Api;

impl Api {
    pub async fn create<C: ConnectionTrait>(
        c: &C,
        popularity_create: PopularityCreate,
    ) -> Result<Model, DbErr> {
        let mut active: ActiveModel = Default::default();

        active.name = Set(popularity_create.name);
        active.level_template_name = Set(popularity_create.level_template_name);
        active.count = Set(popularity_create.count);
        active.level = Set(popularity_create.level);
        active.user_id = Set(popularity_create.user_id);

        let now = Local::now();

        active.create_at = Set(now.naive_local());
        active.update_at = Set(now.naive_local());

        active.insert(c).await
    }

    pub async fn update<C: ConnectionTrait>(
        c: &C,
        level_template_update: PopularityUpdate,
    ) -> Result<Model, DbErr> {
        let mut active: ActiveModel = Default::default();

        if let Some(name) = level_template_update.name {
            active.name = Set(name);
        }
        if let Some(level) = level_template_update.level {
            active.level = Set(level);
        }
        if let Some(count) = level_template_update.count {
            active.count = Set(count);
        }
        active.update(c).await
    }

    pub async fn find<C: ConnectionTrait>(c: &C, user_id: i32) -> Result<Option<Model>, DbErr> {
        let sql = Entity::find().filter(Column::UserId.eq(user_id));

        sql.one(c).await
    }

    pub async fn query<C: ConnectionTrait>(
        c: &C,
        level_template_query: PopularityQuery,
    ) -> Result<Vec<Model>, DbErr> {
        let sql = Entity::find();

        let paginate = sql.paginate(c, level_template_query.page_size as usize);
        paginate
            .fetch_page(level_template_query.page as usize)
            .await
    }
}
