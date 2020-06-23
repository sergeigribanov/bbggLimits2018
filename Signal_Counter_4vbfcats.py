##f = open("ws_SM_c2v01_noCosTheta_run2_ggHHNLO_looseMVAggf_C2V36/Node_SM/hhbbgg_13TeV_DataCard_envelope.txt", "r")
f = open("/afs/cern.ch/work/i/ivovtin/public/ForMaxim/RUN2_ggHHNLO_MVAggHHLoose/C2Vscan/ws_run2_ggHHNLO_MVAggHHLoose_C2V36/Node_SM/hhbbgg_13TeV_DataCard_envelope.txt", "r")
print("\n")
line = "1"
while line:
    line = f.readline()
    #print line
    if "rate " in line:
        #print line
        nEvt = line.split()
        nVBFHH = sum(map(float, nEvt[1:len(nEvt):8]))
        nGGHH = sum(map(float, nEvt[2:len(nEvt):8]))
        nvbf = sum(map(float, nEvt[3:len(nEvt):8]))
        nbbh = sum(map(float, nEvt[4:len(nEvt):8]))
        nvh = sum(map(float, nEvt[5:len(nEvt):8]))
        nttH = sum(map(float, nEvt[6:len(nEvt):8]))
        #nBkg = sum(map(float, nEvt[7:len(nEvt):8]))
        nggh = sum(map(float, nEvt[8:len(nEvt):8]))
        print "16 cats \tnVBFHH = ", nVBFHH, " nggHH = ", nGGHH, " nvbf = ", nvbf, " nbbh = ", nbbh," nvh = ", nvh," nttH = ", nttH," nggh = ", nggh
        VBF = nEvt[1:len(nEvt):8]
        ggHH = nEvt[2:len(nEvt):8]
        print "in cats 1-12 \tnVBF = ", nVBFHH-sum(map(float,VBF[len(VBF)-4:])), "\tnggHH = ", nGGHH-sum(map(float,ggHH[len(ggHH)-4:]))
        print "in cat 13 \tnVBF = ", VBF[len(VBF)-4], "\tnggHH = ", ggHH[len(ggHH)-4] 
        print "in cat 14 \tnVBF = ", VBF[len(VBF)-3], "\tnggHH = ", ggHH[len(ggHH)-3] 
        print "in cat 15 \tnVBF = ", VBF[len(VBF)-2], "\tnggHH = ", ggHH[len(ggHH)-2] 
        print "in cat 16 \tnVBF = ", VBF[len(VBF)-1], "\tnggHH = ", ggHH[len(ggHH)-1] 

        print "VBF HH: ", map(float, nEvt[1:len(nEvt):8])
        print "gg HH: ", map(float, nEvt[2:len(nEvt):8])
        print "vbf: ", map(float, nEvt[3:len(nEvt):8])
        print "bbh: ", map(float, nEvt[4:len(nEvt):8])
        print "vh: ", map(float, nEvt[5:len(nEvt):8])
        print "ttH: ", map(float, nEvt[6:len(nEvt):8])
        #print "Bkg: ", map(float, nEvt[7:len(nEvt):8])
        print "ggh: ", map(float, nEvt[8:len(nEvt):8])
        
