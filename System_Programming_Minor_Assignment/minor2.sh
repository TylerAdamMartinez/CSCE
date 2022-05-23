#!/bin/bash

## Name: Tyler Adam Martinez
## Course: CSCE3600
## Date: 02-27-2021
## Description: 

trap escape_protocol SIGINT

function escape_protocol() {
  echo " (SIGINT) received"
  echo "Terminate Program [Y/N]: "
  read response

  case $response in
    [Yy]* ) echo "Terminating Program"; exit 1;;
    * ) printf "Contining Execution\n\n";; 
  esac 
}

function print_title() {
    RED=$(tput setaf 1)
    GREEN=$(tput setaf 2)
    NORMAL=$(tput sgr0)

    users_title="${RED}User ID${NORMAL}"
    count_title="${GREEN}Count${NORMAL}"
    printf "%-30s %22s\n" "$users_title" "$count_title"
}

function print_footer() {
    totalProcesses=`ps -e | wc -l`
    totalUsers=`who | wc -l`
    printf "%d USERS, TOTAL PROCESSES %5d\n" "$totalUsers" "$totalProcesses"
    printf "\n"
}

function log() {
    print_title
    ps -eo user=|sort|uniq -c | sort -nr | awk '{ printf "%-25s %5s\n", $2, $1 }'
    print_footer
}

while true; do
  echo `date '+%a %d %b %Y %I:%M:%S %p %Z'`
  if [ $# -eq 0 ]
  then
    log
  else
    print_title
    for arg in $@
      do
        ps -eo user=|sort|uniq -c | sort -nr | awk -v name=$arg '$2 ~ name { printf "%-25s %5s\n", $2, $1 }'
      done
    print_footer
  fi
  sleep 5
done
