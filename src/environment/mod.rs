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

#[derive(Clone, Clap, Debug)]
#[clap(
name = "demo-api",
rename_all = "kebab-case",
rename_all_env = "screaming-snake"
)]
pub struct Args {
    #[clap(short, long)]
    debug: bool,

    #[clap(required = true, short = 'D', long, env)]
    database_url: String,

    #[clap(required = true, long, env)]
    jwt_secret: String,
    #[clap(required = true, long, env)]
    argon_secret: String,
    #[clap(long, env)]
    argon_iterations: Option<u32>,
    #[clap(long, env)]
    argon_memory_size: Option<u32>,

    #[clap(default_value = "0.0.0.0:8080", env)]
    pub host: SocketAddr,
}