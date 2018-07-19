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

## Limit trees

* Run the limit tree maker:

```
./makeLT.py /eos/cms/store/group/phys_higgs/resonant_HH/RunII/FlatTrees/2016/2018_05_04_HHTaggerETH/ -x nonres -o LT_OutDir [-c Y]
```

The goal of this code is to categorize events and make a new tree which *catID* variable,
as well as *mgg* and *mjj*. Different type of categorizations can be done chosen by option `-c Y`:  
```
 Y = 0: 2016 tagger with categorization used in 2016 analysis (4 categories)
 Y = 1: 2017 ETH tagger, using 2016 style categorization (4 categories);
 Y = 2: 2017 ETH tagger, with optimized categorization (12 categories);
 Y = 3: 2017 ETH tagger, with optimized categorization and mjj cuts (12 categories);
```

The core code that makes the trees is `bbggLTMaker.C`. It is based on
[TSelector](https://root.cern.ch/developing-tselector) and does not depend on CMSSW, just
the ROOT.

### Notes on limit trees
* The scale factors for b-tagging are not included in this code. This is because a) the
  method we used in 2016 is outdated; and b) it requires compilation together with some
  CMSSW packages which is not trivial in TSelector code. For the reference,
  [this code](https://github.com/ResonantHbbHgg/bbggLimits/blob/edda480b80455c3d14f644629e5faaa5997cc9f0/src/bbggLTMaker.cc#L278-L340)
  was used in 2016 to apply the SF fpr b-tagging.
* In order to implement another categorization, one has to put it
  [here](https://github.com/ResonantHbbHgg/bbggLimits2018/blob/f031e57c6e938be983b006fc1f81a01ec53ea61a/bbggLTMaker.C#L268)
  in _bbggLTMaker_ code.

## Fits and limits
* Run the fits and limits on the produced LTs:

```
./runLimit.py -f conf_default.json --node=SM -o LIMS_OutDir
```
where config file can be edited to point to input directory and set various parameters. The process may take a while to complete.

The results of the limit will be in `LIMS_OutDir/Node_SM/result_1.log`. In case of problems,
the logfile _mainLog_data-time.log[.bbgg2D]_ can be useful


### Notes on datacards and limits

* Systematic uncertainties are not real (especially the ones for b-tagging and JEC) for
  the case of 2017 categorization (the older numbers are used). Once proper systematics
  are obtained they need to be propagated to template datacards for each category (in
  _templates_ directory).
* The same background fit function is used in all categories (3d order Benrnstein). It may
  be necessary in the future to have different functions in each category. This should be
  modifien from _templates/models_2D_higgs_mjj70_cat*.rs_ files and then taken care in
  _src/bbgg2DFitter.cc_
  


## The plots 

* Run the following command to do fit diagnostics of combine and make quick plots:
``` 
combine -M FitDiagnostics LIMS_OutDir/Node_SM/hhbbgg_13TeV_DataCard.txt --plots --out LIMS_OutDir/
```
  
Note that you may see many warnings. They are ignored at the moment, but should be fixed in the future.

* non-integer bin entry:  
```
[#0] WARNING:Plotting -- RooHist::addBin(ch4_plot__mgg) WARNING: non-integer bin entry 14.5154 with Poisson errors, interpolating between Poisson errors of adjacent integer
```  
These are probably due to the fact that the observed data are taken from MC with weights and the events are not integers. 
* parameters at boundary:
```
[WARNING] Found [CMS_hhbbgg_13TeV_mjj_bkg_slope2_cat0] at boundary.
```
