void add_branch_ggHHKL()
{  
   //TString indirFT = "/afs/cern.ch/work/i/ivovtin/public/legacy_branch_flattrees/vbfhh/2018/2018_SM_c2v01_noCosTheta_conversion/";
   //TString indirFT = "/afs/cern.ch/work/i/ivovtin/public/legacy_branch_flattrees/vbfhh/2016/2016_SM_c2v01_noCosTheta_conversion/";
   TString indirFT = "/afs/cern.ch/work/i/ivovtin/public/legacy_branch_flattrees/vbfhh/2017/2017_SM_c2v01_noCosTheta_conversion/";

   TString outdirFT;

   gSystem->Exec("mkdir " + indirFT + "ggHHKL");
   outdirFT = indirFT + "ggHHKL/";

   //==========================================
   //['ggHH_kl_1_kt_1', 'ggHH_kl_0_kt_1', 'ggHH_kl_5_kt_1', 'dummy1', 'dummy2', 'dummy3']

   TString Sig1 = "output_GluGluToHHTo2B2G_node_cHHH1_TuneCP5_PSWeights_13TeV-powheg-pythia8.root";
   TString Sig2 = "output_GluGluToHHTo2B2G_node_cHHH0_TuneCP5_PSWeights_13TeV-powheg-pythia8.root";
   TString Sig3 = "output_GluGluToHHTo2B2G_node_cHHH5_TuneCP5_PSWeights_13TeV-powheg-pythia8.root";

   for (TString fname : {Sig1,Sig2,Sig3})
   {
      TString fnameAll = indirFT + fname;   
      TFile *inputAll = TFile::Open(fnameAll);
      TTree *inTree = (TTree*)inputAll->Get("bbggSelectionTree");                                     

      char branchname[81];     
      for (int n = 0; n < 80; n++)
      {
         sprintf(branchname,"KL%d",n+1);
         inTree->SetBranchStatus(branchname,0);
      }

      TFile *newfile = new TFile(outdirFT + fname,"recreate");   
      TTree *newtree = inTree->CloneTree();
      
      //TTree *newtree = inTree->CloneTree(-1,"KL2");   
     
      float KL[81];
      TBranch *brKL[81];     
      
      for (int n = 0; n < 81; n++) 
      {
         sprintf(branchname,"KL%d",n+1);
         brKL[n] = newtree->Branch(branchname, &KL[n]); 
      }
     
      TString fweights="outKLweight_ggHH.txt";

      ifstream KLweightfile (fweights);

      float inKL=0, weight1=0, weight2=0, weight3=0, weight4=0, weight5=0, weight6=0; 
      Long64_t nentries = newtree->GetEntries(); 
      cout<<"nentries="<<nentries<<endl;
      for (Long64_t i=0; i<nentries; i++) 
      { 
        newtree->GetEntry(i);

       while(!KLweightfile.eof())
       {
            KLweightfile >> inKL >> weight1 >> weight2 >> weight3 >> weight4 >> weight5 >> weight6;
            //cout<<"inKL="<<inKL<<"\t"<<"weight1="<<weight1<<endl; 
            //cout<<i<<endl; 
            for(int k = 0; k < 81; k++)
            {
               if( fname==Sig1 && (k+1)==inKL ) { KL[k]=weight1; /*cout<<"inKL="<<inKL<<"\t"<<"weight1="<<weight1<<endl;*/ }
               if( fname==Sig2 && (k+1)==inKL ) { KL[k]=weight2; /*cout<<"inKL="<<inKL<<"\t"<<"weight2="<<weight2<<endl;*/ }
               if( fname==Sig3 && (k+1)==inKL ) { KL[k]=weight3; /*cout<<"inKL="<<inKL<<"\t"<<"weight3="<<weight3<<endl;*/ }
            }
        }
       
         for (int n = 0; n < 81; n++) brKL[n]->Fill();
      }   
      KLweightfile.close ();   
      newtree->Print();       
      newfile->Write();
      newfile->Close();
  }
}
