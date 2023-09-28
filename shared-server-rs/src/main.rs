use jni::{
    errors::Result,
    objects::{JObject, JValue},
    signature::{Primitive, ReturnType},
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

    let add_route_method_id = env
        .get_static_method_id(&server_class, "addRoute", "(Ljava/lang/String;)I")
        .unwrap();
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

    let handler_id = unsafe {
        env.call_static_method_unchecked(
            &server_class,
            add_route_method_id,
            ReturnType::Primitive(Primitive::Int),
            &[JValue::Object(&j_string_path).as_jni()],
        )
    }
    .unwrap()
    .i()
    .unwrap();
    println!("registered handler {}", handler_id);
    
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
