use jni::JNIEnv;
use jni::objects::{JClass, JString, JObject};
use jni::sys::jstring;

use android_logger::Config;
use log::{info, debug, warn, error};
// use jni::JNIEnv;
// use jni::objects::JClass;

/// 手動で呼ぶ初期化関数
#[no_mangle]
pub extern "system" fn Java_com_example_rustjnilib_NativeLib_initLogger(
    _env: JNIEnv,
    _class: JClass,
) {
    android_logger::init_once(
        Config::default()
            .with_min_level(log::Level::Debug) // 最低レベルを Debug に設定
            .with_tag("RustJNI")               // logcat タグ
    );

    info!("Logger initialized manually");
}




// static method
#[no_mangle]
pub extern "system" fn Java_com_example_rustjnilib_NativeLib_helloWorld(
    env: JNIEnv,
    _class: JClass,
) -> jstring {
    let output = "Hello from Static!!!";
   
   log::info!("Logger initialized manually");

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
