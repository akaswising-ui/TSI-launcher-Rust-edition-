#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ENGINE="$ROOT/vendor/horizon-android"
OUTPUT="$ROOT/build/TSI-Orbit-Game-ARM64.apk"

if [[ ! -f "$ENGINE/android/build.sh" ]]; then
  echo "Pinned engine submodule is missing. Clone with: git submodule update --init --recursive" >&2
  exit 2
fi

export ANDROID_HOME="${ANDROID_HOME:-${ANDROID_SDK_ROOT:-$HOME/android-sdk}}"
export ANDROID_SDK_ROOT="$ANDROID_HOME"
export ANDROID_NDK_HOME="${ANDROID_NDK_HOME:-$ANDROID_HOME/ndk/29.0.14206865}"
export ANDROID_NDK_ROOT="$ANDROID_NDK_HOME"
export HORIZON_ANDROID_PROFILE="${HORIZON_ANDROID_PROFILE:-android}"
export HORIZON_ANDROID_JOBS="${HORIZON_ANDROID_JOBS:-2}"

python3 "$ROOT/scripts/prepare-horizon-engine.py" "$ENGINE"

(
  cd "$ENGINE"
  ./android/build.sh
)

VERSION="$(sed -n 's/^version = "\(.*\)"/\1/p' "$ENGINE/crates/horizon-client/Cargo.toml" | head -n 1)"
SOURCE_APK="$ENGINE/android/build/horizon-$VERSION-android-arm64.apk"
if [[ ! -s "$SOURCE_APK" ]]; then
  echo "Expected game APK was not created: $SOURCE_APK" >&2
  exit 3
fi

mkdir -p "$(dirname "$OUTPUT")"
cp "$SOURCE_APK" "$OUTPUT"
echo "Built playable Horizon Java Edition game client APK: $OUTPUT"
