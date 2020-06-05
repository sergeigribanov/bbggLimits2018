#!/bin/bash/
list=( 01 02 03 04 05 06 07 08 09 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60 61 62 63 64 65 66 67 68 69 70 71 72 73 74 75 76 77 78 79 80)

for ext in ${list[@]}
do
  echo "Processing C2V=" $ext
  sed -e 's/\(C2V\)\([0-9][0-9]\)/\1'${ext}'/' conf_default_forC2V_.json >conf_defaultC2V${ext}.json
  sed -e 's/\(C2V\)\([0-9][0-9]\)/\1'${ext}'/g' runWS_forC2V_.sh >runWSC2V${ext}.sh
  sed -e 's/\(C2V\)\([0-9][0-9]\)/\1'${ext}'/g' condor_job_runWS_forC2V_.sub >condor_job_runWSC2V${ext}.sub
  #. runWSC2V${ext}.sh
  condor_submit condor_job_runWSC2V${ext}.sub 
done
