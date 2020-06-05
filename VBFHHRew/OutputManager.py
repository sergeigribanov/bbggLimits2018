import ROOT
import collections
import fnmatch
import array

from VBFReweightModules import inputSample, VBFReweight, printProgressBar

A13tev = [2.09078, 10.1517, 0.282307, 0.101205, 1.33191, -8.51168, -1.37309, 2.82636, 1.45767, -4.91761, -0.675197, 1.86189, 0.321422, -0.836276, -0.568156]



class OutputManager:
    """ Handles the input from AnalysisHelper and manages the output to a file
    to be used for datacards and analysis"""
    def __init__(self):
        self.histos      = []
        self.variables   = ["MX"]

    # VBF Reweighting
    # Inputs:
    # - inputSigs  : name of 6 input samples
    # - target_kl  : vector of kl target values (float or int)
    # - target_cv  : vector of cv target values (float or int)
    # - target_c2v : vector of c2v target values (float or int)
    # - target BF can be used to weight to the final state BF

    def functionGF(self,kl,kt,c2,cg,c2g):
        A = A13tev
        return A[0]*kt**4 + A[1]*c2**2 + (A[2]*kt**2 + A[3]*cg**2)*kl**2 + A[4]*c2g**2 + ( A[5]*c2 \
+ A[6]*kt*kl )*kt**2  + (A[7]*kt*kl + A[8]*cg*kl )*c2 + A[9]*c2*c2g  + (A[10]*cg*kl + A[11]*c2g)*kt\
**2+ (A[12]*kl*cg + A[13]*c2g )*kt*kl + A[14]*cg*c2g*kl




    def makeVBFrew(self, inputSigs, target_kl, target_cv, target_c2v, target_bf, makePlot = 0):

        print "-- VBF reweighting --"
        print "Input samples:", inputSigs
        print "Target kl    :", target_kl
        print "Target cv    :", target_cv
        print "Target c2v   :", target_c2v
        print "BF:", target_bf

        totIterations = len(target_kl) * len(target_cv) * len(target_c2v) * len(self.variables)
        nIteration = 0

        # Loop on variables and selections to reweight all the histograms
        for var in self.variables:

# this is what I've seen in Soumya mail. To be checked
#sample A: CV = 1    , C2V = 1   , klambda = 1    xs = 0.0015929203539823008  * BR(HH->bbgg)  
#sample B: CV = 1    , C2V = 2   , klambda = 1    xs = 0.013923303834808259   * BR(HH->bbgg) 
#sample C: CV = 1    , C2V = 1   , klambda = 2    xs = 0.0012979351032448377  * BR(HH->bbgg)  
#sample D: CV = 1    , C2V = 1   , klambda = 0    xs = 0.004277286135693214   * BR(HH->bbgg) 
#sample E: CV = 0.5  , C2V = 1   , klambda = 1    xs = 0.010412979351032449   * BR(HH->bbgg) 
#sample F: CV = 1.5  , C2V = 1   , klambda = 1    xs = 0.06339233038348081    * BR(HH->bbgg)

#                inputSample(  1., 1., 1., 0.001499 ), # node 1
#                inputSample(  1., 1., 0., 0.003947 ), # node 2
#                inputSample(  1., 1., 2., 0.001243 ), # node 3
#                inputSample(  1., 2., 1., 0.012719 ), # node 4
#                inputSample( 1.5, 1., 1., 0.057943 ), # node 5
##                inputSample(  1,  0., 2.,   0.0178 ), # node 19
#                inputSample(0.5, 1., 1.,   0.009579 ), # node 6 ---> calculated from Francesco code 

# Mail de Luca
#Sample name                 xs (2017/2018 PDF)      xs (2016 PDF)                                
#VBF_HH_CV_1_C2V_1_C3_1      0.001668                0.001601                                    
#VBF_HH_CV_1_C2V_1_C3_0      0.004454                0.004259                                   
#VBF_HH_CV_1_C2V_1_C3_2      0.001375                0.001327                                  
#VBF_HH_CV_1_C2V_2_C3_1      0.01374                 0.01335                                   
#VBF_HH_CV_0_5_C2V_1_C3_1    0.01046                 0.01009                                      
#VBF_HH_CV_1_5_C2V_1_C3_1    0.0638                  0.06153                                  
#VBF_HH_CV_1_C2V_0_C3_1      0.02617                 0.02526                                       

            sm_sigma_MG = 0.001668
            sm_sigma_HXSWG = 0.001726
            kNNLO = sm_sigma_HXSWG/sm_sigma_MG

                # Create a list of inputSamples
            inputList = [
                inputSample(  1., 1., 1., sm_sigma_MG ), # node 1                                  
                inputSample(  1., 1., 0., 0.004454 ), # node 2                                     
                inputSample(  1., 1., 2., 0.001375 ), # node 3                                     
                inputSample(  1., 2., 1., 0.01374 ), # node 4                                     
                inputSample( 1.5, 1., 1., 0.0638 ), # node 5                                       
                inputSample(0.5, 1., 1.,   0.01046 ), # node 6  
            ]

                # Instantiate a VBFReweight object
            VBFreweighter = VBFReweight(inputList)


            tgRatio = ROOT.TGraph()

            xsecs = []
                # Get the modeled histogram for each (cv,c2v,kl) target point

            for cv in target_cv:
                for c2v in target_c2v:
                    for ikl, kl in enumerate(target_kl):
                            
                        modeled_xs, coefficients = VBFreweighter.modelSignal(cv,c2v,kl,target_bf)
                        xs_gghh = 31.05*self.functionGF(kl, 1, 0, 0, 0)

                        print '\n ====== kl ',kl, ' c2v ', c2v, ' cv ', cv, ': sigma = ', "{0:.3f}".format(modeled_xs*1000), ' fb, sigma*BF = ',"{0:.3f}".format(modeled_xs*target_bf*1000), " k_NNLO/LO = ", "{0:.3f}".format(kNNLO), ' sigma*BF*kNNLO/LO = ',"{0:.3f}".format(modeled_xs*target_bf*1000*kNNLO)
                        print 'coeffs ', "{0:.5f}".format(coefficients[0]*1000), ' ',"{0:.5f}".format(coefficients[1]*1000), "{0:.5f}".format(coefficients[2]*1000), "{0:.5f}".format(coefficients[3]*1000), "{0:.5f}".format(coefficients[4]*1000), "{0:.5f}".format(coefficients[5]*1000), " fb"

                        xsec={
                            "cv": cv,
                            "c2v": c2v,
                            "kl": kl,
                            "xs_vbfhh": modeled_xs*1000,
                            "xs_gghh": xs_gghh
                        }

                        print xsec

                        tgRatio.SetPoint(ikl, kl, xsec.get("xs_vbfhh")/xsec.get("xs_gghh"))

                        xsecs.append(xsec)

            if makePlot:

                tgRatio.SetMarkerStyle(20)
                tgRatio.SetMarkerSize(1)

                plotter = ROOT.TH1F("plotter", "VBF contribution; #kappa_{#lambda}; VBF HH / ggHH", 1, \
-7, 12)
                plotter.SetMaximum(0.5)
                plotter.SetMinimum(0.)
                plotter.SetStats(0)

                c = ROOT.TCanvas("c", "c", 800, 600)  
                c.cd()
                plotter.Draw()                                                                         
                tgRatio.Draw("P")                                                                      
                c.SaveAs("XsecRatio.png")   
