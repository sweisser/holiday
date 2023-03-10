#!/bin/bash
BASEURL=https://holiday-sib76bry.fermyon.app/
#BASEURL=http://127.0.0.1:3000/

for year in {1970..2100}
do
  curl ${BASEURL}?year=${year}
  echo
done
