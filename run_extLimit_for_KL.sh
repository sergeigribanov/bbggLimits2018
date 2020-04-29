#!/bin/bash/
#list=(1 5 15 17 19 25 30 35 40 41 42 44 46 50 55 60 65 70 71 72 74 80)
list=(10 15)
for ext in ${list[@]}
do
  echo "Processing KL=" $ext
  sed -e 's/\(KL\)\([0-9][0-9]\)/\1'${ext}'/g' run_extLimit_.sh >run_extLimitKL${ext}.sh
  sed -e 's/\(KL\)\([0-9][0-9]\)/\1'${ext}'/g' condor_job_runLimit_.sub >condor_job_runLimitKL${ext}.sub
  #. runWS.sh
  condor_submit condor_job_runLimitKL${ext}.sub 
done
