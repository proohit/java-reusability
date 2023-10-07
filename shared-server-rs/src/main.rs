use jni::{
    errors::Result,
    objects::{JObject, JValue},
    signature::{Primitive, ReturnType},
    sys::jstring,
    InitArgsBuilder, JNIEnv, NativeMethod,
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

pub static mut ROUTES: Lazy<RequestHandlerMap> = Lazy::new(|| RequestHandlerMap::new());

#[no_mangle]
pub extern "system" fn Java_shared_server_Server_handle_1request_1external<'local>(
    // Notice that this `env` argument is mutable. Any `JNIEnv` API that may
    // allocate new object references will take a mutable reference to the
    // environment.
    mut env: JNIEnv<'local>,
    // this is the class that owns our static method. Not going to be used, but
    // still needs to have an argument slot
    _class: JClass<'local>,
    fn_id: jint,
    req: JString<'local>,
) -> JString<'local> {
    // First, we have to get the string out of java. Check out the `strings`
    // module for more info on how this works.
    let req_rs: String = env
        .get_string(&req)
        .expect("Couldn't get java string!")
        .into();

    let fn_id_rs: i32 = fn_id as i32;
    println!("id: {}, req: {}", fn_id_rs, req_rs);
    unsafe {
        println!("Count of routes {}", ROUTES.len());
        ROUTES.values().for_each(|req_handler| {
            println!("{}", req_handler.path);
        });
    }
    // Then we have to create a new java string to return. Again, more info
    // in the `strings` module.
    let output = env
        .new_string(format!("Hello, {}!", req_rs))
        .expect("Couldn't create java string!");
    output
}

fn native_apply(mut env: JNIEnv, _class: JClass, input: JString) -> jstring {
    // Convert the Java input string to a Rust string
    let input_str: String = env.get_string(&input).expect("Invalid string").into();

    // Perform your processing on the input here.
    // For example, you can create a result string.
    let result_str = format!("Hello, {}", input_str);

    // Convert the result string back to a Java string
    let result_jstring = env
        .new_string(result_str)
        .expect("Couldn't create Java string");
    **result_jstring
}

use std::sync::{Arc, Once};

use jni::{AttachGuard, JNIVersion, JavaVM};

pub fn print_exception(env: &JNIEnv) {
    let exception_occurred = env.exception_check().unwrap_or_else(|e| panic!("{:?}", e));
    if exception_occurred {
        env.exception_describe()
            .unwrap_or_else(|e| panic!("{:?}", e));
    }
}

pub fn unwrap<T>(res: Result<T>, env: &JNIEnv) -> T {
    res.unwrap_or_else(|e| {
        print_exception(env);
        panic!("{:#?}", e);
    })
}
pub fn jvm() -> &'static Arc<JavaVM> {
    static mut JVM: Option<Arc<JavaVM>> = None;
    static INIT: Once = Once::new();

    INIT.call_once(|| {
        let jvm_args = InitArgsBuilder::new()
            .version(JNIVersion::V8)
            .option("-Xcheck:jni")
            .option("-Djava.class.path=/home/master/workspace/java-reusability/shared-server/lib/build/classes/java/main")
            .option("-Djava.library.path=/home/master/workspace/java-reusability/shared-server-rs/target/debug")
            .build()
            .unwrap_or_else(|e| panic!("{:#?}", e));

        let jvm = JavaVM::new(jvm_args).unwrap_or_else(|e| panic!("{:#?}", e));

        unsafe {
            JVM = Some(Arc::new(jvm));
        }
    });

    unsafe { JVM.as_ref().unwrap() }
}

pub fn attach_current_thread() -> AttachGuard<'static> {
    jvm()
        .attach_current_thread()
        .expect("failed to attach jvm thread")
}

fn main() {
    let mut env = attach_current_thread();

    let server_class = env.find_class("shared/server/Server").unwrap();

    /* get add route method id */

    // let add_route_method_id = env
    //     .get_static_method_id(&server_class, "addRoute", "(Ljava/lang/String;)I")
    //     .unwrap();
    let add_route_method_id = env
        .get_static_method_id(
            &server_class,
            "addRoute",
            "(Lshared/server/RouteHandlerNative;)V",
        )
        .unwrap();

    /* construct some path string */
    let string_path = env.new_string("/test").unwrap();
    let string_class = env.find_class("java/lang/String").unwrap();
    let ctor_method_id = env
        .get_method_id(&string_class, "<init>", "(Ljava/lang/String;)V")
        .unwrap();
    let j_string_path: JObject = unsafe {
        env.new_object_unchecked(
            &string_class,
            ctor_method_id,
            &[JValue::from(&string_path).as_jni()],
        )
    }
    .unwrap();

    /* register native method for apply */
    let method = NativeMethod {
        name: "apply".into(),
        sig: "(Ljava/lang/String;)Ljava/lang/String;".into(),
        fn_ptr: native_apply as *mut std::ffi::c_void,
    };
    let router_handler_native_class = env.find_class("shared/server/RouteHandlerNative").unwrap();
    let _ = env.register_native_methods(&router_handler_native_class, &[method]);
    let router_handler_ctor = env
        .get_method_id(
            &router_handler_native_class,
            "<init>",
            "(Ljava/lang/String;)V",
        )
        .unwrap();

    /* construct router handler object */
    let router_handler = unsafe {
        env.new_object_unchecked(
            &router_handler_native_class,
            router_handler_ctor,
            &[JValue::Object(&j_string_path).as_jni()],
        )
    }
    .unwrap();

    /* add route handler to server */
    // let handler_id = unsafe {
    //     env.call_static_method_unchecked(
    //         &server_class,
    //         add_route_method_id,
    //         ReturnType::Primitive(Primitive::Int),
    //         &[JValue::Object(&j_string_path).as_jni()],
    //     )
    // }
    // .unwrap()
    // .i()
    // .unwrap();
    let _ = unsafe {
        env.call_static_method_unchecked(
            &server_class,
            add_route_method_id,
            ReturnType::Primitive(Primitive::Void),
            &[JValue::Object(&router_handler).as_jni()],
        )
    }
    .unwrap()
    .v()
    .unwrap();
    // println!("registered handler {}", handler_id);
    // unsafe {
    //     ROUTES.insert(
    //         handler_id,
    //         RequestHandler {
    //             path: "/test".to_string(),
    //             handler: |req| req,
    //         },
    //     );
    // }

    let start_method_id = env
        .get_static_method_id(&server_class, "start", "(Z)V")
        .unwrap();
    let _ = unsafe {
        env.call_static_method_unchecked(
            &server_class,
            start_method_id,
            ReturnType::Primitive(Primitive::Void),
            &[JValue::Bool(1).as_jni()],
        )
    }
    .unwrap()
    .v()
    .unwrap();

    loop {}
    // let s = env.new_string(TESTING_OBJECT_STR).unwrap();

    // let v: jint = env
    //     .call_method(s, "indexOf", "(I)I", &[JValue::Int('S' as i32)])
    //     .expect("JNIEnv#call_method should return JValue")
    //     .i()
    //     .unwrap();

    // print!("{}", val)
}
