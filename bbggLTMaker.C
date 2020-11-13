#define bbggLTMaker_cxx

#include "bbggLTMaker.h"
#include <iostream>
#include <string>
#include <fstream>

const bool DEBUG = 0;

// Cuts for 2016 categorization (_which==0):
const Float_t cutJetBtag = 0.8;
const Float_t boundary_MX[3]  = {0,350,35000};
const Float_t boundary_MVA_tagger2016_LM[3] = {0.600, 0.985, 1.0};
const Float_t boundary_MVA_tagger2016_HM[3] = {0.600, 0.970, 1.0};

// Cuts for 2017 tagger, using 2016 categorization (_which==1):
const Float_t boundary_MVA_tagger2017_LM[3] = {0.115, 0.597, 1.0};
const Float_t boundary_MVA_tagger2017_HM[3] = {0.226, 0.6195, 1.0};

// Cuts for 2017 tagger, using 2017 optimized categorization (_which==2):
//const Float_t boundary_MVA_tagger2017[4] = {0.271, 0.543, 0.740, 1.05};
//const Float_t boundary_MX_2017[5]  = {250, 341.4, 426.1, 544, 35000};

//// upating the categrisation number according to this part of 2017 analysis code https://github.com/michelif/flashgg/blob/hh_tag_94X_20180601/Taggers/python/flashggDoubleHTag_cfi.py#L31-L34
//eth categorisation - std!
//const Float_t boundary_MVA_2017[4]   = {0.29, 0.441, 0.724, 1.0}; // category boundaries for MVA ###boundaries before 29/10/2019
//const Float_t boundary_MX_2017[5]   = {250., 354., 478., 560., 35000};// .. and MX
//eth categorisation - new boundaries for extra variables - prev flashgg
//const Float_t boundary_MVA_2017[4]   = {0.283, 0.504, 0.775, 1.0}; 
//const Float_t boundary_MX_2017[5]   = {250., 370., 481., 521., 35000};
//eth categorisation - new boundaries for extra variables - dev_legacy - 2016-17
//const Float_t boundary_MVA_2017[4]   = {0.267, 0.465, 0.718, 1.0}; 
//const Float_t boundary_MX_2017[5]   = {250., 357., 478., 574., 35000};
//eth categorisation - new boundaries for extra variables - dev_legacy - 2016-17-18
//const Float_t boundary_MVA_2017[4]   = {0.283, 0.524, 0.717, 1.0}; 
//const Float_t boundary_MX_2017[5]   = {250., 360., 417., 663., 35000};
//eth categorisation for Mjj - dev_legacy - 2016-17
//const Float_t boundary_MVA_2017[4]   = {0.275, 0.483, 0.736, 1.0};
//const Float_t boundary_MX_2017[5]   = {250., 386., 454., 556., 35000};
///eth categorisation for Mjj - dev_legacy - 2016-17-18
//const Float_t boundary_MVA_2017[4]   = {0.290, 0.496, 0.741, 1.0};
//const Float_t boundary_MX_2017[5]   = {250., 367., 450., 504., 35000};
//eth categorisation for Mjj 
//const Float_t boundary_MVA_2017[4]   = {0.23, 0.455, 0.709, 1.0}; // category boundaries for MVA
//const Float_t boundary_MX_2017[5]   = {250., 336., 411., 556., 35000};// .. and MX
//New categorisation - new boundaries for extra variables - legacy - Tatyina
//const Float_t boundary_MVA_2017[4]   = {0.307, 0.551, 0.766, 1.0}; 
//const Float_t boundary_MX_2017[5]   = {250., 386., 422., 563., 35000};
//Flashgg, eth categorisation - dev_legacy - 2016-17-18 - with Mjj
//const Float_t boundary_MVA_2017[4]   = {0.230, 0.455, 0.709, 1.0};
//const Float_t boundary_MX_2017[5]   = {250., 336., 411., 556., 35000};
//const Float_t boundary_MVA_2019[4]   = {0.320, 0.540, 0.700, 1.0};
//const Float_t boundary_MX_2019[15]   = {250., 370., 480., 585., 35000., 250., 335., 380., 545., 35000., 250., 330., 360., 530., 35000};
//const Float_t boundary_MVA_2019[4]   = {0.330, 0.560, 0.700, 1.0};
//const Float_t boundary_MX_2019[15]   = {250., 375., 470., 600., 35000., 250., 325., 365., 585., 35000., 250., 330., 360., 520., 35000};
//from presentation ETH 11/11/2019 
//const Float_t boundary_MVA_2019[4]   = {0.300, 0.540, 0.700, 1.0};
//const Float_t boundary_MX_2019[15]   = {250., 330., 375., 555., 35000., 250., 315., 365., 580., 35000., 250., 360., 470., 575., 35000};
//from AN - for std+Mjj
//const Float_t boundary_MVA_2019[4]   = {0.320, 0.540, 0.720, 1.0};
//const Float_t boundary_MX_2019[15]   = {250., 330., 360., 530., 35000., 250., 335., 380., 545., 35000., 250., 370., 480., 585., 35000};
//Flashgg, categorisation Tatyana - dev_legacy - 2016-17-18 - reweight - for 2D!!! 
//const Float_t boundary_MVA_2019[4]   = {0.249, 0.450, 0.729, 1.0};
//const Float_t boundary_MX_2019[15]   = {250., 383., 461., 514., 35000., 250., 343., 437., 563., 35000., 250., 342., 470., 553., 35000};
//from presentation ETH 03/02/2020
//for 1D
//const Float_t boundary_MVA_2019[4]   = {0.440, 0.670, 0.790, 1.0};
//const Float_t boundary_MX_2019[15]   = {250., 330., 365., 645., 35000., 250., 345., 440., 515., 35000., 250., 385., 470., 640., 35000};
//for 2D
const Float_t boundary_MVA_2019[4]   = {0.37, 0.620, 0.78, 1.0};
const Float_t boundary_MX_2019[15]   = {250., 330., 375., 585., 35000., 250., 330., 360., 540., 35000., 250., 385., 510., 600., 35000};
//16/03/2020 - form Tatyana - b enriched MC
//const Float_t boundary_MVA_2019[4]   = {0.19, 0.41, 0.70, 1.0};
//const Float_t boundary_MX_2019[15]   = {250., 346., 660., 781., 35000., 250., 351., 660., 755., 35000., 250., 356., 698., 745., 35000};
//Flashgg, eth categorisation - dev_legacy - 2016-17-18 - wo Mjj
//const Float_t boundary_MVA_2017[4]   = {0.290, 0.441, 0.724, 1.0};
//const Float_t boundary_MX_2017[5]   = {250., 354., 478., 560., 35000};
//Flashgg, diphoton categorisation (Tatyana) - dev_legacy - 2016-17-18 - reweight
//const Float_t boundary_MVA_2017[4]   = {0.242, 0.447, 0.724, 1.0};
//const Float_t boundary_MX_2017[5]   = {250., 385., 423., 563., 35000};
//Flashgg, new categorisation (Tatyana) - dev_legacy - 2016-17-18 - reweight
//const Float_t boundary_MVA_2017[4]   = {0.249, 0.450, 0.729, 1.0};
//const Float_t boundary_MX_2017[5]   = {250., 375., 553., 637., 35000};
//new categorisation (Tatyana) - dev_legacy - 2016-17-18 - reweight - 22/01/2020
//for our 2D result 0.41 
//const Float_t boundary_MVA_2017[4]   = {0.248, 0.450, 0.728, 1.0};
//const Float_t boundary_MX_2017[5]   = {250., 376., 521., 603., 35000};
//for withbtagreweight from Tatyana - mvawithmjj - 1D
//const Float_t boundary_MVA_2017[4]   = {0.36, 0.56 , 0.79, 1.0};
//const Float_t boundary_MX_2017[5]   = {250., 390., 570., 652., 35000};
//for withbtagreweight from Tatyana - mvawomjj - 2D
//const Float_t boundary_MVA_2017[4]   = {0.38, 0.61 , 0.81, 1.0};
//const Float_t boundary_MX_2017[5]   = {250., 409., 533., 632., 35000};
//for withbtagreweight from Tatyana - mvawomjj, our train - 2D
//const Float_t boundary_MVA_2017[4]   = {0.30, 0.52 , 0.78, 1.0};
//const Float_t boundary_MX_2017[5]   = {250., 383., 471., 515., 35000};
//const Float_t boundary_MVA_2017[4]   = {0.38, 0.62 , 0.82, 1.0};
//const Float_t boundary_MX_2017[5]   = {250., 415., 490., 602., 35000};
//16/03/2020 - form Tatyana - b enriched MC
//vrsion 1
//const Float_t boundary_MVA_2017[4]   = {0.19, 0.41 , 0.70, 1.0};
//const Float_t boundary_MX_2017[5]   = {250., 356., 595., 755., 35000};
//version 2
const Float_t boundary_MVA_2017[4]   = {0.19, 0.41 , 0.70, 1.0};
const Float_t boundary_MX_2017[5]   = {250., 330., 641., 742., 35000};


