f = open("/afs/cern.ch/work/i/ivovtin/public/ForMaxim/RUN2_vbfhhsamples_ggHHonly_KL36_12cats/ws_run2_ttHon0.26_2D_ggHHonly_04062020/Node_SM/hhbbgg_13TeV_DataCard_envelope.txt", "r")
print("\n")
line = "1"
while line:
    line = f.readline()
    #print line
    if "rate " in line:
        #print line
        nEvt = line.split()
        nGGHH = sum(map(float, nEvt[1:len(nEvt):7]))
        nBkg = sum(map(float, nEvt[2:len(nEvt):7]))
        nvbf = sum(map(float, nEvt[3:len(nEvt):7]))
        nbbh = sum(map(float, nEvt[4:len(nEvt):7]))
        nvh = sum(map(float, nEvt[5:len(nEvt):7]))
        nttH = sum(map(float, nEvt[6:len(nEvt):7]))
        nggh = sum(map(float, nEvt[7:len(nEvt):7]))
        print "12 cats \tnggHH = ", nGGHH, " nvbf = ", nvbf, " nbbh = ", nbbh," nvh = ", nvh," nttH = ", nttH," nggh = ", nggh
        
        print "gg HH: ", map(float, nEvt[1:len(nEvt):7])
        print "Bkg: ", map(float, nEvt[2:len(nEvt):7])
        print "vbf: ", map(float, nEvt[3:len(nEvt):7])
        print "bbh: ", map(float, nEvt[4:len(nEvt):7])
        print "vh: ", map(float, nEvt[5:len(nEvt):7])
        print "ttH: ", map(float, nEvt[6:len(nEvt):7])
        print "ggh: ", map(float, nEvt[7:len(nEvt):7])
