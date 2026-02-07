#!/bin/sh

ARTIFACT="textris"
ASSETS="res"
CURDIR=`pwd`
EXECUTABLE="${ARTIFACT}.exe"
PACKAGE="package"
TARGET="x86_64-pc-windows-gnu"
TARGET_DIR="target"

RELEASE_DIR="${TARGET_DIR}/${TARGET}/release"
ZIP_ARCHIVE="${ARTIFACT}.zip"

cargo clean
sed -i 's/"\/usr\/share\/games\/textris"/"res"/g' src/context.rs
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

cargo clean
