pub mod base;
pub mod index;

use nickel::{Nickel, HttpRouter};

pub fn index_route(server: &mut Nickel, path_data: &str) -> () {
    let p = path_data.to_string();
    server.get("/index/:name", middleware! { |request|
        index::root(request, &p)
    });
    server.get("/index/:name/count", middleware! { |request|
        index::count(request)
    });
    server.get("/index/:name/search", middleware! { |request|
        index::search(request)
    });
}