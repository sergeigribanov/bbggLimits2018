#!/bin/bash
cd /afs/cern.ch/work/i/ivovtin/Hggbb/CMSSW_8_1_0/src/HiggsAnalysis/bbggLimits2018/ 
#cmsenv
eval `scramv1 runtime -sh`
echo "CMSSW: "$CMSSW_BASE
#Run your program
./runLimit.py -f conf_defaultC2V10.json --node=SM -o ws_2D_vbfhhC2V10 --overwrite --ttHTaggerCut 0.26
