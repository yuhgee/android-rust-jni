use std::result;

use jni::JNIEnv;
use jni::objects::{JClass, JObject, JValue, JString};
use serde::{de, Deserialize};
use crate::jni::jni_impl::with_env;

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
pub fn fetch_cell_towers_safe(context_obj: &JObject) -> Result<String, Box<dyn std::error::Error>> {
    with_env(|env| {
        let mut env = env;
        get_cell_towers_from_java(&mut env, context_obj)
    })
}


