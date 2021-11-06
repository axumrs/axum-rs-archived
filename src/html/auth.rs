use askama::Template;
#[derive(Template)]
#[template(path = "auth/login.html")]
pub struct LoginTemplate {}
