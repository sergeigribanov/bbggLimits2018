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
target_kl  = [-10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
target_bf  = 0.0026

#output_VBFHHTo2B2G_CV_0_5_C2V_1_C3_1_TuneCP5_PSWeights_13TeV-madgraph-pythia8.root
#output_VBFHHTo2B2G_CV_1_5_C2V_1_C3_1_TuneCP5_PSWeights_13TeV-madgraph-pythia8.root
#output_VBFHHTo2B2G_CV_1_C2V_1_C3_0_TuneCP5_PSWeights_13TeV-madgraph-pythia8.root
#output_VBFHHTo2B2G_CV_1_C2V_1_C3_1_TuneCP5_PSWeights_13TeV-madgraph-pythia8.root
#output_VBFHHTo2B2G_CV_1_C2V_1_C3_2_TuneCP5_PSWeights_13TeV-madgraph-pythia8.root
#output_VBFHHTo2B2G_CV_1_C2V_2_C3_1_TuneCP5_PSWeights_13TeV-madgraph-pythia8.root



inputSigList = ['VBFHHTo2B2G_CV_1_C2V_1_C3_1', 'VBFHHTo2B2G_CV_1_C2V_1_C3_0', 'VBFHHTo2B2G_CV_1_C2V_1_C3_2', 'VBFHHTo2B2G_CV_1_C2V_2_C3_1', 'VBFHHTo2B2G_CV_1_5_C2V_1_C3_1', 'VBFHHTo2B2G_CV_0_5_C2V_1_C3_1']

omngr = omng.OutputManager()

makePlot = False

omngr.makeVBFrew(inputSigList, target_kl, target_cv, target_c2v, target_bf, makePlot)

