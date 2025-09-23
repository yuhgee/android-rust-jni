package com.example.rustjnilib.cell

data class CellTowerInfo(
    val type: String,        // LTE, GSM, WCDMA, NR
    val mcc: Int?,
    val mnc: Int?,
    val tac: Int?,           // LTE/NR: TAC, GSM/WCDMA: LAC
    val cellId: Long?,       // CID, CI, NCI
    val pci: Int?,           // NR/LTE: PCI
    val signalLevel: Int?    // 電波強度レベル (0~4)
)
