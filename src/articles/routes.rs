use warp::{Filter, Reply};
use warp::filters::BoxedFilter;

use crate::{auth, environment};
use crate::articles::handlers;
use crate::auth::models::Role;
use crate::environment::Environment;

pub fn routes(_env: Environment) -> BoxedFilter<(impl Reply, )> {
    let get_home_article_headers_route = warp::get().and(warp::path!("api" / "articles_home")
        .and(environment::with_env(_env.clone()))
        .and_then(handlers::get_home_article_headers_handler));

    let get_article_headers_route = warp::get().and(warp::path!("api" / "articles")
        .and(environment::with_env(_env.clone()))
        .and_then(handlers::get_article_headers_handler));

    let get_article_route = warp::get().and(warp::path!("api" / "articles" / String)
        .and(environment::with_env(_env.clone()))
        .and_then(handlers::get_article_by_url_handler));
}
