//////////////////////////////////////////////////////////
// This class has been automatically generated on
// Thu Jun 21 13:14:45 2018 by ROOT version 6.06/09
// from TTree bbggSelectionTree/Flat tree for HH->bbgg analyses (after pre selection)
// found on file: output_GluGluToHHTo2B2G_node_SM_13TeV-madgraph.root
//////////////////////////////////////////////////////////

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
 
   // Declaration of leaf types
   //vector<double>  *genWeights;
   Double_t        genTotalWeight;
   //Double_t        gen_mHH;
   //Double_t        gen_cosTheta;
   //Double_t        gen_NRW;
   LorentzVector   *leadingPhoton;
//   vector<int>     *leadingPhotonID;
//   vector<int>     *leadingPhotonISO;
//   Int_t           leadingPhotonEVeto;
//   Float_t         leadingPhotonIDMVA;
//   Float_t         customLeadingPhotonIDMVA;
   Float_t         leadingPhotonR9full5x5;
//   Int_t           leadingPhotonHasGain1;
//   Int_t           leadingPhotonHasGain6;
   LorentzVector   *subleadingPhoton;
   //vector<int>     *subleadingPhotonID;
   //vector<int>     *subleadingPhotonISO;
   //Int_t           subleadingPhotonEVeto;
   //Float_t         subleadingPhotonIDMVA;
   //Float_t         customSubLeadingPhotonIDMVA;
   Float_t         subleadingPhotonR9full5x5;
   //Int_t           subLeadingPhotonHasGain1;
   //Int_t           subLeadingPhotonHasGain6;
   //Float_t         leadingPhotonSigOverE;
   //Float_t         subleadingPhotonSigOverE;
   //Float_t         sigmaMOverM;
   //Float_t         sigmaMOverMDecorr;
   //Float_t         diphoMVA;
 //ROOT::Math::LorentzVector<ROOT::Math::PxPyPzE4D<double> > *diphotonCandidate;
   //Double_t        fCoordinates_fX;
   //Double_t        fCoordinates_fY;
   //Double_t        fCoordinates_fZ;
   //Double_t        fCoordinates_fT;
   Int_t           nPromptInDiPhoton;
 //ROOT::Math::LorentzVector<ROOT::Math::PxPyPzE4D<double> > *leadingJet;
   //Double_t        fCoordinates_fX;
   //Double_t        fCoordinates_fY;
   //Double_t        fCoordinates_fZ;
   //Double_t        fCoordinates_fT;
 //ROOT::Math::LorentzVector<ROOT::Math::PxPyPzE4D<double> > *leadingJet_KF;
   //Double_t        fCoordinates_fX;
   //Double_t        fCoordinates_fY;
   //Double_t        fCoordinates_fZ;
   //Double_t        fCoordinates_fT;
 //ROOT::Math::LorentzVector<ROOT::Math::PxPyPzE4D<double> > *leadingJet_Reg;
   //Double_t        fCoordinates_fX;
   //Double_t        fCoordinates_fY;
   //Double_t        fCoordinates_fZ;
   //Double_t        fCoordinates_fT;
 //ROOT::Math::LorentzVector<ROOT::Math::PxPyPzE4D<double> > *leadingJet_RegKF;
   //Double_t        fCoordinates_fX;
   //Double_t        fCoordinates_fY;
   //Double_t        fCoordinates_fZ;
   //Double_t        fCoordinates_fT;
   Float_t         leadingJet_bDis;
  // Float_t         leadingJet_CSVv2;
  // Float_t         leadingJet_cMVA;
  // Int_t           leadingJet_flavour;
  // Int_t           leadingJet_hadFlavour;
 //ROOT::Math::LorentzVector<ROOT::Math::PxPyPzE4D<double> > *subleadingJet;
   //Double_t        fCoordinates_fX;
   //Double_t        fCoordinates_fY;
   //Double_t        fCoordinates_fZ;
   //Double_t        fCoordinates_fT;
 //ROOT::Math::LorentzVector<ROOT::Math::PxPyPzE4D<double> > *subleadingJet_KF;
   //Double_t        fCoordinates_fX;
   //Double_t        fCoordinates_fY;
   //Double_t        fCoordinates_fZ;
   //Double_t        fCoordinates_fT;
 //ROOT::Math::LorentzVector<ROOT::Math::PxPyPzE4D<double> > *subleadingJet_Reg;
   //Double_t        fCoordinates_fX;
   //Double_t        fCoordinates_fY;
   //Double_t        fCoordinates_fZ;
   //Double_t        fCoordinates_fT;
 //ROOT::Math::LorentzVector<ROOT::Math::PxPyPzE4D<double> > *subleadingJet_RegKF;
   //Double_t        fCoordinates_fX;
   //Double_t        fCoordinates_fY;
   //Double_t        fCoordinates_fZ;
   //Double_t        fCoordinates_fT;
   Float_t         subleadingJet_bDis;
//   Float_t         subleadingJet_CSVv2;
//   Float_t         subleadingJet_cMVA;
//   Int_t           subleadingJet_flavour;
//   Int_t           subleadingJet_hadFlavour;
//   Float_t         leadingJet_genPtb;
//   Float_t         leadingJet_genPartonidb;
//   Float_t         leadingJet_genFlavourb;
//   Float_t         leadingJet_genPartonFlavourb;
//   Float_t         leadingJet_genHadronFlavourb;
//   Float_t         leadingJet_genNbHadronsb;
//   Float_t         leadingJet_genNcHadronsb;
//   Float_t         subleadingJet_genPtb;
//   Float_t         subleadingJet_genPartonidb;
//   Float_t         subleadingJet_genFlavourb;
//   Float_t         subleadingJet_genPartonFlavourb;
//   Float_t         subleadingJet_genHadronFlavourb;
//   Float_t         subleadingJet_genNbHadronsb;
//   Float_t         subleadingJet_genNcHadronsb;
   Float_t         HHTagger2017;
   Float_t         HHTagger2017_transform;
   LorentzVector   *dijetCandidate;

 //ROOT::Math::LorentzVector<ROOT::Math::PxPyPzE4D<double> > *dijetCandidate_KF;
   //Double_t        fCoordinates_fX;
   //Double_t        fCoordinates_fY;
   //Double_t        fCoordinates_fZ;
   //Double_t        fCoordinates_fT;
 //ROOT::Math::LorentzVector<ROOT::Math::PxPyPzE4D<double> > *dijetCandidate_Reg;
   //Double_t        fCoordinates_fX;
   //Double_t        fCoordinates_fY;
   //Double_t        fCoordinates_fZ;
   //Double_t        fCoordinates_fT;
 //ROOT::Math::LorentzVector<ROOT::Math::PxPyPzE4D<double> > *dijetCandidate_RegKF;