// Cuts for 2020 tagger (_which==4): for check with 2 categories
//const Float_t boundary_MVA_2020[2] = {0.250, 1.0};
//const Float_t boundary_MX_2020[3] = {250., 350., 35000};
//15 categories - MC b-encriched
//const Float_t boundary_MVA_cat15[4]   = {0.19, 0.41, 0.70, 1.0};
//const Float_t boundary_MX_cat15[15]   = {250., 335., 380., 670., 744., 35000.};
//15 categories - MC DiPhoton+Jets
const Float_t boundary_MVA_cat15[4]   = {0.38, 0.62, 0.82, 1.0};
const Float_t boundary_MX_cat15[15]   = {250., 402., 469., 531., 619., 35000.};


//const Float_t MjjCuts_Low[] = {97.,95.,95.,95.,99.,100.,95.,95.,95.,95.,95.,108.};
//const Float_t MjjCuts_High[] = {146.,153.,150.,155.,150.,147.,154.,151.,155.,155.,155.,148.};

const Float_t MjjCuts_Low[] = {98.0,95.0,97.0,96.0,95.0,95.0,95.0,95.0,95.0,95.0,95.0,95.0};
const Float_t MjjCuts_High[] = {150.0,150.0,143.0,150.0,150.0,150.0,150.0,145.0,155.0,142.0,146.0,152.0};


