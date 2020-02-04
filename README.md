# Instuctions how to run 

* Get combine tools:

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
Get this repository code:
```
cd ../
git clone git@github.com:ivovtin/bbggLimits2018.git -b run2_analysis 
cd bbggLimits2018
scramv1 b
```
## 1. You should use samples without categorisation obtained from the default branch of flashgg ([link](https://github.com/cms-analysis/flashgg)) 

## 2. Limit trees

* Run the limit tree maker (makeLT.py):

```
. run1.sh
```
The goal of this code is to categorize events and make a new tree which *catID* variable, as well as *mgg* and *mjj*. 

### Notes on limit trees

The core code that makes the trees is `bbggLTMaker.C`. It is based on
[TSelector](https://root.cern.ch/developing-tselector) and does not depend on CMSSW, just
the ROOT.  
Different type of categorizations can be done chosen by option `-c Y`
The recommended option is Y = 2:  
```
 Y = 0: HIG-17-008 categorization used in 2016 analysis (4 categories)
 Y = 1: HIG-19-018 tagger, using 2016 style categorization (4 categories);
 Y = 2: HIG-19-018 tagger, with optimized categorization without mjj cut (12 categories);
 Y = 3: HIG-19-018 tagger, with optimized categorization and mjj cuts (12 categories);
```

## 3. Fits and limits

 1. The prepare file ([Envelopejson](https://github.com/ivovtin/bbggLimits2018/blob/run2_analysis/jsonsForEnvelope/Env_json_2D_ttHon0.26_31012020_emplty.dat)) with single orders (for *mgg* and *mjj* proections) and without indicate best functions for each category. The path to file indicate here ([link](https://github.com/ivovtin/bbggLimits2018/blob/227d17cf267e3520ee8f9830a2849ce370b34d54/runLimit.py#L300))

 2. Create workspace on the produced LTs:
```
. run2.sh
```

The config file `json/conf_default.json` can be edited to provide needed parameters. Some of them are:
```
 LTDIR: location of the input Limit Trees (expected to be in the local diractory, after running previous step)
 ncat: number of categories. This should much the number of categories produced in limit tries (currently, should be 4 or 12)
 fitStrategy: 2 - for 2D fit of (mgg, mjj); 1 - for 1D fit of mgg, in which case a cut is set to 100<mjj<150 somewhere in runLimit.py script.
```

The results of the limit will be in `LIMS_OutDir/Node_SM/result_1.log`. In case of problems,
the logfile _mainLog_data-time.log[.bbgg2D]_ can be useful


 3. Run the FTest on the workspace ([link](https://github.com/ivovtin/Envelop#ftest))

 4. Write the orders in the file from step 1 how here ([Envelopejson](https://github.com/ivovtin/bbggLimits2018/blob/run2_analysis/jsonsForEnvelope/Env_json_2D_ttHon0.26_31012020.dat)) 

 5. Create new workspace with received orders from FTEST:
```
. run2.sh
```

 6. Search best functions for Envelope ([link](https://github.com/ivovtin/Envelop#ftest))

 7. Write the best functions in the file from step 1 how here ([Envelopejson](https://github.com/ivovtin/bbggLimits2018/blob/run2_analysis/jsonsForEnvelope/Env_json_2D_ttHon0.26_31012020.dat)) in line after orders

 8. Create new workspace with received functions:
```
. run2.sh
```

 9. Extraction limit: 
```
. run3.sh
```

The process may take a while to complete, especially when running with many categories (about 30 minuts for 2D method).  



