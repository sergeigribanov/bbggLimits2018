#!/bin/bash/
#list=( 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60 61 62 63 64 65 66 67 68 69 70 71 72 73 74 75 76 77 78 79 80)
list=(36)

WS="ws_run2_ggHHNLO_MjjReg_KL"

for ext in ${list[@]}
do
  echo "Processing KL=" $ext
  COUNTER=0
  file="VBFHHRew/outXS_KL.txt"
  while read -r kl c2v cv xsvbfhh xsgghhh 
  do
   ((COUNTER++))
   if [ $COUNTER -eq "$ext" ]
    then
         printf 'kl: %s, c2v: %i, cv: %s, xsvbfhh: %s, xsgghh: %s \n' "$kl" "$c2v" "$cv" "$xsvbfhh" "$xsgghhh"     
         #Sig ggHH
         #sed -i 's/0.512345/'${xsgghhh}'/g' ${WS}${ext}/Node_SM/*.txt
         #Sig VBFHH
         sed -i 's/0.554321/'${xsvbfhh}'/g' ${WS}${ext}/Node_SM/*.txt
    fi 
  done <"$file"
  echo "Processing NLO ggHH KL=" $ext
  COUNTER2=0
  file2="VBFHHRew/outXS_KL_ggHH.txt"
  while read -r kl xsgghhh 
  do
   ((COUNTER2++))
   if [ $COUNTER2 -eq "$ext" ]
    then
         printf 'kl: %s, xsgghh: %s \n' "$kl" "$xsgghhh"     
         #Sig ggHH
         sed -i 's/0.512345/'${xsgghhh}'/g' ${WS}${ext}/Node_SM/*.txt
    fi 
  done <"$file2"
done
