package com.senyoshu.app

import android.content.Intent
import android.net.Uri
import android.net.VpnService
import android.webkit.JavascriptInterface
import android.widget.Toast
import androidx.activity.ComponentActivity


class ExportToJavascript(private val context: ComponentActivity) {
    @JavascriptInterface
    fun launch() {
        val intent = VpnService.prepare(context)
        if (intent != null) {
            Toast.makeText(context, "request permission", Toast.LENGTH_SHORT).show()
            context.startActivityForResult(intent, 0)
        } else {
            SurfingService.stopSurfingService(context)
            context.startService(Intent(context, SurfingService::class.java))
        }
    }

    @JavascriptInterface
    fun config() {
        context.startActivity(Intent(context, SurfingActivity::class.java))
    }

    @JavascriptInterface
    fun select() {
        context.startActivity(Intent(context, SurfingSelectActivity::class.java))
    }

    @JavascriptInterface
    fun stop() {
        SurfingService.stopSurfingService(context)
    }


    @JavascriptInterface
    fun openUrl(url: String) {
        val uri = Uri.parse(url)
        val intent = Intent(Intent.ACTION_VIEW, uri)
        context.startActivity(intent)
    }

    @JavascriptInterface
    fun isSurfing(): Boolean {
        return uniffi.native.isSurfing()
    }


}