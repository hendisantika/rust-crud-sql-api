use warp::{Filter, Reply};
use warp::filters::BoxedFilter;

use crate::{auth, environment};
use crate::articles::handlers;
use crate::auth::models::Role;
use crate::environment::Environment;

pub fn routes(_env: Environment) -> BoxedFilter<(impl Reply, )> {}
