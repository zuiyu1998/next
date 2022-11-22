use poem::{get, handler, post, Endpoint, Route};

//用户修改昵称
#[handler]
async fn update_nike_name() -> poem::Result<()> {
    todo!()
}

//用户修改密码
#[handler]
async fn update_password() -> poem::Result<()> {
    todo!()
}

//用户忘记密码
#[handler]
async fn forget_password() -> poem::Result<()> {
    todo!()
}

//用户信息
#[handler]
async fn info() -> poem::Result<()> {
    todo!()
}

//用户登录
#[handler]
async fn login() -> poem::Result<()> {
    todo!()
}

//创建用户
#[handler]
async fn create() -> poem::Result<()> {
    todo!()
}

pub fn config() -> impl Endpoint {
    Route::new()
        .at("/crate", post(create))
        .at("/login", post(login))
        .at("/info", get(info))
        .at("/forget_password", post(forget_password))
        .at("/update_password", post(update_password))
        .at("/update_nike_name", post(update_nike_name))
}
