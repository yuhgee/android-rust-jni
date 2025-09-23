// src/jni_service.rs
use std::sync::Mutex;
use std::sync::Arc;

use crate::data_store::service::Service;
use crate::data_store::store::SharedStore;
use crate::data_store::my_data::MyData;

use jni::objects::JClass;
use jni::sys::{jlong, JNIEnv};
use lazy_static::lazy_static;

lazy_static! {
    static ref SERVICE: Mutex<Option<Service>> = Mutex::new(None);
}

#[no_mangle]
pub extern "system" fn Java_com_example_rustjnilib_NativeLib_start(_env: JNIEnv, _class: JClass) {
    let mut svc_guard = SERVICE.lock().unwrap();
    if svc_guard.is_none() {
        let mut service = Service::new();
        service.start();
        *svc_guard = Some(service);
    } else {
        println!("Service already running");
    }
}

#[no_mangle]
pub extern "system" fn Java_com_example_rustjnilib_NativeLib_stop(_env: JNIEnv, _class: JClass) {
    let mut svc_guard = SERVICE.lock().unwrap();
    if let Some(service) = svc_guard.as_mut() {
        service.stop();
    }
    *svc_guard = None;
}

#[no_mangle]
pub extern "system" fn Java_com_example_rustjnilib_NativeLib_addData(
    _env: JNIEnv,
    _class: JClass,
    value: jlong,
) {
    let svc_guard = SERVICE.lock().unwrap();
    if let Some(service) = svc_guard.as_ref() {
        service.add_data(value as u64);
    } else {
        println!("Service not running, cannot add data");
    }
}
