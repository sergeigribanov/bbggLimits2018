#!/bin/bash

## 1) Make MaxLikelihood fit

LIMFOLDER=$1/Node_SM

#combine --datacard ${LIMFOLDER}/CombinedCard_Node_SM${POINT}/hhbbgg_13TeV_DataCard.txt -M MaxLikelihoodFit --saveWorkspace --saveShapes --saveNormalization --X-rtd TMCSO_AdaptivePseudoAsimov=50 -n SMHHForBkgPlots

#mv mlfitSMHHForBkgPlots.root ${LIMFOLDER}
#mv higgsCombineSMHHForBkgPlots.MaxLikelihoodFit.mH120.root ${LIMFOLDER}
#mv MaxLikelihoodFitResult.root ${LIMFOLDER}

## 2) Make plots with fit

INFILE=${LIMFOLDER}/higgsCombineTest.FitDiagnostics.mH120.123456.root
DATAFILE=${LIMFOLDER}/ws_hhbbgg.data_bkg.root

BKG=${LIMFOLDER}/Background
mkdir ${BKG}

echo $INFILE
echo $BKG

FACT=1.


for icat in {0..5}
do

    python scripts/MakeFullBackgroundFit.py -i ${INFILE} -o ${BKG} -c ${icat} --signalFactor ${FACT} ${FACT} --unblind
    python scripts/MakeFullBackgroundFit_Data.py -i ${INFILE} -d ${DATAFILE} -o ${BKG} -c ${icat} --signalFactor ${FACT} ${FACT} --unblind
done
