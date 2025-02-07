package com.senyoshu.app


import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.Service
import android.content.Context
import android.content.Intent
import android.net.VpnService
import android.os.ParcelFileDescriptor
import android.util.Log
import android.widget.RemoteViews
import androidx.core.app.NotificationCompat
import uniffi.native.ConfigLite
import uniffi.native.stopSurfing
import uniffi.native.surfing
import java.io.FileOutputStream


class SurfingService : VpnService() {

    companion object {
        private var tunDevice: ParcelFileDescriptor? = null

        fun stopSurfingService(context: Context) {
            val notificationManager = context.getSystemService(NotificationManager::class.java)
            notificationManager.cancel(NOTIFICATION_ID)
            stopSurfing()
            try {
                tunDevice?.close()
            } catch (e: Exception) {
                e.printStackTrace()
            }
            tunDevice = null
        }
    }


    override fun onCreate() {
        super.onCreate()
        val input = assets.open("acl.txt")
        val output = FileOutputStream("$filesDir/acl.txt")
        input.copyTo(output)
        input.close()
        output.close()
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        if (tunDevice == null) {
            val builder: Builder =
                Builder().setMtu(1500).addAddress("10.0.0.1", 24).addDnsServer("1.1.1.1")
                    .addRoute("0.0.0.0", 0)

            val sp = getOrInitSurfingConfig(this)

            for (pack in sp.all) {
                try {
                    if (pack.key != packageName) {
                        builder.addAllowedApplication(pack.key)
                    }
                } catch (e: Exception) {
                    e.printStackTrace()
                }
            }

            tunDevice = builder.establish()
            val tunDevice = tunDevice!!

            try {
                surfing(
                    ConfigLite(
                        "$filesDir/acl.txt",
                        tunDevice.detachFd()
                    )
                )


            } catch (e: Exception) {
                Log.e("SurfingService", e.toString())
            }
        }


        val notificationLayout = RemoteViews(this.packageName, R.layout.notification_save)
        val notificationManager = getSystemService(NotificationManager::class.java)
        notificationManager.createNotificationChannel(channel)
        val customNotification = NotificationCompat.Builder(this, CHANNEL_ID)
            .setSmallIcon(R.drawable.ic_launcher_background)
            .setCustomContentView(notificationLayout)
            .setPriority(NotificationCompat.PRIORITY_DEFAULT).setOngoing(true) //不可划掉
            .build()
        notificationManager.notify(NOTIFICATION_ID, customNotification)

        return Service.START_STICKY
    }

    private val channel: NotificationChannel by lazy {
        NotificationChannel(CHANNEL_ID, CHANNEL_NAME, NotificationManager.IMPORTANCE_LOW).also {
            it.enableLights(false)
            it.enableVibration(false)
            it.setSound(null, null)
        }
    }

    override fun onDestroy() {
        super.onDestroy()
        stopSurfingService(this)
    }


}

const val CHANNEL_NAME = "save"
const val CHANNEL_ID = "save_it"
const val NOTIFICATION_ID = 0
const val CONSTANT_EXTRA_LEVEL = "level"



