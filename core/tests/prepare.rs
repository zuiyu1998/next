use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

#[allow(unused_imports)]
use next_entity::sea_orm::*;
#[allow(unused_imports)]
use next_entity::users;

pub fn get_time() -> NaiveDateTime {
    let d = NaiveDate::from_ymd_opt(2015, 6, 3).unwrap();
    let t = NaiveTime::from_hms_milli_opt(12, 34, 56, 789).unwrap();
    NaiveDateTime::new(d, t)
}

#[cfg(feature = "mock")]
pub fn prepare_mock_db() -> DatabaseConnection {
    let dt = get_time();

    MockDatabase::new(DatabaseBackend::MySql)
        .append_query_results(vec![vec![users::Model {
            id: 1,
            uid: "01".to_owned(),
            nike_name: "01".to_owned(),
            email: "01".to_owned(),
            password: vec![],
            create_at: NaiveDateTime::from(dt),
            update_at: NaiveDateTime::from(dt),
            status: true,
        }]])
        .append_exec_results(vec![
            MockExecResult {
                last_insert_id: 6,
                rows_affected: 1,
            },
            MockExecResult {
                last_insert_id: 6,
                rows_affected: 5,
            },
        ])
        .into_connection()
}
