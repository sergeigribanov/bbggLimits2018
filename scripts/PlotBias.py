from ROOT import *

vmu = [1, 1, 1, 10, 10, 10, 10, 20, 20, 20, 20, 30]

fbase = '2016_2017_withPU_BerFast_2D_FORBIAS/Node_SM/BIAS/mlfit.catICAT.tTRUTH.fFUNC.musigMU0.root'

out = TFile('bias_out.root', 'RECREATE')
grs = []
for truth in ltruths:
  gr = TGraphErrors()
  gr.SetName('gr_t'+truth_+"fBer")
  for icat in :
    tf = TFile(fbase.replace('ICAT', str(icat)).replace('TRUTH', truth).replace('MU0', str(vmu[icat])))
    tt = tf.Get('tree_fit_sb')
    hname = 'hist'+str(n)+str(p)
    hist = TH1F(hname, '', 60, -2, 2)
    tt.Draw('(mu-1)/muErr>>'+hname, 'numbadnll>-1','goff')
    fitres = hist.Fit('gaus', 'S')
    mean = fitres.GetParams()[1]
    e_mean = fitres.GetErrors()[1]
    hname2 = 'hist'+str(n)+str(p)+'2'
    hist2 = TH1F(hname2, '', 120, -5, 5)
    tt.Draw('(mu-1)/muErr>>'+hname2, 'numbadnll>-1','goff')
    fitres2 = hist2.Fit('gaus', 'S')
    mean2 = fitres2.GetParams()[1]
    e_mean2 = fitres2.GetErrors()[1]
    mymean = mean
    myerr = e_mean
    if abs(mean2) < abs(mean):
      mymean = mean2
      myerr = e_mean2
    print mymean, myerr
    gr.SetPoint(ip, lbs[ip], mymean)
    gr.SetPointError(ip, float(0), myerr)
  grs.append(gr)

out.cd()
for igr,gr in enumerate(grs):
  gr.Draw("A")
  gr.GetXaxis().SetRangeUser(-0.1, 1.1)
  gr.GetXaxis().SetLimits(-0.1, 1.1)
  gr.SetLineColor(igr+1)
  gr.SetMarkerColor(igr+1)
  gr.Write()
out.Close()
