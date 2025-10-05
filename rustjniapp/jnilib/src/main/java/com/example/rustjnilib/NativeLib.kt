package com.example.rustjnilib

import android.content.Context
import android.util.Log
import com.example.rustjnilib.cell.Accelerometer
import com.example.rustjnilib.cell.CellTowerInfo
import com.example.rustjnilib.cell.CellTowerManager
import kotlinx.serialization.*
import kotlinx.serialization.json.*

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

    external fun initLogger()

    private external fun initialize(context: Context)
    private external fun finalize()
    private external fun start()
    private external fun stop()
    private external fun addData(value: Long)

    // ---------------------------------------
    //

    // 確認用だから同期にしてあるが、非同期になる。
    fun testService() {
        Log.d(TAG, "start!")
        // Rust サービス起動
        start()

        // サービス起動中
        Thread.sleep(10000)

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
//        Log.d(TAG, "Json: $ret")
        return ret
    }

    @JvmStatic
    fun getAccelerometer(): String {
        val value = Accelerometer(
            1_000_000L,
            10.0F, 20.0F, 30.0F,
        )
        val json = Json.encodeToString(value)
        val ret = json.toString()
//        Log.d(TAG, "Json: $ret")
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
