
## VBF Reweighting Module
## Implements the formula from slides 11 and 12 of:
## https://indico.cern.ch/event/890825/contributions/3759797/attachments/1993521/3324873/2020_25_02_bbtautau_HIGPAG.pdf
## C -> vector of couplings
## S -> vector of samples (generated xs)
## V -> vector of components (a,b,c,iab,ibc,iac)
## M -> matrix that describes the single samples as sum of components, so that S = M * V
## so that M^-1 * S = V
##
## And the final cross section is sigma(CV, C2V, kl) = C^T * V = C^T * M-1 * S

from sympy import *
from sympy import Matrix
from array import array
import sys
import ROOT


#########################
### General functions ###
#########################

# General function to rescale any histogram to a specific value
def scaleTo(hIn, val):
    hIn.Scale(val)

# General function to build a sympy.Matrix from a list of "inputSample"
def build_matrix(sample_list):    
    if len(sample_list) != 6:
        print "[ERROR] : expecting 6 samples in input"
        raise RuntimeError("malformed input sample list")

    M_tofill = [
        [None,None,None,None,None,None],
        [None,None,None,None,None,None],
        [None,None,None,None,None,None],
        [None,None,None,None,None,None],
        [None,None,None,None,None,None],
        [None,None,None,None,None,None]
    ]

    # Implement the 6 scalings
    for isample, sample in enumerate(sample_list):
        M_tofill[isample][0] = sample.val_CV**2 * sample.val_kl**2
        M_tofill[isample][1] = sample.val_CV**4
        M_tofill[isample][2] = sample.val_C2V**2
        M_tofill[isample][3] = sample.val_CV**3 * sample.val_kl
        M_tofill[isample][4] = sample.val_CV    * sample.val_C2V    * sample.val_kl
        M_tofill[isample][5] = sample.val_CV**2 * sample.val_C2V
    
    res = Matrix(M_tofill)
    return res

