#ifndef bbggLTMaker_h
#define bbggLTMaker_h

#include <TROOT.h>
#include <TChain.h>
#include <TFile.h>
#include <TSelector.h>
#include "TH2F.h"
#include "TH3F.h"

// Header file for the classes stored in the TTree if any.
#include "vector"
#include "Math/GenVector/LorentzVector.h"
#include "Math/GenVector/PxPyPzE4D.h"
#include "vector"
#include "map"

class bbggLTMaker : public TSelector {
public :
   TTree          *fChain;   //!pointer to the analyzed TTree or TChain


   // Output tree and variables:
   TString _outFileName;
   TFile *_outFile;
   TTree *_outTree;

   Double_t o_weight;
   Double_t o_mgg, o_mjj, o_bbggMass, o_MX;
   Int_t o_catID;
   UInt_t o_run;
   ULong64_t o_evt;

   // Some external variables:
   Double_t _normalization;
   Bool_t _genDiPhotonFilter;
   
   //Photon ID SF stuff
   TFile* photonidFile;
   TH2F* photonIDhist;
   TFile* csevFile;
   TH2F* csevhist;

   //Trigger stuff
   TFile* triggerFile;
   TH3F* ltriggerHist;
   TH3F* striggerHist;

   Int_t _phoVariation, _trigVariation;


   
   //Variables from the input tree:
   
   typedef ROOT::Math::LorentzVector<ROOT::Math::PxPyPzE4D<double> > LorentzVector;
 
   Double_t        genTotalWeight;
   //Double_t        gen_mHH;
   //Double_t        gen_cosTheta;
   LorentzVector   *leadingPhoton;
   Float_t         leadingPhotonR9full5x5;
   LorentzVector   *subleadingPhoton;
   Float_t         subleadingPhotonR9full5x5;
   Int_t           nPromptInDiPhoton;
   Float_t         leadingJet_bDis;
   Float_t         subleadingJet_bDis;
   Float_t         HHTagger2017;
   Float_t         HHTagger2017_transform;
   LorentzVector   *dijetCandidate;

   Int_t           isSignal;

   Float_t         HHTagger;
   Float_t         HHTagger_LM;
   Float_t         HHTagger_HM;

   Float_t         MX;

   //Int_t           nmus;
   //Int_t           nmus_loose;
   //Int_t           nelecs;
   //Int_t           nelecs_loose;
   //Float_t         rho;
   //Int_t           nvtx;
   ULong64_t       event;
   UInt_t          lumi;
   //Int_t           processIndex;
   UInt_t          run;
   //Int_t           nvtx;
   //Float_t         npu;
   //Float_t         puweight;
   //Double_t        lumiFactor;

   // List of branches
   //TBranch        *b_genWeights;   //!
   TBranch        *b_genTotalWeight;   //!
   //TBranch        *b_gen_mHH;   //!
   //TBranch        *b_cosTheta;   //!
      
   TBranch        *b_leadingPhoton; //!
   TBranch        *b_leadingPhotonR9full5x5;   //!
   TBranch        *b_subleadingPhoton;
   TBranch        *b_subleadingPhotonR9full5x5;   //!
   TBranch        *b_nPromptInDiPhoton;   //!
   TBranch        *b_leadingJet_bDis;   //!
   TBranch        *b_subleadingJet_bDis;   //!

   TBranch        *b_HHTagger2017;   //!
   TBranch        *b_HHTagger2017_transform;   //!
   TBranch        *b_dijetCandidate;

   TBranch        *b_isSignal;   //!

   TBranch        *b_HHTagger;   //!
   TBranch        *b_HHTagger_LM;   //!
   TBranch        *b_HHTagger_HM;   //!

   TBranch        *b_MX;   //!

   //TBranch        *b_nelecs;   //!
   //TBranch        *b_nelecs_loose;   //!
   //TBranch        *b_rho;   //!
   //TBranch        *b_nvtx;   //!
   TBranch        *b_event;   //!
   TBranch        *b_lumi;   //!
   //TBranch        *b_processIndex;   //!
   TBranch        *b_run;   //!
   //TBranch        *b_nvtx;   //!
   //TBranch        *b_npu;   //!
   //TBranch        *b_puweight;   //!
   //TBranch        *b_lumiFactor;   //!

