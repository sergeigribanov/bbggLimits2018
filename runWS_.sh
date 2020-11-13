#!/bin/bash
cd /afs/cern.ch/user/s/segriban/test_Hggbb/CMSSW_8_1_0/src/HiggsAnalysis/bbggLimits2018/
#cmsenv
eval `scramv1 runtime -sh`
echo "CMSSW: "$CMSSW_BASE
#Run your program
./runLimit.py -f conf_defaultKL36.json --node=SM -o ws_run2_ttHon0.26_2D_SM_ggHHonly_04062020 --overwrite --ttHTaggerCut 0.26
#./runLimit.py -f conf_defaultKL36.json --node=SM -o ws_run2_ttHon0.26_2D_ggHHonly_NLO_Mjjreg_KL36 --overwrite --ttHTaggerCut 0.26