void bbggLTMaker::Begin(TTree * /*tree*/)
{
  TString option = GetOption();

  cout<<"Options are: \n \t "<<option.Data()<<endl;
  TObjArray *args = (TObjArray*)option.Tokenize(" ");
  _normalization = std::stod((string)((TObjString*)args->At(0))->GetString());
  _outFileName = (string)((TObjString*)args->At(1))->GetString();
  _genDiPhotonFilter = (Int_t) std::stoi( (string)((TObjString*)args->At(2))->GetString() );
  _whichCategorization = (Int_t) std::stoi( (string)((TObjString*)args->At(3))->GetString() );
  _ttHTagger = (Bool_t) std::stoi( (string)((TObjString*)args->At(4))->GetString() );
  std::cout<<"Input paremeters:\n"
	   <<"_normalization: "<<_normalization<<"\n"
	   <<"_outFileName: "<< _outFileName<<"\n"
	   <<"_genDiPhotonFilter: "<< _genDiPhotonFilter<<"\n"
           <<"_ttHTagger: "<< _ttHTagger<<"\n"
	   <<std::endl;

  // Could make those come from external options as well:
  _phoVariation = 0;
  _trigVariation = 0;

  _outFile = new TFile(_outFileName, "RECREATE");
  _outTree = new TTree("LT", "A tree for studying new particles");

  _outTree->Branch("run", &o_run, "o_run/i");
  _outTree->Branch("evt", &o_evt, "o_evt/l");

  _outTree->Branch("evWeight", &o_weight, "o_weight/D");
  _outTree->Branch("mgg", &o_mgg, "o_mgg/D");
  _outTree->Branch("mjj", &o_mjj, "o_mjj/D");
  _outTree->Branch("MX",  &o_MX,  "o_MX/D");
  _outTree->Branch("mbbgg", &o_bbggMass, "o_bbggMass/D");
  _outTree->Branch("catID", &o_catID, "o_catID/I");
  _outTree->Branch("ttHTagger", &o_ttHTagger, "ttHTagger/D");

  _outTree->Branch("vbf_Cat", &o_vbf_Cat, "o_vbf_Cat/I");
  _outTree->Branch("vbf_Cat_Selected", &o_vbf_Cat_Selected, "o_vbf_Cat_Selected/I");

  TString phoSFID_file = TString(std::getenv("CMSSW_BASE")) + TString("/src/HiggsAnalysis/bbggLimits2018/Weights/MVAID/egammaEffi.txt_EGM2D.root");
  if (DEBUG) cout << "phoSFsID file: " << phoSFID_file << endl;
  TString phoSFeveto_file = TString(std::getenv("CMSSW_BASE")) + TString("/src/HiggsAnalysis/bbggLimits2018/Weights/MVAID/ScalingFactors_80X_Summer16.root");
  if (DEBUG) cout << "phoSFsEV file: " << phoSFeveto_file << endl;
  bbggLTMaker::SetupPhotonSF( phoSFID_file, phoSFeveto_file);

  TString trig_file = TString(std::getenv("CMSSW_BASE")) + TString("/src/HiggsAnalysis/bbggLimits2018/Weights/TriggerSF/TriggerSFs.root");
  if (DEBUG) cout << "TriggSF file: " << trig_file << endl;
  bbggLTMaker::SetupTriggerSF(trig_file);

  ifstream normfact;
  normfact.open ("normalisation_numbers.txt");
  for (int i=0;i<82;i++)
      normfact>>NF_2016[i]>>NF_2017[i]>>NF_2018[i];

}

void bbggLTMaker::SlaveBegin(TTree * /*tree*/)
{
  TString option = GetOption();
}

