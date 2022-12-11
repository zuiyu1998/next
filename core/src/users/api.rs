use super::{UserCreate, UserFind, UserQuery, UserUpdate};
use chrono::offset::Local;
use next_entity::sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, PaginatorTrait,
    QueryFilter, Set,
};
use next_entity::users::{ActiveModel, Column, Entity, Model};

pub struct Api;

impl Api {
    pub async fn create<C: ConnectionTrait>(
        c: &C,
        user_create: UserCreate,
    ) -> Result<Model, DbErr> {
        let mut active: ActiveModel = Default::default();

        active.email = Set(user_create.email);
        active.password = Set(user_create.password);

        let now = Local::now();

        active.create_at = Set(now.naive_local());
        active.update_at = Set(now.naive_local());

        let uid = xid::new().to_string();

        active.uid = Set(uid.to_owned());
        active.nike_name = Set(String::from("uid-") + &uid);

        active.status = Set(true);

        active.insert(c).await
    }

    pub async fn update<C: ConnectionTrait>(
        c: &C,
        user_update: UserUpdate,
    ) -> Result<Model, DbErr> {
        let mut active: ActiveModel = Default::default();

        if let Some(nike_name) = user_update.nike_name {
            active.nike_name = Set(nike_name);
        }
        if let Some(password) = user_update.password {
            active.password = Set(password);
        }
        active.update(c).await
    }

    pub async fn find<C: ConnectionTrait>(
        c: &C,
        user_find: UserFind,
    ) -> Result<Option<Model>, DbErr> {
        let mut sql = Entity::find();

        if user_find.id.is_some() {
            sql = sql.filter(Column::Id.eq(user_find.id.unwrap()));
        } else if user_find.uid.is_some() {
            sql = sql.filter(Column::Uid.like(&user_find.uid.unwrap()));
        } else if user_find.email.is_some() {
            sql = sql.filter(Column::Email.eq(user_find.email.unwrap()));
        }

        sql.one(c).await
    }

    pub async fn query<C: ConnectionTrait>(
        c: &C,
        user_query: UserQuery,
    ) -> Result<Vec<Model>, DbErr> {
        let sql = Entity::find();

        let paginate = sql.paginate(c, user_query.page_size as usize);
        paginate.fetch_page(user_query.page as usize).await
    }
}
