#!/bin/bash/
#list=(1 5 15 17 19 25 30 35 40 41 42 44 46 50 55 60 65 70 71 72 74 80)
#list=(47 49 51 53 57 59)
#list=(10 20)
list=(10 15)
#list=(82 83 84 85 86 )
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
