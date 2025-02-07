package com.senyoshu.app

import android.content.Context
import android.os.Bundle
import android.speech.tts.TextToSpeech
import android.speech.tts.UtteranceProgressListener
import android.speech.tts.Voice
import android.util.Log
import android.webkit.JavascriptInterface
import android.widget.Toast
import com.google.gson.Gson
import java.util.Locale
import java.util.stream.Collectors


class SpeechSynthesis(private val context: Context) : TextToSpeech.OnInitListener {
    private val tts: TextToSpeech = TextToSpeech(context, this)
    override fun onInit(status: Int) {
        if (status == TextToSpeech.SUCCESS) {
            // TTS engine is successfully initialized.
            tts.language = Locale.JAPANESE
            tts.setOnUtteranceProgressListener(object : UtteranceProgressListener() {
                override fun onStart(utteranceId: String?) {}
                override fun onDone(utteranceId: String?) {
                    hashMap.set(utteranceId!!, true)
                }

                override fun onError(utteranceId: String?) {}
            })


        } else {
            // Failed to initialize TTS engine.
            Toast.makeText(context, "Init failed", Toast.LENGTH_SHORT).show()
        }
    }


    private val voices by lazy {
        tts.voices.stream()
            .filter { it.locale == Locale.JAPANESE || it.locale == Locale.JAPAN }
            .filter { !it.isNetworkConnectionRequired }
            .filter { !it.name.endsWith("-network") }
            .collect(Collectors.toList())
    }


    @JavascriptInterface
    fun getVoices(): String {
        val list = voices.stream()
            .map { it.name }
            .collect(Collectors.toList())
        val json = Gson().toJson(list)
        Log.d("TTS", "getVoices-len: " + list.size)
        return json
    }


    private var last = 0
    private val hashMap = HashMap<String, Boolean>()

    @JavascriptInterface
    fun speakWithSpeaker(
        text: String,
        speaker: String?,
        volume: Float,
    ): String {
        Log.d("TTS", "try speak")

        var voice: Voice? = null;
        if (speaker == null) {
            voice = voices.randomOrNull()
        } else {
            for (voiceTmp in voices.stream()) {
                if (voiceTmp.name.equals(speaker, false)) {
                    voice = voiceTmp
                    break
                }
            }
            if(voice==null){
                voice=voices.randomOrNull()
                Log.w("speakWithSpeaker","speaker not found, name: $speaker")
            }
        }
        if (voice == null) {
            return ""
        }

        if (tts.voice != voice) {
            tts.voice = voice
        }
        val params = Bundle()
        params.putFloat(TextToSpeech.Engine.KEY_PARAM_VOLUME, volume)
        params.putInt(TextToSpeech.Engine.KEY_FEATURE_NETWORK_TIMEOUT_MS, 0)
        params.putInt(TextToSpeech.Engine.KEY_FEATURE_NETWORK_RETRIES_COUNT, 0)
        this.last += 1
        tts.speak(text, TextToSpeech.QUEUE_ADD, params, "$last")
        return "$last"


    }

    @JavascriptInterface
    fun isDone(utteranceId: String): Boolean {
        if (this.hashMap.get(utteranceId) == true) {
            this.hashMap.remove(utteranceId)
            Log.d("TTS", "speak done")
            return true
        } else if (utteranceId.isBlank()) {
            return true
        } else {
            return false
        }
    }
}