   bbggLTMaker(TTree * /*tree*/ =0) : fChain(0) { }
   virtual ~bbggLTMaker() { }
   virtual Int_t   Version() const { return 2; }
   virtual void    Begin(TTree *tree);
   virtual void    SlaveBegin(TTree *tree);
   virtual void    Init(TTree *tree);
   virtual Bool_t  Notify();
   virtual Bool_t  Process(Long64_t entry);
   virtual Int_t   GetEntry(Long64_t entry, Int_t getall = 0) { return fChain ? fChain->GetTree()->GetEntry(entry, getall) : 0; }
   virtual void    SetOption(const char *option) { fOption = option; }
   virtual void    SetObject(TObject *obj) { fObject = obj; }
   virtual void    SetInputList(TList *input) { fInput = input; }
   virtual TList  *GetOutputList() const { return fOutput; }
   virtual void    SlaveTerminate();
   virtual void    Terminate();



   void SetupPhotonSF(TString idfile, TString evfile);
   float PhotonSF(LorentzVector pho, int phovar = 0);
   void SetupTriggerSF(TString trig_file);
   float TriggerSF(LorentzVector lpho, float lr9, LorentzVector spho, float sr9, int var);
   
   ClassDef(bbggLTMaker,0);
};

#endif

#ifdef bbggLTMaker_cxx
void bbggLTMaker::Init(TTree *tree)
{
   // The Init() function is called when the selector needs to initialize
   // a new tree or chain. Typically here the branch addresses and branch
   // pointers of the tree will be set.
   // It is normally not necessary to make changes to the generated
   // code, but the routine can be extended by the user if needed.
   // Init() will be called many times when running on PROOF
   // (once per file to be processed).

   // Set object pointer
   //genWeights = 0;
   leadingPhoton = 0;
   subleadingPhoton = 0;
   dijetCandidate = 0;
   // Set branch addresses and branch pointers
   if (!tree) return;
   fChain = tree;
   //fChain->SetMakeClass(1); //This thing breaks LorentzVectors
   
//   fChain->SetBranchAddress("genWeights", &genWeights, &b_genWeights);
   fChain->SetBranchAddress("genTotalWeight", &genTotalWeight, &b_genTotalWeight);
//   fChain->SetBranchAddress("gen_mHH", &gen_mHH, &b_gen_mHH);
//   fChain->SetBranchAddress("gen_cosTheta", &gen_cosTheta, &b_cosTheta);
   fChain->SetBranchAddress("leadingPhoton", &leadingPhoton, &b_leadingPhoton);
   fChain->SetBranchAddress("subleadingPhoton", &subleadingPhoton, &b_subleadingPhoton);

   fChain->SetBranchAddress("leadingPhotonR9full5x5", &leadingPhotonR9full5x5, &b_leadingPhotonR9full5x5);
   fChain->SetBranchAddress("subleadingPhotonR9full5x5", &subleadingPhotonR9full5x5, &b_subleadingPhotonR9full5x5);
   fChain->SetBranchAddress("nPromptInDiPhoton", &nPromptInDiPhoton, &b_nPromptInDiPhoton);
   fChain->SetBranchAddress("leadingJet_bDis", &leadingJet_bDis, &b_leadingJet_bDis);
   fChain->SetBranchAddress("subleadingJet_bDis", &subleadingJet_bDis, &b_subleadingJet_bDis);

   fChain->SetBranchAddress("HHTagger2017", &HHTagger2017, &b_HHTagger2017);
   fChain->SetBranchAddress("HHTagger2017_transform", &HHTagger2017_transform, &b_HHTagger2017_transform);
   fChain->SetBranchAddress("dijetCandidate", &dijetCandidate, &b_dijetCandidate);

   fChain->SetBranchAddress("isSignal", &isSignal, &b_isSignal);

   fChain->SetBranchAddress("HHTagger", &HHTagger, &b_HHTagger);
   fChain->SetBranchAddress("HHTagger_LM", &HHTagger_LM, &b_HHTagger_LM);
   fChain->SetBranchAddress("HHTagger_HM", &HHTagger_HM, &b_HHTagger_HM);

   fChain->SetBranchAddress("MX", &MX, &b_MX);
   fChain->SetBranchAddress("event", &event, &b_event);

   fChain->SetBranchAddress("run", &run, &b_run);
}

Bool_t bbggLTMaker::Notify()
{
   // The Notify() function is called when a new file is opened. This
   // can be either for a new TTree in a TChain or when when a new TTree
   // is started when using PROOF. It is normally not necessary to make changes
   // to the generated code, but the routine can be extended by the
   // user if needed. The return value is currently not used.

   return kTRUE;
}

#endif // #ifdef bbggLTMaker_cxx
