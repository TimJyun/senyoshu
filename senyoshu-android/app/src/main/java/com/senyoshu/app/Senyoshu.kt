package com.senyoshu.app

import android.app.Application
import uniffi.native.init


class Senyoshu : Application() {
    override fun onCreate() {
        super.onCreate()
        init(filesDir.path)
    }
}