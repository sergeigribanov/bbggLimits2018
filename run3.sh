##combine -M AsymptoticLimits -d ws_2016-17-18_ttHon0.26_2D_DatatoMC_04022020/Node_SM/hhbbgg_13TeV_DataCard_envelope.txt --run blind -m 125 -n SM_13TeV_3ab --X-rtd TMCSO_AdaptivePseudoAsimov=0 --X-rtd TMCSO_PseudoAsimov=0 --cminDefaultMinimizerStrategy 0 --X-rtd MINIMIZER_freezeDisassociatedParams --X-rtd MINIMIZER_multiMin_hideConstants --X-rtd MINIMIZER_multiMin_maskConstraints --X-rtd MINIMIZER_multiMin_maskChannels=2 --freezeParameters CMS_eff_DiphoTrig,CMS_eff_PhoID,CMS_eff_mxCat,CMS_btag,QCDscale_tth,QCDscale_vbf,QCDscale_vh,lumi_13TeV,pdf,theory_bbH_scale,theory_ggHN3LO

##combine -M AsymptoticLimits -d ws_2016-17-18_ttHon0.26_2D_btagRew_20022020_woMjj/Node_SM/hhbbgg_13TeV_DataCard_envelope.txt --run blind -m 125 -n SM_13TeV_3ab --X-rtd TMCSO_AdaptivePseudoAsimov=0 --X-rtd TMCSO_PseudoAsimov=0 --freezeParameters CMS_eff_DiphoTrig,CMS_eff_PhoID,CMS_eff_mxCat,CMS_btag,QCDscale_tth,QCDscale_vbf,QCDscale_vh,lumi_13TeV,pdf,theory_bbH_scale,theory_ggHN3LO

combine -M AsymptoticLimits -d ws_run2_ttHon0.26_2D_SM_ggHHonly_04062020/Node_SM/hhbbgg_13TeV_DataCard_envelope.txt --run blind -m 125 -n SM_13TeV_3ab --X-rtd TMCSO_AdaptivePseudoAsimov=0 --X-rtd TMCSO_PseudoAsimov=0 --freezeParameters CMS_eff_DiphoTrig,CMS_eff_PhoID,CMS_eff_mxCat,CMS_btag,QCDscale_tth,QCDscale_vbf,QCDscale_vh,lumi_13TeV,pdf,theory_bbH_scale,theory_ggHN3LO  --cminDefaultMinimizerStrategy 0 --cminFallbackAlgo Minuit2,Migrad,0:0.1 --X-rt MINIMIZER_freezeDisassociatedParams --X-rtd MINIMIZER_multiMin_hideConstants --X-rtd MINIMIZER_multiMin_maskConstraints --X-rtd MINIMIZER_multiMin_maskChannels=2

##combine -M AsymptoticLimits -d ws_2016-17-18_ttHon0.26_2D_22012020/Node_SM/hhbbgg_13TeV_DataCard_envelope.txt --run blind -m 125 -n SM_13TeV_3ab --X-rtd TMCSO_AdaptivePseudoAsimov=0 --X-rtd TMCSO_PseudoAsimov=0

