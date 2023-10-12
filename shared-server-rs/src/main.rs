use jni::{
    objects::{JObject, JValue},
    sys::jstring,
    JNIEnv, NativeMethod,
};
use std::collections::HashMap;

use jni::{
    objects::{JClass, JString},
    sys::jint,
};
use once_cell::sync::Lazy;
pub struct RequestHandler {
    pub path: String,
    pub handler: fn(String) -> String,
}
pub type RequestHandlerMap = HashMap<i32, RequestHandler>;
mod jvm;

pub static mut ROUTES: Lazy<RequestHandlerMap> = Lazy::new(|| RequestHandlerMap::new());

fn native_apply(mut env: JNIEnv, _class: JClass, fn_id: jint, request: JString) -> jstring {
    let input_str: String = match env.get_string(&request) {
        Ok(val) => val.into(),
        Err(_) => "".to_string(),
    };
    let fn_id_rs: i32 = fn_id as i32;
    let response = unsafe { (ROUTES.get(&fn_id_rs).unwrap().handler)(input_str) };
    let result_jstring = env
        .new_string(response)
        .expect("Couldn't create Java string");
    **result_jstring
}

fn main() {
    let mut env = jvm::attach_current_thread();

    let server_class = env.find_class("shared/server/Server").unwrap();

    /* construct some path string */
    let string_path = env.new_string("/test").unwrap();
    let string_path2 = env.new_string("/test2").unwrap();
    let string_class = env.find_class("java/lang/String").unwrap();
    let j_string_path: JObject = env
        .new_object(
            &string_class,
            "(Ljava/lang/String;)V",
            &[JValue::from(&string_path)],
        )
        .unwrap();

    let j_string_path2: JObject = env
        .new_object(
            &string_class,
            "(Ljava/lang/String;)V",
            &[JValue::from(&string_path2)],
        )
        .unwrap();

    /* register native method for apply */
    let method = NativeMethod {
        name: "handle_request_external".into(),
        sig: "(ILjava/lang/String;)Ljava/lang/String;".into(),
        fn_ptr: native_apply as *mut std::ffi::c_void,
    };
    let router_handler_native_class = env.find_class("shared/server/Server").unwrap();
    let _ = env.register_native_methods(&router_handler_native_class, &[method]);

    /* add route handler to server */
    let handler1_id = env
        .call_static_method(
            &server_class,
            "addRoute",
            "(Ljava/lang/String;)I",
            &[JValue::Object(&j_string_path)],
        )
        .unwrap()
        .i()
        .unwrap();

    let handler2_id = env
        .call_static_method(
            &server_class,
            "addRoute",
            "(Ljava/lang/String;)I",
            &[JValue::Object(&j_string_path2)],
        )
        .unwrap()
        .i()
        .unwrap();

    println!("registered handler {}", handler1_id);
    unsafe {
        ROUTES.insert(
            handler1_id,
            RequestHandler {
                path: "/test".to_string(),
                handler: |_| "test".to_string(),
            },
        );
        ROUTES.insert(
            handler2_id,
            RequestHandler {
                path: "/test2".to_string(),
                handler: |_| "test2".to_string(),
            },
        );
    }

    let _ = env
        .call_static_method(&server_class, "start", "(Z)V", &[JValue::Bool(1)])
        .unwrap()
        .v()
        .unwrap();

    loop {}
}
