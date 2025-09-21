use jni::JNIEnv;
use jni::objects::{JClass, JString, JObject};
use jni::sys::jstring;

// static method
#[no_mangle]
pub extern "system" fn Java_com_example_rustjnilib_NativeLib_helloWorld(
    env: JNIEnv,
    _class: JClass,
) -> jstring {
    let output = "Hello from Static!";

    // JString を生成
    let java_str: JString = env.new_string(output).unwrap();

    // JString -> JObject -> jstring (* deref)
    let java_obj: JObject = java_str.into();
    *java_obj
}

// instance method
#[no_mangle]
pub extern "system" fn Java_com_example_rustjnilib_NativeLibInstance_helloWorld(
    env: JNIEnv,
    _this: JObject,   // インスタンス
) -> jstring {
    let output = "Hello from Instance!";
    let java_str = env.new_string(output).unwrap();
    let java_obj: JObject = java_str.into();
    *java_obj
}
