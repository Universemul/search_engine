pub mod base;
pub mod index;

use crate::warp::Filter;

pub fn index_route(path_data: &str) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let p = path_data.to_string();
    let count = warp::path!("index" / String / "count").map(move |_index_name: String| index::count(_index_name));
    let search = warp::path!("index" / String / "search").map(move |_index_name: String| index::search(_index_name));
    let index = warp::path("index").and(warp::path::param::<String>()).map(move |name: String| index::root(name, &p));
    count.or(index.or(search))
} 