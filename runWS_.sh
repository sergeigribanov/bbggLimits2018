#!/bin/bash
cd /afs/cern.ch/user/i/ivovtin/HHggbb/CMSSW_8_1_0/src/HiggsAnalysis/bbggLimits2018/
#cmsenv
eval `scramv1 runtime -sh`
echo "CMSSW: "$CMSSW_BASE
#Run your program
./runLimit.py -f conf_defaultKL36.json --node=SM -o ws_run2_ttHon0.26_2D_ggHHonly_NLO_MjjReg_04112020_KL36 --overwrite --ttHTaggerCut 0.26
