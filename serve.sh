#!/bin/sh

CARGO_TARGET_DIR=target-trunk trunk serve -w src/ -w index.html -w assets/ -w sass/
