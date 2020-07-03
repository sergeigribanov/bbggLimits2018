{
  TFile *oldfile = new TFile("/afs/cern.ch/work/i/ivovtin/public/legacy_branch_flattrees/vbfhhMjjreg/2018/output_GluGluToHHTo2B2G_node_cHHH0_TuneCP5_PSWeights_13TeV-powheg-pythia8.root");
  TTree *oldtree = (TTree*)oldfile->Get("bbggSelectionTree");
  Int_t nentries = (Int_t)oldtree->GetEntries();
  ULong64_t event = 0;
  oldtree->SetBranchAddress("event",&event);

  TFile *newfile = new TFile("/afs/cern.ch/work/i/ivovtin/public/legacy_branch_flattrees/vbfhhMjjreg/2018/output_GluGluToHHTo2B2G_node_cHHH0_TuneCP5_PSWeights_13TeV-powheg-pythia8_delEv.root","recreate");
  TTree *newtree = oldtree->CloneTree(0);

  for (Int_t i=0;i<nentries; i++) {
      oldtree->GetEntry(i);
      if (event != 174489) newtree->Fill();
  }
  newtree->Print();
  newtree->AutoSave();
}
