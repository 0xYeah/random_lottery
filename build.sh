#!/bin/bash
# Rust cross-platform build script — 3 platforms × 2 architectures = 6 binaries
# Tools required: cargo, cargo-zigbuild, zig, zip
# Install: cargo install cargo-zigbuild && brew install zig
set -e
set -x

# ── Config ────────────────────────────────────────────────────────────────────

PRODUCT_NAME=$(grep '^name' Cargo.toml | head -1 | awk -F '"' '{print $2}')
CURRENT_VERSION=$(grep '^version' Cargo.toml | head -1 | awk -F '"' '{print $2}')

BUILD_PATH=./build
UPLOAD_TMP_DIR=./build/upload_tmp_dir
RUN_MODE=release

# Glibc minimum version for Linux targets (broad compatibility)
GLIBC_MIN="2.17"

# ── Target table: "rust_target:os_label:arch_label:ext:use_zigbuild" ──────────
# use_zigbuild=1 → cargo zigbuild   use_zigbuild=0 → cargo build (native)
declare -a TARGETS=(
    "aarch64-apple-darwin:macos:arm64::0"
    "x86_64-apple-darwin:macos:x86_64::0"
    "aarch64-unknown-linux-gnu:linux:arm64::1"
    "x86_64-unknown-linux-gnu:linux:x86_64::1"
    "x86_64-pc-windows-gnu:windows:x86_64:.exe:1"
    "aarch64-pc-windows-gnullvm:windows:arm64:.exe:1"
)

# ── Helpers ───────────────────────────────────────────────────────────────────

check_deps() {
    local missing=0
    for cmd in cargo zip git; do
        command -v "$cmd" &>/dev/null || { echo "❌ missing: $cmd"; missing=1; }
    done
    command -v cargo-zigbuild &>/dev/null || {
        echo "❌ cargo-zigbuild not found. Run: cargo install cargo-zigbuild && brew install zig"
        missing=1
    }
    command -v zig &>/dev/null || {
        echo "❌ zig not found. Run: brew install zig"
        missing=1
    }
    if [[ $missing -eq 1 ]]; then exit 1; fi
}

is_target_installed() {
    rustup target list --installed 2>/dev/null | grep -qx "$1"
}

build_target() {
    local rust_target="$1"
    local os_label="$2"
    local arch_label="$3"
    local ext="$4"
    local use_zig="$5"

    if ! is_target_installed "$rust_target"; then
        echo "⚠️  target $rust_target not installed — skipping (run: rustup target add $rust_target)"
        return 0
    fi

    echo "▶ building $rust_target ($os_label/$arch_label) ..."

    local mode_flag=""
    [[ "$RUN_MODE" == "release" ]] && mode_flag="--release"
    local cargo_flags="$mode_flag --target $rust_target"

    if [[ "$use_zig" == "1" ]]; then
        local zig_target="$rust_target"
        # Linux targets use glibc version suffix for broad compatibility
        [[ "$os_label" == "linux" ]] && zig_target="${rust_target}.${GLIBC_MIN}"
        cargo zigbuild $mode_flag --target "$zig_target"
    else
        cargo build $cargo_flags
    fi

    local bin_src="target/${rust_target}/${RUN_MODE}/${PRODUCT_NAME}${ext}"
    local out_dir="${BUILD_PATH}/${RUN_MODE}/${os_label}_${arch_label}"
    mkdir -p "$out_dir"
    cp "$bin_src" "$out_dir/${PRODUCT_NAME}${ext}"

    # macOS binaries are merged into universal — skip individual zips
    if [[ "$os_label" == "macos" ]]; then
        echo "✔ $rust_target built (will merge into universal)"
        return 0
    fi

    local zip_name="${PRODUCT_NAME}_${os_label}_${arch_label}_v${CURRENT_VERSION}.zip"
    mkdir -p "$UPLOAD_TMP_DIR"
    (cd "$out_dir" && zip -r "../../upload_tmp_dir/${zip_name}" .)
    echo "✅ $zip_name"
}

make_universal_mac() {
    local arm_bin="${BUILD_PATH}/${RUN_MODE}/macos_arm64/${PRODUCT_NAME}"
    local x86_bin="${BUILD_PATH}/${RUN_MODE}/macos_x86_64/${PRODUCT_NAME}"
    if [[ ! -f "$arm_bin" || ! -f "$x86_bin" ]]; then
        echo "⚠️  skipping universal — one or both macOS binaries missing"
        return 0
    fi
    local out_dir="${BUILD_PATH}/${RUN_MODE}/macos_universal"
    mkdir -p "$out_dir"
    lipo -create "$arm_bin" "$x86_bin" -output "${out_dir}/${PRODUCT_NAME}"
    local zip_name="${PRODUCT_NAME}_macos_universal_v${CURRENT_VERSION}.zip"
    (cd "$out_dir" && zip -r "../../upload_tmp_dir/${zip_name}" .)
    echo "✅ $zip_name"
}

# ── Main ──────────────────────────────────────────────────────────────────────

handle_mode() {
    case "$1" in
        release|"") RUN_MODE=release ;;
        debug)      RUN_MODE=debug ;;
        *)
            echo "Usage: bash build_rs_project.sh [release|debug]"
            exit 1
            ;;
    esac
}

main() {
    handle_mode "$1"
    check_deps

    local commit_hash build_time
    commit_hash=$(git show -s --format=%H 2>/dev/null || echo "unknown")
    build_time=$(date +"%Y-%m-%d_%H:%M:%S")

    echo "════════════════════════════════════════"
    echo "  ${PRODUCT_NAME} v${CURRENT_VERSION}"
    echo "  mode:   ${RUN_MODE}"
    echo "  commit: ${commit_hash:0:8}"
    echo "  time:   ${build_time}"
    echo "════════════════════════════════════════"

    rm -rf "${BUILD_PATH}/${RUN_MODE}"
    mkdir -p "$UPLOAD_TMP_DIR"

    for entry in "${TARGETS[@]}"; do
        IFS=':' read -r rust_target os_label arch_label ext use_zig <<< "$entry"
        build_target "$rust_target" "$os_label" "$arch_label" "$ext" "$use_zig" || \
            echo "❌ $rust_target failed — skipping"
    done

    make_universal_mac

    echo ""
    echo "════════════════════════════════════════"
    echo "  packages → ${UPLOAD_TMP_DIR}/"
    ls -1 "$UPLOAD_TMP_DIR"/*.zip 2>/dev/null | xargs -I{} basename {}
    echo "════════════════════════════════════════"
}

main "$@"
