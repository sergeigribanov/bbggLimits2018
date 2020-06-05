#!/bin/bash
cd /afs/cern.ch/work/i/ivovtin/Hggbb/CMSSW_10_2_13/src/HiggsAnalysis/CombinedLimit/
#cmsenv
eval `scramv1 runtime -sh`
echo "CMSSW: "$CMSSW_BASE
#Run your program
combine -M AsymptoticLimits -d /afs/cern.ch/work/i/ivovtin/Hggbb/CMSSW_8_1_0/src/HiggsAnalysis/bbggLimits2018/ws_SM_c2v01_noCosTheta_run2_C2V17/Node_SM/hhbbgg_13TeV_DataCard_envelope.txt --run blind -m 125 -n SM_13TeV_3ab --X-rtd TMCSO_AdaptivePseudoAsimov=0 --X-rtd TMCSO_PseudoAsimov=0 --freezeParameters CMS_eff_DiphoTrig,CMS_eff_PhoID,CMS_eff_mxCat,CMS_btag,QCDscale_tth,QCDscale_vbf,QCDscale_vh,lumi_13TeV,pdf,theory_bbH_scale,theory_ggHN3LO  --cminDefaultMinimizerStrategy 0 --cminFallbackAlgo Minuit2,Migrad,0:0.1 --X-rt MINIMIZER_freezeDisassociatedParams --X-rtd MINIMIZER_multiMin_hideConstants --X-rtd MINIMIZER_multiMin_maskConstraints --X-rtd MINIMIZER_multiMin_maskChannels=2 &> /afs/cern.ch/work/i/ivovtin/public/ForMaxim/RUN2_SM_c2v01_noCosTheta/C2Vscan/Limit_stat_C2V17.txt

 


