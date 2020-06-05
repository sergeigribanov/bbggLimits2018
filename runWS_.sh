#!/bin/bash
cd /afs/cern.ch/work/i/ivovtin/Hggbb/CMSSW_8_1_0/src/HiggsAnalysis/bbggLimits2018/ 
#cmsenv
eval `scramv1 runtime -sh`
echo "CMSSW: "$CMSSW_BASE
#Run your program
./runLimit.py -f conf_defaultKL36.json --node=SM -o ws_SM_c2v01_noCosTheta_run2_KL36 --overwrite --ttHTaggerCut 0.26
