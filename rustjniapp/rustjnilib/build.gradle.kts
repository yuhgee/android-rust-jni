plugins {
    alias(libs.plugins.android.library)
    alias(libs.plugins.kotlin.android)
}

android {
    namespace = "com.example.rustjnilib"
    compileSdk = 36

    defaultConfig {
        minSdk = 31

        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
        consumerProguardFiles("consumer-rules.pro")
    }

    buildTypes {
        release {
            isMinifyEnabled = false
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_11
        targetCompatibility = JavaVersion.VERSION_11
    }
    kotlinOptions {
        jvmTarget = "11"
    }
}

dependencies {

    implementation(libs.androidx.core.ktx)
    implementation(libs.androidx.appcompat)
    implementation(libs.material)
    testImplementation(libs.junit)
    androidTestImplementation(libs.androidx.junit)
    androidTestImplementation(libs.androidx.espresso.core)
}

val rustBuild = tasks.register("rustBuild") {
    group = "build"
    description = "Build Rust via cargo-ndk"

    doLast {
        exec {
            workingDir = file("../../rustjni")
            executable = "/Users/user_path/.cargo/bin/cargo"
            args = listOf(
                "ndk",
                "-t", "arm64-v8a",
                "-o", "../rustjniapp/rustjnilib/src/main/jniLibs",
                "build"
            )
        }
    }
}

// Android ビルド時に必ず Rust ビルドも実行
tasks.named("preBuild") {
//    dependsOn(rustBuild)
}