//   Double_t        fCoordinates_fX;
//   Double_t        fCoordinates_fY;
//   Double_t        fCoordinates_fZ;
//   Double_t        fCoordinates_fT;
// //ROOT::Math::LorentzVector<ROOT::Math::PxPyPzE4D<double> > *diHiggsCandidate;
//   Double_t        fCoordinates_fX;
//   Double_t        fCoordinates_fY;
//   Double_t        fCoordinates_fZ;
//   Double_t        fCoordinates_fT;
// //ROOT::Math::LorentzVector<ROOT::Math::PxPyPzE4D<double> > *diHiggsCandidate_KF;
//   Double_t        fCoordinates_fX;
//   Double_t        fCoordinates_fY;
//   Double_t        fCoordinates_fZ;
//   Double_t        fCoordinates_fT;
// //ROOT::Math::LorentzVector<ROOT::Math::PxPyPzE4D<double> > *diHiggsCandidate_Reg;
//   Double_t        fCoordinates_fX;
//   Double_t        fCoordinates_fY;
//   Double_t        fCoordinates_fZ;
//   Double_t        fCoordinates_fT;
// //ROOT::Math::LorentzVector<ROOT::Math::PxPyPzE4D<double> > *diHiggsCandidate_RegKF;
//   Double_t        fCoordinates_fX;
//   Double_t        fCoordinates_fY;
//   Double_t        fCoordinates_fZ;
//   Double_t        fCoordinates_fT;
   Int_t           isSignal;
   //Int_t           isPhotonCR;
//  Float_t         CosThetaStar;
   //  Float_t         CosThetaStar_CS;
//   Float_t         CosTheta_bb;
//   Float_t         CosTheta_gg;
//   Float_t         CosTheta_bbgg;
//   Float_t         CosTheta_ggbb;
//   Float_t         Phi0;
//   Float_t         Phi1;
//   map<string,int> *TriggerResults;
//   Float_t         DiJetDiPho_DR;
//   Float_t         PhoJetMinDr;
 //ROOT::Math::LorentzVector<ROOT::Math::PxPyPzE4D<double> > *leadingJet_VBF;
   //Double_t        fCoordinates_fX;
   //Double_t        fCoordinates_fY;
   //Double_t        fCoordinates_fZ;
   //Double_t        fCoordinates_fT;
 //ROOT::Math::LorentzVector<ROOT::Math::PxPyPzE4D<double> > *subleadingJet_VBF;
   //Double_t        fCoordinates_fX;
   //Double_t        fCoordinates_fY;
   //Double_t        fCoordinates_fZ;
   //Double_t        fCoordinates_fT;
   //Float_t         dEta_VBF;
   //Float_t         Mjj_VBF;
   Float_t         HHTagger;
   Float_t         HHTagger_LM;
   Float_t         HHTagger_HM;
   //Float_t         ResHHTagger;
   //Float_t         ResHHTagger_LM;
   //Float_t         ResHHTagger_HM;
   Float_t         MX;
 //ROOT::Math::LorentzVector<ROOT::Math::PxPyPzE4D<double> > *MET;
   //Double_t        fCoordinates_fX;
   //Double_t        fCoordinates_fY;
   //Double_t        fCoordinates_fZ;
   //Double_t        fCoordinates_fT;
   //Float_t         sumEt;
   //Int_t           njets;
   //Float_t         Xtt0;
   //Float_t         Xtt1;
   //Float_t         MjjW0;
   //Float_t         MjjW1;
   //Float_t         Mjjbt0;
   //Float_t         Mjjbt1;
 //ROOT::Math::LorentzVector<ROOT::Math::PxPyPzE4D<double> > *leadingMuon;
   //Double_t        fCoordinates_fX;
   //Double_t        fCoordinates_fY;
   //Double_t        fCoordinates_fZ;
   //Double_t        fCoordinates_fT;
 ////ROOT::Math::LorentzVector<ROOT::Math::PxPyPzE4D<double> > *subleadingMuon;
   //Double_t        fCoordinates_fX;
   //Double_t        fCoordinates_fY;
   //Double_t        fCoordinates_fZ;
   //Double_t        fCoordinates_fT;
   Int_t           nmus;
   Int_t           nmus_loose;
 //ROOT::Math::LorentzVector<ROOT::Math::PxPyPzE4D<double> > *leadingElectron;
   //Double_t        fCoordinates_fX;
   //Double_t        fCoordinates_fY;
   //Double_t        fCoordinates_fZ;
   //Double_t        fCoordinates_fT;
 ////ROOT::Math::LorentzVector<ROOT::Math::PxPyPzE4D<double> > *subleadingElectron;
   //Double_t        fCoordinates_fX;
   //Double_t        fCoordinates_fY;
   //Double_t        fCoordinates_fZ;
   //Double_t        fCoordinates_fT;
   Int_t           nelecs;
   Int_t           nelecs_loose;
   Float_t         rho;
   Int_t           nvtx;
   ULong64_t       event;
   UInt_t          lumi;
   Int_t           processIndex;
   UInt_t          run;
   //Int_t           nvtx;
   Float_t         npu;
   Float_t         puweight;
   Double_t        lumiFactor;

   // List of branches
   TBranch        *b_genWeights;   //!
   TBranch        *b_genTotalWeight;   //!
   TBranch        *b_gen_mHH;   //!
   TBranch        *b_cosTheta;   //!
   TBranch        *b_NRW;   //!
   
   TBranch        *b_leadingPhoton; //!
//   TBranch        *b_leadingPhotonID;   //!
//   TBranch        *b_leadingPhotonISO;   //!
//   TBranch        *b_leadingPhotonEVeto;   //!
//   TBranch        *b_leadingPhotonIDMVA;   //!
//   TBranch        *b_customLeadingPhotonIDMVA;   //!
   TBranch        *b_leadingPhotonR9full5x5;   //!
//   TBranch        *b_leadingPhotonHasGain1;   //!
//   TBranch        *b_leadingPhotonHasGain6;   //!
//   TBranch        *b_subleadingPhoton_fCoordinates_fX;   //!
   TBranch        *b_subleadingPhoton;
   //TBranch        *b_subleadingPhotonISO;   //!
   //TBranch        *b_subleadingPhotonEVeto;   //!
   //TBranch        *b_subleadingPhotonIDMVA;   //!
   //TBranch        *b_customSubLeadingPhotonIDMVA;   //!
   TBranch        *b_subleadingPhotonR9full5x5;   //!
   //TBranch        *b_subLeadingPhotonHasGain1;   //!
   //TBranch        *b_subLeadingPhotonHasGain6;   //!
   //TBranch        *b_leadingPhotonSigOverE;   //!
   //TBranch        *b_subleadingPhotonSigOverE;   //!
   //TBranch        *b_sigmaMOverM;   //!
   //TBranch        *b_sigmaMOverMDecorr;   //!
   //TBranch        *b_diphoMVA;   //!
   //TBranch        *b_diphotonCandidate_fCoordinates_fX;   //!
   //TBranch        *b_diphotonCandidate_fCoordinates_fY;   //!
   //TBranch        *b_diphotonCandidate_fCoordinates_fZ;   //!
   //TBranch        *b_diphotonCandidate_fCoordinates_fT;   //!
   TBranch        *b_nPromptInDiPhoton;   //!
