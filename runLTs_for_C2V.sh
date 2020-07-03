#!/bin/bash/
#list=( 01 02 03 04 05 06 07 08 09 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60 61 62 63 64 65 66 67 68 69 70 71 72 73 74 75 76 77 78 79 80)
list=( 16 18 26 34 36 38 41 46 51 56 )

for ext in ${list[@]}
do
  echo "Processing C2V=" $ext
    sed -e 's/\(C2V\)\([0-9][0-9]\)/\1'${ext}'/' runC2V_.sh >runC2V.sh 
    sed -i 's/\(C2V\)\([0-9][0-9]\)/\1'${ext}'/' bbggLTMaker_C2V.h >bbggLTMaker.h  
    sed -i 's/\(C2V\)\([0-9][0-9]\)/\1'${ext}'/' bbggLTMaker_C2V.C >bbggLTMaker.C   
  . ./runC2V.sh 
  #. ./runC2V.sh > LT_outC2V${ext}.dat
done
