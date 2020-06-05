#!/bin/bash/
#list=( 01 02 03 04 05 06 07 08 09 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 37 38 39 40 41 42 43 44 45 46 47 48 49 50 51 52 53 54 55 56 57 58 59 60 61 62 63 64 65 66 67 68 69 70 71 72 73 74 75 76 77 78 79 80)
#list=( 21 31 36 )
##
#list=( 01 02 03 04 05 07 08 09 10 11 12 13 14 16 17 18 20 21 22 24 25 27 28 29 30 32 33 34 35 36 37 39 40 41 42 44 45 47 49 50 53 56 58 59 61 62 63 64 66 70 71 73 74 76 78 79 80 )
list=( 68 72 43 46 54 )

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
         sed -i 's/0.512345/'${xsgghhh}'/g' ws_SM_c2v01_noCosTheta_run2_C2V${ext}/Node_SM/*.txt
         #Sig VBFHH
         sed -i 's/0.554321/'${xsvbfhh}'/g' ws_SM_c2v01_noCosTheta_run2_C2V${ext}/Node_SM/*.txt
         #-1 to 7 : ggHH Signal to Bkg
         # 12 cats
         #sed -i '120s/0/7/g' ws_12cats_SM_posmix_noCosTheta_C2V${ext}/Node_SM/hhbbgg_13TeV_DataCard.txt
         #sed -i '22s/0/7/g' ws_12cats_SM_posmix_noCosTheta_C2V${ext}/Node_SM/hhbbgg_13TeV_DataCard_cat*
         #sed -i '120s/0/7/g' ws_12cats_SM_posmix_noCosTheta_C2V${ext}/Node_SM/hhbbgg_13TeV_DataCard_envelope.txt
         #sed -i '22s/0/7/g' ws_12cats_SM_posmix_noCosTheta_C2V${ext}/Node_SM/hhbbgg_13TeV_DataCard_envelope_cat* 
         # 13 cats
         #sed -i '129s/0/7/g' c2v2_ws_2D_vbfhh_C2V${ext}/Node_SM/hhbbgg_13TeV_DataCard.txt
         #sed -i '22s/0/7/g' c2v2_ws_2D_vbfhh_C2V${ext}/Node_SM/hhbbgg_13TeV_DataCard_cat*
         #sed -i '129s/0/7/g' c2v2_ws_2D_vbfhh_C2V${ext}/Node_SM/hhbbgg_13TeV_DataCard_envelope.txt
         #sed -i '22s/0/7/g' c2v2_ws_2D_vbfhh_C2V${ext}/Node_SM/hhbbgg_13TeV_DataCard_envelope_cat*
         # 14 cats
         #sed -i '138s/0/7/g' ws_SM_c2v01_noCosTheta_14catsMX_C2V${ext}/Node_SM/hhbbgg_13TeV_DataCard.txt
         #sed -i '22s/0/7/g' ws_SM_c2v01_noCosTheta_14catsMX_C2V${ext}/Node_SM/hhbbgg_13TeV_DataCard_cat*
         #sed -i '138s/0/7/g' ws_SM_c2v01_noCosTheta_14catsMX_C2V${ext}/Node_SM/hhbbgg_13TeV_DataCard_envelope.txt
         #sed -i '22s/0/7/g' ws_SM_c2v01_noCosTheta_14catsMX_C2V${ext}/Node_SM/hhbbgg_13TeV_DataCard_envelope_cat*
         # 16 cats
         sed -i '156s/0/7/g' ws_SM_c2v01_noCosTheta_run2_C2V${ext}/Node_SM/hhbbgg_13TeV_DataCard.txt
         sed -i '22s/0/7/g' ws_SM_c2v01_noCosTheta_run2_C2V${ext}/Node_SM/hhbbgg_13TeV_DataCard_cat*
         sed -i '156s/0/7/g' ws_SM_c2v01_noCosTheta_run2_C2V${ext}/Node_SM/hhbbgg_13TeV_DataCard_envelope.txt
         sed -i '22s/0/7/g' ws_SM_c2v01_noCosTheta_run2_C2V${ext}/Node_SM/hhbbgg_13TeV_DataCard_envelope_cat*
    fi 
  done <"$file"
done
