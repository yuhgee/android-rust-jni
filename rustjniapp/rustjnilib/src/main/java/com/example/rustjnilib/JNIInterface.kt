package com.example.rustjnilib

import android.util.Log

object JNIInterface {
    private val TAG = JNIInterface::class.java.simpleName

    fun displayMessage(message:String) {
        Log.d(TAG, "message:${message}")
    }
}