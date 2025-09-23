package com.example.rustjnilib.cell
import android.Manifest
import android.content.Context
import android.content.pm.PackageManager
import android.telephony.*
import androidx.core.app.ActivityCompat

class CellTowerManager(private val context: Context) {

    fun getCellTowers(): List<CellTowerInfo> {
        val result = mutableListOf<CellTowerInfo>()

        val telephonyManager =
            context.getSystemService(Context.TELEPHONY_SERVICE) as TelephonyManager

        if (ActivityCompat.checkSelfPermission(
                context,
                Manifest.permission.ACCESS_FINE_LOCATION
            ) != PackageManager.PERMISSION_GRANTED
        ) {
            // パーミッションが無ければ空リスト
            return result
        }

        val cellInfoList = telephonyManager.allCellInfo
        cellInfoList?.forEach { cellInfo ->
            when (cellInfo) {
                is CellInfoLte -> {
                    val id = cellInfo.cellIdentity
                    val sig = cellInfo.cellSignalStrength
                    result.add(
                        CellTowerInfo(
                            type = "LTE",
                            mcc = id.mcc,
                            mnc = id.mnc,
                            tac = id.tac,
                            cellId = id.ci.toLong(),
                            pci = id.pci,
                            signalLevel = sig.level
                        )
                    )
                }

                is CellInfoGsm -> {
                    val id = cellInfo.cellIdentity
                    val sig = cellInfo.cellSignalStrength
                    result.add(
                        CellTowerInfo(
                            type = "GSM",
                            mcc = id.mcc,
                            mnc = id.mnc,
                            tac = id.lac,
                            cellId = id.cid.toLong(),
                            pci = null,
                            signalLevel = sig.level
                        )
                    )
                }

                is CellInfoWcdma -> {
                    val id = cellInfo.cellIdentity
                    val sig = cellInfo.cellSignalStrength
                    result.add(
                        CellTowerInfo(
                            type = "WCDMA",
                            mcc = id.mcc,
                            mnc = id.mnc,
                            tac = id.lac,
                            cellId = id.cid.toLong(),
                            pci = id.psc,
                            signalLevel = sig.level
                        )
                    )
                }

                is CellInfoNr -> { // 5G
                    val id = cellInfo.cellIdentity as CellIdentityNr
                    val sig = cellInfo.cellSignalStrength
                    result.add(
                        CellTowerInfo(
                            type = "NR",
                            mcc = id.mccString?.toIntOrNull(),
                            mnc = id.mncString?.toIntOrNull(),
                            tac = id.tac,
                            cellId = id.nci,
                            pci = id.pci,
                            signalLevel = sig.level
                        )
                    )
                }
            }
        }

        return result
    }
}
