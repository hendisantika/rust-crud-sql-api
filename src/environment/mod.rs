use argon::Argon;
use clap::Clap;
use sqlx::postgres::PgPool;
use warp::Filter;

mod argon;

#[derive(Clone, Debug)]
pub struct Environment {
    db_pool: PgPool,
    config: Args,
    argon: Argon,
}