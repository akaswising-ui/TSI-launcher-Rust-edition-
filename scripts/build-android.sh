#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUTPUT="$ROOT/build/TSI-Orbit-Rust-ARM64.apk"

export ANDROID_HOME="${ANDROID_HOME:-${ANDROID_SDK_ROOT:-$HOME/android-sdk}}"
export ANDROID_SDK_ROOT="$ANDROID_HOME"
export ANDROID_NDK_HOME="${ANDROID_NDK_HOME:-$ANDROID_HOME/ndk/29.0.14206865}"
export ANDROID_NDK_ROOT="$ANDROID_NDK_HOME"

cargo apk build --manifest-path "$ROOT/Cargo.toml" --lib --target aarch64-linux-android

SOURCE_APK="$(find "$ROOT/target" -type f -name "*.apk" -print -quit)"
if [[ -z "$SOURCE_APK" || ! -s "$SOURCE_APK" ]]; then
  echo "Launcher APK was not created." >&2
  exit 2
fi

mkdir -p "$ROOT/build"
cp "$SOURCE_APK" "$OUTPUT"
echo "Built TSI Orbit launcher APK: $OUTPUT"
