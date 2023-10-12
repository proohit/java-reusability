use std::collections::HashMap;

use jni::objects::JValue;
use once_cell::sync::Lazy;

use crate::{jvm, module_wrapper::register_native_method};
type HandleFn = fn(String) -> String;

pub struct RequestHandler {
    pub path: String,
    pub handle: HandleFn,
}
pub type RequestHandlerMap = HashMap<i32, RequestHandler>;
pub static mut ROUTES: Lazy<RequestHandlerMap> = Lazy::new(|| RequestHandlerMap::new());

pub fn add_route(handler: RequestHandler) {
    let mut env = jvm::attach_current_thread();
    let server_class = env.find_class("shared/server/Server").unwrap();

    let string_path = env.new_string(&(handler.path)).unwrap();

    let handler_id = env
        .call_static_method(
            &server_class,
            "addRoute",
            "(Ljava/lang/String;)I",
            &[JValue::Object(&string_path)],
        )
        .unwrap()
        .i()
        .unwrap();

    unsafe {
        ROUTES.insert(
            handler_id,
            RequestHandler {
                path: handler.path,
                handle: handler.handle,
            },
        );
    }

    println!("registered handler {}", handler_id);
}

pub fn start() {
    let mut env = jvm::attach_current_thread();
    let server_class = env.find_class("shared/server/Server").unwrap();
    register_native_method();

    let _ = env
        .call_static_method(&server_class, "start", "(Z)V", &[JValue::Bool(1)])
        .unwrap()
        .v()
        .unwrap();
}
