#!/bin/bash
cd /afs/cern.ch/user/i/ivovtin/HHggbb/CMSSW_8_1_0/src/HiggsAnalysis/bbggLimits2018/
#cmsenv
eval `scramv1 runtime -sh`
echo "CMSSW: "$CMSSW_BASE
#Run your program
./runLimit.py -f conf_defaultC236.json --node=SM -o ws_run2_ggHHonly_C236 --overwrite --ttHTaggerCut 0.26
