pub mod base;
pub mod index;

use nickel::{Nickel, HttpRouter};
use nickel::hyper::method::Method;

pub fn index_route(server: &mut nickel::Nickel) -> () {
    server.add_route(Method::Get, "/index/:name", middleware! { |request|
        index::root(request)
    });
    server.add_route(Method::Get, "/index/:name/count", middleware! { |request|
        index::count(request)
    });
    server.add_route(Method::Get, "/index/:name/search", middleware! { |request|
        index::search(request)
    });
}

pub fn startup(port: i16) -> () {
    let mut server = Nickel::new();
    server.add_route(Method::Get, "/", middleware! { |request|
        base::home(request)
    });
    index_route(&mut server);

    let addr = format!("127.0.0.1:{}", port);
    println!("Start Rest Api successfully");
    server.listen(addr).unwrap();
}