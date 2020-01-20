pub mod base;
pub mod index;

use nickel::{Nickel, HttpRouter};
use nickel::hyper::method::Method;

pub fn index_route(server: &mut Nickel, path_data: &str) -> () {
    server.add_route(Method::Get, "/index/:name", middleware! { |request|
        index::root(request, path_data)
    });
    server.add_route(Method::Get, "/index/:name/count", middleware! { |request|
        index::count(request)
    });
    server.add_route(Method::Get, "/index/:name/search", middleware! { |request|
        index::search(request)
    });
}