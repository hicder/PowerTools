#!/bin/bash

echo "Build backend"
pushd backend || exit
./build.sh || exit
popd || exit

echo "Build frontend"
pnpm run build || exit

echo "Build successfully"