//  TBranch        *b_leadingJet_fCoordinates_fX;   //!
//  TBranch        *b_leadingJet_fCoordinates_fY;   //!
//  TBranch        *b_leadingJet_fCoordinates_fZ;   //!
//  TBranch        *b_leadingJet_fCoordinates_fT;   //!
//  TBranch        *b_leadingJet_KF_fCoordinates_fX;   //!
//  TBranch        *b_leadingJet_KF_fCoordinates_fY;   //!
//  TBranch        *b_leadingJet_KF_fCoordinates_fZ;   //!
//  TBranch        *b_leadingJet_KF_fCoordinates_fT;   //!
//  TBranch        *b_leadingJet_Reg_fCoordinates_fX;   //!
//  TBranch        *b_leadingJet_Reg_fCoordinates_fY;   //!
//  TBranch        *b_leadingJet_Reg_fCoordinates_fZ;   //!
//  TBranch        *b_leadingJet_Reg_fCoordinates_fT;   //!
//  TBranch        *b_leadingJet_RegKF_fCoordinates_fX;   //!
//  TBranch        *b_leadingJet_RegKF_fCoordinates_fY;   //!
//  TBranch        *b_leadingJet_RegKF_fCoordinates_fZ;   //!
//  TBranch        *b_leadingJet_RegKF_fCoordinates_fT;   //!
   TBranch        *b_leadingJet_bDis;   //!
//  TBranch        *b_leadingJet_CSVv2;   //!
//  TBranch        *b_leadingJet_cMVA;   //!
//  TBranch        *b_leadingJet_flavour;   //!
//  TBranch        *b_leadingJet_hadFlavour;   //!
//  TBranch        *b_subleadingJet_fCoordinates_fX;   //!
//  TBranch        *b_subleadingJet_fCoordinates_fY;   //!
//  TBranch        *b_subleadingJet_fCoordinates_fZ;   //!
//  TBranch        *b_subleadingJet_fCoordinates_fT;   //!
//  TBranch        *b_subleadingJet_KF_fCoordinates_fX;   //!
//  TBranch        *b_subleadingJet_KF_fCoordinates_fY;   //!
//  TBranch        *b_subleadingJet_KF_fCoordinates_fZ;   //!
//  TBranch        *b_subleadingJet_KF_fCoordinates_fT;   //!
//  TBranch        *b_subleadingJet_Reg_fCoordinates_fX;   //!
//  TBranch        *b_subleadingJet_Reg_fCoordinates_fY;   //!
//  TBranch        *b_subleadingJet_Reg_fCoordinates_fZ;   //!
//  TBranch        *b_subleadingJet_Reg_fCoordinates_fT;   //!
//  TBranch        *b_subleadingJet_RegKF_fCoordinates_fX;   //!
//  TBranch        *b_subleadingJet_RegKF_fCoordinates_fY;   //!
//  TBranch        *b_subleadingJet_RegKF_fCoordinates_fZ;   //!
//  TBranch        *b_subleadingJet_RegKF_fCoordinates_fT;   //!
   TBranch        *b_subleadingJet_bDis;   //!
//  TBranch        *b_subleadingJet_CSVv2;   //!
//  TBranch        *b_subleadingJet_cMVA;   //!
//  TBranch        *b_subleadingJet_flavour;   //!
//  TBranch        *b_subleadingJet_hadFlavour;   //!
//  TBranch        *b_leadingJet_genPtb;   //!
//  TBranch        *b_leadingJet_genPartonidb;   //!
//  TBranch        *b_leadingJet_genFlavourb;   //!
//  TBranch        *b_leadingJet_genPartonFlavourb;   //!
//  TBranch        *b_leadingJet_genHadronFlavourb;   //!
//  TBranch        *b_leadingJet_genNbHadronsb;   //!
//  TBranch        *b_leadingJet_genNcHadronsb;   //!
//  TBranch        *b_subleadingJet_genPtb;   //!
//  TBranch        *b_subleadingJet_genPartonidb;   //!
//  TBranch        *b_subleadingJet_genFlavourb;   //!
//  TBranch        *b_subleadingJet_genPartonFlavourb;   //!
//  TBranch        *b_subleadingJet_genHadronFlavourb;   //!
//  TBranch        *b_subleadingJet_genNbHadronsb;   //!
//  TBranch        *b_subleadingJet_genNcHadronsb;   //!
   TBranch        *b_HHTagger2017;   //!
   TBranch        *b_HHTagger2017_transform;   //!
   TBranch        *b_dijetCandidate;
//  TBranch        *b_dijetCandidate_fCoordinates_fY;   //!
//  TBranch        *b_dijetCandidate_fCoordinates_fZ;   //!
//  TBranch        *b_dijetCandidate_fCoordinates_fT;   //!
//  TBranch        *b_dijetCandidate_KF_fCoordinates_fX;   //!
//  TBranch        *b_dijetCandidate_KF_fCoordinates_fY;   //!
//  TBranch        *b_dijetCandidate_KF_fCoordinates_fZ;   //!
//  TBranch        *b_dijetCandidate_KF_fCoordinates_fT;   //!
//  TBranch        *b_dijetCandidate_Reg_fCoordinates_fX;   //!
//  TBranch        *b_dijetCandidate_Reg_fCoordinates_fY;   //!
//  TBranch        *b_dijetCandidate_Reg_fCoordinates_fZ;   //!
//  TBranch        *b_dijetCandidate_Reg_fCoordinates_fT;   //!
//  TBranch        *b_dijetCandidate_RegKF_fCoordinates_fX;   //!
//  TBranch        *b_dijetCandidate_RegKF_fCoordinates_fY;   //!
//  TBranch        *b_dijetCandidate_RegKF_fCoordinates_fZ;   //!
//  TBranch        *b_dijetCandidate_RegKF_fCoordinates_fT;   //!
//  TBranch        *b_diHiggsCandidate_fCoordinates_fX;   //!
//  TBranch        *b_diHiggsCandidate_fCoordinates_fY;   //!
//  TBranch        *b_diHiggsCandidate_fCoordinates_fZ;   //!
//  TBranch        *b_diHiggsCandidate_fCoordinates_fT;   //!
//  TBranch        *b_diHiggsCandidate_KF_fCoordinates_fX;   //!
//  TBranch        *b_diHiggsCandidate_KF_fCoordinates_fY;   //!
//  TBranch        *b_diHiggsCandidate_KF_fCoordinates_fZ;   //!
//  TBranch        *b_diHiggsCandidate_KF_fCoordinates_fT;   //!
//  TBranch        *b_diHiggsCandidate_Reg_fCoordinates_fX;   //!
//  TBranch        *b_diHiggsCandidate_Reg_fCoordinates_fY;   //!
//  TBranch        *b_diHiggsCandidate_Reg_fCoordinates_fZ;   //!
//  TBranch        *b_diHiggsCandidate_Reg_fCoordinates_fT;   //!
//  TBranch        *b_diHiggsCandidate_RegKF_fCoordinates_fX;   //!
//  TBranch        *b_diHiggsCandidate_RegKF_fCoordinates_fY;   //!
//  TBranch        *b_diHiggsCandidate_RegKF_fCoordinates_fZ;   //!
//  TBranch        *b_diHiggsCandidate_RegKF_fCoordinates_fT;   //!
   TBranch        *b_isSignal;   //!
