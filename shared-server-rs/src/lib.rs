use jni::{
    objects::{JClass, JString},
    sys::jint,
    JNIEnv,
};

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

    // Then we have to create a new java string to return. Again, more info
    // in the `strings` module.
    let output = env
        .new_string(format!("Hello, {}!", req_rs))
        .expect("Couldn't create java string!");
    output
}
