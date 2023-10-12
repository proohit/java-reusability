use server::RequestHandler;

mod jvm;
mod module_wrapper;
mod server;

fn main() {
    server::add_route(RequestHandler {
        path: "/test1".to_string(),
        handle: |_| "test1".to_string(),
    });

    server::add_route(RequestHandler {
        path: "/test2".to_string(),
        handle: |_| "test2".to_string(),
    });

    server::start();

    loop {}
}
