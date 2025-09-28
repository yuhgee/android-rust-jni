import java.util.Properties

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

val localProperties = Properties()
val localPropertiesFile = rootProject.file("local.properties")
if (localPropertiesFile.exists()) {
    localProperties.load(localPropertiesFile.inputStream())
} else {
    println("⚠️ local.properties not found!")
}
val cargoPath: String? = localProperties.getProperty("cargoPath")

val rustBuild = tasks.register("rustBuild") {
    group = "build"
    description = "Build Rust via cargo-ndk"

    doLast {
        exec {
            workingDir = file("../../rustjni")
            executable = cargoPath
            args = listOf(
                "clean",
            )
        }
        exec {
            workingDir = file("../../rustjni")
            executable = cargoPath
            args = listOf(
                "ndk",
                "-t", "arm64-v8a",
                "-o", "../rustjniapp/jnilib/src/main/jniLibs",
                "build"
            )
        }
    }
}

// Android ビルド時に必ず Rust ビルドも実行
tasks.named("preBuild") {
    dependsOn(rustBuild)
}
