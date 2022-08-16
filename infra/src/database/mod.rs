use crate::env;
use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod, Runtime};
use tokio_postgres::NoTls;

pub async fn conn(cfg: &env::Config) -> Pool {
    let mut pg = Config::new();
    pg.host = Some(cfg.db_host.to_owned());
    pg.port = Some(cfg.db_port);
    pg.user = Some(cfg.db_user.to_owned());
    pg.password = Some(cfg.db_password.to_owned());
    pg.dbname = Some(cfg.db_name.to_owned());
    pg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });

    pg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap()
}