Bool_t bbggLTMaker::Process(Long64_t entry)
{
  GetEntry(entry);
  o_run = run;
  o_evt = event;
  
  //For SM 
  F_2016 = 62.57257893;
  F_2017 = 8.27769036;
  F_2018 = 8.360941357;
  /*
  //For 36  -> kl=1    (6+kl)/0.2+1=36 
  F_2016 = 61.2285468;
  F_2017 = 8.196101254;
  F_2018 = 8.268999711;
 */ 
  o_weight = weight*_normalization;
 
  //if (o_evt<20) cout<<"event="<<event<<"\t"<<"weight="<<weight<<"\t"<<"Mjj="<<Mjj<<"\t"<<"CMS_hgg_mass="<<CMS_hgg_mass<<endl;
  //if (o_evt<20) cout<<"_normalization="<<_normalization<<endl;
  //if (o_evt<20) cout<<"o_weight="<<o_weight<<endl;
  

  /* 
  //for LO 
  if (_normalization == 35.9) { F_year=F_2016; btagnorm=1.01171; }
  if (_normalization == 41.5) { F_year=F_2017; btagnorm=1.008805; }
  if (_normalization == 59.4) { F_year=F_2018; btagnorm=1.001397; }
  */
  
  ///for NLO ggHH samples
  if (_normalization == 35.9) { F_year=F_2016; btagnorm=1.011; }
  if (_normalization == 41.5) { F_year=F_2017; btagnorm=1.013; }
  if (_normalization == 59.4) { F_year=F_2018; btagnorm=1.004; }
 
  if ( _normalization!=1 && _genDiPhotonFilter==0) {
    //o_weight=o_weight*btagnorm*reweight/(F_year/1.06); //ggHH signal
    o_weight=o_weight*btagnorm*reweight; //ggHH signal
    //cout<<"gghh, o_weight="<<o_weight<<"\t"<<"reweight="<<reweight<<"\t"<<"F_year="<<F_year<<endl;
  }
//  if ( _normalization!=1 && _genDiPhotonFilter==2) 
//  {
//    o_weight=o_weight*btagnorm*reweight*1.06; //VBFHH signal
//    //cout<<"vbfhh, o_weight="<<o_weight<<"\t"<<"reweight="<<reweight<<endl; 
//  }
  //===========FIXME for extraction limits on MC - indicate right path to out LT
/*
  TString option = GetOption(); 
  TObjArray *args = (TObjArray*)option.Tokenize(" "); 
 
  if( _normalization==1 && ((string)((TObjString*)args->At(1))->GetString() == "/afs/cern.ch/work/i/ivovtin/Hggbb/LT_2D_Y5_DatatoMC_15cat_20032020/2016/LT_DoubleEG.root") ) {o_weight=o_weight*2.9*35.9; 
   if (o_evt<20) cout<<"(string)((TObjString*)args->At(1))->GetString()="<<(string)((TObjString*)args->At(1))->GetString()<<endl; 
   if (o_evt<20) cout<<"o_weight="<<o_weight<<endl;
  }
  if( _normalization==1 && ((string)((TObjString*)args->At(1))->GetString() == "/afs/cern.ch/work/i/ivovtin/Hggbb/LT_2D_Y5_DatatoMC_15cat_20032020/2017/LT_DoubleEG.root") ) o_weight=o_weight*2.9*41.5;
  //if( _normalization==1 && ((string)((TObjString*)args->At(1))->GetString() == "/afs/cern.ch/work/i/ivovtin/Hggbb/LT_2D_Y4_DatatoMC_benriched_05032020/2018/LT_DoubleEG.root") ) o_weight=o_weight*2.9*59.4;
  if( _normalization==1 && ((string)((TObjString*)args->At(1))->GetString() == "/afs/cern.ch/work/i/ivovtin/Hggbb/LT_2D_Y5_DatatoMC_15cat_20032020/2018/LT_DoubleEG.root") ) o_weight=o_weight*2.9;
*/   
  //===========

  //if (o_evt<20) cout<<"o_weight="<<o_weight<<endl;

  //o_weight = 1;

  o_mgg = CMS_hgg_mass;
  o_mjj = Mjj;
  o_bbggMass = diHiggs_mass;

  o_MX = MX;

  o_catID = -1;

  o_vbf_Cat = vbf_Cat;
  o_vbf_Cat_Selected = vbf_Cat_Selected;

  if(_ttHTagger)
    o_ttHTagger = ttHScore;
  else     
    o_ttHTagger = -1;

  if (_whichCategorization==0){
    if (o_MX > boundary_MX_2017[0] && o_MX <= boundary_MX_2017[1]){
      if( leadingJet_bDis < cutJetBtag || subleadingJet_bDis < cutJetBtag)
	return kTRUE; // Some clean-up based on b-tag scores

      if (HHbbggMVA > boundary_MVA_2017[0] && HHbbggMVA <= boundary_MVA_2017[1])
	o_catID = 3;
      else if (HHbbggMVA > boundary_MVA_2017[1] && HHbbggMVA <= boundary_MVA_2017[2])
	o_catID = 2;
      else {
	return kTRUE;
      }
    }
    else if (o_MX > boundary_MX[1] && o_MX <= boundary_MX[2]){
      if (HHbbggMVA > boundary_MVA_tagger2016_HM[0] && HHbbggMVA <= boundary_MVA_tagger2016_HM[1])
	o_catID = 1;
      else if (HHbbggMVA > boundary_MVA_tagger2016_HM[1] && HHbbggMVA <= boundary_MVA_tagger2016_HM[2])
	o_catID = 0;
      else {
	//if (DEBUG) std::cout<<"MVA is out of bounds!  MVA="<<HHTagger_HM<<std::endl;
	return kTRUE;
      }
    }
    else  {
      std::cout<<"MX is out of bounds!  MX="<<o_MX<<std::endl;
      return kTRUE;
    }
  }
  else if (_whichCategorization==1){
    if (o_MX > boundary_MX[0] && o_MX <= boundary_MX[1]){
      if( leadingJet_bDis < cutJetBtag || subleadingJet_bDis < cutJetBtag)
	return kTRUE; // Some clean-up based on b-tag scores

      if (HHbbggMVA > boundary_MVA_tagger2017_LM[0] && HHbbggMVA <= boundary_MVA_tagger2017_LM[1])
	o_catID = 3;
      else if (HHbbggMVA > boundary_MVA_tagger2017_LM[1] && HHbbggMVA <= boundary_MVA_tagger2017_LM[2])
	o_catID = 2;
      else {
	//if (DEBUG) std::cout<<"MVA is out of bounds!  MVA="<<HHTagger_LM<<std::endl;
	return kTRUE;
      }
    }
    else if (o_MX > boundary_MX[1] && o_MX <= boundary_MX[2]){
      if (HHbbggMVA > boundary_MVA_tagger2017_HM[0] && HHbbggMVA <= boundary_MVA_tagger2017_HM[1])
	o_catID = 1;
      else if (HHbbggMVA > boundary_MVA_tagger2017_HM[1] && HHbbggMVA <= boundary_MVA_tagger2017_HM[2])
	o_catID = 0;
      else {
	//if (DEBUG) std::cout<<"MVA is out of bounds!  MVA="<<HHTagger_HM<<std::endl;
	return kTRUE;
      }
    }
    else  {
      std::cout<<"MX is out of bounds!  MX="<<o_MX<<std::endl;
      return kTRUE;
    }
  }

//// using categrisation 2 for 2017
//For fix "Turn on Mjj" apply cut ptj1/Mjj>0.55  =================================================================
   
  else if (_whichCategorization==2 || _whichCategorization==3){
    if (o_MX > boundary_MX_2017[0] && o_MX <= boundary_MX_2017[1]){
      if (HHbbggMVA > boundary_MVA_2017[0] && HHbbggMVA <= boundary_MVA_2017[1] && leadingJet_pt/Mjj > 0.55 ){
	o_catID = 11;
	if (_whichCategorization==3 && (o_mjj < MjjCuts_Low[o_catID] || o_mjj > MjjCuts_High[o_catID]) )
	  return kTRUE;
      }
      else if (HHbbggMVA > boundary_MVA_2017[1] && HHbbggMVA <= boundary_MVA_2017[2] && leadingJet_pt/Mjj > 0.55 ){
	o_catID = 7;
	if (_whichCategorization==3 && (o_mjj < MjjCuts_Low[o_catID] || o_mjj > MjjCuts_High[o_catID]) )
	  return kTRUE;
      }
      else if (HHbbggMVA > boundary_MVA_2017[2] && HHbbggMVA <= boundary_MVA_2017[3] && leadingJet_pt/Mjj > 0.55 ){
        o_catID = 3;
        if (_whichCategorization==3 && (o_mjj < MjjCuts_Low[o_catID] || o_mjj > MjjCuts_High[o_catID]) )
          return kTRUE;
      }
      else {
	return kTRUE;
      }
    }

    else if (o_MX > boundary_MX_2017[1] && o_MX <= boundary_MX_2017[2]){
      if (HHbbggMVA > boundary_MVA_2017[0] && HHbbggMVA <= boundary_MVA_2017[1] && leadingJet_pt/Mjj > 0.55 ){
	o_catID = 10;
	if (_whichCategorization==3 && (o_mjj < MjjCuts_Low[o_catID] || o_mjj > MjjCuts_High[o_catID]) )
	  return kTRUE;
      }
      else if (HHbbggMVA > boundary_MVA_2017[1] && HHbbggMVA <= boundary_MVA_2017[2] && leadingJet_pt/Mjj > 0.55 ){
	o_catID = 6;
	if (_whichCategorization==3 && (o_mjj < MjjCuts_Low[o_catID] || o_mjj > MjjCuts_High[o_catID]) )
	  return kTRUE;
      }
      else if (HHbbggMVA > boundary_MVA_2017[2] && HHbbggMVA <= boundary_MVA_2017[3] && leadingJet_pt/Mjj > 0.55 ){
        o_catID = 2;
        if (_whichCategorization==3 && (o_mjj < MjjCuts_Low[o_catID] || o_mjj > MjjCuts_High[o_catID]) )
          return kTRUE;
      }
      else {
	return kTRUE;
      }
    }

    else if (o_MX > boundary_MX_2017[2] && o_MX <= boundary_MX_2017[3]){
      if (HHbbggMVA > boundary_MVA_2017[0] && HHbbggMVA <= boundary_MVA_2017[1] && leadingJet_pt/Mjj > 0.55 ){
	o_catID = 9;
	if (_whichCategorization==3 && (o_mjj < MjjCuts_Low[o_catID] || o_mjj > MjjCuts_High[o_catID]) )
	  return kTRUE;
      }
      else if (HHbbggMVA > boundary_MVA_2017[1] && HHbbggMVA <= boundary_MVA_2017[2] && leadingJet_pt/Mjj > 0.55 ){
	o_catID = 5;
	if (_whichCategorization==3 && (o_mjj < MjjCuts_Low[o_catID] || o_mjj > MjjCuts_High[o_catID]) )
	  return kTRUE;
      }
      else if (HHbbggMVA > boundary_MVA_2017[2] && HHbbggMVA <= boundary_MVA_2017[3] && leadingJet_pt/Mjj > 0.55 ){
        o_catID = 1;
        if (_whichCategorization==3 && (o_mjj < MjjCuts_Low[o_catID] || o_mjj > MjjCuts_High[o_catID]) )
          return kTRUE;
      }
      else {
	return kTRUE;
      }
    }
    
    else if (o_MX > boundary_MX_2017[3] && o_MX <= boundary_MX_2017[4]){
      if (HHbbggMVA > boundary_MVA_2017[0] && HHbbggMVA <= boundary_MVA_2017[1] && leadingJet_pt/Mjj > 0.55 ){
	o_catID = 8;
	if (_whichCategorization==3 && (o_mjj < MjjCuts_Low[o_catID] || o_mjj > MjjCuts_High[o_catID]) )
	  return kTRUE;
      }
      else if (HHbbggMVA > boundary_MVA_2017[1] && HHbbggMVA <= boundary_MVA_2017[2] && leadingJet_pt/Mjj > 0.55 ){
	o_catID = 4;
	if (_whichCategorization==3 && (o_mjj < MjjCuts_Low[o_catID] || o_mjj > MjjCuts_High[o_catID]) )
	  return kTRUE;
      }
      else if (HHbbggMVA > boundary_MVA_2017[2] && HHbbggMVA <= boundary_MVA_2017[3] && leadingJet_pt/Mjj > 0.55 ){
        o_catID = 0;
        if (_whichCategorization==3 && (o_mjj < MjjCuts_Low[o_catID] || o_mjj > MjjCuts_High[o_catID]) )
          return kTRUE;
      }
      else {
        return kTRUE;
      }

     }

    else  {
      std::cout<<"MX is out of bounds!  MX="<<o_MX<<std::endl;
      return kTRUE;
    }
  }
   
//=========with cut =========================
  //Categorisation for boundaries from flashgg
  else if (_whichCategorization==4){    
    if (o_MX > boundary_MX_2019[0] && o_MX <= boundary_MX_2019[14] ){
    if (HHbbggMVA > boundary_MVA_2019[0] && HHbbggMVA <= boundary_MVA_2019[1] && leadingJet_pt/Mjj > 0.55 ){
      if (o_MX > boundary_MX_2019[0] && o_MX <= boundary_MX_2019[1]){
        o_catID = 11;
      }
      else if (o_MX > boundary_MX_2019[1] && o_MX <= boundary_MX_2019[2]){
        o_catID = 10;
      }
      else if (o_MX > boundary_MX_2019[2] && o_MX <= boundary_MX_2019[3]){
        o_catID = 9;
      }
      else if (o_MX > boundary_MX_2019[3] && o_MX <= boundary_MX_2019[4]){
        o_catID = 8;
      }
      else {
        return kTRUE;
      }
    }

    else if (HHbbggMVA > boundary_MVA_2019[1] && HHbbggMVA <= boundary_MVA_2019[2] && leadingJet_pt/Mjj > 0.55 ){
      if (o_MX > boundary_MX_2019[5] && o_MX <= boundary_MX_2019[6]){
        o_catID = 7;
      }
      else if (o_MX > boundary_MX_2019[6] && o_MX <= boundary_MX_2019[7]){
        o_catID = 6;
      }
      else if (o_MX > boundary_MX_2019[7] && o_MX <= boundary_MX_2019[8]){
        o_catID = 5;
      }
      else if (o_MX > boundary_MX_2019[8] && o_MX <= boundary_MX_2019[9]){
        o_catID = 4;
      }
      else {
        return kTRUE;
      }
    }

    else if (HHbbggMVA > boundary_MVA_2019[2] && HHbbggMVA <= boundary_MVA_2019[3] && leadingJet_pt/Mjj > 0.55 ){
      if (o_MX > boundary_MX_2019[10] && o_MX <= boundary_MX_2019[11]){
        o_catID = 3;
      }
      else if (o_MX > boundary_MX_2019[11] && o_MX <= boundary_MX_2019[12]){
        o_catID = 2;
      }
      else if (o_MX > boundary_MX_2019[12] && o_MX <= boundary_MX_2019[13]){
        o_catID = 1;
      }
      else if (o_MX > boundary_MX_2019[13] && o_MX <= boundary_MX_2019[14]){
        o_catID = 0;
      }
      else {
        return kTRUE;
      }
    }
    else  { 
      return kTRUE;
    }
    }
    else  {
      std::cout<<"MX is out of bounds!  MX="<<o_MX<<"\t"<<"MVA="<<HHbbggMVA<<std::endl;
      return kTRUE;
    }
  }
 
  //=============================================================================================================
  /*
  //2 categories for test
  else if (_whichCategorization==4){
    if (o_MX > boundary_MX_2020[0] && o_MX <= boundary_MX_2020[1]){

      if (HHbbggMVA > boundary_MVA_2020[0] && HHbbggMVA <= boundary_MVA_2020[1])
	o_catID = 1;
      else {
	//if (DEBUG) std::cout<<"MVA is out of bounds!  MVA="<<HHTagger_LM<<std::endl;
	return kTRUE;
      }
    }
    else if (o_MX > boundary_MX_2020[1] && o_MX <= boundary_MX_2020[2]){
      if (HHbbggMVA > boundary_MVA_2020[0] && HHbbggMVA <= boundary_MVA_2020[1])
	o_catID = 0;
      else {
	//if (DEBUG) std::cout<<"MVA is out of bounds!  MVA="<<HHTagger_HM<<std::endl;
	return kTRUE;
      }
    }
    else  {
      std::cout<<"MX is out of bounds!  MX="<<o_MX<<std::endl;
      return kTRUE;
    }
  }
  */
  //=============================================================================================================
  //15 Categories
  else if (_whichCategorization==5){    
    if (o_MX > boundary_MX_cat15[0] && o_MX <= boundary_MX_cat15[5]){
    if (HHbbggMVA > boundary_MVA_cat15[0] && HHbbggMVA <= boundary_MVA_cat15[1] && leadingJet_pt/Mjj > 0.55 ){
      if (o_MX > boundary_MX_cat15[0] && o_MX <= boundary_MX_cat15[1]){
        o_catID = 14;
      }
      else if (o_MX > boundary_MX_cat15[1] && o_MX <= boundary_MX_cat15[2]){
        o_catID = 13;
      }
      else if (o_MX > boundary_MX_cat15[2] && o_MX <= boundary_MX_cat15[3]){
        o_catID = 12;
      }
      else if (o_MX > boundary_MX_cat15[3] && o_MX <= boundary_MX_cat15[4]){
        o_catID = 11;
      }
      else if (o_MX > boundary_MX_cat15[4] && o_MX <= boundary_MX_cat15[5]){
        o_catID = 10;
      }
      else {
        return kTRUE;
      }
    }

    else if (HHbbggMVA > boundary_MVA_cat15[1] && HHbbggMVA <= boundary_MVA_cat15[2] && leadingJet_pt/Mjj > 0.55 ){
      if (o_MX > boundary_MX_cat15[0] && o_MX <= boundary_MX_cat15[1]){
        o_catID = 9;
      }
      else if (o_MX > boundary_MX_cat15[1] && o_MX <= boundary_MX_cat15[2]){
        o_catID = 8;
      }
      else if (o_MX > boundary_MX_cat15[2] && o_MX <= boundary_MX_cat15[3]){
        o_catID = 7;
      }
      else if (o_MX > boundary_MX_cat15[3] && o_MX <= boundary_MX_cat15[4]){
        o_catID = 6;
      }
      else if (o_MX > boundary_MX_cat15[4] && o_MX <= boundary_MX_cat15[5]){
        o_catID = 5;
      }
      else {
        return kTRUE;
      }
    }

    else if (HHbbggMVA > boundary_MVA_cat15[2] && HHbbggMVA <= boundary_MVA_cat15[3] && leadingJet_pt/Mjj > 0.55 ){
      if (o_MX > boundary_MX_cat15[0] && o_MX <= boundary_MX_cat15[1]){
        o_catID = 4;
      }
      else if (o_MX > boundary_MX_cat15[1] && o_MX <= boundary_MX_cat15[2]){
        o_catID = 3;
      }
      else if (o_MX > boundary_MX_cat15[2] && o_MX <= boundary_MX_cat15[3]){
        o_catID = 2;
      }
      else if (o_MX > boundary_MX_cat15[3] && o_MX <= boundary_MX_cat15[4]){
        o_catID = 1;
      }
      else if (o_MX > boundary_MX_cat15[4] && o_MX <= boundary_MX_cat15[5]){
        o_catID = 0;
      }
      else {
        return kTRUE;
      }
    }
    else  { 
      return kTRUE;
    }
    }
    else  {
      std::cout<<"MX is out of bounds!  MX="<<o_MX<<"\t"<<"MVA="<<HHbbggMVA<<std::endl;
      return kTRUE;
    }
  }
//=========with cut =========================
  //Categorisation for boundaries from flashgg. VBFHH categories
  else if (_whichCategorization==6){
    //if( (o_vbf_Cat_Selected!=0) && ttHScore>0.3 && MVAOutput_vbf_gg>0.99 && MVAOutput_vbf_gg <= 1.0 ) {  //sm, c2v2 
    //if( (o_vbf_Cat_Selected!=0) && ttHScore>0.3 && MVAOutput_vbf_gg>0.98 && MVAOutput_vbf_gg <= 1.0 ) {    //c2vmix
    //if( MVAOutput_vbf_gg>0.75 && ttHScore>0.3 ) { //c2v0 
    	//std::cout<<"o_vbf_Cat="<<o_vbf_Cat_Selected<<"\t"<<"o_vbf_Cat_Selected="<<o_vbf_Cat_Selected<<"\t"<<"Mjj="<<Mjj<<std::endl;
    //	o_catID = 12;
    //} 
    //=== 
    //C2V2trainVBF2018C2V_16cats
    /*
    if ( MVAOutput_vbf_ggf > 0.1 && ttHScore>0.3 ) {
      if( o_MX > 1100 && MVAOutput_vbf_gg > 0.85 ){
          o_catID = 15; 
      }
      else if ( o_MX > 650 && o_MX <= 1100 && MVAOutput_vbf_gg > 0.992 ){
          o_catID = 14; 
      }  
      else if ( o_MX > 400 && o_MX <= 650 && MVAOutput_vbf_gg > 0.987 ){
          o_catID = 13; 
      }  
      else if ( o_MX <= 400 && MVAOutput_vbf_gg > 0.95 ){
          o_catID = 12; 
      }  
       else {
        return kTRUE;
      }
    }
    */ 
    //=== 
    /*    
    if ( o_vbf_Cat_Selected!=0 && ttHScore>0.3 ) {
    if( MVAOutput_vbf_gg > 0.974 && MVAOutput_vbf_gg <= 0.99 && leadingJet_pt/Mjj > 0.55 ){
          o_catID = 13; 
    }
    else if ( MVAOutput_vbf_gg > 0.99 && MVAOutput_vbf_gg <= 1.0 && leadingJet_pt/Mjj > 0.55 ){
          o_catID = 12; 
    }  
    }
    */
    //=== 
        
    //C2V2trainVBF2018C2V_14cats_v2 - best!
    if( MVAOutput_vbf_gg > 0.992 && MVAOutput_vbf_gg < 0.9965 && MVAOutput_vbf_ggf > 0.1 && ttHScore > 0.3 && leadingJet_pt/Mjj > 0.55 ){
          o_catID = 13; 
    }
    else if ( MVAOutput_vbf_gg > 0.9965 && MVAOutput_vbf_ggf > 0.1 && ttHScore > 0.3 && leadingJet_pt/Mjj > 0.55 ){
          o_catID = 12; 
    }
      
    //===
    
    else{   
    if (o_MX > boundary_MX_2019[0] && o_MX <= boundary_MX_2019[14] ){
    if (HHbbggMVA > boundary_MVA_2019[0] && HHbbggMVA <= boundary_MVA_2019[1] && leadingJet_pt/Mjj > 0.55 ){
      if (o_MX > boundary_MX_2019[0] && o_MX <= boundary_MX_2019[1]){
        o_catID = 11;
      }
      else if (o_MX > boundary_MX_2019[1] && o_MX <= boundary_MX_2019[2]){
        o_catID = 10;
      }
      else if (o_MX > boundary_MX_2019[2] && o_MX <= boundary_MX_2019[3]){
        o_catID = 9;
      }
      else if (o_MX > boundary_MX_2019[3] && o_MX <= boundary_MX_2019[4]){
        o_catID = 8;
      }
      else {
        return kTRUE;
      }
    }

    else if (HHbbggMVA > boundary_MVA_2019[1] && HHbbggMVA <= boundary_MVA_2019[2] && leadingJet_pt/Mjj > 0.55 ){
      if (o_MX > boundary_MX_2019[5] && o_MX <= boundary_MX_2019[6]){
        o_catID = 7;
      }
      else if (o_MX > boundary_MX_2019[6] && o_MX <= boundary_MX_2019[7]){
        o_catID = 6;
      }
      else if (o_MX > boundary_MX_2019[7] && o_MX <= boundary_MX_2019[8]){
        o_catID = 5;
      }
      else if (o_MX > boundary_MX_2019[8] && o_MX <= boundary_MX_2019[9]){
        o_catID = 4;
      }
      else {
        return kTRUE;
      }
    }

    else if (HHbbggMVA > boundary_MVA_2019[2] && HHbbggMVA <= boundary_MVA_2019[3] && leadingJet_pt/Mjj > 0.55 ){
      if (o_MX > boundary_MX_2019[10] && o_MX <= boundary_MX_2019[11]){
        o_catID = 3;
      }
      else if (o_MX > boundary_MX_2019[11] && o_MX <= boundary_MX_2019[12]){
        o_catID = 2;
      }
      else if (o_MX > boundary_MX_2019[12] && o_MX <= boundary_MX_2019[13]){
        o_catID = 1;
      }
      else if (o_MX > boundary_MX_2019[13] && o_MX <= boundary_MX_2019[14]){
        o_catID = 0;
      }
      else {
        return kTRUE;
      }
    }
    else  { 
      return kTRUE;
    }
    }
    else  {
      std::cout<<"MX is out of bounds!  MX="<<o_MX<<"\t"<<"MVA="<<HHbbggMVA<<std::endl;
      return kTRUE;
    }
   }
  }
 
  //=============================================================================================================
 

  else {
    std::cout<<"This categorization is not supported: "<<_whichCategorization<<std::endl;
    return kTRUE;
  }

  //For MC need comment out  - FIXME for extraction limits on MC  !!!!!
  const Double_t preweight = 1*_normalization;                                                 
  if( preweight == 1) o_weight = 1; // When preweight == 1 it's the data, no SF needed.           


  /*else {
    const Double_t pho1_sf = bbggLTMaker::PhotonSF(*leadingPhoton, _phoVariation);
    const Double_t pho2_sf = bbggLTMaker::PhotonSF(*subleadingPhoton, _phoVariation);

    const Double_t trig_sf = bbggLTMaker::TriggerSF(*leadingPhoton, leadingPhotonR9full5x5, *subleadingPhoton, subleadingPhotonR9full5x5, _trigVariation);
    if(DEBUG)
      cout << "Trigger sf: " << trig_sf << " lpt " << leadingPhoton->Pt()<< " spt " << subleadingPhoton->Pt()
	   << " leta "<< leadingPhoton->Eta() << " seta "<< subleadingPhoton->Eta()
	   << " lr9 "<< leadingPhotonR9full5x5 << " sr9 "<< subleadingPhotonR9full5x5 << std::endl;

    const Double_t bDiffWeight = 0.96; // Use an average SF for b-tagging

    o_weight = preweight*bDiffWeight*pho1_sf*pho2_sf*trig_sf;
  }*/

  // This is temporary, to check effect on fake data
  //if (_normalization == 1) o_weight = 1;

  _outTree->Fill();

  return kTRUE;
}

