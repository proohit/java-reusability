use crate::jvm;
use crate::server;
use jni::{
    objects::{JClass, JString},
    sys::{jint, jstring},
    JNIEnv, NativeMethod,
};

fn native_apply(mut env: JNIEnv, _class: JClass, fn_id: jint, request: JString) -> jstring {
    let input_str: String = match env.get_string(&request) {
        Ok(val) => val.into(),
        Err(_) => "".to_string(),
    };
    let fn_id_rs: i32 = fn_id as i32;
    let response = unsafe { (server::ROUTES.get(&fn_id_rs).unwrap().handle)(input_str) };
    let result_jstring = env
        .new_string(response)
        .expect("Couldn't create Java string");
    **result_jstring
}

pub fn register_native_method() {
    let mut env = jvm::attach_current_thread();
    let method = NativeMethod {
        name: "handle_request_external".into(),
        sig: "(ILjava/lang/String;)Ljava/lang/String;".into(),
        fn_ptr: native_apply as *mut std::ffi::c_void,
    };
    let router_handler_native_class = env.find_class("shared/server/Server").unwrap();
    let _ = env.register_native_methods(&router_handler_native_class, &[method]);
}
