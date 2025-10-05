package com.example.rustjnilib.cell

import kotlinx.serialization.Serializable

@Serializable
data class Accelerometer(
    override val timestamp: Long,
    val x: Float,
    val y: Float,
    val z: Float
) : SensorData()