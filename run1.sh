#!/bin/bash/

##./makeLT.py /afs/cern.ch/work/i/ivovtin/public/inDir/ -x nonres -o /afs/cern.ch/work/i/ivovtin/public/outDir -c 2

##Input directory with samples
indir16="/afs/cern.ch/user/i/ivovtin/workarea/public/legacy_branch_flattrees/rho_rew_2016_v2/"
indir17="/afs/cern.ch/user/i/ivovtin/workarea/public/legacy_branch_flattrees/rho_rew_2017_v2/"
indir18="/afs/cern.ch/user/i/ivovtin/workarea/public/legacy_branch_flattrees/rho_rew_2018_v2/"

##Output directory with Limit Tree (LT)
path=/afs/cern.ch/work/i/ivovtin/Hggbb
LT=LT_2D_Y2_31012020

out="$path/$LT_$method_Y$cat"

mkdir $out

outdir16="$path/$LT/2016"
outdir17="$path/$LT/2017"
outdir18="$path/$LT/2018"

mkdir $outdir16
mkdir $outdir17
mkdir $outdir18

echo "Start makeLT ..."

##For 1D method
#./makeLT.py  $indir16 -x nonres -l 35.9 -o $outdir16 -c 3
#./makeLT.py  $indir17 -x nonres -l 41.5 -o $outdir17 -c 3
#./makeLT.py  $indir18 -x nonres -l 59.4 -o $outdir18 -c 3

##For 2D method
./makeLT.py  $indir16 -x nonres -l 35.9 -o $outdir16 -c 2
./makeLT.py  $indir17 -x nonres -l 41.5 -o $outdir17 -c 2
./makeLT.py  $indir18 -x nonres -l 59.4 -o $outdir18 -c 2

echo "Finish makeLT ..."

execute_cmd=hadd

dirout="$path/$LT/merged"
mkdir $dirout
dir1="$path/$LT/2016"
dir2="$path/$LT/2017"
dir3="$path/$LT/2018"

file1="LT_DoubleEG.root"
file2="LT_output_GluGluToHHTo2B2G_node_SM_13TeV-madgraph.root"
file3="LT_output_VBFHToGG_M-125_13TeV_powheg_pythia8.root"
file4="LT_output_GluGluHToGG_M-125_13TeV_powheg_pythia8.root"
file5="LT_output_VHToGG_M125_13TeV_amcatnloFXFX_madspin_pythia8.root"
file6="LT_output_ttHToGG_M125_13TeV_powheg_pythia8.root"
file7="LT_output_bbHToGG_M-125_13TeV_amcatnlo.root"

echo "Merging files ..."

$execute_cmd $dirout/$file1 $dir1/$file1  $dir2/$file1  $dir3/$file1
$execute_cmd $dirout/$file2 $dir1/$file2  $dir2/$file2  $dir3/$file2
$execute_cmd $dirout/$file3 $dir1/$file3  $dir2/$file3  $dir3/$file3
$execute_cmd $dirout/$file4 $dir1/$file4  $dir2/$file4  $dir3/$file4
$execute_cmd $dirout/$file5 $dir1/$file5  $dir2/$file5  $dir3/$file5
$execute_cmd $dirout/$file6 $dir1/$file6  $dir2/$file6  $dir3/$file6
$execute_cmd $dirout/$file7 $dir1/$file7  $dir2/$file7  $dir3/$file7

echo "Merge of files completed"

