#!/usr/bin/env sh

set -eu

npm install
npm run watch:css & npm run watch:js
