#!/bin/bash/
#list=(1 5 15 17 19 25 30 35 40 41 42 44 46 50 55 60 65 70 71 72 74 80)
#list=(1 10 20 25 30 40 50 60 70 80)
list=( 20 )

for ext in ${list[@]}
do
  echo "Processing C2V=" $ext
  COUNTER=0
  file="VBFHHRew/outXS_C2V.txt"
  while read -r kl c2v cv xsvbfhh xsgghhh 
  do
   ((COUNTER++))
   if [ $COUNTER -eq "$ext" ]
    then
         printf 'kl: %s, c2v: %i, cv: %s, xsvbfhh: %s, xsgghh: %s \n' "$kl" "$c2v" "$cv" "$xsvbfhh" "$xsgghhh"     
         #Sig ggHH
         sed -i 's/0.512345/'${xsgghhh}'/g' test_ws_2D_vbfhhC2V${ext}/Node_SM/*.txt
         #Sig VBFHH
         sed -i 's/0.554321/'${xsvbfhh}'/g' test_ws_2D_vbfhhC2V${ext}/Node_SM/*.txt
    fi 
  done <"$file"
done
