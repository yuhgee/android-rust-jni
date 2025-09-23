package com.example.rustjniapp

import android.Manifest
import android.content.pm.PackageManager
import android.os.Bundle
import android.util.Log
import androidx.activity.ComponentActivity
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.compose.setContent
import androidx.activity.result.contract.ActivityResultContracts
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import com.example.rustjnilib.NativeLib
import com.example.rustjnilib.cell.CellTowerInfo
import com.example.rustjnilib.cell.CellTowerManager

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        NativeLib.initializeLib(this)
        NativeLib.testService()

        setContent {
            MaterialTheme {
                CellTowerScreen()
            }
        }
    }
}

@Composable
fun CellTowerScreen(modifier: Modifier = Modifier) {
    val context = LocalContext.current
    val cellManager = remember { CellTowerManager(context) }

    var cellTowers by remember { mutableStateOf<List<CellTowerInfo>>(emptyList()) }
    // 複数パーミッション用 Launcher
    val permissionsLauncher = rememberLauncherForActivityResult(
        contract = ActivityResultContracts.RequestMultiplePermissions()
    ) { result: Map<String, Boolean> ->
        val allGranted = result.values.all { it }
        if (allGranted) {
            cellTowers = cellManager.getCellTowers()
        } else {
            Log.d("Permissions", "Some permissions denied: $result")
        }
    }

    CellTowerContent(
        cellTowers = cellTowers,
        onRequestData = {
            permissionsLauncher.launch(
                arrayOf(
                    Manifest.permission.ACCESS_FINE_LOCATION,
                    Manifest.permission.READ_PHONE_STATE
                )
            )
        },
        modifier = modifier
    )
}

@Composable
fun CellTowerContent(
    cellTowers: List<CellTowerInfo>,
    onRequestData: () -> Unit,
    modifier: Modifier = Modifier
) {
    Column(
        modifier = modifier
            .fillMaxSize()
            .padding(16.dp)
    ) {
        Button(
            onClick = { onRequestData() },
            modifier = Modifier.fillMaxWidth()
        ) {
            Text("基地局情報を取得")
        }

        Spacer(modifier = Modifier.height(16.dp))

        if (cellTowers.isEmpty()) {
            Text("データなし")
        } else {
            LazyColumn {
                items(cellTowers) { tower ->
                    Card(
                        modifier = Modifier
                            .fillMaxWidth()
                            .padding(vertical = 4.dp),
                        elevation = CardDefaults.cardElevation(4.dp)
                    ) {
                        Column(modifier = Modifier.padding(12.dp)) {
                            Text("Type: ${tower.type}")
                            Text("MCC: ${tower.mcc}, MNC: ${tower.mnc}")
                            Text("TAC/LAC: ${tower.tac}, CellId: ${tower.cellId}")
                            Text("PCI: ${tower.pci}")
                            Text("Signal Level: ${tower.signalLevel}")
                        }
                    }
                }
            }
        }
    }
}

@Preview(showBackground = true)
@Composable
fun PreviewCellTowerScreen() {
    val dummyData = listOf(
        CellTowerInfo("LTE", 440, 10, 12345, 678901, 123, 3),
        CellTowerInfo("NR", 440, 20, 54321, 987654321, 321, 4)
    )
    MaterialTheme {
        CellTowerContent(
            cellTowers = dummyData,
            onRequestData = {},
            modifier = Modifier.fillMaxSize()
        )
    }
}
