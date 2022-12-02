pub use sea_orm;

pub mod level_template;
pub mod popularity;
pub mod users;

pub mod prelude {
    pub use crate::level_template::Model as LevelTemplate;
    pub use crate::popularity::Model as Popularity;
    pub use crate::users::Model as User;
}