# General function to print progression bar
def printProgressBar (iteration, total, prefix = '', suffix = '', decimals = 1, length = 100, fill = '#'):
    """
    Call in a loop to create terminal progress bar
    @params:
        iteration   - Required  : current iteration (Int)
        total       - Required  : total iterations (Int)
        prefix      - Optional  : prefix string (Str)
        suffix      - Optional  : suffix string (Str)
        decimals    - Optional  : positive number of decimals in percent complete (Int)
        length      - Optional  : character length of bar (Int)
        fill        - Optional  : bar fill character (Str)
    """
    percent = ("{0:." + str(decimals) + "f}").format(100 * (iteration / float(total)))
    filledLength = int(length * iteration // total)
    bar = fill * filledLength + '-' * (length - filledLength)
    sys.stdout.write('\r%s |%s| %s%s %s' % (prefix, bar, percent, '%', suffix))
    if iteration == total:
        sys.stdout.write('\n')


################################################
### Class for handling the six input samples ###
################################################
class inputSample:
    def __init__(self, val_CV, val_C2V, val_kl, val_xs):
        self.val_CV  = val_CV
        self.val_C2V = val_C2V
        self.val_kl  = val_kl
        self.val_xs  = val_xs



########################################
### Class for actual VBF reweighting ###
########################################
class VBFReweight:
    def __init__(self, sample_list):
        self.sample_list = sample_list
        
        # Declare symbols to be used in the solution of the systems
        CV, C2V, kl, a, b, c, iab, iac, ibc, s1, s2, s3, s4, s5, s6 = symbols('CV C2V kl a b c iab iac ibc s1 s2 s3 s4 s5 s6')
        self.CV  = CV
        self.C2V = C2V
        self.kl  = kl
        self.s1  = s1
        self.s2  = s2
        self.s3  = s3
        self.s4  = s4
        self.s5  = s5
        self.s6  = s6

        # Automatically build the matrix of the initial 6-equation system given the list of samples
        M = build_matrix(sample_list)

        # Declare the vectors of samples, couplings and components
        # The vector of couplings
        C = Matrix([
            [CV**2 * kl**2] ,
            [CV**4]         ,
            [C2V**2]        ,
            [CV**3 * kl]    ,
            [CV * C2V * kl] ,
            [CV**2 * C2V]   
        ])

        # The vector of components
        V = Matrix([
            [a]   ,
            [b]   ,
            [c]   ,
            [iab] ,
            [iac] ,
            [ibc]  
        ])

        # The vector of samples (i.e. cross sections of the input samples)
        S = Matrix([
            [s1] ,
            [s2] ,
            [s3] ,
            [s4] ,
            [s5] ,
            [s6] 
        ])

        # Invert the system matrix to get the resolution
        Minv   = M.inv()

        # Multiply the Minv for the couplings to get the coefficients of the six addendums of the final cross sections
        coeffs = C.transpose() * Minv
        self.coefficients = coeffs

        # Final cross section from multiplying the coefficients for the samples
        sigma  = coeffs*S
        self.modeled_cross_section = sigma

    # Returns the modeled cross section and histogram given the target couplings
    def modelSignal(self,t_cv,t_c2v,t_kl,target_xs):

        # Eval the final cross section
        total_xs = self.modeled_cross_section[0].evalf(subs={
            self.CV  : t_cv,
            self.C2V : t_c2v,
            self.kl  : t_kl,
            self.s1  : self.sample_list[0].val_xs,
            self.s2  : self.sample_list[1].val_xs,
            self.s3  : self.sample_list[2].val_xs,
            self.s4  : self.sample_list[3].val_xs,
            self.s5  : self.sample_list[4].val_xs,
            self.s6  : self.sample_list[5].val_xs,
        })

        total_xs_17 = self.modeled_cross_section[0].evalf(subs={
            self.CV  : 1,
            self.C2V : 0,
            self.kl  : 0,
            self.s1  : self.sample_list[0].val_xs,
            self.s2  : self.sample_list[1].val_xs,
            self.s3  : self.sample_list[2].val_xs,
            self.s4  : self.sample_list[3].val_xs,
            self.s5  : self.sample_list[4].val_xs,
            self.s6  : self.sample_list[5].val_xs,
        })

        total_xs_18 = self.modeled_cross_section[0].evalf(subs={
            self.CV  : 0,
            self.C2V : 1,
            self.kl  : 0,
                self.s1  : self.sample_list[0].val_xs,
            self.s2  : self.sample_list[1].val_xs,
            self.s3  : self.sample_list[2].val_xs,
                self.s4  : self.sample_list[3].val_xs,
            self.s5  : self.sample_list[4].val_xs,
            self.s6  : self.sample_list[5].val_xs,
        })

        total_xs_14 = self.modeled_cross_section[0].evalf(subs={
            self.CV  : 1,
            self.C2V : 0,
            self.kl  : 1,
                    self.s1  : self.sample_list[0].val_xs,
            self.s2  : self.sample_list[1].val_xs,
            self.s3  : self.sample_list[2].val_xs,
                    self.s4  : self.sample_list[3].val_xs,
            self.s5  : self.sample_list[4].val_xs,
            self.s6  : self.sample_list[5].val_xs,
        })

        total_xs_1 = self.modeled_cross_section[0].evalf(subs={
            self.CV  : 1,
                self.C2V : 1,
            self.kl  : 1,
                    self.s1  : self.sample_list[0].val_xs,
            self.s2  : self.sample_list[1].val_xs,
            self.s3  : self.sample_list[2].val_xs,
                        self.s4  : self.sample_list[3].val_xs,
            self.s5  : self.sample_list[4].val_xs,
            self.s6  : self.sample_list[5].val_xs,
        })

        total_xs_2 = self.modeled_cross_section[0].evalf(subs={
            self.CV  : 1,
                self.C2V : 1,
            self.kl  : 0,
                    self.s1  : self.sample_list[0].val_xs,
            self.s2  : self.sample_list[1].val_xs,
            self.s3  : self.sample_list[2].val_xs,
                        self.s4  : self.sample_list[3].val_xs,
            self.s5  : self.sample_list[4].val_xs,
            self.s6  : self.sample_list[5].val_xs,
        })

        total_xs_19 = self.modeled_cross_section[0].evalf(subs={
            self.CV  : 1,
                self.C2V : 0,
            self.kl  : 2,
                    self.s1  : self.sample_list[0].val_xs,
            self.s2  : self.sample_list[1].val_xs,
            self.s3  : self.sample_list[2].val_xs,
                            self.s4  : self.sample_list[3].val_xs,
                self.s5  : self.sample_list[4].val_xs,
            self.s6  : self.sample_list[5].val_xs,
        })

        b = total_xs_17
        c = total_xs_18
        a = 0.5*(total_xs_19-2*total_xs_14+b)
        iab = total_xs_14-a-b
        ibc= total_xs_2-b-c
        iac= total_xs_1-a-b-c-iab-ibc

#        print 'a = ', a
#        print 'b = ', b
#        print 'c = ', c
#        print 'iab = ', iab
#        print 'ibc = ', ibc
#        print 'iac = ', iac



        # Eval the single coefficients
        eval_coeffs = []
        for coeff in self.coefficients:
            eval_coeffs.append(coeff.evalf(subs={self.CV:t_cv, self.C2V:t_c2v, self.kl:t_kl}))

        for i in range(0,6):
            eval_coeffs[i] = eval_coeffs[i]*self.sample_list[i].val_xs

#        print "coeff0 = ", eval_coeffs[0]*self.sample_list[0].val_xs
#        print "coeff1 = ", eval_coeffs[1]*self.sample_list[1].val_xs
#        print "coeff2 = ", eval_coeffs[2]*self.sample_list[2].val_xs
#        print "coeff3 = ", eval_coeffs[3]*self.sample_list[3].val_xs
#        print "coeff4 = ", eval_coeffs[4]*self.sample_list[4].val_xs
#        print "coeff5 = ", eval_coeffs[5]*self.sample_list[5].val_xs

#        print "xSums = ", eval_coeffs[0]*self.sample_list[0].val_xs+eval_coeffs[1]*self.sample_list[1].val_xs+eval_coeffs[2]*self.sample_list[2].val_xs+eval_coeffs[3]*self.sample_list[3].val_xs+eval_coeffs[4]*self.sample_list[4].val_xs+eval_coeffs[5]*self.sample_list[5].val_xs
#        print "xs = ", total_xs

        return total_xs,eval_coeffs
