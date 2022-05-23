#!/bin/bash

## Name: Tyler Adam Martinez
## Course: CSCE3600
## Date: 03-01-2022

function curve() {
  incr=$1
  shift
  arr=("$@")
  length=${#arr[*]}
  index=0

for (( i=0 ; $i<${length}; i++ ))
do
  let grades[$index]+=$incr
  let index++
done
}

if [[ $# -ne 1 ]];
then
  echo "usage: ./rec05.sh <curve amount>"
  exit 0
fi

for x in {1..5}
do
  read -p "Enter QUIZ #$x: " grades[$x-1]
done
incr=$1
curve $incr ${grades[@]}

echo "Curved Grades:"
index=0
for y in "${grades[@]}"
do
  echo "grades[$index] = $y"
  let index++
done
