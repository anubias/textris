#!/bin/sh

ASSETS="assets"
CURDIR=`pwd`
PACKAGE="package"
TARGET_DIR="target"
ARTIFACT="textris"


generate_artifact() {
    if [ "$@" = "linux" ]; then
        EXECUTABLE="textris"
        TARGET="x86_64-unknown-linux-gnu"
        VARIANT="lin"
    elif [ "$@" = "windows" ]; then
        EXECUTABLE="textris.exe"
        TARGET="x86_64-pc-windows-gnu"
        VARIANT="win"
    else
        return
    fi

    RELEASE_DIR="${TARGET_DIR}/${TARGET}/release"
    ZIP_ARCHIVE="${ARTIFACT}-${VARIANT}.zip"

    cargo clean
    cargo build --release --target ${TARGET}

    mkdir -p ${PACKAGE}/${ASSETS}

    echo `pwd`
    cp ${RELEASE_DIR}/${EXECUTABLE} ${PACKAGE}
    cp ${ASSETS}/* ${PACKAGE}/${ASSETS}

    cd ${PACKAGE}
    zip -r ${ZIP_ARCHIVE} .

    cd ${CURDIR}
    mv ${PACKAGE}/${ZIP_ARCHIVE} .
    rm -rf ${PACKAGE}
}

generate_artifact linux
generate_artifact windows

cargo clean
