# How to

* Get latest combine tools ([link](https://cms-hcomb.gitbooks.io/combine/content/part1/#for-end-users-that-dont-need-to-commit-or-do-any-development)):

```
export SCRAM_ARCH=slc6_amd64_gcc530
cmsrel CMSSW_8_1_0
cd CMSSW_8_1_0/src 
cmsenv
git clone https://github.com/cms-analysis/HiggsAnalysis-CombinedLimit.git HiggsAnalysis/CombinedLimit
cd HiggsAnalysis/CombinedLimit
git fetch origin
git checkout v7.0.9
scramv1 b clean; scramv1 b

```

* Run the limit tree maker:

```
./makeLT.py /eos/cms/store/group/phys_higgs/resonant_HH/RunII/FlatTrees/2016/2018_05_04_HHTaggerETH/ -x nonres -o LT_OutDir
```

* Run the fits and limits on the produced LTs:

```
./runLimit.py -f conf_default.json --node=SM -o OutDir
```
where config file can be edited to point to input directory and various parameters.

The results of the limit will be in `OutDir/Node_SM/result_1.log`. In case of problems,
the logfile _mainLog_data-time.log[.bbgg2D]_ can be useful
