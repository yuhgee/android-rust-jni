package com.example.rustjnilib

import android.util.Log

object NativeLib {
    private val TAG = NativeLib::class.java.simpleName
    private val INSTANCE = NativeLibInstance()

    init {
        System.loadLibrary("rustjni") // librustjni.so をロード
    }

    // スタティックメソッド(_class: JClass)
    private external fun helloWorld(): String

    fun staticMethod() {
        Log.d(TAG, "return: ${helloWorld()}")
    }

    fun instanceMethod() {
        Log.d(TAG, "return: ${INSTANCE.helloWorld()}")
    }
}

class NativeLibInstance {
    private val TAG = NativeLib::class.java.simpleName

    // インスタンスメソッド(_this: JObject)
    external fun helloWorld(): String
}
