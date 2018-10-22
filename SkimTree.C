void Skim(string sample){

  cout << sample.c_str() << endl;

  int frontier = 490;

  string predir =  string("/afs/cern.ch/work/l/lata/public/HH_bbyy/hadd_root/");

  string dir = predir + string("");
  string new_predir =  Form("SplitHigh_v3_Mass_%d/", frontier);

  string mkdir = string("mkdir ") + predir + new_predir;
  gSystem->Exec(mkdir.c_str());
  mkdir = string("mkdir ") + predir + new_predir + "LimitTrees_HighMass";
  gSystem->Exec(mkdir.c_str());
  mkdir = string("mkdir ") + predir + new_predir + "LimitTrees_LowMass";
  gSystem->Exec(mkdir.c_str());
		       

  string filename = dir + "/" + sample;
  string fileout_high = dir + Form("/High_%d", frontier) + sample;
  string newfileout_high = predir + new_predir + "/LimitTrees_HighMass/" + sample;

  TFile *_file0 = TFile::Open(filename.c_str());

  
  TTree *oldtree = (TTree*) _file0->Get("TCVARS;1");    

  TFile *newfile_high = new TFile(fileout_high.c_str(), "recreate");  
  TTree *newtree_high = oldtree->CloneTree(0); 

  newtree_high->SetMakeClass(1);
  oldtree->CopyAddresses(newtree_high);

  float mtot   = 0;
  oldtree->SetBranchAddress("mtot",&mtot);

  for (Long64_t iEntry=0; iEntry<oldtree->GetEntriesFast(); iEntry++)
    { // Main loop
      oldtree->GetEntry(iEntry); // get ith entry

      if(mtot > frontier)
	  newtree_high->Fill();
    }

  newfile_high->Write();
  newfile_high->Close();
  


  string cp = "cp " + fileout_high + " " + newfileout_high;
  gSystem->Exec(cp.c_str());



}

void SkimTree(){

  Skim("FakeData/DoubleEG.root");
  Skim("output_GluGluHToGG_M-125_13TeV_powheg_pythia8.root");
  Skim("output_GluGluToHHTo2B2G_node_SM_13TeV-madgraph_0.root");
  Skim("output_VBFHToGG_M-125_13TeV_powheg_pythia8.root");
  Skim("output_VHToGG_M125_13TeV_amcatnloFXFX_madspin_pythia8.root");
  Skim("output_bbHToGG_M-125_13TeV_amcatnlo.root");
  Skim("output_ttHToGG_M125_13TeV_powheg_pythia8_v2.root");


  


}
