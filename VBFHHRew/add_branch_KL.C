void add_branch_KL()
{  
   TString indirFT = "/afs/cern.ch/work/i/ivovtin/public/legacy_branch_flattrees/vbfhh/2018/withMVA/";
   TString outdirFT;
   bool KLscan=0;  
   bool C2Vscan=1; 

   if ( KLscan ) outdirFT = "/afs/cern.ch/work/i/ivovtin/public/legacy_branch_flattrees/vbfhh/2018/withMVA/KL/";
   if ( C2Vscan ) outdirFT = "/afs/cern.ch/work/i/ivovtin/public/legacy_branch_flattrees/vbfhh/2018/withMVA/C2V/"; 

   //==========================================
   //['VBFHHTo2B2G_CV_1_C2V_1_C3_1', 'VBFHHTo2B2G_CV_1_C2V_1_C3_0', 'VBFHHTo2B2G_CV_1_C2V_1_C3_2', 'VBFHHTo2B2G_CV_1_C2V_2_C3_1', 'VBFHHTo2B2G_CV_1_5_C2V_1_C3_1', 'VBFHHTo2B2G_CV_0_5_C2V_1_C3_1']

   TString Sig1 = "output_VBFHHTo2B2G_CV_1_C2V_1_C3_1_TuneCP5_PSWeights_13TeV-madgraph-pythia8.root";
   TString Sig2 = "output_VBFHHTo2B2G_CV_1_C2V_1_C3_0_TuneCP5_PSWeights_13TeV-madgraph-pythia8.root";
   TString Sig3 = "output_VBFHHTo2B2G_CV_1_C2V_1_C3_2_TuneCP5_PSWeights_13TeV-madgraph-pythia8.root";
   TString Sig4 = "output_VBFHHTo2B2G_CV_1_C2V_2_C3_1_TuneCP5_PSWeights_13TeV-madgraph-pythia8.root";
   TString Sig5 = "output_VBFHHTo2B2G_CV_1_5_C2V_1_C3_1_TuneCP5_PSWeights_13TeV-madgraph-pythia8.root";
   TString Sig6 = "output_VBFHHTo2B2G_CV_0_5_C2V_1_C3_1_TuneCP5_PSWeights_13TeV-madgraph-pythia8.root";

   for (TString fname : {Sig1,Sig2,Sig3,Sig4,Sig5,Sig6})
   {
      TString fnameAll = indirFT + fname;   
      TFile *inputAll = TFile::Open(fnameAll);
      TTree *inTree = (TTree*)inputAll->Get("bbggSelectionTree");                                     

      TFile *newfile = new TFile(outdirFT + fname,"recreate");   
      TTree *newtree = inTree->CloneTree();   

      char branchname[81];     
      float KL[81], C2V[81];
      TBranch *brKL;     
      TBranch *brC2V;     
      
      if ( KLscan )
      {
         for (int n = 0; n < 81; n++) 
         {
           sprintf(branchname,"KL%d",n+1);
           brKL = newtree->Branch(branchname, &KL[n]); 
         }
      }
       if ( C2Vscan )
      {
         for (int n = 0; n < 81; n++) 
         {
           sprintf(branchname,"C2V%d",n+1);
           brC2V = newtree->Branch(branchname, &C2V[n]); 
         }
      }
    
      float inKL=0, inC2V=0, inCV=0,  weight1=0, weight2=0, weight3=0, weight4=0, weight5=0, weight6=0; 
      ifstream KLweightfile ("outKLweight.txt");

      Long64_t nentries = newtree->GetEntries(); 
      for (Long64_t i=0; i<nentries; i++) 
      { 
        newtree->GetEntry(i); 
 
        while(!KLweightfile.eof())
        {
            KLweightfile >> inKL >> inC2V >> inCV >> weight1 >> weight2 >> weight3 >> weight4 >> weight5 >> weight6;
            //cout<<"inKL="<<inKL<<"\t"<<"weight1="<<weight1<<endl; 
            if ( KLscan )
            {   
              for(int k = 0; k < 81; k++)
              {
                if( fname==Sig1 && (k+1)==inKL ) { KL[k]=weight1; cout<<"inKL="<<inKL<<"\t"<<"weight1="<<weight1<<endl; }
                if( fname==Sig2 && (k+1)==inKL ) { KL[k]=weight2; cout<<"inKL="<<inKL<<"\t"<<"weight2="<<weight2<<endl; }
                if( fname==Sig3 && (k+1)==inKL ) { KL[k]=weight3; cout<<"inKL="<<inKL<<"\t"<<"weight3="<<weight3<<endl; }
                if( fname==Sig4 && (k+1)==inKL ) { KL[k]=weight4; cout<<"inKL="<<inKL<<"\t"<<"weight4="<<weight4<<endl; }
                if( fname==Sig5 && (k+1)==inKL ) { KL[k]=weight5; cout<<"inKL="<<inKL<<"\t"<<"weight5="<<weight5<<endl; }
                if( fname==Sig6 && (k+1)==inKL ) { KL[k]=weight6; cout<<"inKL="<<inKL<<"\t"<<"weight6="<<weight6<<endl; }
              }
            }

            if ( C2Vscan )
            {   
                for(int k = 0; k < 81; k++)
              {
                if( fname==Sig1 && (k+1)==inC2V ) { C2V[k]=weight1; cout<<k<<"\t"<<"inC2V="<<inC2V<<"\t"<<"weight1="<<weight1<<endl; }
                if( fname==Sig2 && (k+1)==inC2V ) { C2V[k]=weight2; cout<<k<<"\t"<<"inC2V="<<inC2V<<"\t"<<"weight2="<<weight2<<endl; }
                if( fname==Sig3 && (k+1)==inC2V ) { C2V[k]=weight3; cout<<k<<"\t"<<"inC2V="<<inC2V<<"\t"<<"weight3="<<weight3<<endl; }
                if( fname==Sig4 && (k+1)==inC2V ) { C2V[k]=weight4; cout<<k<<"\t"<<"inC2V="<<inC2V<<"\t"<<"weight4="<<weight4<<endl; }
                if( fname==Sig5 && (k+1)==inC2V ) { C2V[k]=weight5; cout<<k<<"\t"<<"inC2V="<<inC2V<<"\t"<<"weight5="<<weight5<<endl; }
                if( fname==Sig6 && (k+1)==inC2V ) { C2V[k]=weight6; cout<<k<<"\t"<<"inC2V="<<inC2V<<"\t"<<"weight6="<<weight6<<endl; }
              }
            }
 
        }
        
        newtree->Fill();
      }   
      KLweightfile.close ();   
      newtree->Print();       
      newfile->Write();
      newfile->Close();
  }
}
