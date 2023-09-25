use jni::{
    descriptors::Desc,
    errors::{Error, Result},
    objects::{AutoLocal, JValue, JString, JObject},
    signature::{Primitive, ReturnType},
    sys::{jint, JavaVMInitArgs},
    InitArgsBuilder, JNIEnv,
};
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
            .option("-Djava.class.path=/home/master/workspace/java-reusability/shared-server/lib/bin/main")
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
static EXCEPTION_CLASS: &str = "java/lang/Exception";
static TESTING_OBJECT_STR: &str = "TESTING OBJECT";

fn main() {
    let mut env = attach_current_thread();

    let server_class = env.find_class("shared/server/Server").unwrap();

    // let add_route_method_id = env
    //     .get_static_method_id(&server_class, "addRoute", "(Lshared/server/RouteHandler;)V")
    //     .unwrap();

    // let test_str = env.new_string("/").unwrap();

    // // Get a reference to the Function interface class
    // let function_class = env
    //     .find_class("java/util/function/Function")
    //     .expect("Failed to find Function class");

    // // Find the apply method of the Function interface
    // let apply_method_id = env
    //     .get_method_id(
    //         function_class,
    //         "apply",
    //         "(Ljava/lang/Object;)Ljava/lang/Object;",
    //     )
    //     .expect("Failed to get apply method");

    // // Create a Rust closure that implements the Function<String, String> interface
    // let function = Box::new(|env: &JNIEnv, input: JString| -> JString {
    //     let input_str = env.get_string(&input).expect("Couldn't convert input to String");
    //     let input_str = input_str.to_str().unwrap();
    //     let result_str = input_str.to_uppercase();
    //     let result_jstring = env.new_string(result_str).expect("Couldn't create JString");
    //     result_jstring.into()
    // });

    // // Convert the closure to a jobject representing a Function<String, String>
    // let function_obj = JValue::Object(function);
    
    // // Wrap the function_obj in a JValue
    // let function_arg = JValue::from(function_obj);

    // let route_handler_class = env.find_class("shared/server/RouteHandler").unwrap();
    
    // let route_handler_1 = env.new_object(&route_handler_class, "(Ljava/lang/String;Ljava/util/function/Function<Ljava/lang/String;Ljava/lang/String;>)", &[JValue::from(&test_str)]).unwrap();
    // unsafe {
    //     let _ = env.call_static_method_unchecked(
    //         &server_class,
    //         add_route_method_id,
    //         ReturnType::Primitive(Primitive::Void),
    //         &[JValue::Object(&route_handler_1).as_jni()],
    //     );
    // }
    let start_method_id = env
        .get_static_method_id(&server_class, "start", "()V")
        .unwrap();
    let _ = unsafe {
        env.call_static_method_unchecked(
            &server_class,
            start_method_id,
            ReturnType::Primitive(Primitive::Void),
            &[],
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