void bbggLTMaker::SlaveTerminate()
{
}

void bbggLTMaker::Terminate()
{
  _outFile->Write();
  _outFile->Close();
}


void bbggLTMaker::SetupPhotonSF(TString idfile, TString evfile)
{
  photonidFile = new TFile(idfile, "READ");
  photonIDhist = (TH2F*) photonidFile->Get("EGamma_SF2D;1");
  csevFile = new TFile(evfile, "READ");
  csevhist = (TH2F*) csevFile->Get("Scaling_Factors_CSEV_R9 Inclusive");
}


float bbggLTMaker::PhotonSF(bbggLTMaker::LorentzVector pho, int phovar)
{
  float sf_id = -99, sf_ev = -99, sf_id_err = -99, sf_ev_err = -99;
  sf_id_err = photonIDhist->GetBinError( photonIDhist->FindBin(pho.eta(), pho.pt()) );
  sf_id = photonIDhist->GetBinContent( photonIDhist->FindBin(pho.eta(), pho.pt()) ) + phovar*sf_id_err;
  sf_ev_err = csevhist->GetBinContent( csevhist->FindBin(pho.eta(), pho.pt()) );
  if(sf_ev_err==1) sf_ev_err=0;
  sf_ev = csevhist->GetBinContent( csevhist->FindBin(fabs(pho.eta()), pho.pt()) ) + phovar*sf_ev_err;
  float totSF = sf_id*sf_ev;
  if (totSF < 1E-1) return 1;
  else return totSF;
}

