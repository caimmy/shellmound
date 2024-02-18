use futures::executor::block_on;
use sea_orm::{Database, DbErr, ActiveValue, EntityTrait};
use shellmound::models::entities::{prelude::*, *, sea_orm_active_enums::Status};

const DATABASE_URL: &str = "mysql://shellmound:abcd1234@127.0.0.1:3306/shellmound";

async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;
    let user_item = users::ActiveModel{
        email: ActiveValue::Set("caimmy@qq.com".to_owned()),
        reg_ip: ActiveValue::Set(Some("127.0.0.1".to_string())),
        // reg_tm: ActiveValue::Set(Some("2024-01-10 18:05:00".to_owned())),
        status: ActiveValue::Set(Some(Status::_0)),
        ..Default::default()
    };
    let res = Users::insert(user_item).exec(&db).await.unwrap();
    println!("{:#?}", res.last_insert_id);
    Ok(())
}


fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}
