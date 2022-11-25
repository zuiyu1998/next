mod prepare;

use next_core::users;
use prepare::prepare_mock_db;
use tokio;

#[tokio::test]
async fn main() {
    let db = &prepare_mock_db();

    {
        let mut find = users::UserFind::default();

        find.id = Some(1);

        let user = users::Api::find(db, find).await.unwrap().unwrap();

        assert_eq!(user.id, 1);
    }
}
