#!/bin/bash

DIR=$1

#OUT=${DIR}/Node_SM/SignalShapes
OUT=${DIR}/Node_SM/SignalShapesVBFHH

mkdir ${OUT}

#for icat in {0..11}
for icat in {0..13}
do
    ##python scripts/MakeSigPlot.py -p ${OUT} -w ${DIR}/Node_SM/ws_hhbbgg.HH.sig.mH125_13TeV.root -c ${icat} -o "mjj,mgg" -l "2016+2017+2018 analysis" -a "#font[61]{pp#rightarrowHH#rightarrowb#bar{b}#gamma#gamma}|SM-like HH" -b 24,160 -d 1
    ##python scripts/MakeSigPlot.py -p ${OUT} -w ${DIR}/Node_SM/ws_hhbbgg.HH.sig.mH125_13TeV.root -c ${icat} -o "mjj,mgg" -l "2018 analysis" -a "#font[61]{pp#rightarrowHH#rightarrowb#bar{b}#gamma#gamma}|SM-like HH" -b 24,160 -d 1
    python scripts/MakeSigPlot.py -p ${OUT} -w ${DIR}/Node_SM/ws_hhbbgg.HH.sigVBF.mH125_13TeV.root -c ${icat} -o "mjj,mgg" -l "2018 analysis" -a "#font[61]{pp#rightarrowHH#rightarrowb#bar{b}#gamma#gamma}|SM-like HH" -b 24,160 -d 1

done


