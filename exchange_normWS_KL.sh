#!/bin/bash/
#list=(36)
#list=( 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60 61 62 63 64 65 66 67 68 69 70 71 72 73 74 75 76 77 78 79 80)
list=( 49 52 54 56 57 )

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
         sed -i 's/0.512345/'${xsgghhh}'/g' ws_SM_c2v01_noCosTheta_run2_KL${ext}/Node_SM/*.txt
         #Sig VBFHH
         sed -i 's/0.554321/'${xsvbfhh}'/g' ws_SM_c2v01_noCosTheta_run2_KL${ext}/Node_SM/*.txt
         #-1 to 7 : ggHH Signal to Bkg
         # 13 cats
         #sed -i '129s/0/7/g' ws_2D_vbfhhKL${ext}/Node_SM/hhbbgg_13TeV_DataCard.txt
         #sed -i '22s/0/7/g' ws_2D_vbfhhKL${ext}/Node_SM/hhbbgg_13TeV_DataCard_cat*
         #sed -i '129s/0/7/g' ws_2D_vbfhhKL${ext}/Node_SM/hhbbgg_13TeV_DataCard_envelope.txt
         #sed -i '22s/0/7/g' ws_2D_vbfhhKL${ext}/Node_SM/hhbbgg_13TeV_DataCard_envelope_cat*
         # 14 cats
         #sed -i '138s/0/7/g' ws_SM_posmix_wCosTheta_KL${ext}/Node_SM/hhbbgg_13TeV_DataCard.txt
         #sed -i '22s/0/7/g' ws_SM_posmix_wCosTheta_KL${ext}/Node_SM/hhbbgg_13TeV_DataCard_cat*
         #sed -i '138s/0/7/g' ws_SM_posmix_wCosTheta_KL${ext}/Node_SM/hhbbgg_13TeV_DataCard_envelope.txt
         #sed -i '22s/0/7/g' ws_SM_posmix_wCosTheta_KL${ext}/Node_SM/hhbbgg_13TeV_DataCard_envelope_cat*
         # 16 cats
         #sed -i '156s/0/7/g' ws_c2v0_noCosTheta_KL${ext}/Node_SM/hhbbgg_13TeV_DataCard.txt
         #sed -i '22s/0/7/g' ws_c2v0_noCosTheta_KL${ext}/Node_SM/hhbbgg_13TeV_DataCard_cat*
         #sed -i '156s/0/7/g' ws_c2v0_noCosTheta_KL${ext}/Node_SM/hhbbgg_13TeV_DataCard_envelope.txt
         #sed -i '22s/0/7/g' ws_c2v0_noCosTheta_KL${ext}/Node_SM/hhbbgg_13TeV_DataCard_envelope_cat*
 
    fi 
  done <"$file"
done
