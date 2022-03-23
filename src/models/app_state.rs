use diesel::{PgConnection, r2d2};
use diesel::r2d2::ConnectionManager;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct AppState {
    pub pool: DbPool,
    pub log: slog::Logger,
}
