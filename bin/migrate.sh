#!/usr/bin/env sh

color="\033[0;35m"
alert="\033[0;31m"
reset="\033[1;37m"

if [ "$1" = "generate-entity" ]; then
  echo "$color>>> Generate entity$reset"
  sea-orm-cli generate entity -o entity/src/entities
elif [ "$1" = "generate" ]; then
  echo "$color>>> Migration generate $2 $reset"
  sea-orm-cli migrate generate "$2"
elif [ "$1" = "up" ]; then
  echo "$color>>> Migration up$reset"
  sea-orm-cli migrate up
elif [ "$1" = "down" ]; then
  echo "$color>>> Migration down$reset"
  sea-orm-cli migrate down
elif [ "$1" = "fresh" ]; then
  echo "$color>>> Fresh$reset"
  sea-orm-cli migrate fresh
else
  echo "$alert>>> No command found"
fi