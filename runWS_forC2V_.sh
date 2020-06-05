#!/bin/bash
cd /afs/cern.ch/work/i/ivovtin/Hggbb/CMSSW_8_1_0/src/HiggsAnalysis/bbggLimits2018/ 
#cmsenv
eval `scramv1 runtime -sh`
echo "CMSSW: "$CMSSW_BASE
#Run your program
##./runLimit.py -f conf_defaultC2V10.json --node=SM -o  ws_c2v0_noCosTheta_C2V10 --overwrite --ttHTaggerCut 0.26
##./runLimit.py -f conf_defaultC2V10.json --node=SM -o  ws_SM_wCosTheta_C2V10 --overwrite --ttHTaggerCut 0.26
##./runLimit.py -f conf_defaultC2V10.json --node=SM -o  ws_SM_posmix_noCosTheta_C2V10 --overwrite --ttHTaggerCut 0.26
##./runLimit.py -f conf_defaultC2V10.json --node=SM -o ws_SM_posmix_noCosTheta_C2V10_cats16 --overwrite --ttHTaggerCut 0.26

./runLimit.py -f conf_defaultC2V10.json --node=SM -o  ws_SM_c2v01_noCosTheta_run2_C2V10 --overwrite --ttHTaggerCut 0.26
##./runLimit.py -f conf_defaultC2V10.json --node=SM -o  ws_SM_c2v01_noCosTheta_16catsCosMVA_C2V10 --overwrite --ttHTaggerCut 0.26
##./runLimit.py -f conf_defaultC2V10.json --node=SM -o  ws_SM_c2v01_wCosTheta_C2V10 --overwrite --ttHTaggerCut 0.26
