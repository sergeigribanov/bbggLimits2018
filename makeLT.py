#!/usr/bin/env python

from ROOT import *
import os,json
gROOT.SetBatch()
from SMHiggsSamples import *
from utils import *

import argparse
parser =  argparse.ArgumentParser(description='Limit Tree maker')
parser.add_argument("indir", help = "Input directory with flat trees.")
parser.add_argument('-x', nargs='+', choices=['res', 'nonres'], required=True, default=None,
                    help = "Choose which samlples to create the trees from.")
parser.add_argument("-v", "--verbosity",  dest="verb", action="store_true", default=False,
                    help="Print out more stuff")
parser.add_argument("-l", "--lumi", dest="lumi", type=float, default=0.,
#parser.add_argument("-l", "--lumi", dest="lumi", default=59.4,  #2018 year
#parser.add_argument("-l", "--lumi", dest="lumi", default=41.5,  #2017 year
#parser.add_argument("-l", "--lumi", dest="lumi", default=35.9,   #2016 year
                    help="Integrated lumi to scale signal")
parser.add_argument('-o', '--outDir', dest="outDir", type=str, default=None,
                    required=True, help="Output directory (will be created).")
parser.add_argument('-c', '--categ', dest="categ", type=int, default=0,
                    choices = [0,1,2,3,4,5], help="Which categorization to use. 0 - 2016 tagger; 1 - 2017 ETH tagger, using 2016 style categorization; 2 - 2017 ETH tagger, with optimized categorization; 3 - 2017 tagger with mjj cuts; 4 - 2019 tagger with Mjj in train; 5 - 15 categories")

opt = parser.parse_args()

nodes = [ ["box", 50000], ["SM", 369000],
          [2, 49600], [3, 50000], [ 4, 50000], [ 5, 50000], [ 6, 50000], [ 7, 50000],
          [8, 50000], [9, 49600], [10, 49800], [11, 50000], [12, 50000], [13, 50000] ]
_ttHTagger=0;
if opt.verb>0:
  print SMHiggsNodes


if __name__ == "__main__":
  print "This is the __main__ part"

  createDir(opt.outDir)
  if 'nonres' in opt.x:

    print "Doing signal"
    for n in nodes:
      if n[0]!="SM": continue # Only do SM node for now             

      #fChain = TChain("tagsDumper/trees/bbggSelectionTree")
      fChain = TChain("bbggSelectionTree")
      #fname = opt.indir+"/output_GluGluToHHTo2B2G_node_"+str(n[0])+"_13TeV-madgraph.root"  
      #fname = opt.indir+"/output_GluGluToHHTo2B2G_allnodes_no_unit_norm.root"
      #fname = opt.indir+"/output_GluGluToHHTo2B2G_all_nodes.root"
      #fname = opt.indir+"/output_GluGluToHHTo2B2G_node_cHHH1.root"
      #fname = opt.indir+"/output_GluGluToHHTo2B2G_node_cHHH5.root"
      #fname = opt.indir+"/output_GluGluToHHTo2B2G_all_node.root"
      #fname = opt.indir+"/output_GluGluToHHTo2B2G_node_cHHH1_TuneCP5_PSWeights_13TeV-powheg-pythia8.root"
      fname = opt.indir+"/output_GluGluToHHTo2B2G_node_cHHH015_TuneCP5_PSWeights_13TeV-powheg-pythia8.root"
      #fname = opt.indir+"/output_GluGluToHHTo2B2G_allnodes_C2Scan.root"
      #fname = opt.indir+"/KL/output_VBFHHTo2B2G_allnodes_TuneCP5_PSWeights_13TeV-madgraph-pythia8.root"
      fChain.Add(fname)
      ttHkiller = fChain.GetListOfBranches().FindObject("ttHScore");
      if ttHkiller: _ttHTagger=1
      rootName = fname[fname.rfind('/')+1:]
      if opt.verb: print rootName

      #outFileName = opt.outDir+"/LT_"+rootName
      outFileName = opt.outDir+"/LT_output_GluGluToHHTo2B2G_node_SM_13TeV-madgraph.root"

      fChain.Process("bbggLTMaker.C+", "%.10f %s %i %i %i" % ( opt.lumi, outFileName, 0, opt.categ, _ttHTagger) )

    print "Done with signal"

    print "Doing Single Higgs samples"
    for n in SMHiggsNodes:
      if opt.verb: print n
      #fChain = TChain("tagsDumper/trees/bbggSelectionTree")
      fChain = TChain("bbggSelectionTree")
      fname = opt.indir+n[0]
#     
      fChain.Add(fname)
      outFileName = opt.outDir+"/LT_"+n[0]

      fChain.Process("bbggLTMaker.C+", "%.10f %s %i %i %i" % ( opt.lumi, outFileName, 1, opt.categ, _ttHTagger) )

    os.system('hadd -f '+opt.outDir+'/LT_output_bbHToGG_M-125_13TeV_amcatnlo.root '+opt.outDir+'/LT_output_bbHToGG_M-125_4FS_yb*.root')

    print "Done with Single Higgs"

    fChain = TChain("bbggSelectionTree")
    fname = opt.indir+'/Data.root'
#    fname = opt.indir+'/DiPhotBenriched_GJet.root'
#    fname = opt.indir+'/DiPhot_GJet.root'
#    fname = opt.indir+'/DiPhot_GJet_L2reg.root'
    fChain.Add(fname)
    outFileName = opt.outDir+"/LT_DoubleEG.root"
    
    fChain.Process("bbggLTMaker.C+", "%f %s %i %i %i" % ( 1, outFileName, 0, opt.categ, _ttHTagger) )
    
    print "Done with Data"
   
  print "Done with Main"

