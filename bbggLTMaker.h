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
   Int_t _whichCategorization;
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
   Int_t           candidate_id;
   Float_t         weight;
   Float_t         CMS_hgg_mass;
   Float_t         dZ;
   Float_t         centralObjectWeight;
   Float_t         leadingJet_bDis;
   Float_t         subleadingJet_bDis;
   Float_t         absCosThetaStar_CS;
   Float_t         absCosTheta_bb;
   Float_t         absCosTheta_gg;
   Float_t         diphotonCandidatePtOverdiHiggsM;
   Float_t         dijetCandidatePtOverdiHiggsM;
   Float_t         customLeadingPhotonIDMVA;
   Float_t         customSubLeadingPhotonIDMVA;
   Float_t         leadingPhotonSigOverE;
   Float_t         subleadingPhotonSigOverE;
   Float_t         sigmaMOverM;
   Float_t         sigmaMOverMDecorr;
   Float_t         sigmaMJets;
   Float_t         PhoJetMinDr;
   Float_t         HHbbggMVA;
   Float_t         MX;
   Float_t         Mjj;
   Float_t         dijet_pt;
   Float_t         dijet_eta;
   Float_t         dijet_phi;
   Float_t         diphoton_pt;
   Float_t         diphoton_eta;
   Float_t         diphoton_phi;
   Float_t         diHiggs_pt;
   Float_t         diHiggs_mass;
   Float_t         diHiggs_eta;
   Float_t         diHiggs_phi;
   Float_t         category;
   Float_t         leadingPhoton_pt;
   Float_t         leadingPhoton_eta;
   Float_t         leadingPhoton_phi;
   Float_t         subleadingPhoton_pt;
   Float_t         subleadingPhoton_eta;
   Float_t         subleadingPhoton_phi;
   Float_t         leadingJet_pt;
   Float_t         leadingJet_eta;
   Float_t         leadingJet_phi;
   Float_t         leadingJet_mass;
   Float_t         leadingJet_bRegNNCorr;
   Float_t         leadingJet_bRegNNResolution;
   Float_t         subleadingJet_pt;
   Float_t         subleadingJet_eta;
   Float_t         subleadingJet_phi;
   Float_t         subleadingJet_mass;
   Float_t         subleadingJet_bRegNNCorr;
   Float_t         subleadingJet_bRegNNResolution;
   Float_t         ttHMVA_MET;
   Float_t         ttHMVA_njets;
   Float_t         ttHMVA_Xtt0;
   Float_t         ttHMVA_Xtt1;
   Float_t         ttHMVA_MjjW0;
   Float_t         ttHMVA_MjjW1;
   Float_t         ttHMVA_Mjjbt0;
   Float_t         ttHMVA_Mjjbt1;
   Float_t         ttHMVA_leadingMuon;
   Float_t         ttHMVA_subleadingMuon;
   Float_t         ttHMVA_nmus;
   Float_t         ttHMVA_leadingElectron;
   Float_t         ttHMVA_subleadingElectron;
   Float_t         ttHMVA_nelecs;
   Float_t         ttHHHbggMVA;
   Float_t         leadingJet_bRegNNCorr;
   Float_t         leadingJet_bRegNNResolution;
   Float_t         subleadingJet_bRegNNCorr;
   Float_t         subleadingJet_bRegNNResolution;
   Float_t         sigmaMJets;
   Float_t         rho;
   Int_t           nvtx;
   ULong64_t       event;
   UInt_t          lumi;
   Int_t           processIndex;
   UInt_t          run;
   Int_t           nvtx;
   Float_t         npu;
   Float_t         puweight;

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
