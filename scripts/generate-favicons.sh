#!/bin/bash

set -e

IMAGES_DIR=static/images
magick "$IMAGES_DIR/favicon-512.png" -define icon:auto-resize=16,32,48 nginx/assets/favicon.ico
magick "$IMAGES_DIR/favicon-512.png" -resize 192x192 "$IMAGES_DIR/favicon-192.png"
magick "$IMAGES_DIR/favicon-512.png" -resize 180x180 "$IMAGES_DIR/favicon-180.png"
magick "$IMAGES_DIR/favicon-512.png" -resize 32x32 "$IMAGES_DIR/favicon-32.png"
