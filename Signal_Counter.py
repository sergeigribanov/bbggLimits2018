f = open("ws_2D_vbfhhC2V36/Node_SM/hhbbgg_13TeV_DataCard_envelope.txt", "r")
print("\n")
line = "1"
while line:
    line = f.readline()
    #print line
    if "rate " in line:
        #print line
        nEvt = line.split()
        nVBF = sum(map(float, nEvt[1:len(nEvt):8]))
        nGGHH = sum(map(float, nEvt[2:len(nEvt):8]))
        print "13 cats \tnVBF = ", nVBF, " nggHH = ", nGGHH
        VBF = nEvt[1:len(nEvt):8]
        ggHH = nEvt[2:len(nEvt):8]
        print "in cats 1-12 \tnVBF = ", nVBF-float(VBF[len(VBF)-1]), "\tnggHH = ", nGGHH-float(ggHH[len(ggHH)-1]) 
        print "in cat 13 \tnVBF = ", VBF[len(VBF)-1], "\tnggHH = ", ggHH[len(ggHH)-1] 
        
        print "VBF HH: ", map(float, nEvt[1:len(nEvt):8])
        print "gg HH: ", map(float, nEvt[2:len(nEvt):8])
