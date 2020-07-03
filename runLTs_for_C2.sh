#!/bin/bash/
list=( 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 27 28 29 30 31 32 33 34 35 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60 61 62 63 64 65 66 67 68 69 70 71 72 73 74 75 76 77 78 79 80)
#list=( 36 26 )

for ext in ${list[@]}
do
  echo "Processing C2=" $ext
  sed -e 's/\(C2\)\([0-9][0-9]\)/\1'${ext}'/' runC2_.sh >runC2.sh
  sed -e 's/\(C2\)\([0-9][0-9]\)/\1'${ext}'/' bbggLTMaker_C2.h >bbggLTMaker.h 
  sed -e 's/C2/'${ext}'/' bbggLTMaker_C2.C >bbggLTMaker.C 
  . ./runC2.sh
done
