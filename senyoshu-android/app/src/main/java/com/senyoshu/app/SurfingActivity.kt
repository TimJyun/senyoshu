package com.senyoshu.app

import android.content.Context
import android.content.SharedPreferences
import android.content.pm.ApplicationInfo
import android.content.pm.PackageManager
import android.os.Bundle
import android.util.Log
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.Image
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material3.Checkbox
import androidx.compose.material3.Text
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateMapOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.asImageBitmap
import androidx.compose.ui.graphics.painter.BitmapPainter
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import androidx.core.graphics.drawable.toBitmap
import java.util.stream.Collectors


class SurfingActivity : ComponentActivity() {


    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)


        val sp = getOrInitSurfingConfig(this)

        setContent {
            var showSystemApp by remember { mutableStateOf(false) }
            var packageList by remember {
                val packageManager = packageManager
                var packageList =
                    packageManager.getInstalledPackages(PackageManager.GET_PERMISSIONS)
                packageList = packageList.stream().filter {
                    return@filter (it.applicationInfo.flags.and(ApplicationInfo.FLAG_SYSTEM)) == 0
                }.collect(Collectors.toList())
                packageList.sortBy { it.packageName }
                mutableStateOf(packageList)
            }

            val hashMap = remember {
                val map = mutableStateMapOf<String, Boolean>()
                for (i in sp.all) {
                    if (i.value == true) {
                        map[i.key] = true
                    }
                }
                map
            }
            Column {
                Row(
                    Modifier.height(48.dp),
                    verticalAlignment = Alignment.CenterVertically
                ) {

                    Image(
                        painter = painterResource(R.mipmap.icons8_back),
                        contentDescription = "back_key",
                        modifier = Modifier.clickable {
                            finish()
                        }
                    )
                    Column(Modifier.weight(1F)) { }
                    Text("显示系统应用")
                    Checkbox(
                        checked = showSystemApp,
                        onCheckedChange = {
                            showSystemApp = !showSystemApp
                            val packageManager = packageManager
                            var packageListNew =
                                packageManager.getInstalledPackages(PackageManager.GET_PERMISSIONS)
                            if (!showSystemApp) {
                                packageListNew = packageListNew.stream().filter {
                                    return@filter (it.applicationInfo.flags.and(ApplicationInfo.FLAG_SYSTEM)) == 0
                                }.collect(Collectors.toList())
                            }
                            packageListNew.sortBy { it.packageName }
                            packageList = packageListNew
                        })


                }
                Row(
                    Modifier.weight(1F)
                ) {
                    LazyColumn(
                        modifier = Modifier.fillMaxSize(),
//                        verticalArrangement = Arrangement.Center
                    ) {
                        items(packageList.size) {
                            val packageInfo = packageList[it]
                            Row {
                                Column {
                                    val drawable =
                                        packageInfo.applicationInfo.loadIcon(this@SurfingActivity.packageManager)
                                    val bitmap = drawable.toBitmap(48, 48).asImageBitmap()
                                    Image(
                                        painter = BitmapPainter(bitmap),
                                        contentDescription = "icon:" + packageInfo.packageName,
                                        modifier = Modifier
                                            .height(Dp(48F))
                                            .width(Dp(48F))
                                    )
                                }
                                Column(modifier = Modifier.weight(1F)) {
                                    Row {
                                        Text(text = packageInfo.applicationInfo.loadLabel(this@SurfingActivity.packageManager)
                                            .toString().let {
                                                if (it == "Filled") {
                                                    ""
                                                } else {
                                                    it
                                                }
                                            })
                                    }
                                    Row {
                                        Text(
                                            text = packageInfo.packageName,
                                        )
                                    }
                                }
                                Column {
                                    Checkbox(checked = hashMap[packageInfo.packageName] == true,
                                        onCheckedChange = {
                                            Log.d("SurfingActivity", "onCheckedChange: $it")
                                            if (it) {
                                                sp.edit().putBoolean(packageInfo.packageName, true)
                                                    .apply()
                                                hashMap[packageInfo.packageName] = true
                                            } else {
                                                sp.edit().remove(packageInfo.packageName).apply()
                                                hashMap.remove(packageInfo.packageName)
                                            }
                                        })
                                }
                            }
                        }
                    }
                }

            }
        }
    }

}


fun getOrInitSurfingConfig(context: Context): SharedPreferences {
    val sp = context.getSharedPreferences("surfing_config", Context.MODE_PRIVATE)

    if (sp.all.isEmpty()) {
        val packageManager = context.packageManager
        val packageList = packageManager.getInstalledPackages(PackageManager.GET_PERMISSIONS)
        packageList.sortBy { it.packageName }
        sp.edit().also {
            it.putBoolean("org.mozilla.firefox", true)
            it.putBoolean("org.telegram.messenger", true)
            it.putBoolean("im.vector.app", true)
            it.putBoolean("jp.pxv.android", true)
            it.putBoolean("moe.tarsin.ehviewer", true)
            for (pkgs in packageList) {
                if (pkgs.packageName.startsWith("com.google")) {
                    it.putBoolean(pkgs.packageName, true)
                }
            }
        }.apply()
    }
    return sp
}