#!/bin/sh

set -e

m4txblog collect-static /app/static
exec m4txblog "$@"
