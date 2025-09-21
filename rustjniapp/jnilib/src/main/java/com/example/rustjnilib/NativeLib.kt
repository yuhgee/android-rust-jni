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
        initLogger()
        Log.d(TAG, "return: ${helloWorld()} ---")
    }

    fun instanceMethod() {
        Log.d(TAG, "return: ${INSTANCE.helloWorld()}")
    }

    external fun initLogger()

    external fun start()
    external fun stop()
    external fun addData(value: Long)

    fun testService() {
        Log.d(TAG, "start!")
        // Rust サービス起動
        start()

        // データ追加
        addData(42)
        addData(100)

        // 少し待って確認
        Thread.sleep(2000)

        // サービス停止
        stop()
        Log.d(TAG, "stop!")
    }
}

class NativeLibInstance {
    private val TAG = NativeLib::class.java.simpleName

    // インスタンスメソッド(_this: JObject)
    external fun helloWorld(): String
}
