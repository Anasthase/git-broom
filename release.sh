#!/bin/bash

version=$1

if [ -z  "$version" ]
then
  version=$(git rev-parse --abbrev-ref HEAD)
fi

if [ -d "release" ]
then
  rm -rf ./release
fi

mkdir release

echo "Building release for version $version."

echo "Building x86_64-pc-windows-gnu..."
cross build --release --target x86_64-pc-windows-gnu
zip -j ./release/git-broom-"$version"-x86_64-pc-windows-gnu.zip ./target/x86_64-pc-windows-gnu/release/git-broom.exe
echo "Done."

echo "Building x86_64-unknown-linux-musl..."
cross build --release --target x86_64-unknown-linux-musl
tar -zcvf ./release/git-broom-"$version"-x86_64-unknown-linux-musl.tar.gz -C ./target/x86_64-unknown-linux-musl/release git-broom
echo "Done."