//   TBranch        *b_isPhotonCR;   //!
//  TBranch        *b_CosThetaStar;   //!
//   TBranch        *b_CosThetaStar_CS;   //!
//  TBranch        *b_CosTheta_bb;   //!
//  TBranch        *b_CosTheta_gg;   //!
//  TBranch        *b_CosTheta_bbgg;   //!
//  TBranch        *b_CosTheta_ggbb;   //!
//  TBranch        *b_Phi0;   //!
//  TBranch        *b_Phi1;   //!
//  TBranch        *b_TriggerResults;   //!
//  TBranch        *b_DiJetDiPho_DR;   //!
//  TBranch        *b_PhoJetMinDr;   //!
//  TBranch        *b_leadingJet_VBF_fCoordinates_fX;   //!
//  TBranch        *b_leadingJet_VBF_fCoordinates_fY;   //!
//  TBranch        *b_leadingJet_VBF_fCoordinates_fZ;   //!
//  TBranch        *b_leadingJet_VBF_fCoordinates_fT;   //!
//  TBranch        *b_subleadingJet_VBF_fCoordinates_fX;   //!
//  TBranch        *b_subleadingJet_VBF_fCoordinates_fY;   //!
//  TBranch        *b_subleadingJet_VBF_fCoordinates_fZ;   //!
//  TBranch        *b_subleadingJet_VBF_fCoordinates_fT;   //!
//  TBranch        *b_dEta_VBF;   //!
//  TBranch        *b_Mjj_VBF;   //!
   TBranch        *b_HHTagger;   //!
   TBranch        *b_HHTagger_LM;   //!
   TBranch        *b_HHTagger_HM;   //!
//  TBranch        *b_ResHHTagger;   //!
//  TBranch        *b_ResHHTagger_LM;   //!
//  TBranch        *b_ResHHTagger_HM;   //!
   TBranch        *b_MX;   //!
//  TBranch        *b_MET_fCoordinates_fX;   //!
//  TBranch        *b_MET_fCoordinates_fY;   //!
//  TBranch        *b_MET_fCoordinates_fZ;   //!
//  TBranch        *b_MET_fCoordinates_fT;   //!
//  TBranch        *b_sumEt;   //!
//  TBranch        *b_njets;   //!
//  TBranch        *b_Xtt0;   //!
//  TBranch        *b_Xtt1;   //!
//  TBranch        *b_MjjW0;   //!
//  TBranch        *b_MjjW1;   //!
//  TBranch        *b_Mjjbt0;   //!
//  TBranch        *b_Mjjbt1;   //!
//  TBranch        *b_leadingMuon_fCoordinates_fX;   //!
//  TBranch        *b_leadingMuon_fCoordinates_fY;   //!
//  TBranch        *b_leadingMuon_fCoordinates_fZ;   //!
//  TBranch        *b_leadingMuon_fCoordinates_fT;   //!
//  TBranch        *b_subleadingMuon_fCoordinates_fX;   //!
//  TBranch        *b_subleadingMuon_fCoordinates_fY;   //!
//  TBranch        *b_subleadingMuon_fCoordinates_fZ;   //!
//  TBranch        *b_subleadingMuon_fCoordinates_fT;   //!
//  TBranch        *b_nmus;   //!
//  TBranch        *b_nmus_loose;   //!
//  TBranch        *b_leadingElectron_fCoordinates_fX;   //!
//  TBranch        *b_leadingElectron_fCoordinates_fY;   //!
//  TBranch        *b_leadingElectron_fCoordinates_fZ;   //!
//  TBranch        *b_leadingElectron_fCoordinates_fT;   //!
//  TBranch        *b_subleadingElectron_fCoordinates_fX;   //!
//  TBranch        *b_subleadingElectron_fCoordinates_fY;   //!
//  TBranch        *b_subleadingElectron_fCoordinates_fZ;   //!
//  TBranch        *b_subleadingElectron_fCoordinates_fT;   //!
   TBranch        *b_nelecs;   //!
   TBranch        *b_nelecs_loose;   //!
   TBranch        *b_rho;   //!
   TBranch        *b_nvtx;   //!
   TBranch        *b_event;   //!
   TBranch        *b_lumi;   //!
   TBranch        *b_processIndex;   //!
   TBranch        *b_run;   //!
   //TBranch        *b_nvtx;   //!
   TBranch        *b_npu;   //!
   TBranch        *b_puweight;   //!
   TBranch        *b_lumiFactor;   //!

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
   //leadingPhotonID = 0;
   //leadingPhotonISO = 0;
   subleadingPhoton = 0;
   //subleadingPhotonID = 0;
   //subleadingPhotonISO = 0;
   dijetCandidate = 0;
   //TriggerResults = 0;
   // Set branch addresses and branch pointers
   if (!tree) return;
   fChain = tree;
   //fChain->SetMakeClass(1); //This thing breaks LorentzVectors
   
//   fChain->SetBranchAddress("genWeights", &genWeights, &b_genWeights);
   fChain->SetBranchAddress("genTotalWeight", &genTotalWeight, &b_genTotalWeight);
