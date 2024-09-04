#!/usr/bin/env sh


if [ "$1" = "cli" ]; then
  shift
  exec ./cli "$@"
elif [ "$1" = "api" ]; then
  exec ./api
elif [ "$1" = "reverse_proxy" ]; then
  exec ./reverse_proxy
elif [ "$1" = "migration" ]; then
  shift
  exec ./migration "$@"
else
  exec "$@"
fi