import OutputManager as omng
import argparse
import fnmatch
import os
import sys
import ROOT
import itertools


# VBF HH Reweighting
# reads from the config the 6 input samples and the target couplings

target_cv  = [1]
target_c2v = [1]
target_kl  = [-6]
target_bf  = 0.0026

#output_VBFHHTo2B2G_CV_0_5_C2V_1_C3_1_TuneCP5_PSWeights_13TeV-madgraph-pythia8.root
#output_VBFHHTo2B2G_CV_1_5_C2V_1_C3_1_TuneCP5_PSWeights_13TeV-madgraph-pythia8.root
#output_VBFHHTo2B2G_CV_1_C2V_1_C3_0_TuneCP5_PSWeights_13TeV-madgraph-pythia8.root
#output_VBFHHTo2B2G_CV_1_C2V_1_C3_1_TuneCP5_PSWeights_13TeV-madgraph-pythia8.root
#output_VBFHHTo2B2G_CV_1_C2V_1_C3_2_TuneCP5_PSWeights_13TeV-madgraph-pythia8.root
#output_VBFHHTo2B2G_CV_1_C2V_2_C3_1_TuneCP5_PSWeights_13TeV-madgraph-pythia8.root



inputSigList = ['VBFHHTo2B2G_CV_1_C2V_1_C3_1', 'VBFHHTo2B2G_CV_1_C2V_1_C3_0', 'VBFHHTo2B2G_CV_1_C2V_1_C3_2', 'VBFHHTo2B2G_CV_1_C2V_2_C3_1', 'VBFHHTo2B2G_CV_1_5_C2V_1_C3_1', 'VBFHHTo2B2G_CV_0_5_C2V_1_C3_1']

omngr = omng.OutputManager()

print "ikL is an integer, a labelling convention used by Lata: ikl = (6+kl)/0.2+1. In the tree the branch with ikl weight shall be named \"KL\",ikl is KL1, KL2, KL80. You can scan kL by steps of 0.2 starting from -6"

omngr.makeVBFrew(inputSigList, target_kl, target_cv, target_c2v, target_bf)

