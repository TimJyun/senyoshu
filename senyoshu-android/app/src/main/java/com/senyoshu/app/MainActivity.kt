package com.senyoshu.app

import android.annotation.SuppressLint
import android.content.pm.ApplicationInfo
import android.os.Bundle
import android.util.Log
import android.webkit.WebResourceRequest
import android.webkit.WebResourceResponse
import android.webkit.WebSettings
import android.webkit.WebView
import android.webkit.WebViewClient
import android.widget.Toast
import androidx.activity.ComponentActivity
import androidx.activity.OnBackPressedCallback
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.ui.Modifier
import androidx.compose.ui.viewinterop.AndroidView
import java.net.URLConnection


class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        if (0 != (applicationInfo.flags and ApplicationInfo.FLAG_DEBUGGABLE)) {
            WebView.setWebContentsDebuggingEnabled(true)
        }
        
        val assetManager = this.assets
        val myWebView = WebView(this)
        @SuppressLint("WrongThread", "SetJavaScriptEnabled")
        myWebView.settings.javaScriptEnabled = true
        myWebView.settings.domStorageEnabled = true
        myWebView.settings.mediaPlaybackRequiresUserGesture = false
        myWebView.settings.cacheMode = WebSettings.LOAD_NO_CACHE

        myWebView.addJavascriptInterface(ExportToJavascript(this), "android")
        myWebView.addJavascriptInterface(SpeechSynthesis(this), "tts")

        myWebView.webViewClient = object : WebViewClient() {
            override fun shouldInterceptRequest(
                view: WebView, request: WebResourceRequest
            ): WebResourceResponse? {
                return if (request.url.path?.startsWith("/api/") == true) {
                    Log.d("WebViewClient", "call api : " + request.url.path)
                    super.shouldInterceptRequest(view, request)
                } else if (!request.url.host.equals("senyoshu.com", false)) {
                    Log.d("WebViewClient", "host error : " + request.url.host)
                    return null
                } else {
                    val path: String = request.url.path ?: "/index.html"
                    val mineType = when {
                        path.endsWith(".wasm") -> {
                            "application/wasm"
                        }
                        path.endsWith(".avif") -> {
                            "image/avif"
                        }
                        else -> {
                            URLConnection.guessContentTypeFromName(path) ?: "text/html"
                        }
                    }
                    try {
                        WebResourceResponse(
                            mineType, "utf-8", assetManager.open("dist$path")
                        )
                    } catch (e: Exception) {
                        Log.e("WebViewClient", e.toString())
                        try {
                            WebResourceResponse(
                                "text/html",
                                "utf-8",
                                assetManager.open("dist/index.html")
                            )
                        } catch (e: Exception) {
                            e.printStackTrace()
                            null
                        }
                    }
                }
            }
        }



        myWebView.loadUrl("https://senyoshu.com/home")

        setContent {
            AndroidView(
                modifier = Modifier.fillMaxSize(),
                factory = { myWebView }
            ) {}
        }

        onBackPressedDispatcher.addCallback(this, object : OnBackPressedCallback(true) {
            override fun handleOnBackPressed() {
                System.currentTimeMillis()
                if (System.currentTimeMillis() - lastClickTime > 1200) {
                    if (!myWebView.canGoBack()) {
                        lastClickTime = System.currentTimeMillis()
                        Toast.makeText(this@MainActivity, "再按一遍退出应用", Toast.LENGTH_SHORT)
                            .show()
                    } else {
                        myWebView.goBack()
                    }
                } else {
                    finish()
                }
            }
        })
    }

    private var lastClickTime = 0L
}


//fun copyAssetFolder(
//    assetManager: AssetManager, fromAssetPath: String, toPath: String
//): Boolean {
//    val files = assetManager.list(fromAssetPath)!!
//    File(toPath).mkdirs()
//    var result = true
//
//    for (file in files) {
//        if (file.contains(".")) {
//            try {
//                val input = assetManager.open("$fromAssetPath/$file")
//                File(toPath).createNewFile()
//                val out = FileOutputStream("$toPath/$file")
//                input.copyTo(out, 8192)
//                input.close()
//                out.flush()
//                out.close()
//            } catch (e: Exception) {
//                e.printStackTrace()
//                result = false
//            }
//        } else {
//            if (!copyAssetFolder(assetManager, "$fromAssetPath/$file", "$toPath/$file")) {
//                result = false
//            }
//        }
//    }
//    return result
//}