//   fChain->SetBranchAddress("gen_mHH", &gen_mHH, &b_gen_mHH);
//   fChain->SetBranchAddress("gen_cosTheta", &gen_cosTheta, &b_cosTheta);
//   fChain->SetBranchAddress("gen_NRW", &gen_NRW, &b_NRW);
   fChain->SetBranchAddress("leadingPhoton", &leadingPhoton, &b_leadingPhoton);
   fChain->SetBranchAddress("subleadingPhoton", &subleadingPhoton, &b_subleadingPhoton);

//   fChain->SetBranchAddress("leadingPhotonID", &leadingPhotonID, &b_leadingPhotonID);
//   fChain->SetBranchAddress("leadingPhotonISO", &leadingPhotonISO, &b_leadingPhotonISO);
//   fChain->SetBranchAddress("leadingPhotonEVeto", &leadingPhotonEVeto, &b_leadingPhotonEVeto);
//   fChain->SetBranchAddress("leadingPhotonIDMVA", &leadingPhotonIDMVA, &b_leadingPhotonIDMVA);
//   fChain->SetBranchAddress("customLeadingPhotonIDMVA", &customLeadingPhotonIDMVA, &b_customLeadingPhotonIDMVA);
   fChain->SetBranchAddress("leadingPhotonR9full5x5", &leadingPhotonR9full5x5, &b_leadingPhotonR9full5x5);
//   fChain->SetBranchAddress("leadingPhotonHasGain1", &leadingPhotonHasGain1, &b_leadingPhotonHasGain1);
//   fChain->SetBranchAddress("leadingPhotonHasGain6", &leadingPhotonHasGain6, &b_leadingPhotonHasGain6);
////    fChain->SetBranchAddress("fCoordinates.fX", &fCoordinates_fX, &b_subleadingPhoton_fCoordinates_fX);
////    fChain->SetBranchAddress("fCoordinates.fY", &fCoordinates_fY, &b_subleadingPhoton_fCoordinates_fY);
////    fChain->SetBranchAddress("fCoordinates.fZ", &fCoordinates_fZ, &b_subleadingPhoton_fCoordinates_fZ);
////    fChain->SetBranchAddress("fCoordinates.fT", &fCoordinates_fT, &b_subleadingPhoton_fCoordinates_fT);
//   fChain->SetBranchAddress("subleadingPhotonISO", &subleadingPhotonISO, &b_subleadingPhotonISO);
//   fChain->SetBranchAddress("subleadingPhotonEVeto", &subleadingPhotonEVeto, &b_subleadingPhotonEVeto);
//   fChain->SetBranchAddress("subleadingPhotonIDMVA", &subleadingPhotonIDMVA, &b_subleadingPhotonIDMVA);
//   fChain->SetBranchAddress("customSubLeadingPhotonIDMVA", &customSubLeadingPhotonIDMVA, &b_customSubLeadingPhotonIDMVA);
   fChain->SetBranchAddress("subleadingPhotonR9full5x5", &subleadingPhotonR9full5x5, &b_subleadingPhotonR9full5x5);
//   fChain->SetBranchAddress("subLeadingPhotonHasGain1", &subLeadingPhotonHasGain1, &b_subLeadingPhotonHasGain1);
//   fChain->SetBranchAddress("subLeadingPhotonHasGain6", &subLeadingPhotonHasGain6, &b_subLeadingPhotonHasGain6);
//   fChain->SetBranchAddress("leadingPhotonSigOverE", &leadingPhotonSigOverE, &b_leadingPhotonSigOverE);
//   fChain->SetBranchAddress("subleadingPhotonSigOverE", &subleadingPhotonSigOverE, &b_subleadingPhotonSigOverE);
//   fChain->SetBranchAddress("sigmaMOverM", &sigmaMOverM, &b_sigmaMOverM);
//   fChain->SetBranchAddress("sigmaMOverMDecorr", &sigmaMOverMDecorr, &b_sigmaMOverMDecorr);
//   fChain->SetBranchAddress("diphoMVA", &diphoMVA, &b_diphoMVA);
////    fChain->SetBranchAddress("fCoordinates.fX", &fCoordinates_fX, &b_diphotonCandidate_fCoordinates_fX);
////    fChain->SetBranchAddress("fCoordinates.fY", &fCoordinates_fY, &b_diphotonCandidate_fCoordinates_fY);
////    fChain->SetBranchAddress("fCoordinates.fZ", &fCoordinates_fZ, &b_diphotonCandidate_fCoordinates_fZ);
////    fChain->SetBranchAddress("fCoordinates.fT", &fCoordinates_fT, &b_diphotonCandidate_fCoordinates_fT);
   fChain->SetBranchAddress("nPromptInDiPhoton", &nPromptInDiPhoton, &b_nPromptInDiPhoton);
////    fChain->SetBranchAddress("fCoordinates.fX", &fCoordinates_fX, &b_leadingJet_fCoordinates_fX);
////    fChain->SetBranchAddress("fCoordinates.fY", &fCoordinates_fY, &b_leadingJet_fCoordinates_fY);
////    fChain->SetBranchAddress("fCoordinates.fZ", &fCoordinates_fZ, &b_leadingJet_fCoordinates_fZ);
////    fChain->SetBranchAddress("fCoordinates.fT", &fCoordinates_fT, &b_leadingJet_fCoordinates_fT);
////    fChain->SetBranchAddress("fCoordinates.fX", &fCoordinates_fX, &b_leadingJet_KF_fCoordinates_fX);
////    fChain->SetBranchAddress("fCoordinates.fY", &fCoordinates_fY, &b_leadingJet_KF_fCoordinates_fY);
////    fChain->SetBranchAddress("fCoordinates.fZ", &fCoordinates_fZ, &b_leadingJet_KF_fCoordinates_fZ);
////    fChain->SetBranchAddress("fCoordinates.fT", &fCoordinates_fT, &b_leadingJet_KF_fCoordinates_fT);
////    fChain->SetBranchAddress("fCoordinates.fX", &fCoordinates_fX, &b_leadingJet_Reg_fCoordinates_fX);
////    fChain->SetBranchAddress("fCoordinates.fY", &fCoordinates_fY, &b_leadingJet_Reg_fCoordinates_fY);
////    fChain->SetBranchAddress("fCoordinates.fZ", &fCoordinates_fZ, &b_leadingJet_Reg_fCoordinates_fZ);
////    fChain->SetBranchAddress("fCoordinates.fT", &fCoordinates_fT, &b_leadingJet_Reg_fCoordinates_fT);
////    fChain->SetBranchAddress("fCoordinates.fX", &fCoordinates_fX, &b_leadingJet_RegKF_fCoordinates_fX);
////    fChain->SetBranchAddress("fCoordinates.fY", &fCoordinates_fY, &b_leadingJet_RegKF_fCoordinates_fY);
////    fChain->SetBranchAddress("fCoordinates.fZ", &fCoordinates_fZ, &b_leadingJet_RegKF_fCoordinates_fZ);
////    fChain->SetBranchAddress("fCoordinates.fT", &fCoordinates_fT, &b_leadingJet_RegKF_fCoordinates_fT);
   fChain->SetBranchAddress("leadingJet_bDis", &leadingJet_bDis, &b_leadingJet_bDis);
