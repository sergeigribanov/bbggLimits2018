LIMFOLDER=$1/Node_SM


cd $LIMFOLDER
mkdir BIAS
mkdir BIAS/LOGS
arr=(Ber Exp Pow)
func=0
NGEN=500

echo " ========== Func used " ${arr[func]} " with " ${NGEN} " events " 
echo " "

for icat in {0..3}
do
    for truth in {0..2}
    do
	seed=$(($icat*100+$func+1)) 
	DATACARD=hhbbgg_13TeV_DataCard_bias_cat${icat}.txt
	PDFIDX=pdf_index_cat${icat}
	TOYS=BIAS/higgsCombineTest.GenerateOnly.cat${icat}.tBer.f${arr[func]}.root
	MLFIT=BIAS/mlfit_cat${icat}.tBer.f${arr[${func}]}.root

	echo "cat" ${icat} " with truth " ${arr[truth]} " fit by function " ${arr[func]} " with a random seed " ${seed}
	combine $DATACARD -M GenerateOnly --setPhysicsModelParameters $PDFIDX=${truth} --toysFrequentist -t ${NGEN} --expectSignal 1 --saveToys -m ${icat} --freezeNuisances $PDFIDX -s ${seed} -v -1 &>BIAS/LOGS/Gen_cat${icat}.t${arr[${truth}]}.f${arr[${func}]}.txt

	mv higgsCombineTest.GenerateOnly.mH${icat}.${seed}.root $TOYS
	echo "generated " $TOYS

	combine $DATACARD -M MaxLikelihoodFit  --setPhysicsModelParameters $PDFIDX=${func}  --toysFile $TOYS  -t ${NGEN} --rMin -5 --rMax 5  --freezeNuisances $PDFIDX -v -1 &>BIAS/LOGS/Fits_cat${icat}.t${arr[${truth}]}.f${arr[${func}]}.txt
	mv mlfit.root $MLFIT
	echo "fitted " $MLFIT
	
    done
done

