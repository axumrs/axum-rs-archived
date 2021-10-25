use tower_cookies::{Cookie, Cookies};

const COOKE_NAME: &str = "AXUMRS_FLASH";

pub fn set(cookie: Cookies, msg: &str) {
    cookie.add(Cookie::new(COOKE_NAME, msg.to_owned()));
}

pub fn get(cookie: Cookies) -> String {
    let c = cookie.get(COOKE_NAME);
    let msg = match c {
        Some(c) => c.value().to_string(),
        None => "".to_owned(),
    };
    //cookie.add(Cookie::new(COOKE_NAME, ""));
    cookie.remove(Cookie::new(COOKE_NAME, ""));
    msg
}
