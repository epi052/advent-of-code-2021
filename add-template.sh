#!/usr/bin/env bash

if [[ "${#}" != 1 ]]; then
  echo "usage: ${0} DAY_NUMBER"
  exit 1
fi

day="${1}"
cp template.rs src/bin/day-$day.rs