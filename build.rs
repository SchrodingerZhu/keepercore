fn main() {
    println!("cargo:rustc-link-search=native={}", "/opt/android-ndk/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib/aarch64-linux-android/libc++abi.a");
    println!("cargo:rustc-link-lib=static=c++abi");
    println!("cargo:rustc-link-search=native={}", "/opt/android-ndk/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib/aarch64-linux-android/libc++_static.a");
    println!("cargo:rustc-link-lib=static=c++_static");
    println!("cargo:rustc-link-search=native={}", "/opt/android-ndk/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib/aarch64-linux-android/libm.a");
    println!("cargo:rustc-link-lib=static=m");
    println!("cargo:rustc-link-search=native={}", "/opt/android-libs/aarch64/lib/");
    println!("cargo:rustc-link-lib=static=crypto");
    println!("cargo:rustc-link-lib=static=ssl");
}