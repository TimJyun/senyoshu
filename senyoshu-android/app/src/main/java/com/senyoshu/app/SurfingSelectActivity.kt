package com.senyoshu.app


import android.os.Bundle
import android.util.Log
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.Image
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.MutableState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.sync.Mutex
import uniffi.native.ServerChangeHandler
import uniffi.native.SurfServerExport
import uniffi.native.getCurrentServer
import uniffi.native.initServersList
import uniffi.native.setCurrentServer
import uniffi.native.setServerChangeHandler


class SurfingSelectActivity : ComponentActivity() {

    private val mutex = Mutex()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            var servers by remember {
                //顺便初始化了
                mutableStateOf<List<SurfServerExport>>(ArrayList())
            }
            val currentServer = remember {
                mutableStateOf(getCurrentServer())
            }

            LaunchedEffect(Unit) {
                setServerChangeHandler(
                    object : ServerChangeHandler {
                        override fun onUpdate(serversNew: List<SurfServerExport>) {
                            Log.d("onCreate", "onServerUpdate")
                            runBlocking { mutex.lock() }
                            try {
                                Log.d("onServerUpdate", "try get servers")
                                servers = serversNew
                                Log.d("onServerUpdate", "servers-len: ${servers.size}")
                                servers.forEach {
                                    Log.d(
                                        "onServerUpdate",
                                        "name: ${it.name} , delay: ${it.delay}"
                                    )
                                }
                            } catch (e: Exception) {
                                e.printStackTrace()
                            }
                            mutex.unlock()

                        }
                    }
                )
                initServersList()
            }

            Column {
                Row(
                    Modifier.height(48.dp),
                    verticalAlignment = Alignment.CenterVertically
                ) {
                    Image(
                        painter = painterResource(R.mipmap.icons8_back),
                        contentDescription = "back_key",
                        modifier = Modifier.clickable { finish() }
                    )
                    Column(Modifier.weight(1F)) { }
                }
                Column {
                    Log.d("onCreate", "refresh: 1")
                    var i = 0
                    while (servers.getOrNull(i) !== null) {
                        val serverLeft = servers.get(i)
                        Row(
                            horizontalArrangement = Arrangement.Center,
                        ) {
                            Column(
                                Modifier.weight(1f),
                                Arrangement.Center,
                                Alignment.CenterHorizontally
                            ) {
                                Log.d("onCreate", "refresh: 2")
                                ServerNode(serverLeft, currentServer)
                            }
                            Column(
                                Modifier.weight(1f),
                                Arrangement.Center,
                                Alignment.CenterHorizontally
                            ) {
                                val serverRight = servers.getOrNull(i + 1)
                                if (serverRight != null) {
                                    Log.d("onCreate", "refresh: 3")
                                    ServerNode(serverRight, currentServer)
                                }
                            }
                        }
                        i += 2
                    }
                }

            }
        }
    }

    override fun onDestroy() {
        super.onDestroy()
        setServerChangeHandler(null)
    }
}

@Composable
fun ServerNode(server: SurfServerExport, currentServer: MutableState<SurfServerExport?>) {
    Column(
        Modifier
            .let {
                if (compareServer(server, currentServer.value)) {
                    it.border(1.dp, Color.Black)
                } else {
                    it
                }
            }
            .fillMaxWidth(0.8f)
            .padding(16.dp)
            .clickable {
                setCurrentServer(server)
                currentServer.value = server
            }) {
        Row { Text(server.name) }
        Row {
            val delay = if (server.delay != null) {
                if (server.delay!! >= 5000) {
                    "Timeout"
                } else {
                    "${server.delay} ms"
                }
            } else {
                "Unknown"
            }
            Text("Delay: $delay")
        }
    }


}


fun compareServer(a: SurfServerExport?, b: SurfServerExport?): Boolean {
    return (a == null && b == null) ||
            (a != null && b != null &&
                    a.server == b.server &&
                    a.serverPort == b.serverPort &&
                    a.password == b.password &&
                    a.method == b.method)
}
