use askama::Template;
#[derive(Template)]
#[template(path = "auth/login.html")]
pub struct LoginTemplate {
    pub site_key: String,
}
