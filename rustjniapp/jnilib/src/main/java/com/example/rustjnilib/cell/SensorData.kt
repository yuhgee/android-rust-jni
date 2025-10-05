package com.example.rustjnilib.cell

import kotlinx.serialization.Serializable

@Serializable
sealed class SensorData {
    abstract val timestamp: Long
}