//   fChain->SetBranchAddress("leadingJet_CSVv2", &leadingJet_CSVv2, &b_leadingJet_CSVv2);
//   fChain->SetBranchAddress("leadingJet_cMVA", &leadingJet_cMVA, &b_leadingJet_cMVA);
//   fChain->SetBranchAddress("leadingJet_flavour", &leadingJet_flavour, &b_leadingJet_flavour);
//   fChain->SetBranchAddress("leadingJet_hadFlavour", &leadingJet_hadFlavour, &b_leadingJet_hadFlavour);
////    fChain->SetBranchAddress("fCoordinates.fX", &fCoordinates_fX, &b_subleadingJet_fCoordinates_fX);
////    fChain->SetBranchAddress("fCoordinates.fY", &fCoordinates_fY, &b_subleadingJet_fCoordinates_fY);
////    fChain->SetBranchAddress("fCoordinates.fZ", &fCoordinates_fZ, &b_subleadingJet_fCoordinates_fZ);
////    fChain->SetBranchAddress("fCoordinates.fT", &fCoordinates_fT, &b_subleadingJet_fCoordinates_fT);
////    fChain->SetBranchAddress("fCoordinates.fX", &fCoordinates_fX, &b_subleadingJet_KF_fCoordinates_fX);
////    fChain->SetBranchAddress("fCoordinates.fY", &fCoordinates_fY, &b_subleadingJet_KF_fCoordinates_fY);
////    fChain->SetBranchAddress("fCoordinates.fZ", &fCoordinates_fZ, &b_subleadingJet_KF_fCoordinates_fZ);
////    fChain->SetBranchAddress("fCoordinates.fT", &fCoordinates_fT, &b_subleadingJet_KF_fCoordinates_fT);
////    fChain->SetBranchAddress("fCoordinates.fX", &fCoordinates_fX, &b_subleadingJet_Reg_fCoordinates_fX);
////    fChain->SetBranchAddress("fCoordinates.fY", &fCoordinates_fY, &b_subleadingJet_Reg_fCoordinates_fY);
////    fChain->SetBranchAddress("fCoordinates.fZ", &fCoordinates_fZ, &b_subleadingJet_Reg_fCoordinates_fZ);
////    fChain->SetBranchAddress("fCoordinates.fT", &fCoordinates_fT, &b_subleadingJet_Reg_fCoordinates_fT);
////    fChain->SetBranchAddress("fCoordinates.fX", &fCoordinates_fX, &b_subleadingJet_RegKF_fCoordinates_fX);
////    fChain->SetBranchAddress("fCoordinates.fY", &fCoordinates_fY, &b_subleadingJet_RegKF_fCoordinates_fY);
////    fChain->SetBranchAddress("fCoordinates.fZ", &fCoordinates_fZ, &b_subleadingJet_RegKF_fCoordinates_fZ);
////    fChain->SetBranchAddress("fCoordinates.fT", &fCoordinates_fT, &b_subleadingJet_RegKF_fCoordinates_fT);
   fChain->SetBranchAddress("subleadingJet_bDis", &subleadingJet_bDis, &b_subleadingJet_bDis);
//   fChain->SetBranchAddress("subleadingJet_CSVv2", &subleadingJet_CSVv2, &b_subleadingJet_CSVv2);
//   fChain->SetBranchAddress("subleadingJet_cMVA", &subleadingJet_cMVA, &b_subleadingJet_cMVA);
//   fChain->SetBranchAddress("subleadingJet_flavour", &subleadingJet_flavour, &b_subleadingJet_flavour);
//   fChain->SetBranchAddress("subleadingJet_hadFlavour", &subleadingJet_hadFlavour, &b_subleadingJet_hadFlavour);
//   fChain->SetBranchAddress("leadingJet_genPtb", &leadingJet_genPtb, &b_leadingJet_genPtb);
//   fChain->SetBranchAddress("leadingJet_genPartonidb", &leadingJet_genPartonidb, &b_leadingJet_genPartonidb);
//   fChain->SetBranchAddress("leadingJet_genFlavourb", &leadingJet_genFlavourb, &b_leadingJet_genFlavourb);
//   fChain->SetBranchAddress("leadingJet_genPartonFlavourb", &leadingJet_genPartonFlavourb, &b_leadingJet_genPartonFlavourb);
//   fChain->SetBranchAddress("leadingJet_genHadronFlavourb", &leadingJet_genHadronFlavourb, &b_leadingJet_genHadronFlavourb);
//   fChain->SetBranchAddress("leadingJet_genNbHadronsb", &leadingJet_genNbHadronsb, &b_leadingJet_genNbHadronsb);
//   fChain->SetBranchAddress("leadingJet_genNcHadronsb", &leadingJet_genNcHadronsb, &b_leadingJet_genNcHadronsb);
//   fChain->SetBranchAddress("subleadingJet_genPtb", &subleadingJet_genPtb, &b_subleadingJet_genPtb);
//   fChain->SetBranchAddress("subleadingJet_genPartonidb", &subleadingJet_genPartonidb, &b_subleadingJet_genPartonidb);
//   fChain->SetBranchAddress("subleadingJet_genFlavourb", &subleadingJet_genFlavourb, &b_subleadingJet_genFlavourb);
//   fChain->SetBranchAddress("subleadingJet_genPartonFlavourb", &subleadingJet_genPartonFlavourb, &b_subleadingJet_genPartonFlavourb);
//   fChain->SetBranchAddress("subleadingJet_genHadronFlavourb", &subleadingJet_genHadronFlavourb, &b_subleadingJet_genHadronFlavourb);
//   fChain->SetBranchAddress("subleadingJet_genNbHadronsb", &subleadingJet_genNbHadronsb, &b_subleadingJet_genNbHadronsb);
//   fChain->SetBranchAddress("subleadingJet_genNcHadronsb", &subleadingJet_genNcHadronsb, &b_subleadingJet_genNcHadronsb);
   fChain->SetBranchAddress("HHTagger2017", &HHTagger2017, &b_HHTagger2017);
   fChain->SetBranchAddress("HHTagger2017_transform", &HHTagger2017_transform, &b_HHTagger2017_transform);
   fChain->SetBranchAddress("dijetCandidate", &dijetCandidate, &b_dijetCandidate);
