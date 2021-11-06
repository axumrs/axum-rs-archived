use uuid::Uuid;

use crate::config::SessionConfig;

pub struct GeneratedKey {
    pub id: String,
    pub cookie_key: String,
    pub redis_key: String,
}

pub fn id() -> String {
    Uuid::new_v4().to_simple().to_string()
}
pub fn gen_key(cfg: &SessionConfig) -> GeneratedKey {
    let id = id();
    let cookie_key = cfg.id_name.to_string();
    let redis_key = gen_redis_key(cfg, &id);
    GeneratedKey {
        id,
        cookie_key,
        redis_key,
    }
}
pub fn gen_redis_key(cfg: &SessionConfig, id: &str) -> String {
    format!("{}{}", &cfg.prefix, id)
}
