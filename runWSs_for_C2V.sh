#!/bin/bash/
#list=(1 5 15 17 19 25 30 35 40 41 42 44 46 50 55 60 65 70 71 72 74 80)
list=(10 15)
for ext in ${list[@]}
do
  echo "Processing C2V=" $ext
  sed -e 's/\(C2V\)\([0-9][0-9]\)/\1'${ext}'/' conf_default_forC2V_.json >conf_defaultC2V${ext}.json
  sed -e 's/\(C2V\)\([0-9][0-9]\)/\1'${ext}'/g' runWS_forC2V_.sh >runWSC2V${ext}.sh
  sed -e 's/\(C2V\)\([0-9][0-9]\)/\1'${ext}'/g' condor_job_runWS_forC2V_.sub >condor_job_runWSC2V${ext}.sub
  #. runWS.sh
  condor_submit condor_job_runWSC2V${ext}.sub 
done
