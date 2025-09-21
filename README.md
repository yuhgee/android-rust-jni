# android-rust-jni

This repository contains an Android project integrated with a Rust-based JNI library.

## Repository Structure

```text
GitRoot/
├─ rustjniapp/   ← Android Studio project
│  ├─ app/...
│  ├─ jnilibs/...
│  └─ settings.gradle.kts
├─ rustjni/      ← Rust project for JNI library
│  ├─ Cargo.toml
│  └─ src/
├─ .gitignore
├─ LICENSE
└─ README.md
```

## Build Instructions

### Rust
Build the Rust library for Android targets:

```bash
cd rust
cargo build --target=<android-target>
```

Copy the generated `.so` files to:

```
MyAndroidApp/app/src/main/jniLibs/<abi>/
```

### Android
Open `MyAndroidApp` in Android Studio and build the app as usual.

## License
This project is licensed under the MIT License.
