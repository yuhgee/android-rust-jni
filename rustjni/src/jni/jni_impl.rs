use jni::JNIEnv;
use jni::objects::{JClass, JString, JObject};
use jni::sys::jstring;

use android_logger::Config;
use log::{info, debug, warn, error};

use jni::JavaVM;
use once_cell::sync::OnceCell;
use jni::objects::GlobalRef;
use std::sync::Mutex;

static CONTEXT_CELL: OnceCell<Mutex<GlobalRef>> = OnceCell::new();

pub fn init_context(env: &JNIEnv, context: JObject) {
    let global_ref = env.new_global_ref(context).unwrap();
    CONTEXT_CELL.set(Mutex::new(global_ref)).unwrap();
}

pub fn finalize_context() {
    if let Some(mutex_ref) = CONTEXT_CELL.get() {
        let _global_ref = mutex_ref.lock().unwrap();
    }
}
pub fn get_context() -> std::sync::MutexGuard<'static, GlobalRef> {
    CONTEXT_CELL
        .get()
        .expect("Context not initialized")
        .lock()
        .expect("Failed to lock GlobalRef")
}

pub static JVM: OnceCell<JavaVM> = OnceCell::new();
pub fn init_jvm(env: &JNIEnv) {
    let jvm = env.get_java_vm().expect("Failed to get JavaVM");
    JVM.set(jvm).ok();
}

pub fn with_env<F, R>(f: F) -> R
where
    F: FnOnce(&mut JNIEnv) -> R,
{
    let jvm = JVM.get().expect("JVM not initialized");
    let mut env = jvm.attach_current_thread().expect("Failed to attach thread");
    f(&mut env)
}


#[no_mangle]
pub extern "C" fn Java_com_example_rustjnilib_NativeLib_initialize(
    env: JNIEnv,
    _class: jni::objects::JClass,
    context: JObject,
) {
    init_jvm(&env);
    init_context(&env, context);
}

#[no_mangle]
pub extern "C" fn Java_com_example_rustjnilib_NativeLib_finalize(
    env: JNIEnv,
    _class: jni::objects::JClass,
) {
    finalize_context();
}


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