////    fChain->SetBranchAddress("fCoordinates.fY", &fCoordinates_fY, &b_dijetCandidate_fCoordinates_fY);
////    fChain->SetBranchAddress("fCoordinates.fZ", &fCoordinates_fZ, &b_dijetCandidate_fCoordinates_fZ);
////    fChain->SetBranchAddress("fCoordinates.fT", &fCoordinates_fT, &b_dijetCandidate_fCoordinates_fT);
////    fChain->SetBranchAddress("fCoordinates.fX", &fCoordinates_fX, &b_dijetCandidate_KF_fCoordinates_fX);
////    fChain->SetBranchAddress("fCoordinates.fY", &fCoordinates_fY, &b_dijetCandidate_KF_fCoordinates_fY);
////    fChain->SetBranchAddress("fCoordinates.fZ", &fCoordinates_fZ, &b_dijetCandidate_KF_fCoordinates_fZ);
////    fChain->SetBranchAddress("fCoordinates.fT", &fCoordinates_fT, &b_dijetCandidate_KF_fCoordinates_fT);
////    fChain->SetBranchAddress("fCoordinates.fX", &fCoordinates_fX, &b_dijetCandidate_Reg_fCoordinates_fX);
////    fChain->SetBranchAddress("fCoordinates.fY", &fCoordinates_fY, &b_dijetCandidate_Reg_fCoordinates_fY);
////    fChain->SetBranchAddress("fCoordinates.fZ", &fCoordinates_fZ, &b_dijetCandidate_Reg_fCoordinates_fZ);
////    fChain->SetBranchAddress("fCoordinates.fT", &fCoordinates_fT, &b_dijetCandidate_Reg_fCoordinates_fT);
////    fChain->SetBranchAddress("fCoordinates.fX", &fCoordinates_fX, &b_dijetCandidate_RegKF_fCoordinates_fX);
////    fChain->SetBranchAddress("fCoordinates.fY", &fCoordinates_fY, &b_dijetCandidate_RegKF_fCoordinates_fY);
////    fChain->SetBranchAddress("fCoordinates.fZ", &fCoordinates_fZ, &b_dijetCandidate_RegKF_fCoordinates_fZ);
////    fChain->SetBranchAddress("fCoordinates.fT", &fCoordinates_fT, &b_dijetCandidate_RegKF_fCoordinates_fT);
////    fChain->SetBranchAddress("fCoordinates.fX", &fCoordinates_fX, &b_diHiggsCandidate_fCoordinates_fX);
////    fChain->SetBranchAddress("fCoordinates.fY", &fCoordinates_fY, &b_diHiggsCandidate_fCoordinates_fY);
////    fChain->SetBranchAddress("fCoordinates.fZ", &fCoordinates_fZ, &b_diHiggsCandidate_fCoordinates_fZ);
////    fChain->SetBranchAddress("fCoordinates.fT", &fCoordinates_fT, &b_diHiggsCandidate_fCoordinates_fT);
////    fChain->SetBranchAddress("fCoordinates.fX", &fCoordinates_fX, &b_diHiggsCandidate_KF_fCoordinates_fX);
////    fChain->SetBranchAddress("fCoordinates.fY", &fCoordinates_fY, &b_diHiggsCandidate_KF_fCoordinates_fY);
////    fChain->SetBranchAddress("fCoordinates.fZ", &fCoordinates_fZ, &b_diHiggsCandidate_KF_fCoordinates_fZ);
////    fChain->SetBranchAddress("fCoordinates.fT", &fCoordinates_fT, &b_diHiggsCandidate_KF_fCoordinates_fT);
////    fChain->SetBranchAddress("fCoordinates.fX", &fCoordinates_fX, &b_diHiggsCandidate_Reg_fCoordinates_fX);
////    fChain->SetBranchAddress("fCoordinates.fY", &fCoordinates_fY, &b_diHiggsCandidate_Reg_fCoordinates_fY);
////    fChain->SetBranchAddress("fCoordinates.fZ", &fCoordinates_fZ, &b_diHiggsCandidate_Reg_fCoordinates_fZ);
////    fChain->SetBranchAddress("fCoordinates.fT", &fCoordinates_fT, &b_diHiggsCandidate_Reg_fCoordinates_fT);
////    fChain->SetBranchAddress("fCoordinates.fX", &fCoordinates_fX, &b_diHiggsCandidate_RegKF_fCoordinates_fX);
////    fChain->SetBranchAddress("fCoordinates.fY", &fCoordinates_fY, &b_diHiggsCandidate_RegKF_fCoordinates_fY);
////    fChain->SetBranchAddress("fCoordinates.fZ", &fCoordinates_fZ, &b_diHiggsCandidate_RegKF_fCoordinates_fZ);
////    fChain->SetBranchAddress("fCoordinates.fT", &fCoordinates_fT, &b_diHiggsCandidate_RegKF_fCoordinates_fT);
   fChain->SetBranchAddress("isSignal", &isSignal, &b_isSignal);
//   fChain->SetBranchAddress("isPhotonCR", &isPhotonCR, &b_isPhotonCR);
   //   fChain->SetBranchAddress("CosThetaStar", &CosThetaStar, &b_CosThetaStar);
   //   fChain->SetBranchAddress("CosThetaStar_CS", &CosThetaStar_CS, &b_CosThetaStar_CS);