void bbggLTMaker::SetupTriggerSF(TString sfFile)
{
  triggerFile = new TFile(sfFile, "READ");
  ltriggerHist = (TH3F*) triggerFile->Get("leadingPhotonTSF");
  striggerHist = (TH3F*) triggerFile->Get("subleadingPhotonTSF");
}

float bbggLTMaker::TriggerSF(LorentzVector lpho, float lr9, LorentzVector spho, float sr9, int var)
{
   int leadingBin = ltriggerHist->FindBin(lr9, fabs(lpho.Eta()), lpho.Pt());
   int subleadingBin = striggerHist->FindBin(sr9, fabs(spho.Eta()), spho.Pt());
   if(DEBUG) cout << "[bbggLTMaker::TriggerSF] leading Bin: " << leadingBin << " subleadingBin " << subleadingBin << std::endl;

   float leadingSF = ltriggerHist->GetBinContent(leadingBin);
   float sleadingSF =striggerHist->GetBinContent(subleadingBin);
   if(DEBUG) cout << "[bbggLTMaker::TriggerSF] leading SF: " << leadingSF << " subleadingSF " << sleadingSF << std::endl;

   float leadingErr = ltriggerHist->GetBinError(leadingBin);
   float subleadingErr = striggerHist->GetBinError(subleadingBin);
   if(DEBUG) cout << "[bbggLTMaker::TriggerSF] leading SFerr: " << leadingErr << " subleadingSFerr " << subleadingErr << std::endl;

   return (leadingSF + var*leadingErr)*(sleadingSF + var*subleadingErr);
}
