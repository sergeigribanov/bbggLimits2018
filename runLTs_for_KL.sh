#!/bin/bash/
#list=(1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 )
#list=(21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40)
#list=(41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60)
#list=(61 62 63 64 65 66 67 68 69 70 71 72 73 74 80)
list=(10 15)
for ext in ${list[@]}
do
  echo "Processing KL=" $ext
  sed -e 's/\(KL\)\([0-9][0-9]\)/\1'${ext}'/' runKL_.sh >runKL.sh
  sed -e 's/\(KL\)\([0-9][0-9]\)/\1'${ext}'/' bbggLTMaker.h >bbggLTMaker_K.h
  sed -e 's/KL/'${ext}'/' bbggLTMaker.C >bbggLTMaker_K.C
  . ./runKL.sh
  #sed -e 's/\(KL\)\([0-9][0-9]\)/\1'${ext}'/' merge.sh >mergeKL.sh
  #. ./mergeKL.sh
done
