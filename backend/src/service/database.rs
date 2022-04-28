use common::sea_orm::DatabaseConnection;
use std::convert::Infallible;
use std::sync::Arc;
use warp::Filter;

pub type Db = Arc<DatabaseConnection>;

pub fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
