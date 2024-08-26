#!/usr/bin/env sh

color="\033[0;35m"
reset="\033[1;37m"
working_dir="$(pwd)/microservices/"
cd "$working_dir" || exit

if [ "$1" = "--build" ]; then
  echo "$color>>> Start and build containers$reset"
  docker compose up -d --build
else
  echo "$color>>> Start containers$reset"
  docker compose up -d
fi