//   fChain->SetBranchAddress("CosTheta_bb", &CosTheta_bb, &b_CosTheta_bb);
//   fChain->SetBranchAddress("CosTheta_gg", &CosTheta_gg, &b_CosTheta_gg);
//   fChain->SetBranchAddress("CosTheta_bbgg", &CosTheta_bbgg, &b_CosTheta_bbgg);
//   fChain->SetBranchAddress("CosTheta_ggbb", &CosTheta_ggbb, &b_CosTheta_ggbb);
//   fChain->SetBranchAddress("Phi0", &Phi0, &b_Phi0);
//   fChain->SetBranchAddress("Phi1", &Phi1, &b_Phi1);
//   fChain->SetBranchAddress("TriggerResults", &TriggerResults, &b_TriggerResults);
//   fChain->SetBranchAddress("DiJetDiPho_DR", &DiJetDiPho_DR, &b_DiJetDiPho_DR);
//   fChain->SetBranchAddress("PhoJetMinDr", &PhoJetMinDr, &b_PhoJetMinDr);
////    fChain->SetBranchAddress("fCoordinates.fX", &fCoordinates_fX, &b_leadingJet_VBF_fCoordinates_fX);
////    fChain->SetBranchAddress("fCoordinates.fY", &fCoordinates_fY, &b_leadingJet_VBF_fCoordinates_fY);
////    fChain->SetBranchAddress("fCoordinates.fZ", &fCoordinates_fZ, &b_leadingJet_VBF_fCoordinates_fZ);
////    fChain->SetBranchAddress("fCoordinates.fT", &fCoordinates_fT, &b_leadingJet_VBF_fCoordinates_fT);
////    fChain->SetBranchAddress("fCoordinates.fX", &fCoordinates_fX, &b_subleadingJet_VBF_fCoordinates_fX);
////    fChain->SetBranchAddress("fCoordinates.fY", &fCoordinates_fY, &b_subleadingJet_VBF_fCoordinates_fY);
////    fChain->SetBranchAddress("fCoordinates.fZ", &fCoordinates_fZ, &b_subleadingJet_VBF_fCoordinates_fZ);
////    fChain->SetBranchAddress("fCoordinates.fT", &fCoordinates_fT, &b_subleadingJet_VBF_fCoordinates_fT);
//   fChain->SetBranchAddress("dEta_VBF", &dEta_VBF, &b_dEta_VBF);
//   fChain->SetBranchAddress("Mjj_VBF", &Mjj_VBF, &b_Mjj_VBF);
   fChain->SetBranchAddress("HHTagger", &HHTagger, &b_HHTagger);
   fChain->SetBranchAddress("HHTagger_LM", &HHTagger_LM, &b_HHTagger_LM);
   fChain->SetBranchAddress("HHTagger_HM", &HHTagger_HM, &b_HHTagger_HM);
//   fChain->SetBranchAddress("ResHHTagger", &ResHHTagger, &b_ResHHTagger);
//   fChain->SetBranchAddress("ResHHTagger_LM", &ResHHTagger_LM, &b_ResHHTagger_LM);
//   fChain->SetBranchAddress("ResHHTagger_HM", &ResHHTagger_HM, &b_ResHHTagger_HM);
   fChain->SetBranchAddress("MX", &MX, &b_MX);
////    fChain->SetBranchAddress("fCoordinates.fX", &fCoordinates_fX, &b_MET_fCoordinates_fX);
////    fChain->SetBranchAddress("fCoordinates.fY", &fCoordinates_fY, &b_MET_fCoordinates_fY);
////    fChain->SetBranchAddress("fCoordinates.fZ", &fCoordinates_fZ, &b_MET_fCoordinates_fZ);
////    fChain->SetBranchAddress("fCoordinates.fT", &fCoordinates_fT, &b_MET_fCoordinates_fT);
//   fChain->SetBranchAddress("sumEt", &sumEt, &b_sumEt);
//   fChain->SetBranchAddress("njets", &njets, &b_njets);
//   fChain->SetBranchAddress("Xtt0", &Xtt0, &b_Xtt0);
//   fChain->SetBranchAddress("Xtt1", &Xtt1, &b_Xtt1);
//   fChain->SetBranchAddress("MjjW0", &MjjW0, &b_MjjW0);
//   fChain->SetBranchAddress("MjjW1", &MjjW1, &b_MjjW1);
//   fChain->SetBranchAddress("Mjjbt0", &Mjjbt0, &b_Mjjbt0);
//   fChain->SetBranchAddress("Mjjbt1", &Mjjbt1, &b_Mjjbt1);
////    fChain->SetBranchAddress("fCoordinates.fX", &fCoordinates_fX, &b_leadingMuon_fCoordinates_fX);
////    fChain->SetBranchAddress("fCoordinates.fY", &fCoordinates_fY, &b_leadingMuon_fCoordinates_fY);
////    fChain->SetBranchAddress("fCoordinates.fZ", &fCoordinates_fZ, &b_leadingMuon_fCoordinates_fZ);
////    fChain->SetBranchAddress("fCoordinates.fT", &fCoordinates_fT, &b_leadingMuon_fCoordinates_fT);
////    fChain->SetBranchAddress("fCoordinates.fX", &fCoordinates_fX, &b_subleadingMuon_fCoordinates_fX);
////    fChain->SetBranchAddress("fCoordinates.fY", &fCoordinates_fY, &b_subleadingMuon_fCoordinates_fY);
////    fChain->SetBranchAddress("fCoordinates.fZ", &fCoordinates_fZ, &b_subleadingMuon_fCoordinates_fZ);
////    fChain->SetBranchAddress("fCoordinates.fT", &fCoordinates_fT, &b_subleadingMuon_fCoordinates_fT);
//   fChain->SetBranchAddress("nmus", &nmus, &b_nmus);
//   fChain->SetBranchAddress("nmus_loose", &nmus_loose, &b_nmus_loose);
////    fChain->SetBranchAddress("fCoordinates.fX", &fCoordinates_fX, &b_leadingElectron_fCoordinates_fX);
////    fChain->SetBranchAddress("fCoordinates.fY", &fCoordinates_fY, &b_leadingElectron_fCoordinates_fY);
////    fChain->SetBranchAddress("fCoordinates.fZ", &fCoordinates_fZ, &b_leadingElectron_fCoordinates_fZ);
////    fChain->SetBranchAddress("fCoordinates.fT", &fCoordinates_fT, &b_leadingElectron_fCoordinates_fT);
////    fChain->SetBranchAddress("fCoordinates.fX", &fCoordinates_fX, &b_subleadingElectron_fCoordinates_fX);
////    fChain->SetBranchAddress("fCoordinates.fY", &fCoordinates_fY, &b_subleadingElectron_fCoordinates_fY);
////    fChain->SetBranchAddress("fCoordinates.fZ", &fCoordinates_fZ, &b_subleadingElectron_fCoordinates_fZ);
////    fChain->SetBranchAddress("fCoordinates.fT", &fCoordinates_fT, &b_subleadingElectron_fCoordinates_fT);
//   fChain->SetBranchAddress("nelecs", &nelecs, &b_nelecs);
//   fChain->SetBranchAddress("nelecs_loose", &nelecs_loose, &b_nelecs_loose);
//   fChain->SetBranchAddress("rho", &rho, &b_rho);
//   fChain->SetBranchAddress("nvtx", &nvtx, &b_nvtx);
   fChain->SetBranchAddress("event", &event, &b_event);
//   fChain->SetBranchAddress("lumi", &lumi, &b_lumi);
//   fChain->SetBranchAddress("processIndex", &processIndex, &b_processIndex);
   fChain->SetBranchAddress("run", &run, &b_run);
////    fChain->SetBranchAddress("nvtx", &nvtx, &b_nvtx);
//   fChain->SetBranchAddress("npu", &npu, &b_npu);
//   fChain->SetBranchAddress("puweight", &puweight, &b_puweight);
//   fChain->SetBranchAddress("lumiFactor", &lumiFactor, &b_lumiFactor);
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
