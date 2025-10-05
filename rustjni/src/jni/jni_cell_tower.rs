use std::result;

use jni::JNIEnv;
use jni::objects::{JClass, JObject, JValue, JString};
use serde::de::value;
use serde::{de, Deserialize};
use crate::jni::jni_impl::with_env;
use crate::jni::jni_impl::get_context_ref;
use crate::data::acceleromater::Accelerometer;
use crate::data::acceleromater::parse_json;

pub fn get_cell_towers_from_java(
    env: &mut JNIEnv,
    context_obj: &JObject, // Activity / Application のインスタンス
) -> Result<String, Box<dyn std::error::Error>> {

    // Context から ClassLoader を取得
    let class_loader = env
        .call_method(context_obj, "getClassLoader", "()Ljava/lang/ClassLoader;", &[])?
        .l()?; // JObject

    // NativeLib クラス名を JString に変換
    let class_name_jstring = env.new_string("com.example.rustjnilib.NativeLib")?;
    let class_name_jobject: JObject = class_name_jstring.into();

    // loadClass で Kotlin クラスをロード
    let provider_class = env
        .call_method(
            class_loader,
            "loadClass",
            "(Ljava/lang/String;)Ljava/lang/Class;",
            &[jni::objects::JValue::Object(&class_name_jobject)],
        )?
        .l()?; // JObject
    let provider_class_jclass: JClass = JClass::from(provider_class);
    
    // static メソッド getCellTowerInfo を呼び出す
    let jvalue = jni::objects::JValue::from(context_obj);
    let jstring: JString = env
        .call_static_method(provider_class_jclass, "getCellTowerInfo", "(Landroid/content/Context;)Ljava/lang/String;", &[jvalue])?
        .l()?
        .into();

    // Rust String に変換して返す
    let json: String = env.get_string(&jstring)?.into();

    Ok(json)
}

/// with_env 内で呼ぶ安全ラッパー
pub fn fetch_cell_towers_safe_with_context(context_obj: &JObject) -> Result<String, Box<dyn std::error::Error>> {
    with_env(|env| {
        let mut env = env;
        let _ = get_accelerometer_from_java(&mut env, context_obj);
        get_cell_towers_from_java(&mut env, context_obj)
    })
}

// ContextはJNI外部に晒さないようにする。
pub fn fetch_cell_towers_safe() -> Result<String, Box<dyn std::error::Error>> {
    let context_guard = get_context_ref();
    let context_ref = context_guard
        .as_ref()
        .expect("Context not initialized");
    let context_obj = context_ref.as_obj();
    with_env(|env| {
        let mut env = env;

        // 変換確認
        // accelerometer test
        match get_accelerometer_from_java(&mut env, context_obj) {
            Ok(json) => {
                let class_data = parse_json(&json);
                match class_data {
                    Ok(data) => {
                        log::info!("Accelerometer data: {:?}", data);
                    },
                    Err(e) => {
                        log::error!("Error fetching accelerometer: {:?}", e);
                    }
                }
            },
            Err(e) => {
                log::error!("Error fetching accelerometer: {:?}", e);
            }
        }

        get_cell_towers_from_java(&mut env, context_obj)
    })
}

//---------------- Accelerometer ----------------
pub fn get_accelerometer_from_java(
    env: &mut JNIEnv,
    context_obj: &JObject,
) -> Result<String, Box<dyn std::error::Error>> {

    // Context から ClassLoader を取得
    let class_loader = env
        .call_method(context_obj, "getClassLoader", "()Ljava/lang/ClassLoader;", &[])?
        .l()?; // JObject

    // NativeLib クラス名を JString に変換
    let class_name_jstring = env.new_string("com.example.rustjnilib.NativeLib")?;
    let class_name_jobject: JObject = class_name_jstring.into();

    // loadClass で Kotlin クラスをロード
    let provider_class = env
        .call_method(
            class_loader,
            "loadClass",
            "(Ljava/lang/String;)Ljava/lang/Class;",
            &[jni::objects::JValue::Object(&class_name_jobject)],
        )?
        .l()?; // JObject
    let provider_class_jclass: JClass = JClass::from(provider_class);

    // static メソッド getAccelerometer を呼び出す
    let jvalue = jni::objects::JValue::from(context_obj);
    let jstring: JString = env
        .call_static_method(provider_class_jclass, "getAccelerometer", "()Ljava/lang/String;", &[])?
        .l()?
        .into();

    // Rust String に変換して返す
    let json: String = env.get_string(&jstring)?.into();
    log::info!("get_accelerometer_from_java!!: {}", json);

    Ok(json)
}

