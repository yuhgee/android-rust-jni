use jni::JNIEnv;
use jni::objects::{JClass, JString, JObject};
use jni::sys::jstring;

use android_logger::Config;
use std::os::raw::c_void;

use jni::JavaVM;
use once_cell::sync::OnceCell;
use jni::objects::GlobalRef;
use std::sync::Mutex;
use std::sync::MutexGuard;


/// JNI_OnLoad: ライブラリがロードされたときに呼ばれる
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn JNI_OnLoad(vm: *mut jni::sys::JavaVM, _reserved: *mut c_void) -> jni::sys::jint {
    android_logger::init_once(
        Config::default()
            .with_min_level(log::Level::Debug)
            .with_tag("RustJNI")
    );

    JVM.set(unsafe { JavaVM::from_raw(vm).expect("Failed to get JavaVM") } ).ok();

    log::info!("JNI_OnLoad called");

    jni::JNIVersion::V6.into() // JNI_VERSION_1_6
}

// JavaVMを保持するためのOnceCell
pub static JVM: OnceCell<JavaVM> = OnceCell::new();
pub fn with_env<F, R>(f: F) -> R
where
    F: FnOnce(&mut JNIEnv) -> R,
{
    let jvm = JVM.get().expect("JVM not initialized");
    let mut env = jvm.attach_current_thread().expect("Failed to attach thread");
    f(&mut env)
}


// ContextのGlobalRefを保持するためのOnceCell
static CONTEXT_CELL: OnceCell<Mutex<Option<GlobalRef>>> = OnceCell::new();

// 初期化
pub fn init_context(env: &jni::JNIEnv, context: jni::objects::JObject) {
    let global_ref = env.new_global_ref(context).unwrap();
    CONTEXT_CELL
        .set(Mutex::new(Some(global_ref)))
        .ok(); // すでに初期化済みなら無視
}

// 解放
pub fn finalize_context() {
    if let Some(mutex_ref) = CONTEXT_CELL.get() {
        let mut guard = mutex_ref.lock().unwrap();
        *guard = None; // GlobalRef を drop
    }
}

pub fn get_context_ref() -> MutexGuard<'static, Option<GlobalRef>> {
    CONTEXT_CELL
        .get()
        .expect("Context not initialized")
        .lock()
        .expect("Failed to lock GlobalRef")
}

// ------------------------------------------------
// initialize / finalize
// ------------------------------------------------

#[no_mangle]
pub extern "C" fn Java_com_example_rustjnilib_NativeLib_initialize(
    env: JNIEnv,
    _class: jni::objects::JClass,
    context: JObject,
) {
    init_context(&env, context);
}

#[no_mangle]
pub extern "C" fn Java_com_example_rustjnilib_NativeLib_finalize(
    _env: JNIEnv,
    _class: jni::objects::JClass,
) {
    finalize_context();
}


// ------------------------------------------------
// 動作検証用
// ------------------------------------------------
// static method
#[no_mangle]
pub extern "system" fn Java_com_example_rustjnilib_NativeLib_helloWorld(
    env: JNIEnv,
    _class: JClass,
) -> jstring {
    let output = "Hello from Static!!!";
   
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
