pub use sea_orm;

pub mod users;

pub mod prelude {
    pub use crate::users::Model as User;
}
