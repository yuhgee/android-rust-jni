package com.example.rustjnilib

import android.content.Context
import android.util.Log
import com.example.rustjnilib.cell.CellTowerInfo
import com.example.rustjnilib.cell.CellTowerManager

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

    // ---------------------------------------
    // JNI method
    private external fun initLogger()

    private external fun initialize(context: Context)
    private external fun finalize()
    private external fun start()
    private external fun stop()
    private external fun addData(value: Long)

    // ---------------------------------------
    //

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

    fun initializeLib(context: Context) {
        initialize(context) // Rust に Context を渡す
    }

    // called from Rust
    @JvmStatic
    fun getCellTowerInfo(context: Context): String {
        val cellManager = CellTowerManager(context)
        val cellTowers = cellManager.getCellTowers()
        val ret = cellTowers.toJson()
        Log.d(TAG, "Json: $ret")
        return ret
    }

    // JSON変換用 拡張関数
    private fun List<CellTowerInfo>.toJson(): String {
        val array = org.json.JSONArray()
        this.forEach { tower ->
            val obj = org.json.JSONObject()
            obj.put("type", tower.type)
            obj.put("mcc", tower.mcc)
            obj.put("mnc", tower.mnc)
            obj.put("tac", tower.tac)
            obj.put("cellId", tower.cellId)
            obj.put("pci", tower.pci)
            obj.put("signalLevel", tower.signalLevel)
            array.put(obj)
        }
        return array.toString()
    }
}

class NativeLibInstance {
    private val TAG = NativeLib::class.java.simpleName

    // インスタンスメソッド(_this: JObject)
    external fun helloWorld(): String
}
