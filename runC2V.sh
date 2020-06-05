#!/bin/bash/

indir16="/afs/cern.ch/work/i/ivovtin/public/legacy_branch_flattrees/vbfhh/2016/2016_SM_c2v01_noCosTheta_conversion/"
indir17="/afs/cern.ch/work/i/ivovtin/public/legacy_branch_flattrees/vbfhh/2017/2017_SM_c2v01_noCosTheta_conversion/"
indir18="/afs/cern.ch/work/i/ivovtin/public/legacy_branch_flattrees/vbfhh/2018/2018_SM_c2v01_noCosTheta_conversion/"

##Output directory with Limit Tree (LT)
path=/afs/cern.ch/work/i/ivovtin/Hggbb
#path=.
Train=RUN2_SM_c2v01_noCosTheta
Scan=C2Vscan
#Scan=KLscan
LT=LT_vbfhh_C2V79

out="$path/$Train"

mkdir $out
mkdir "$out/$Scan"

mkdir "$out/$Scan/2016"
mkdir "$out/$Scan/2017"
mkdir "$out/$Scan/2018"

outdir16="$path/$Train/$Scan/2016/$LT"
outdir17="$path/$Train/$Scan/2017/$LT"
outdir18="$path/$Train/$Scan/2018/$LT"

mkdir $outdir16
mkdir $outdir17
mkdir $outdir18

echo "Start makeLT ..."

./makeLT.py  $indir16 -x nonres -l 35.9 -o $outdir16 -v -c 6
./makeLT.py  $indir17 -x nonres -l 41.5 -o $outdir17 -v -c 6
./makeLT.py  $indir18 -x nonres -l 59.4 -o $outdir18 -v -c 6

##./makeLT.py /afs/cern.ch/work/i/ivovtin/public/legacy_branch_flattrees/vbfhh/2018/2018_260520_SM_c2v01_noCosTheta_conversion/ -x nonres -l 59.4 -o /afs/cern.ch/work/i/ivovtin/Hggbb/SM_c2v01_noCosTheta_14catsMX/C2Vscan/LT_vbfhh_C2V79 -c 6

echo "Finish makeLT ..."

execute_cmd=hadd

mkdir "$path/$Train/$Scan/merged"
dirout="$path/$Train/$Scan/merged/$LT"
mkdir "$dirout"

dir1="$path/$Train/$Scan/2016/$LT"
dir2="$path/$Train/$Scan/2017/$LT"
dir3="$path/$Train/$Scan/2018/$LT"

file1="LT_DoubleEG.root"
file2="LT_output_GluGluToHHTo2B2G_node_SM_13TeV-madgraph.root"
file3="LT_output_VBFHToGG_M-125_13TeV_powheg_pythia8.root"
file4="LT_output_GluGluHToGG_M-125_13TeV_powheg_pythia8.root"
file5="LT_output_VHToGG_M125_13TeV_amcatnloFXFX_madspin_pythia8.root"
file6="LT_output_ttHToGG_M125_13TeV_powheg_pythia8.root"
file7="LT_output_bbHToGG_M-125_13TeV_amcatnlo.root"
file8="LT_output_VBFHHTo2B2G_CV_1_C2V_1_C3_1_TuneCP5_PSWeights_13TeV-madgraph-pythia8.root"

echo "Merging files ..."

#$execute_cmd $dirout/$file1 $dir1/$file1  $dir3/$file1
#$execute_cmd $dirout/$file2 $dir1/$file2  $dir3/$file2
#$execute_cmd $dirout/$file3 $dir1/$file3  $dir3/$file3
#$execute_cmd $dirout/$file4 $dir1/$file4  $dir3/$file4
#$execute_cmd $dirout/$file5 $dir1/$file5  $dir3/$file5
#$execute_cmd $dirout/$file6 $dir1/$file6  $dir3/$file6
#$execute_cmd $dirout/$file7 $dir1/$file7  $dir3/$file7
#$execute_cmd $dirout/$file8 $dir1/$file8  $dir3/$file8

$execute_cmd $dirout/$file1 $dir1/$file1  $dir2/$file1  $dir3/$file1
$execute_cmd $dirout/$file2 $dir1/$file2  $dir2/$file2  $dir3/$file2
$execute_cmd $dirout/$file3 $dir1/$file3  $dir2/$file3  $dir3/$file3
$execute_cmd $dirout/$file4 $dir1/$file4  $dir2/$file4  $dir3/$file4
$execute_cmd $dirout/$file5 $dir1/$file5  $dir2/$file5  $dir3/$file5
$execute_cmd $dirout/$file6 $dir1/$file6  $dir2/$file6  $dir3/$file6
$execute_cmd $dirout/$file7 $dir1/$file7  $dir2/$file7  $dir3/$file7
$execute_cmd $dirout/$file8 $dir1/$file8  $dir2/$file8  $dir3/$file8

echo "Merge of files completed"

rm -r "$out/$Scan/2016"
rm -r "$out/$Scan/2017"
rm -r "$out/$Scan/2018"





