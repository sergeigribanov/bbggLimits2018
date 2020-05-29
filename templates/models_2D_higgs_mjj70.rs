mgg[100,180];
mjj[70,190];
mtot[200,1600];
ttHTagger[-1,1];
mgg_sig_m0_cat0[125., 122, 127];
mgg_sig_sigma_cat0[1.0, 0.1, 3.0];
mgg_sig_alpha1_cat0[1.0, 0.05, 10.0];
mgg_sig_n1_cat0[2.0, 0.1, 10.0];
mgg_sig_alpha2_cat0[1.0, 0.05, 10.0];
mgg_sig_n2_cat0[2.0, 0.1, 10.0];
mggSig_cat0 = RooDoubleCB(mgg, mgg_sig_m0_cat0, mgg_sig_sigma_cat0, mgg_sig_alpha1_cat0, mgg_sig_n1_cat0, mgg_sig_alpha2_cat0, mgg_sig_n2_cat0);

mgg_sigVBF_m0_cat0[125., 122, 127];
mgg_sigVBF_sigma_cat0[1.0, 0.1, 3.0];
mgg_sigVBF_alpha1_cat0[1.0, 0.05, 10.0];
mgg_sigVBF_n1_cat0[2.0, 0.1, 10.0];
mgg_sigVBF_alpha2_cat0[1.0, 0.05, 10.0];
mgg_sigVBF_n2_cat0[2.0, 0.1, 10.0];
mggSigVBF_cat0 = RooDoubleCB(mgg, mgg_sigVBF_m0_cat0, mgg_sigVBF_sigma_cat0, mgg_sigVBF_alpha1_cat0, mgg_sigVBF_n1_cat0, mgg_sigVBF_alpha2_cat0, mgg_sigVBF_n2_cat0);

mgg_hig_m0_ggh_cat0[124.2, 123, 125];
mgg_hig_sigma_ggh_cat0[2.0, 0.1, 3.0];
mgg_hig_alpha1_ggh_cat0[1.0, 0.05, 10.0];
mgg_hig_n1_ggh_cat0[2.0, 0.1, 10.0];
mgg_hig_alpha2_ggh_cat0[1.0, 0.05, 10.0];
mgg_hig_n2_ggh_cat0[2.0, 0.1, 10.0];
mggHig_ggh_cat0 = RooDoubleCB(mgg, mgg_hig_m0_ggh_cat0, mgg_hig_sigma_ggh_cat0, mgg_hig_alpha1_ggh_cat0, mgg_hig_n1_ggh_cat0, mgg_hig_alpha2_ggh_cat0, mgg_hig_n2_ggh_cat0);

mgg_hig_m0_tth_cat0[124.2, 123, 125];
mgg_hig_sigma_tth_cat0[2.0, 0.1, 3.0];
mgg_hig_alpha1_tth_cat0[1.0, 0.05, 10.0];
mgg_hig_n1_tth_cat0[2.0, 0.1, 10.0];
mgg_hig_alpha2_tth_cat0[1.0, 0.05, 10.0];
mgg_hig_n2_tth_cat0[2.0, 0.1, 10.0];
mggHig_tth_cat0 = RooDoubleCB(mgg, mgg_hig_m0_tth_cat0, mgg_hig_sigma_tth_cat0, mgg_hig_alpha1_tth_cat0, mgg_hig_n1_tth_cat0, mgg_hig_alpha2_tth_cat0, mgg_hig_n2_tth_cat0);

mgg_hig_m0_vh_cat0[124.2, 123, 125];
mgg_hig_sigma_vh_cat0[2.0, 0.1, 3.0];
mgg_hig_alpha1_vh_cat0[1.0, 0.05, 10.0];
mgg_hig_n1_vh_cat0[2.0, 0.1, 10.0];
mgg_hig_alpha2_vh_cat0[1.0, 0.05, 10.0];
mgg_hig_n2_vh_cat0[2.0, 0.1, 10.0];
mggHig_vh_cat0 = RooDoubleCB(mgg, mgg_hig_m0_vh_cat0, mgg_hig_sigma_vh_cat0, mgg_hig_alpha1_vh_cat0, mgg_hig_n1_vh_cat0, mgg_hig_alpha2_vh_cat0, mgg_hig_n2_vh_cat0);

mgg_hig_m0_vbf_cat0[124.2, 123, 125];
mgg_hig_sigma_vbf_cat0[2.0, 0.1, 3.0];
mgg_hig_alpha1_vbf_cat0[1.0, 0.05, 10.0];
mgg_hig_n1_vbf_cat0[2.0, 0.1, 10.0];
mgg_hig_alpha2_vbf_cat0[1.0, 0.05, 10.0];
mgg_hig_n2_vbf_cat0[2.0, 0.1, 10.0];
mggHig_vbf_cat0 = RooDoubleCB(mgg, mgg_hig_m0_vbf_cat0, mgg_hig_sigma_vbf_cat0, mgg_hig_alpha1_vbf_cat0, mgg_hig_n1_vbf_cat0, mgg_hig_alpha2_vbf_cat0, mgg_hig_n2_vbf_cat0);

mgg_hig_m0_bbh_cat0[124.2, 123, 125];
mgg_hig_sigma_bbh_cat0[2.0, 0.1, 3.0];
mgg_hig_alpha1_bbh_cat0[1.0, 0.05, 10.0];
mgg_hig_n1_bbh_cat0[2.0, 0.1, 10.0];
mgg_hig_alpha2_bbh_cat0[1.0, 0.05, 10.0];
mgg_hig_n2_bbh_cat0[2.0, 0.1, 10.0];
mggHig_bbh_cat0 = RooDoubleCB(mgg, mgg_hig_m0_bbh_cat0, mgg_hig_sigma_bbh_cat0, mgg_hig_alpha1_bbh_cat0, mgg_hig_n1_bbh_cat0, mgg_hig_alpha2_bbh_cat0, mgg_hig_n2_bbh_cat0);


mjj_sig_m0_cat0[110.0, 99, 140];
mjj_sig_sigma_cat0[10.0, 1.0, 60.0];
mjj_sig_alpha1_cat0[1.0, 0.05, 10.0];
mjj_sig_n1_cat0[2.0, 0.1, 10.0];
mjj_sig_alpha2_cat0[1.0, 0.05, 10.0];
mjj_sig_n2_cat0[2.0, 0.1, 10.0];
mjjSig_cat0 = RooDoubleCB(mjj, mjj_sig_m0_cat0, mjj_sig_sigma_cat0, mjj_sig_alpha1_cat0, mjj_sig_n1_cat0, mjj_sig_alpha2_cat0, mjj_sig_n2_cat0);

mjj_sigVBF_m0_cat0[110.0, 99, 140];
mjj_sigVBF_sigma_cat0[10.0, 1.0, 60.0];
mjj_sigVBF_alpha1_cat0[1.0, 0.05, 10.0];
mjj_sigVBF_n1_cat0[2.0, 0.1, 10.0];
mjj_sigVBF_alpha2_cat0[1.0, 0.05, 10.0];
mjj_sigVBF_n2_cat0[2.0, 0.1, 10.0];
mjjSigVBF_cat0 = RooDoubleCB(mjj, mjj_sigVBF_m0_cat0, mjj_sigVBF_sigma_cat0, mjj_sigVBF_alpha1_cat0, mjj_sigVBF_n1_cat0, mjj_sigVBF_alpha2_cat0, mjj_sigVBF_n2_cat0);

mjj_hig_par1_ggh_cat0[0.1, 0, 10];
mjj_hig_par2_ggh_cat0[0.1, 0, 10];
mjj_hig_par3_ggh_cat0[0.1, 0, 10];

mjj_hig_par1_vbf_cat0[0.1, 0, 10];
mjj_hig_par2_vbf_cat0[0.1, 0, 10];
mjj_hig_par3_vbf_cat0[0.1, 0, 10];

mjj_hig_m0_tth_cat0[100, 70, 190];
mjj_hig_sigma_tth_cat0[50, 10, 100];
mjj_hig_alpha1_tth_cat0[1.0, 0.01, 10];
mjj_hig_n1_tth_cat0[1, 0.01, 10];
mjj_hig_alpha2_tth_cat0[1.0, 0.01, 10];
mjj_hig_n2_tth_cat0[1, 0.01, 10];
mjjHig_tth_cat0 = RooDoubleCB(mjj, mjj_hig_m0_tth_cat0, mjj_hig_sigma_tth_cat0, mjj_hig_alpha1_tth_cat0, mjj_hig_n1_tth_cat0, mjj_hig_alpha2_tth_cat0, mjj_hig_n2_tth_cat0);

mjj_hig_m0_vh_cat0[100, 70, 190];
mjj_hig_sigma_vh_cat0[50, 10, 100];
mjj_hig_alpha1_vh_cat0[1.0, 0.01, 10];
mjj_hig_n1_vh_cat0[1, 0.01, 10];
mjj_hig_alpha2_vh_cat0[1.0, 0.01, 10];
mjj_hig_n2_vh_cat0[1, 0.01, 10];
mjjHig_vh_cat0 = RooDoubleCB(mjj, mjj_hig_m0_vh_cat0, mjj_hig_sigma_vh_cat0, mjj_hig_alpha1_vh_cat0, mjj_hig_n1_vh_cat0, mjj_hig_alpha2_vh_cat0, mjj_hig_n2_vh_cat0);

mjj_hig_m0_bbh_cat0[100, 10, 180];
mjj_hig_sigma_bbh_cat0[50, 1.0, 100];
mjj_hig_alpha1_bbh_cat0[1.0, 0.01, 10];
mjj_hig_n1_bbh_cat0[1, 0.01, 10];
mjj_hig_alpha2_bbh_cat0[1.0, 0.01, 10];
mjj_hig_n2_bbh_cat0[1, 0.01, 10];
mjjHig_bbh_cat0 = RooDoubleCB(mjj, mjj_hig_m0_bbh_cat0, mjj_hig_sigma_bbh_cat0, mjj_hig_alpha1_bbh_cat0, mjj_hig_n1_bbh_cat0, mjj_hig_alpha2_bbh_cat0, mjj_hig_n2_bbh_cat0);
mgg_sig_m0_cat1[125., 122, 127];
mgg_sig_sigma_cat1[1.0, 0.1, 3.0];
mgg_sig_alpha1_cat1[1.0, 0.05, 10.0];
mgg_sig_n1_cat1[2.0, 0.1, 10.0];
mgg_sig_alpha2_cat1[1.0, 0.05, 10.0];
mgg_sig_n2_cat1[2.0, 0.1, 10.0];
mggSig_cat1 = RooDoubleCB(mgg, mgg_sig_m0_cat1, mgg_sig_sigma_cat1, mgg_sig_alpha1_cat1, mgg_sig_n1_cat1, mgg_sig_alpha2_cat1, mgg_sig_n2_cat1);

mgg_sigVBF_m0_cat1[125., 122, 127];
mgg_sigVBF_sigma_cat1[1.0, 0.1, 3.0];
mgg_sigVBF_alpha1_cat1[1.0, 0.05, 10.0];
mgg_sigVBF_n1_cat1[2.0, 0.1, 10.0];
mgg_sigVBF_alpha2_cat1[1.0, 0.05, 10.0];
mgg_sigVBF_n2_cat1[2.0, 0.1, 10.0];
mggSigVBF_cat1 = RooDoubleCB(mgg, mgg_sigVBF_m0_cat1, mgg_sigVBF_sigma_cat1, mgg_sigVBF_alpha1_cat1, mgg_sigVBF_n1_cat1, mgg_sigVBF_alpha2_cat1, mgg_sigVBF_n2_cat1);

mgg_hig_m0_ggh_cat1[124.2, 123, 125];
mgg_hig_sigma_ggh_cat1[2.0, 0.1, 3.0];
mgg_hig_alpha1_ggh_cat1[1.0, 0.05, 10.0];
mgg_hig_n1_ggh_cat1[2.0, 0.1, 10.0];
mgg_hig_alpha2_ggh_cat1[1.0, 0.05, 10.0];
mgg_hig_n2_ggh_cat1[2.0, 0.1, 10.0];
mggHig_ggh_cat1 = RooDoubleCB(mgg, mgg_hig_m0_ggh_cat1, mgg_hig_sigma_ggh_cat1, mgg_hig_alpha1_ggh_cat1, mgg_hig_n1_ggh_cat1, mgg_hig_alpha2_ggh_cat1, mgg_hig_n2_ggh_cat1);

mgg_hig_m0_tth_cat1[124.2, 123, 125];
mgg_hig_sigma_tth_cat1[2.0, 0.1, 3.0];
mgg_hig_alpha1_tth_cat1[1.0, 0.05, 10.0];
mgg_hig_n1_tth_cat1[2.0, 0.1, 10.0];
mgg_hig_alpha2_tth_cat1[1.0, 0.05, 10.0];
mgg_hig_n2_tth_cat1[2.0, 0.1, 10.0];
mggHig_tth_cat1 = RooDoubleCB(mgg, mgg_hig_m0_tth_cat1, mgg_hig_sigma_tth_cat1, mgg_hig_alpha1_tth_cat1, mgg_hig_n1_tth_cat1, mgg_hig_alpha2_tth_cat1, mgg_hig_n2_tth_cat1);

mgg_hig_m0_vh_cat1[124.2, 123, 125];
mgg_hig_sigma_vh_cat1[2.0, 0.1, 3.0];
mgg_hig_alpha1_vh_cat1[1.0, 0.05, 10.0];
mgg_hig_n1_vh_cat1[2.0, 0.1, 10.0];
mgg_hig_alpha2_vh_cat1[1.0, 0.05, 10.0];
mgg_hig_n2_vh_cat1[2.0, 0.1, 10.0];
mggHig_vh_cat1 = RooDoubleCB(mgg, mgg_hig_m0_vh_cat1, mgg_hig_sigma_vh_cat1, mgg_hig_alpha1_vh_cat1, mgg_hig_n1_vh_cat1, mgg_hig_alpha2_vh_cat1, mgg_hig_n2_vh_cat1);

mgg_hig_m0_vbf_cat1[124.2, 123, 125];
mgg_hig_sigma_vbf_cat1[2.0, 0.1, 3.0];
mgg_hig_alpha1_vbf_cat1[1.0, 0.05, 10.0];
mgg_hig_n1_vbf_cat1[2.0, 0.1, 10.0];
mgg_hig_alpha2_vbf_cat1[1.0, 0.05, 10.0];
mgg_hig_n2_vbf_cat1[2.0, 0.1, 10.0];
mggHig_vbf_cat1 = RooDoubleCB(mgg, mgg_hig_m0_vbf_cat1, mgg_hig_sigma_vbf_cat1, mgg_hig_alpha1_vbf_cat1, mgg_hig_n1_vbf_cat1, mgg_hig_alpha2_vbf_cat1, mgg_hig_n2_vbf_cat1);

mgg_hig_m0_bbh_cat1[124.2, 123, 125];
mgg_hig_sigma_bbh_cat1[2.0, 0.1, 3.0];
mgg_hig_alpha1_bbh_cat1[1.0, 0.05, 10.0];
mgg_hig_n1_bbh_cat1[2.0, 0.1, 10.0];
mgg_hig_alpha2_bbh_cat1[1.0, 0.05, 10.0];
mgg_hig_n2_bbh_cat1[2.0, 0.1, 10.0];
mggHig_bbh_cat1 = RooDoubleCB(mgg, mgg_hig_m0_bbh_cat1, mgg_hig_sigma_bbh_cat1, mgg_hig_alpha1_bbh_cat1, mgg_hig_n1_bbh_cat1, mgg_hig_alpha2_bbh_cat1, mgg_hig_n2_bbh_cat1);


mjj_sig_m0_cat1[110.0, 99, 140];
mjj_sig_sigma_cat1[10.0, 1.0, 60.0];
mjj_sig_alpha1_cat1[1.0, 0.05, 10.0];
mjj_sig_n1_cat1[2.0, 0.1, 10.0];
mjj_sig_alpha2_cat1[1.0, 0.05, 10.0];
mjj_sig_n2_cat1[2.0, 0.1, 5.0];
mjjSig_cat1 = RooDoubleCB(mjj, mjj_sig_m0_cat1, mjj_sig_sigma_cat1, mjj_sig_alpha1_cat1, mjj_sig_n1_cat1, mjj_sig_alpha2_cat1, mjj_sig_n2_cat1);

mjj_sigVBF_m0_cat1[110.0, 99, 140];
mjj_sigVBF_sigma_cat1[10.0, 1.0, 60.0];
mjj_sigVBF_alpha1_cat1[1.0, 0.05, 10.0];
mjj_sigVBF_n1_cat1[2.0, 0.1, 10.0];
mjj_sigVBF_alpha2_cat1[1.0, 0.05, 10.0];
mjj_sigVBF_n2_cat1[2.0, 0.1, 5.0];
mjjSigVBF_cat1 = RooDoubleCB(mjj, mjj_sigVBF_m0_cat1, mjj_sigVBF_sigma_cat1, mjj_sigVBF_alpha1_cat1, mjj_sigVBF_n1_cat1, mjj_sigVBF_alpha2_cat1, mjj_sigVBF_n2_cat1);

mjj_hig_par1_ggh_cat1[0.1, 0, 10];
mjj_hig_par2_ggh_cat1[0.1, 0, 10];
mjj_hig_par3_ggh_cat1[0.1, 0, 10];

mjj_hig_par1_vbf_cat1[0.1, 0, 10];
mjj_hig_par2_vbf_cat1[0.1, 0, 10];
mjj_hig_par3_vbf_cat1[0.1, 0, 10];

mjj_hig_m0_tth_cat1[100, 70, 190];
mjj_hig_sigma_tth_cat1[50, 10, 100];
mjj_hig_alpha1_tth_cat1[1.0, 0.01, 10];
mjj_hig_n1_tth_cat1[1, 0.01, 10];
mjj_hig_alpha2_tth_cat1[1.0, 0.01, 10];
mjj_hig_n2_tth_cat1[1, 0.01, 10];
mjjHig_tth_cat1 = RooDoubleCB(mjj, mjj_hig_m0_tth_cat1, mjj_hig_sigma_tth_cat1, mjj_hig_alpha1_tth_cat1, mjj_hig_n1_tth_cat1, mjj_hig_alpha2_tth_cat1, mjj_hig_n2_tth_cat1);

mjj_hig_m0_vh_cat1[100, 70, 190];
mjj_hig_sigma_vh_cat1[50, 10, 100];
mjj_hig_alpha1_vh_cat1[1.0, 0.01, 10];
mjj_hig_n1_vh_cat1[1, 0.01, 10];
mjj_hig_alpha2_vh_cat1[1.0, 0.01, 10];
mjj_hig_n2_vh_cat1[1, 0.01, 10];
mjjHig_vh_cat1 = RooDoubleCB(mjj, mjj_hig_m0_vh_cat1, mjj_hig_sigma_vh_cat1, mjj_hig_alpha1_vh_cat1, mjj_hig_n1_vh_cat1, mjj_hig_alpha2_vh_cat1, mjj_hig_n2_vh_cat1);

mjj_hig_m0_bbh_cat1[100, 10, 180];
mjj_hig_sigma_bbh_cat1[50, 1.0, 100];
mjj_hig_alpha1_bbh_cat1[1.0, 0.01, 10];
mjj_hig_n1_bbh_cat1[1, 0.01, 10];
mjj_hig_alpha2_bbh_cat1[1.0, 0.01, 10];
mjj_hig_n2_bbh_cat1[1, 0.01, 10];
mjjHig_bbh_cat1 = RooDoubleCB(mjj, mjj_hig_m0_bbh_cat1, mjj_hig_sigma_bbh_cat1, mjj_hig_alpha1_bbh_cat1, mjj_hig_n1_bbh_cat1, mjj_hig_alpha2_bbh_cat1, mjj_hig_n2_bbh_cat1);
mgg_sig_m0_cat10[125., 122, 127];
mgg_sig_sigma_cat10[1.0, 0.1, 3.0];
mgg_sig_alpha1_cat10[1.0, 0.05, 10.0];
mgg_sig_n1_cat10[2.0, 0.1, 10.0];
mgg_sig_alpha2_cat10[1.0, 0.05, 10.0];
mgg_sig_n2_cat10[2.0, 0.1, 10.0];
mggSig_cat10 = RooDoubleCB(mgg, mgg_sig_m0_cat10, mgg_sig_sigma_cat10, mgg_sig_alpha1_cat10, mgg_sig_n1_cat10, mgg_sig_alpha2_cat10, mgg_sig_n2_cat10);

mgg_sigVBF_m0_cat10[125., 122, 127];
mgg_sigVBF_sigma_cat10[1.0, 0.1, 3.0];
mgg_sigVBF_alpha1_cat10[1.0, 0.05, 10.0];
mgg_sigVBF_n1_cat10[2.0, 0.1, 10.0];
mgg_sigVBF_alpha2_cat10[1.0, 0.05, 10.0];
mgg_sigVBF_n2_cat10[2.0, 0.1, 10.0];
mggSigVBF_cat10 = RooDoubleCB(mgg, mgg_sigVBF_m0_cat10, mgg_sigVBF_sigma_cat10, mgg_sigVBF_alpha1_cat10, mgg_sigVBF_n1_cat10, mgg_sigVBF_alpha2_cat10, mgg_sigVBF_n2_cat10);

mgg_hig_m0_ggh_cat10[124.2, 123, 125];
mgg_hig_sigma_ggh_cat10[2.0, 0.1, 3.0];
mgg_hig_alpha1_ggh_cat10[1.0, 0.05, 10.0];
mgg_hig_n1_ggh_cat10[2.0, 0.1, 10.0];
mgg_hig_alpha2_ggh_cat10[1.0, 0.05, 10.0];
mgg_hig_n2_ggh_cat10[2.0, 0.1, 10.0];
mggHig_ggh_cat10 = RooDoubleCB(mgg, mgg_hig_m0_ggh_cat10, mgg_hig_sigma_ggh_cat10, mgg_hig_alpha1_ggh_cat10, mgg_hig_n1_ggh_cat10, mgg_hig_alpha2_ggh_cat10, mgg_hig_n2_ggh_cat10);

mgg_hig_m0_tth_cat10[124.2, 123, 125];
mgg_hig_sigma_tth_cat10[2.0, 0.1, 3.0];
mgg_hig_alpha1_tth_cat10[1.0, 0.05, 10.0];
mgg_hig_n1_tth_cat10[2.0, 0.1, 10.0];
mgg_hig_alpha2_tth_cat10[1.0, 0.05, 10.0];
mgg_hig_n2_tth_cat10[2.0, 0.1, 10.0];
mggHig_tth_cat10 = RooDoubleCB(mgg, mgg_hig_m0_tth_cat10, mgg_hig_sigma_tth_cat10, mgg_hig_alpha1_tth_cat10, mgg_hig_n1_tth_cat10, mgg_hig_alpha2_tth_cat10, mgg_hig_n2_tth_cat10);

mgg_hig_m0_vh_cat10[124.2, 123, 125];
mgg_hig_sigma_vh_cat10[2.0, 0.1, 3.0];
mgg_hig_alpha1_vh_cat10[1.0, 0.05, 10.0];
mgg_hig_n1_vh_cat10[2.0, 0.1, 10.0];
mgg_hig_alpha2_vh_cat10[1.0, 0.05, 10.0];
mgg_hig_n2_vh_cat10[2.0, 0.1, 10.0];
mggHig_vh_cat10 = RooDoubleCB(mgg, mgg_hig_m0_vh_cat10, mgg_hig_sigma_vh_cat10, mgg_hig_alpha1_vh_cat10, mgg_hig_n1_vh_cat10, mgg_hig_alpha2_vh_cat10, mgg_hig_n2_vh_cat10);

mgg_hig_m0_vbf_cat10[124.2, 123, 125];
mgg_hig_sigma_vbf_cat10[2.0, 0.1, 3.0];
mgg_hig_alpha1_vbf_cat10[1.0, 0.05, 10.0];
mgg_hig_n1_vbf_cat10[2.0, 0.1, 10.0];
mgg_hig_alpha2_vbf_cat10[1.0, 0.05, 10.0];
mgg_hig_n2_vbf_cat10[2.0, 0.1, 10.0];
mggHig_vbf_cat10 = RooDoubleCB(mgg, mgg_hig_m0_vbf_cat10, mgg_hig_sigma_vbf_cat10, mgg_hig_alpha1_vbf_cat10, mgg_hig_n1_vbf_cat10, mgg_hig_alpha2_vbf_cat10, mgg_hig_n2_vbf_cat10);

mgg_hig_m0_bbh_cat10[124.2, 123, 125];
mgg_hig_sigma_bbh_cat10[2.0, 0.1, 3.0];
mgg_hig_alpha1_bbh_cat10[1.0, 0.05, 10.0];
mgg_hig_n1_bbh_cat10[2.0, 0.1, 10.0];
mgg_hig_alpha2_bbh_cat10[1.0, 0.05, 10.0];
mgg_hig_n2_bbh_cat10[2.0, 0.1, 10.0];
mggHig_bbh_cat10 = RooDoubleCB(mgg, mgg_hig_m0_bbh_cat10, mgg_hig_sigma_bbh_cat10, mgg_hig_alpha1_bbh_cat10, mgg_hig_n1_bbh_cat10, mgg_hig_alpha2_bbh_cat10, mgg_hig_n2_bbh_cat10);


mjj_sig_m0_cat10[110.0, 99, 140];
mjj_sig_sigma_cat10[10.0, 1.0, 60.0];
mjj_sig_alpha1_cat10[1.0, 0.05, 10.0];
mjj_sig_n1_cat10[2.0, 0.1, 10.0];
mjj_sig_alpha2_cat10[1.0, 0.05, 10.0];
mjj_sig_n2_cat10[2.0, 0.1, 10.0];
mjjSig_cat10 = RooDoubleCB(mjj, mjj_sig_m0_cat10, mjj_sig_sigma_cat10, mjj_sig_alpha1_cat10, mjj_sig_n1_cat10, mjj_sig_alpha2_cat10, mjj_sig_n2_cat10);

mjj_sigVBF_m0_cat10[110.0, 99, 140];
mjj_sigVBF_sigma_cat10[10.0, 1.0, 60.0];
mjj_sigVBF_alpha1_cat10[1.0, 0.05, 10.0];
mjj_sigVBF_n1_cat10[2.0, 0.1, 10.0];
mjj_sigVBF_alpha2_cat10[1.0, 0.05, 10.0];
mjj_sigVBF_n2_cat10[2.0, 0.1, 10.0];
mjjSigVBF_cat10 = RooDoubleCB(mjj, mjj_sigVBF_m0_cat10, mjj_sigVBF_sigma_cat10, mjj_sigVBF_alpha1_cat10, mjj_sigVBF_n1_cat10, mjj_sigVBF_alpha2_cat10, mjj_sigVBF_n2_cat10);

mjj_hig_par1_ggh_cat10[0.1, 0, 10];
mjj_hig_par2_ggh_cat10[0.1, 0, 10];
mjj_hig_par3_ggh_cat10[0.1, 0, 10];

mjj_hig_par1_vbf_cat10[0.1, 0, 10];
mjj_hig_par2_vbf_cat10[0.1, 0, 10];
mjj_hig_par3_vbf_cat10[0.1, 0, 10];

mjj_hig_m0_tth_cat10[100, 70, 190];
mjj_hig_sigma_tth_cat10[50, 10, 100];
mjj_hig_alpha1_tth_cat10[1.0, 0.01, 10];
mjj_hig_n1_tth_cat10[1, 0.01, 10];
mjj_hig_alpha2_tth_cat10[1.0, 0.01, 10];
mjj_hig_n2_tth_cat10[1, 0.01, 10];
mjjHig_tth_cat10 = RooDoubleCB(mjj, mjj_hig_m0_tth_cat10, mjj_hig_sigma_tth_cat10, mjj_hig_alpha1_tth_cat10, mjj_hig_n1_tth_cat10, mjj_hig_alpha2_tth_cat10, mjj_hig_n2_tth_cat10);

mjj_hig_m0_vh_cat10[100, 70, 190];
mjj_hig_sigma_vh_cat10[50, 10, 100];
mjj_hig_alpha1_vh_cat10[1.0, 0.01, 10];
mjj_hig_n1_vh_cat10[1, 0.01, 10];
mjj_hig_alpha2_vh_cat10[1.0, 0.01, 10];
mjj_hig_n2_vh_cat10[1, 0.01, 10];
mjjHig_vh_cat10 = RooDoubleCB(mjj, mjj_hig_m0_vh_cat10, mjj_hig_sigma_vh_cat10, mjj_hig_alpha1_vh_cat10, mjj_hig_n1_vh_cat10, mjj_hig_alpha2_vh_cat10, mjj_hig_n2_vh_cat10);

mjj_hig_m0_bbh_cat10[100, 10, 180];
mjj_hig_sigma_bbh_cat10[50, 1.0, 100];
mjj_hig_alpha1_bbh_cat10[1.0, 0.01, 10];
mjj_hig_n1_bbh_cat10[1, 0.01, 10];
mjj_hig_alpha2_bbh_cat10[1.0, 0.01, 10];
mjj_hig_n2_bbh_cat10[1, 0.01, 10];
mjjHig_bbh_cat10 = RooDoubleCB(mjj, mjj_hig_m0_bbh_cat10, mjj_hig_sigma_bbh_cat10, mjj_hig_alpha1_bbh_cat10, mjj_hig_n1_bbh_cat10, mjj_hig_alpha2_bbh_cat10, mjj_hig_n2_bbh_cat10);
mgg_sig_m0_cat11[125., 122, 127];
mgg_sig_sigma_cat11[1.0, 0.1, 3.0];
mgg_sig_alpha1_cat11[1.0, 0.05, 10.0];
mgg_sig_n1_cat11[2.0, 0.1, 10.0];
mgg_sig_alpha2_cat11[1.0, 0.05, 10.0];
mgg_sig_n2_cat11[2.0, 0.1, 10.0];
mggSig_cat11 = RooDoubleCB(mgg, mgg_sig_m0_cat11, mgg_sig_sigma_cat11, mgg_sig_alpha1_cat11, mgg_sig_n1_cat11, mgg_sig_alpha2_cat11, mgg_sig_n2_cat11);

mgg_sigVBF_m0_cat11[125., 122, 127];
mgg_sigVBF_sigma_cat11[1.0, 0.1, 3.0];
mgg_sigVBF_alpha1_cat11[1.0, 0.05, 10.0];
mgg_sigVBF_n1_cat11[2.0, 0.1, 10.0];
mgg_sigVBF_alpha2_cat11[1.0, 0.05, 10.0];
mgg_sigVBF_n2_cat11[2.0, 0.1, 10.0];
mggSigVBF_cat11 = RooDoubleCB(mgg, mgg_sigVBF_m0_cat11, mgg_sigVBF_sigma_cat11, mgg_sigVBF_alpha1_cat11, mgg_sigVBF_n1_cat11, mgg_sigVBF_alpha2_cat11, mgg_sigVBF_n2_cat11);

mgg_hig_m0_ggh_cat11[124.2, 123, 125];
mgg_hig_sigma_ggh_cat11[2.0, 0.1, 3.0];
mgg_hig_alpha1_ggh_cat11[1.0, 0.05, 10.0];
mgg_hig_n1_ggh_cat11[2.0, 0.1, 10.0];
mgg_hig_alpha2_ggh_cat11[1.0, 0.05, 10.0];
mgg_hig_n2_ggh_cat11[2.0, 0.1, 10.0];
mggHig_ggh_cat11 = RooDoubleCB(mgg, mgg_hig_m0_ggh_cat11, mgg_hig_sigma_ggh_cat11, mgg_hig_alpha1_ggh_cat11, mgg_hig_n1_ggh_cat11, mgg_hig_alpha2_ggh_cat11, mgg_hig_n2_ggh_cat11);

mgg_hig_m0_tth_cat11[124.2, 123, 125];
mgg_hig_sigma_tth_cat11[2.0, 0.1, 3.0];
mgg_hig_alpha1_tth_cat11[1.0, 0.05, 10.0];
mgg_hig_n1_tth_cat11[2.0, 0.1, 10.0];
mgg_hig_alpha2_tth_cat11[1.0, 0.05, 10.0];
mgg_hig_n2_tth_cat11[2.0, 0.1, 10.0];
mggHig_tth_cat11 = RooDoubleCB(mgg, mgg_hig_m0_tth_cat11, mgg_hig_sigma_tth_cat11, mgg_hig_alpha1_tth_cat11, mgg_hig_n1_tth_cat11, mgg_hig_alpha2_tth_cat11, mgg_hig_n2_tth_cat11);

mgg_hig_m0_vh_cat11[124.2, 123, 125];
mgg_hig_sigma_vh_cat11[2.0, 0.1, 3.0];
mgg_hig_alpha1_vh_cat11[1.0, 0.05, 10.0];
mgg_hig_n1_vh_cat11[2.0, 0.1, 10.0];
mgg_hig_alpha2_vh_cat11[1.0, 0.05, 10.0];
mgg_hig_n2_vh_cat11[2.0, 0.1, 10.0];
mggHig_vh_cat11 = RooDoubleCB(mgg, mgg_hig_m0_vh_cat11, mgg_hig_sigma_vh_cat11, mgg_hig_alpha1_vh_cat11, mgg_hig_n1_vh_cat11, mgg_hig_alpha2_vh_cat11, mgg_hig_n2_vh_cat11);

mgg_hig_m0_vbf_cat11[124.2, 123, 125];
mgg_hig_sigma_vbf_cat11[2.0, 0.1, 3.0];
mgg_hig_alpha1_vbf_cat11[1.0, 0.05, 10.0];
mgg_hig_n1_vbf_cat11[2.0, 0.1, 10.0];
mgg_hig_alpha2_vbf_cat11[1.0, 0.05, 10.0];
mgg_hig_n2_vbf_cat11[2.0, 0.1, 10.0];
mggHig_vbf_cat11 = RooDoubleCB(mgg, mgg_hig_m0_vbf_cat11, mgg_hig_sigma_vbf_cat11, mgg_hig_alpha1_vbf_cat11, mgg_hig_n1_vbf_cat11, mgg_hig_alpha2_vbf_cat11, mgg_hig_n2_vbf_cat11);

mgg_hig_m0_bbh_cat11[124.2, 123, 125];
mgg_hig_sigma_bbh_cat11[2.0, 0.1, 3.0];
mgg_hig_alpha1_bbh_cat11[1.0, 0.05, 10.0];
mgg_hig_n1_bbh_cat11[2.0, 0.1, 10.0];
mgg_hig_alpha2_bbh_cat11[1.0, 0.05, 10.0];
mgg_hig_n2_bbh_cat11[2.0, 0.1, 10.0];
mggHig_bbh_cat11 = RooDoubleCB(mgg, mgg_hig_m0_bbh_cat11, mgg_hig_sigma_bbh_cat11, mgg_hig_alpha1_bbh_cat11, mgg_hig_n1_bbh_cat11, mgg_hig_alpha2_bbh_cat11, mgg_hig_n2_bbh_cat11);


mjj_sig_m0_cat11[110.0, 99, 140];
mjj_sig_sigma_cat11[10.0, 1.0, 60.0];
mjj_sig_alpha1_cat11[1.0, 0.05, 10.0];
mjj_sig_n1_cat11[2.0, 0.1, 10.0];
mjj_sig_alpha2_cat11[1.0, 0.05, 10.0];
mjj_sig_n2_cat11[2.0, 0.1, 10.0];
mjjSig_cat11 = RooDoubleCB(mjj, mjj_sig_m0_cat11, mjj_sig_sigma_cat11, mjj_sig_alpha1_cat11, mjj_sig_n1_cat11, mjj_sig_alpha2_cat11, mjj_sig_n2_cat11);

mjj_sigVBF_m0_cat11[110.0, 99, 140];
mjj_sigVBF_sigma_cat11[10.0, 1.0, 60.0];
mjj_sigVBF_alpha1_cat11[1.0, 0.05, 10.0];
mjj_sigVBF_n1_cat11[2.0, 0.1, 10.0];
mjj_sigVBF_alpha2_cat11[1.0, 0.05, 10.0];
mjj_sigVBF_n2_cat11[2.0, 0.1, 10.0];
mjjSigVBF_cat11 = RooDoubleCB(mjj, mjj_sigVBF_m0_cat11, mjj_sigVBF_sigma_cat11, mjj_sigVBF_alpha1_cat11, mjj_sigVBF_n1_cat11, mjj_sigVBF_alpha2_cat11, mjj_sigVBF_n2_cat11);

mjj_hig_par1_ggh_cat11[0.1, 0, 10];
mjj_hig_par2_ggh_cat11[0.1, 0, 10];
mjj_hig_par3_ggh_cat11[0.1, 0, 10];

mjj_hig_par1_vbf_cat11[0.1, 0, 10];
mjj_hig_par2_vbf_cat11[0.1, 0, 10];
mjj_hig_par3_vbf_cat11[0.1, 0, 10];

mjj_hig_m0_tth_cat11[100, 70, 190];
mjj_hig_sigma_tth_cat11[50, 10, 100];
mjj_hig_alpha1_tth_cat11[1.0, 0.01, 10];
mjj_hig_n1_tth_cat11[1, 0.01, 10];
mjj_hig_alpha2_tth_cat11[1.0, 0.01, 10];
mjj_hig_n2_tth_cat11[1, 0.01, 10];
mjjHig_tth_cat11 = RooDoubleCB(mjj, mjj_hig_m0_tth_cat11, mjj_hig_sigma_tth_cat11, mjj_hig_alpha1_tth_cat11, mjj_hig_n1_tth_cat11, mjj_hig_alpha2_tth_cat11, mjj_hig_n2_tth_cat11);

mjj_hig_m0_vh_cat11[100, 70, 190];
mjj_hig_sigma_vh_cat11[50, 10, 100];
mjj_hig_alpha1_vh_cat11[1.0, 0.01, 10];
mjj_hig_n1_vh_cat11[1, 0.01, 10];
mjj_hig_alpha2_vh_cat11[1.0, 0.01, 10];
mjj_hig_n2_vh_cat11[1, 0.01, 10];
mjjHig_vh_cat11 = RooDoubleCB(mjj, mjj_hig_m0_vh_cat11, mjj_hig_sigma_vh_cat11, mjj_hig_alpha1_vh_cat11, mjj_hig_n1_vh_cat11, mjj_hig_alpha2_vh_cat11, mjj_hig_n2_vh_cat11);

mjj_hig_m0_bbh_cat11[100, 10, 180];
mjj_hig_sigma_bbh_cat11[50, 1.0, 100];
mjj_hig_alpha1_bbh_cat11[1.0, 0.01, 10];
mjj_hig_n1_bbh_cat11[1, 0.01, 10];
mjj_hig_alpha2_bbh_cat11[1.0, 0.01, 10];
mjj_hig_n2_bbh_cat11[1, 0.01, 10];
mjjHig_bbh_cat11 = RooDoubleCB(mjj, mjj_hig_m0_bbh_cat11, mjj_hig_sigma_bbh_cat11, mjj_hig_alpha1_bbh_cat11, mjj_hig_n1_bbh_cat11, mjj_hig_alpha2_bbh_cat11, mjj_hig_n2_bbh_cat11);
mgg_sig_m0_cat12[125., 122, 127];
mgg_sig_sigma_cat12[1.0, 0.1, 3.0];
mgg_sig_alpha1_cat12[1.0, 0.05, 10.0];
mgg_sig_n1_cat12[2.0, 0.1, 10.0];
mgg_sig_alpha2_cat12[1.0, 0.05, 10.0];
mgg_sig_n2_cat12[2.0, 0.1, 10.0];
mggSig_cat12 = RooDoubleCB(mgg, mgg_sig_m0_cat12, mgg_sig_sigma_cat12, mgg_sig_alpha1_cat12, mgg_sig_n1_cat12, mgg_sig_alpha2_cat12, mgg_sig_n2_cat12);

mgg_sigVBF_m0_cat12[125., 122, 127];
mgg_sigVBF_sigma_cat12[1.0, 0.1, 3.0];
mgg_sigVBF_alpha1_cat12[1.0, 0.05, 10.0];
mgg_sigVBF_n1_cat12[2.0, 0.1, 10.0];
mgg_sigVBF_alpha2_cat12[1.0, 0.05, 10.0];
mgg_sigVBF_n2_cat12[2.0, 0.1, 10.0];
mggSigVBF_cat12 = RooDoubleCB(mgg, mgg_sigVBF_m0_cat12, mgg_sigVBF_sigma_cat12, mgg_sigVBF_alpha1_cat12, mgg_sigVBF_n1_cat12, mgg_sigVBF_alpha2_cat12, mgg_sigVBF_n2_cat12);

mgg_hig_m0_ggh_cat12[124.2, 123, 125];
mgg_hig_sigma_ggh_cat12[2.0, 0.1, 3.0];
mgg_hig_alpha1_ggh_cat12[1.0, 0.05, 10.0];
mgg_hig_n1_ggh_cat12[2.0, 0.1, 10.0];
mgg_hig_alpha2_ggh_cat12[1.0, 0.05, 10.0];
mgg_hig_n2_ggh_cat12[2.0, 0.1, 10.0];
mggHig_ggh_cat12 = RooDoubleCB(mgg, mgg_hig_m0_ggh_cat12, mgg_hig_sigma_ggh_cat12, mgg_hig_alpha1_ggh_cat12, mgg_hig_n1_ggh_cat12, mgg_hig_alpha2_ggh_cat12, mgg_hig_n2_ggh_cat12);

mgg_hig_m0_tth_cat12[124.2, 123, 125];
mgg_hig_sigma_tth_cat12[2.0, 0.1, 3.0];
mgg_hig_alpha1_tth_cat12[1.0, 0.05, 10.0];
mgg_hig_n1_tth_cat12[2.0, 0.1, 10.0];
mgg_hig_alpha2_tth_cat12[1.0, 0.05, 10.0];
mgg_hig_n2_tth_cat12[2.0, 0.1, 10.0];
mggHig_tth_cat12 = RooDoubleCB(mgg, mgg_hig_m0_tth_cat12, mgg_hig_sigma_tth_cat12, mgg_hig_alpha1_tth_cat12, mgg_hig_n1_tth_cat12, mgg_hig_alpha2_tth_cat12, mgg_hig_n2_tth_cat12);

mgg_hig_m0_vh_cat12[124.2, 123, 125];
mgg_hig_sigma_vh_cat12[2.0, 0.1, 3.0];
mgg_hig_alpha1_vh_cat12[1.0, 0.05, 10.0];
mgg_hig_n1_vh_cat12[2.0, 0.1, 10.0];
mgg_hig_alpha2_vh_cat12[1.0, 0.05, 10.0];
mgg_hig_n2_vh_cat12[2.0, 0.1, 10.0];
mggHig_vh_cat12 = RooDoubleCB(mgg, mgg_hig_m0_vh_cat12, mgg_hig_sigma_vh_cat12, mgg_hig_alpha1_vh_cat12, mgg_hig_n1_vh_cat12, mgg_hig_alpha2_vh_cat12, mgg_hig_n2_vh_cat12);

mgg_hig_m0_vbf_cat12[124.2, 123, 125];
mgg_hig_sigma_vbf_cat12[2.0, 0.1, 3.0];
mgg_hig_alpha1_vbf_cat12[1.0, 0.05, 10.0];
mgg_hig_n1_vbf_cat12[2.0, 0.1, 10.0];
mgg_hig_alpha2_vbf_cat12[1.0, 0.05, 10.0];
mgg_hig_n2_vbf_cat12[2.0, 0.1, 10.0];
mggHig_vbf_cat12 = RooDoubleCB(mgg, mgg_hig_m0_vbf_cat12, mgg_hig_sigma_vbf_cat12, mgg_hig_alpha1_vbf_cat12, mgg_hig_n1_vbf_cat12, mgg_hig_alpha2_vbf_cat12, mgg_hig_n2_vbf_cat12);

mgg_hig_m0_bbh_cat12[124.2, 123, 125];
mgg_hig_sigma_bbh_cat12[2.0, 0.1, 3.0];
mgg_hig_alpha1_bbh_cat12[1.0, 0.05, 10.0];
mgg_hig_n1_bbh_cat12[2.0, 0.1, 10.0];
mgg_hig_alpha2_bbh_cat12[1.0, 0.05, 10.0];
mgg_hig_n2_bbh_cat12[2.0, 0.1, 10.0];
mggHig_bbh_cat12 = RooDoubleCB(mgg, mgg_hig_m0_bbh_cat12, mgg_hig_sigma_bbh_cat12, mgg_hig_alpha1_bbh_cat12, mgg_hig_n1_bbh_cat12, mgg_hig_alpha2_bbh_cat12, mgg_hig_n2_bbh_cat12);


mjj_sig_m0_cat12[110.0, 99, 140];
mjj_sig_sigma_cat12[10.0, 1.0, 60.0];
mjj_sig_alpha1_cat12[1.0, 0.05, 10.0];
mjj_sig_n1_cat12[2.0, 0.1, 10.0];
mjj_sig_alpha2_cat12[1.0, 0.05, 10.0];
mjj_sig_n2_cat12[2.0, 0.1, 10.0];
mjjSig_cat12 = RooDoubleCB(mjj, mjj_sig_m0_cat12, mjj_sig_sigma_cat12, mjj_sig_alpha1_cat12, mjj_sig_n1_cat12, mjj_sig_alpha2_cat12, mjj_sig_n2_cat12);

mjj_sigVBF_m0_cat12[110.0, 99, 140];
mjj_sigVBF_sigma_cat12[10.0, 1.0, 60.0];
mjj_sigVBF_alpha1_cat12[1.0, 0.05, 10.0];
mjj_sigVBF_n1_cat12[2.0, 0.1, 10.0];
mjj_sigVBF_alpha2_cat12[1.0, 0.05, 10.0];
mjj_sigVBF_n2_cat12[2.0, 0.1, 10.0];
mjjSigVBF_cat12 = RooDoubleCB(mjj, mjj_sigVBF_m0_cat12, mjj_sigVBF_sigma_cat12, mjj_sigVBF_alpha1_cat12, mjj_sigVBF_n1_cat12, mjj_sigVBF_alpha2_cat12, mjj_sigVBF_n2_cat12);

mjj_hig_par1_ggh_cat12[0.1, 0, 10];
mjj_hig_par2_ggh_cat12[0.1, 0, 10];
mjj_hig_par3_ggh_cat12[0.1, 0, 10];

mjj_hig_par1_vbf_cat12[0.1, 0, 10];
mjj_hig_par2_vbf_cat12[0.1, 0, 10];
mjj_hig_par3_vbf_cat12[0.1, 0, 10];

mjj_hig_m0_tth_cat12[100, 70, 190];
mjj_hig_sigma_tth_cat12[50, 10, 100];
mjj_hig_alpha1_tth_cat12[1.0, 0.01, 10];
mjj_hig_n1_tth_cat12[1, 0.01, 10];
mjj_hig_alpha2_tth_cat12[1.0, 0.01, 10];
mjj_hig_n2_tth_cat12[1, 0.01, 10];
mjjHig_tth_cat12 = RooDoubleCB(mjj, mjj_hig_m0_tth_cat12, mjj_hig_sigma_tth_cat12, mjj_hig_alpha1_tth_cat12, mjj_hig_n1_tth_cat12, mjj_hig_alpha2_tth_cat12, mjj_hig_n2_tth_cat12);

mjj_hig_m0_vh_cat12[100, 70, 190];
mjj_hig_sigma_vh_cat12[50, 10, 100];
mjj_hig_alpha1_vh_cat12[1.0, 0.01, 10];
mjj_hig_n1_vh_cat12[1, 0.01, 10];
mjj_hig_alpha2_vh_cat12[1.0, 0.01, 10];
mjj_hig_n2_vh_cat12[1, 0.01, 10];
mjjHig_vh_cat12 = RooDoubleCB(mjj, mjj_hig_m0_vh_cat12, mjj_hig_sigma_vh_cat12, mjj_hig_alpha1_vh_cat12, mjj_hig_n1_vh_cat12, mjj_hig_alpha2_vh_cat12, mjj_hig_n2_vh_cat12);

mjj_hig_m0_bbh_cat12[100, 10, 180];
mjj_hig_sigma_bbh_cat12[50, 1.0, 100];
mjj_hig_alpha1_bbh_cat12[1.0, 0.01, 10];
mjj_hig_n1_bbh_cat12[1, 0.01, 10];
mjj_hig_alpha2_bbh_cat12[1.0, 0.01, 10];
mjj_hig_n2_bbh_cat12[1, 0.01, 10];
mjjHig_bbh_cat12 = RooDoubleCB(mjj, mjj_hig_m0_bbh_cat12, mjj_hig_sigma_bbh_cat12, mjj_hig_alpha1_bbh_cat12, mjj_hig_n1_bbh_cat12, mjj_hig_alpha2_bbh_cat12, mjj_hig_n2_bbh_cat12);
mgg_sig_m0_cat13[125., 122, 127];
mgg_sig_sigma_cat13[1.0, 0.1, 3.0];
mgg_sig_alpha1_cat13[1.0, 0.05, 10.0];
mgg_sig_n1_cat13[2.0, 0.1, 10.0];
mgg_sig_alpha2_cat13[1.0, 0.05, 10.0];
mgg_sig_n2_cat13[2.0, 0.1, 10.0];
mggSig_cat13 = RooDoubleCB(mgg, mgg_sig_m0_cat13, mgg_sig_sigma_cat13, mgg_sig_alpha1_cat13, mgg_sig_n1_cat13, mgg_sig_alpha2_cat13, mgg_sig_n2_cat13);

mgg_sigVBF_m0_cat13[125., 122, 127];
mgg_sigVBF_sigma_cat13[1.0, 0.1, 3.0];
mgg_sigVBF_alpha1_cat13[1.0, 0.05, 10.0];
mgg_sigVBF_n1_cat13[2.0, 0.1, 10.0];
mgg_sigVBF_alpha2_cat13[1.0, 0.05, 10.0];
mgg_sigVBF_n2_cat13[2.0, 0.1, 10.0];
mggSigVBF_cat13 = RooDoubleCB(mgg, mgg_sigVBF_m0_cat13, mgg_sigVBF_sigma_cat13, mgg_sigVBF_alpha1_cat13, mgg_sigVBF_n1_cat13, mgg_sigVBF_alpha2_cat13, mgg_sigVBF_n2_cat13);

mgg_hig_m0_ggh_cat13[124.2, 123, 125];
mgg_hig_sigma_ggh_cat13[2.0, 0.1, 3.0];
mgg_hig_alpha1_ggh_cat13[1.0, 0.05, 10.0];
mgg_hig_n1_ggh_cat13[2.0, 0.1, 10.0];
mgg_hig_alpha2_ggh_cat13[1.0, 0.05, 10.0];
mgg_hig_n2_ggh_cat13[2.0, 0.1, 10.0];
mggHig_ggh_cat13 = RooDoubleCB(mgg, mgg_hig_m0_ggh_cat13, mgg_hig_sigma_ggh_cat13, mgg_hig_alpha1_ggh_cat13, mgg_hig_n1_ggh_cat13, mgg_hig_alpha2_ggh_cat13, mgg_hig_n2_ggh_cat13);

mgg_hig_m0_tth_cat13[124.2, 123, 125];
mgg_hig_sigma_tth_cat13[2.0, 0.1, 3.0];
mgg_hig_alpha1_tth_cat13[1.0, 0.05, 10.0];
mgg_hig_n1_tth_cat13[2.0, 0.1, 10.0];
mgg_hig_alpha2_tth_cat13[1.0, 0.05, 10.0];
mgg_hig_n2_tth_cat13[2.0, 0.1, 10.0];
mggHig_tth_cat13 = RooDoubleCB(mgg, mgg_hig_m0_tth_cat13, mgg_hig_sigma_tth_cat13, mgg_hig_alpha1_tth_cat13, mgg_hig_n1_tth_cat13, mgg_hig_alpha2_tth_cat13, mgg_hig_n2_tth_cat13);

mgg_hig_m0_vh_cat13[124.2, 123, 125];
mgg_hig_sigma_vh_cat13[2.0, 0.1, 3.0];
mgg_hig_alpha1_vh_cat13[1.0, 0.05, 10.0];
mgg_hig_n1_vh_cat13[2.0, 0.1, 10.0];
mgg_hig_alpha2_vh_cat13[1.0, 0.05, 10.0];
mgg_hig_n2_vh_cat13[2.0, 0.1, 10.0];
mggHig_vh_cat13 = RooDoubleCB(mgg, mgg_hig_m0_vh_cat13, mgg_hig_sigma_vh_cat13, mgg_hig_alpha1_vh_cat13, mgg_hig_n1_vh_cat13, mgg_hig_alpha2_vh_cat13, mgg_hig_n2_vh_cat13);

mgg_hig_m0_vbf_cat13[124.2, 123, 125];
mgg_hig_sigma_vbf_cat13[2.0, 0.1, 3.0];
mgg_hig_alpha1_vbf_cat13[1.0, 0.05, 10.0];
mgg_hig_n1_vbf_cat13[2.0, 0.1, 10.0];
mgg_hig_alpha2_vbf_cat13[1.0, 0.05, 10.0];
mgg_hig_n2_vbf_cat13[2.0, 0.1, 10.0];
mggHig_vbf_cat13 = RooDoubleCB(mgg, mgg_hig_m0_vbf_cat13, mgg_hig_sigma_vbf_cat13, mgg_hig_alpha1_vbf_cat13, mgg_hig_n1_vbf_cat13, mgg_hig_alpha2_vbf_cat13, mgg_hig_n2_vbf_cat13);

mgg_hig_m0_bbh_cat13[124.2, 123, 125];
mgg_hig_sigma_bbh_cat13[2.0, 0.1, 3.0];
mgg_hig_alpha1_bbh_cat13[1.0, 0.05, 10.0];
mgg_hig_n1_bbh_cat13[2.0, 0.1, 10.0];
mgg_hig_alpha2_bbh_cat13[1.0, 0.05, 10.0];
mgg_hig_n2_bbh_cat13[2.0, 0.1, 10.0];
mggHig_bbh_cat13 = RooDoubleCB(mgg, mgg_hig_m0_bbh_cat13, mgg_hig_sigma_bbh_cat13, mgg_hig_alpha1_bbh_cat13, mgg_hig_n1_bbh_cat13, mgg_hig_alpha2_bbh_cat13, mgg_hig_n2_bbh_cat13);


mjj_sig_m0_cat13[110.0, 99, 140];
mjj_sig_sigma_cat13[10.0, 1.0, 60.0];
mjj_sig_alpha1_cat13[1.0, 0.05, 10.0];
mjj_sig_n1_cat13[2.0, 0.1, 10.0];
mjj_sig_alpha2_cat13[1.0, 0.05, 10.0];
mjj_sig_n2_cat13[2.0, 0.1, 10.0];
mjjSig_cat13 = RooDoubleCB(mjj, mjj_sig_m0_cat13, mjj_sig_sigma_cat13, mjj_sig_alpha1_cat13, mjj_sig_n1_cat13, mjj_sig_alpha2_cat13, mjj_sig_n2_cat13);

mjj_sigVBF_m0_cat13[110.0, 99, 140];
mjj_sigVBF_sigma_cat13[10.0, 1.0, 60.0];
mjj_sigVBF_alpha1_cat13[1.0, 0.05, 10.0];
mjj_sigVBF_n1_cat13[2.0, 0.1, 10.0];
mjj_sigVBF_alpha2_cat13[1.0, 0.05, 10.0];
mjj_sigVBF_n2_cat13[2.0, 0.1, 10.0];
mjjSigVBF_cat13 = RooDoubleCB(mjj, mjj_sigVBF_m0_cat13, mjj_sigVBF_sigma_cat13, mjj_sigVBF_alpha1_cat13, mjj_sigVBF_n1_cat13, mjj_sigVBF_alpha2_cat13, mjj_sigVBF_n2_cat13);

mjj_hig_par1_ggh_cat13[0.1, 0, 10];
mjj_hig_par2_ggh_cat13[0.1, 0, 10];
mjj_hig_par3_ggh_cat13[0.1, 0, 10];

mjj_hig_par1_vbf_cat13[0.1, 0, 10];
mjj_hig_par2_vbf_cat13[0.1, 0, 10];
mjj_hig_par3_vbf_cat13[0.1, 0, 10];

mjj_hig_m0_tth_cat13[100, 70, 190];
mjj_hig_sigma_tth_cat13[50, 10, 100];
mjj_hig_alpha1_tth_cat13[1.0, 0.01, 10];
mjj_hig_n1_tth_cat13[1, 0.01, 10];
mjj_hig_alpha2_tth_cat13[1.0, 0.01, 10];
mjj_hig_n2_tth_cat13[1, 0.01, 10];
mjjHig_tth_cat13 = RooDoubleCB(mjj, mjj_hig_m0_tth_cat13, mjj_hig_sigma_tth_cat13, mjj_hig_alpha1_tth_cat13, mjj_hig_n1_tth_cat13, mjj_hig_alpha2_tth_cat13, mjj_hig_n2_tth_cat13);

mjj_hig_m0_vh_cat13[100, 70, 190];
mjj_hig_sigma_vh_cat13[50, 10, 100];
mjj_hig_alpha1_vh_cat13[1.0, 0.01, 10];
mjj_hig_n1_vh_cat13[1, 0.01, 10];
mjj_hig_alpha2_vh_cat13[1.0, 0.01, 10];
mjj_hig_n2_vh_cat13[1, 0.01, 10];
mjjHig_vh_cat13 = RooDoubleCB(mjj, mjj_hig_m0_vh_cat13, mjj_hig_sigma_vh_cat13, mjj_hig_alpha1_vh_cat13, mjj_hig_n1_vh_cat13, mjj_hig_alpha2_vh_cat13, mjj_hig_n2_vh_cat13);

mjj_hig_m0_bbh_cat13[100, 10, 180];
mjj_hig_sigma_bbh_cat13[50, 1.0, 100];
mjj_hig_alpha1_bbh_cat13[1.0, 0.01, 10];
mjj_hig_n1_bbh_cat13[1, 0.01, 10];
mjj_hig_alpha2_bbh_cat13[1.0, 0.01, 10];
mjj_hig_n2_bbh_cat13[1, 0.01, 10];
mjjHig_bbh_cat13 = RooDoubleCB(mjj, mjj_hig_m0_bbh_cat13, mjj_hig_sigma_bbh_cat13, mjj_hig_alpha1_bbh_cat13, mjj_hig_n1_bbh_cat13, mjj_hig_alpha2_bbh_cat13, mjj_hig_n2_bbh_cat13);
mgg_sig_m0_cat14[125., 122, 127];
mgg_sig_sigma_cat14[1.0, 0.1, 3.0];
mgg_sig_alpha1_cat14[1.0, 0.05, 10.0];
mgg_sig_n1_cat14[2.0, 0.1, 10.0];
mgg_sig_alpha2_cat14[1.0, 0.05, 10.0];
mgg_sig_n2_cat14[2.0, 0.1, 10.0];
mggSig_cat14 = RooDoubleCB(mgg, mgg_sig_m0_cat14, mgg_sig_sigma_cat14, mgg_sig_alpha1_cat14, mgg_sig_n1_cat14, mgg_sig_alpha2_cat14, mgg_sig_n2_cat14);

mgg_sigVBF_m0_cat14[125., 122, 127];
mgg_sigVBF_sigma_cat14[1.0, 0.1, 3.0];
mgg_sigVBF_alpha1_cat14[1.0, 0.05, 10.0];
mgg_sigVBF_n1_cat14[2.0, 0.1, 10.0];
mgg_sigVBF_alpha2_cat14[1.0, 0.05, 10.0];
mgg_sigVBF_n2_cat14[2.0, 0.1, 10.0];
mggSigVBF_cat14 = RooDoubleCB(mgg, mgg_sigVBF_m0_cat14, mgg_sigVBF_sigma_cat14, mgg_sigVBF_alpha1_cat14, mgg_sigVBF_n1_cat14, mgg_sigVBF_alpha2_cat14, mgg_sigVBF_n2_cat14);

mgg_hig_m0_ggh_cat14[124.2, 123, 125];
mgg_hig_sigma_ggh_cat14[2.0, 0.1, 3.0];
mgg_hig_alpha1_ggh_cat14[1.0, 0.05, 10.0];
mgg_hig_n1_ggh_cat14[2.0, 0.1, 10.0];
mgg_hig_alpha2_ggh_cat14[1.0, 0.05, 10.0];
mgg_hig_n2_ggh_cat14[2.0, 0.1, 10.0];
mggHig_ggh_cat14 = RooDoubleCB(mgg, mgg_hig_m0_ggh_cat14, mgg_hig_sigma_ggh_cat14, mgg_hig_alpha1_ggh_cat14, mgg_hig_n1_ggh_cat14, mgg_hig_alpha2_ggh_cat14, mgg_hig_n2_ggh_cat14);

mgg_hig_m0_tth_cat14[124.2, 123, 125];
mgg_hig_sigma_tth_cat14[2.0, 0.1, 3.0];
mgg_hig_alpha1_tth_cat14[1.0, 0.05, 10.0];
mgg_hig_n1_tth_cat14[2.0, 0.1, 10.0];
mgg_hig_alpha2_tth_cat14[1.0, 0.05, 10.0];
mgg_hig_n2_tth_cat14[2.0, 0.1, 10.0];
mggHig_tth_cat14 = RooDoubleCB(mgg, mgg_hig_m0_tth_cat14, mgg_hig_sigma_tth_cat14, mgg_hig_alpha1_tth_cat14, mgg_hig_n1_tth_cat14, mgg_hig_alpha2_tth_cat14, mgg_hig_n2_tth_cat14);

mgg_hig_m0_vh_cat14[124.2, 123, 125];
mgg_hig_sigma_vh_cat14[2.0, 0.1, 3.0];
mgg_hig_alpha1_vh_cat14[1.0, 0.05, 10.0];
mgg_hig_n1_vh_cat14[2.0, 0.1, 10.0];
mgg_hig_alpha2_vh_cat14[1.0, 0.05, 10.0];
mgg_hig_n2_vh_cat14[2.0, 0.1, 10.0];
mggHig_vh_cat14 = RooDoubleCB(mgg, mgg_hig_m0_vh_cat14, mgg_hig_sigma_vh_cat14, mgg_hig_alpha1_vh_cat14, mgg_hig_n1_vh_cat14, mgg_hig_alpha2_vh_cat14, mgg_hig_n2_vh_cat14);

mgg_hig_m0_vbf_cat14[124.2, 123, 125];
mgg_hig_sigma_vbf_cat14[2.0, 0.1, 3.0];
mgg_hig_alpha1_vbf_cat14[1.0, 0.05, 10.0];
mgg_hig_n1_vbf_cat14[2.0, 0.1, 10.0];
mgg_hig_alpha2_vbf_cat14[1.0, 0.05, 10.0];
mgg_hig_n2_vbf_cat14[2.0, 0.1, 10.0];
mggHig_vbf_cat14 = RooDoubleCB(mgg, mgg_hig_m0_vbf_cat14, mgg_hig_sigma_vbf_cat14, mgg_hig_alpha1_vbf_cat14, mgg_hig_n1_vbf_cat14, mgg_hig_alpha2_vbf_cat14, mgg_hig_n2_vbf_cat14);

mgg_hig_m0_bbh_cat14[124.2, 123, 125];
mgg_hig_sigma_bbh_cat14[2.0, 0.1, 3.0];
mgg_hig_alpha1_bbh_cat14[1.0, 0.05, 10.0];
mgg_hig_n1_bbh_cat14[2.0, 0.1, 10.0];
mgg_hig_alpha2_bbh_cat14[1.0, 0.05, 10.0];
mgg_hig_n2_bbh_cat14[2.0, 0.1, 10.0];
mggHig_bbh_cat14 = RooDoubleCB(mgg, mgg_hig_m0_bbh_cat14, mgg_hig_sigma_bbh_cat14, mgg_hig_alpha1_bbh_cat14, mgg_hig_n1_bbh_cat14, mgg_hig_alpha2_bbh_cat14, mgg_hig_n2_bbh_cat14);


mjj_sig_m0_cat14[110.0, 99, 140];
mjj_sig_sigma_cat14[10.0, 1.0, 60.0];
mjj_sig_alpha1_cat14[1.0, 0.05, 10.0];
mjj_sig_n1_cat14[2.0, 0.1, 10.0];
mjj_sig_alpha2_cat14[1.0, 0.05, 10.0];
mjj_sig_n2_cat14[2.0, 0.1, 10.0];
mjjSig_cat14 = RooDoubleCB(mjj, mjj_sig_m0_cat14, mjj_sig_sigma_cat14, mjj_sig_alpha1_cat14, mjj_sig_n1_cat14, mjj_sig_alpha2_cat14, mjj_sig_n2_cat14);

mjj_sigVBF_m0_cat14[110.0, 99, 140];
mjj_sigVBF_sigma_cat14[10.0, 1.0, 60.0];
mjj_sigVBF_alpha1_cat14[1.0, 0.05, 10.0];
mjj_sigVBF_n1_cat14[2.0, 0.1, 10.0];
mjj_sigVBF_alpha2_cat14[1.0, 0.05, 10.0];
mjj_sigVBF_n2_cat14[2.0, 0.1, 10.0];
mjjSigVBF_cat14 = RooDoubleCB(mjj, mjj_sigVBF_m0_cat14, mjj_sigVBF_sigma_cat14, mjj_sigVBF_alpha1_cat14, mjj_sigVBF_n1_cat14, mjj_sigVBF_alpha2_cat14, mjj_sigVBF_n2_cat14);

mjj_hig_par1_ggh_cat14[0.1, 0, 10];
mjj_hig_par2_ggh_cat14[0.1, 0, 10];
mjj_hig_par3_ggh_cat14[0.1, 0, 10];

mjj_hig_par1_vbf_cat14[0.1, 0, 10];
mjj_hig_par2_vbf_cat14[0.1, 0, 10];
mjj_hig_par3_vbf_cat14[0.1, 0, 10];

mjj_hig_m0_tth_cat14[100, 70, 190];
mjj_hig_sigma_tth_cat14[50, 10, 100];
mjj_hig_alpha1_tth_cat14[1.0, 0.01, 10];
mjj_hig_n1_tth_cat14[1, 0.01, 10];
mjj_hig_alpha2_tth_cat14[1.0, 0.01, 10];
mjj_hig_n2_tth_cat14[1, 0.01, 10];
mjjHig_tth_cat14 = RooDoubleCB(mjj, mjj_hig_m0_tth_cat14, mjj_hig_sigma_tth_cat14, mjj_hig_alpha1_tth_cat14, mjj_hig_n1_tth_cat14, mjj_hig_alpha2_tth_cat14, mjj_hig_n2_tth_cat14);

mjj_hig_m0_vh_cat14[100, 70, 190];
mjj_hig_sigma_vh_cat14[50, 10, 100];
mjj_hig_alpha1_vh_cat14[1.0, 0.01, 10];
mjj_hig_n1_vh_cat14[1, 0.01, 10];
mjj_hig_alpha2_vh_cat14[1.0, 0.01, 10];
mjj_hig_n2_vh_cat14[1, 0.01, 10];
mjjHig_vh_cat14 = RooDoubleCB(mjj, mjj_hig_m0_vh_cat14, mjj_hig_sigma_vh_cat14, mjj_hig_alpha1_vh_cat14, mjj_hig_n1_vh_cat14, mjj_hig_alpha2_vh_cat14, mjj_hig_n2_vh_cat14);

mjj_hig_m0_bbh_cat14[100, 10, 180];
mjj_hig_sigma_bbh_cat14[50, 1.0, 100];
mjj_hig_alpha1_bbh_cat14[1.0, 0.01, 10];
mjj_hig_n1_bbh_cat14[1, 0.01, 10];
mjj_hig_alpha2_bbh_cat14[1.0, 0.01, 10];
mjj_hig_n2_bbh_cat14[1, 0.01, 10];
mjjHig_bbh_cat14 = RooDoubleCB(mjj, mjj_hig_m0_bbh_cat14, mjj_hig_sigma_bbh_cat14, mjj_hig_alpha1_bbh_cat14, mjj_hig_n1_bbh_cat14, mjj_hig_alpha2_bbh_cat14, mjj_hig_n2_bbh_cat14);
mgg_sig_m0_cat15[125., 122, 127];
mgg_sig_sigma_cat15[1.0, 0.1, 3.0];
mgg_sig_alpha1_cat15[1.0, 0.05, 10.0];
mgg_sig_n1_cat15[2.0, 0.1, 10.0];
mgg_sig_alpha2_cat15[1.0, 0.05, 10.0];
mgg_sig_n2_cat15[2.0, 0.1, 10.0];
mggSig_cat15 = RooDoubleCB(mgg, mgg_sig_m0_cat15, mgg_sig_sigma_cat15, mgg_sig_alpha1_cat15, mgg_sig_n1_cat15, mgg_sig_alpha2_cat15, mgg_sig_n2_cat15);

mgg_sigVBF_m0_cat15[125., 122, 127];
mgg_sigVBF_sigma_cat15[1.0, 0.1, 3.0];
mgg_sigVBF_alpha1_cat15[1.0, 0.05, 10.0];
mgg_sigVBF_n1_cat15[2.0, 0.1, 10.0];
mgg_sigVBF_alpha2_cat15[1.0, 0.05, 10.0];
mgg_sigVBF_n2_cat15[2.0, 0.1, 10.0];
mggSigVBF_cat15 = RooDoubleCB(mgg, mgg_sigVBF_m0_cat15, mgg_sigVBF_sigma_cat15, mgg_sigVBF_alpha1_cat15, mgg_sigVBF_n1_cat15, mgg_sigVBF_alpha2_cat15, mgg_sigVBF_n2_cat15);

mgg_hig_m0_ggh_cat15[124.2, 123, 125];
mgg_hig_sigma_ggh_cat15[2.0, 0.1, 3.0];
mgg_hig_alpha1_ggh_cat15[1.0, 0.05, 10.0];
mgg_hig_n1_ggh_cat15[2.0, 0.1, 10.0];
mgg_hig_alpha2_ggh_cat15[1.0, 0.05, 10.0];
mgg_hig_n2_ggh_cat15[2.0, 0.1, 10.0];
mggHig_ggh_cat15 = RooDoubleCB(mgg, mgg_hig_m0_ggh_cat15, mgg_hig_sigma_ggh_cat15, mgg_hig_alpha1_ggh_cat15, mgg_hig_n1_ggh_cat15, mgg_hig_alpha2_ggh_cat15, mgg_hig_n2_ggh_cat15);

mgg_hig_m0_tth_cat15[124.2, 123, 125];
mgg_hig_sigma_tth_cat15[2.0, 0.1, 3.0];
mgg_hig_alpha1_tth_cat15[1.0, 0.05, 10.0];
mgg_hig_n1_tth_cat15[2.0, 0.1, 10.0];
mgg_hig_alpha2_tth_cat15[1.0, 0.05, 10.0];
mgg_hig_n2_tth_cat15[2.0, 0.1, 10.0];
mggHig_tth_cat15 = RooDoubleCB(mgg, mgg_hig_m0_tth_cat15, mgg_hig_sigma_tth_cat15, mgg_hig_alpha1_tth_cat15, mgg_hig_n1_tth_cat15, mgg_hig_alpha2_tth_cat15, mgg_hig_n2_tth_cat15);

mgg_hig_m0_vh_cat15[124.2, 123, 125];
mgg_hig_sigma_vh_cat15[2.0, 0.1, 3.0];
mgg_hig_alpha1_vh_cat15[1.0, 0.05, 10.0];
mgg_hig_n1_vh_cat15[2.0, 0.1, 10.0];
mgg_hig_alpha2_vh_cat15[1.0, 0.05, 10.0];
mgg_hig_n2_vh_cat15[2.0, 0.1, 10.0];
mggHig_vh_cat15 = RooDoubleCB(mgg, mgg_hig_m0_vh_cat15, mgg_hig_sigma_vh_cat15, mgg_hig_alpha1_vh_cat15, mgg_hig_n1_vh_cat15, mgg_hig_alpha2_vh_cat15, mgg_hig_n2_vh_cat15);

mgg_hig_m0_vbf_cat15[124.2, 123, 125];
mgg_hig_sigma_vbf_cat15[2.0, 0.1, 3.0];
mgg_hig_alpha1_vbf_cat15[1.0, 0.05, 10.0];
mgg_hig_n1_vbf_cat15[2.0, 0.1, 10.0];
mgg_hig_alpha2_vbf_cat15[1.0, 0.05, 10.0];
mgg_hig_n2_vbf_cat15[2.0, 0.1, 10.0];
mggHig_vbf_cat15 = RooDoubleCB(mgg, mgg_hig_m0_vbf_cat15, mgg_hig_sigma_vbf_cat15, mgg_hig_alpha1_vbf_cat15, mgg_hig_n1_vbf_cat15, mgg_hig_alpha2_vbf_cat15, mgg_hig_n2_vbf_cat15);

mgg_hig_m0_bbh_cat15[124.2, 123, 125];
mgg_hig_sigma_bbh_cat15[2.0, 0.1, 3.0];
mgg_hig_alpha1_bbh_cat15[1.0, 0.05, 10.0];
mgg_hig_n1_bbh_cat15[2.0, 0.1, 10.0];
mgg_hig_alpha2_bbh_cat15[1.0, 0.05, 10.0];
mgg_hig_n2_bbh_cat15[2.0, 0.1, 10.0];
mggHig_bbh_cat15 = RooDoubleCB(mgg, mgg_hig_m0_bbh_cat15, mgg_hig_sigma_bbh_cat15, mgg_hig_alpha1_bbh_cat15, mgg_hig_n1_bbh_cat15, mgg_hig_alpha2_bbh_cat15, mgg_hig_n2_bbh_cat15);


mjj_sig_m0_cat15[110.0, 99, 140];
mjj_sig_sigma_cat15[10.0, 1.0, 60.0];
mjj_sig_alpha1_cat15[1.0, 0.05, 10.0];
mjj_sig_n1_cat15[2.0, 0.1, 10.0];
mjj_sig_alpha2_cat15[1.0, 0.05, 10.0];
mjj_sig_n2_cat15[2.0, 0.1, 10.0];
mjjSig_cat15 = RooDoubleCB(mjj, mjj_sig_m0_cat15, mjj_sig_sigma_cat15, mjj_sig_alpha1_cat15, mjj_sig_n1_cat15, mjj_sig_alpha2_cat15, mjj_sig_n2_cat15);

mjj_sigVBF_m0_cat15[110.0, 99, 140];
mjj_sigVBF_sigma_cat15[10.0, 1.0, 60.0];
mjj_sigVBF_alpha1_cat15[1.0, 0.05, 10.0];
mjj_sigVBF_n1_cat15[2.0, 0.1, 10.0];
mjj_sigVBF_alpha2_cat15[1.0, 0.05, 10.0];
mjj_sigVBF_n2_cat15[2.0, 0.1, 10.0];
mjjSigVBF_cat15 = RooDoubleCB(mjj, mjj_sigVBF_m0_cat15, mjj_sigVBF_sigma_cat15, mjj_sigVBF_alpha1_cat15, mjj_sigVBF_n1_cat15, mjj_sigVBF_alpha2_cat15, mjj_sigVBF_n2_cat15);

mjj_hig_par1_ggh_cat15[0.1, 0, 10];
mjj_hig_par2_ggh_cat15[0.1, 0, 10];
mjj_hig_par3_ggh_cat15[0.1, 0, 10];

mjj_hig_par1_vbf_cat15[0.1, 0, 10];
mjj_hig_par2_vbf_cat15[0.1, 0, 10];
mjj_hig_par3_vbf_cat15[0.1, 0, 10];

mjj_hig_m0_tth_cat15[100, 70, 190];
mjj_hig_sigma_tth_cat15[50, 10, 100];
mjj_hig_alpha1_tth_cat15[1.0, 0.01, 10];
mjj_hig_n1_tth_cat15[1, 0.01, 10];
mjj_hig_alpha2_tth_cat15[1.0, 0.01, 10];
mjj_hig_n2_tth_cat15[1, 0.01, 10];
mjjHig_tth_cat15 = RooDoubleCB(mjj, mjj_hig_m0_tth_cat15, mjj_hig_sigma_tth_cat15, mjj_hig_alpha1_tth_cat15, mjj_hig_n1_tth_cat15, mjj_hig_alpha2_tth_cat15, mjj_hig_n2_tth_cat15);

mjj_hig_m0_vh_cat15[100, 70, 190];
mjj_hig_sigma_vh_cat15[50, 10, 100];
mjj_hig_alpha1_vh_cat15[1.0, 0.01, 10];
mjj_hig_n1_vh_cat15[1, 0.01, 10];
mjj_hig_alpha2_vh_cat15[1.0, 0.01, 10];
mjj_hig_n2_vh_cat15[1, 0.01, 10];
mjjHig_vh_cat15 = RooDoubleCB(mjj, mjj_hig_m0_vh_cat15, mjj_hig_sigma_vh_cat15, mjj_hig_alpha1_vh_cat15, mjj_hig_n1_vh_cat15, mjj_hig_alpha2_vh_cat15, mjj_hig_n2_vh_cat15);

mjj_hig_m0_bbh_cat15[100, 10, 180];
mjj_hig_sigma_bbh_cat15[50, 1.0, 100];
mjj_hig_alpha1_bbh_cat15[1.0, 0.01, 10];
mjj_hig_n1_bbh_cat15[1, 0.01, 10];
mjj_hig_alpha2_bbh_cat15[1.0, 0.01, 10];
mjj_hig_n2_bbh_cat15[1, 0.01, 10];
mjjHig_bbh_cat15 = RooDoubleCB(mjj, mjj_hig_m0_bbh_cat15, mjj_hig_sigma_bbh_cat15, mjj_hig_alpha1_bbh_cat15, mjj_hig_n1_bbh_cat15, mjj_hig_alpha2_bbh_cat15, mjj_hig_n2_bbh_cat15);
mgg_sig_m0_cat2[125., 122, 127];
mgg_sig_sigma_cat2[1.0, 0.1, 3.0];
mgg_sig_alpha1_cat2[1.0, 0.05, 10.0];
mgg_sig_n1_cat2[2.0, 0.1, 10.0];
mgg_sig_alpha2_cat2[1.0, 0.05, 10.0];
mgg_sig_n2_cat2[2.0, 0.1, 10.0];
mggSig_cat2 = RooDoubleCB(mgg, mgg_sig_m0_cat2, mgg_sig_sigma_cat2, mgg_sig_alpha1_cat2, mgg_sig_n1_cat2, mgg_sig_alpha2_cat2, mgg_sig_n2_cat2);

mgg_sigVBF_m0_cat2[125., 122, 127];
mgg_sigVBF_sigma_cat2[1.0, 0.1, 3.0];
mgg_sigVBF_alpha1_cat2[1.0, 0.05, 10.0];
mgg_sigVBF_n1_cat2[2.0, 0.1, 10.0];
mgg_sigVBF_alpha2_cat2[1.0, 0.05, 10.0];
mgg_sigVBF_n2_cat2[2.0, 0.1, 10.0];
mggSigVBF_cat2 = RooDoubleCB(mgg, mgg_sigVBF_m0_cat2, mgg_sigVBF_sigma_cat2, mgg_sigVBF_alpha1_cat2, mgg_sigVBF_n1_cat2, mgg_sigVBF_alpha2_cat2, mgg_sigVBF_n2_cat2);

mgg_hig_m0_ggh_cat2[124.2, 123, 125];
mgg_hig_sigma_ggh_cat2[2.0, 0.1, 3.0];
mgg_hig_alpha1_ggh_cat2[1.0, 0.05, 10.0];
mgg_hig_n1_ggh_cat2[2.0, 0.1, 10.0];
mgg_hig_alpha2_ggh_cat2[1.0, 0.05, 10.0];
mgg_hig_n2_ggh_cat2[2.0, 0.1, 10.0];
mggHig_ggh_cat2 = RooDoubleCB(mgg, mgg_hig_m0_ggh_cat2, mgg_hig_sigma_ggh_cat2, mgg_hig_alpha1_ggh_cat2, mgg_hig_n1_ggh_cat2, mgg_hig_alpha2_ggh_cat2, mgg_hig_n2_ggh_cat2);

mgg_hig_m0_tth_cat2[124.2, 123, 125];
mgg_hig_sigma_tth_cat2[2.0, 0.1, 3.0];
mgg_hig_alpha1_tth_cat2[1.0, 0.05, 10.0];
mgg_hig_n1_tth_cat2[2.0, 0.1, 10.0];
mgg_hig_alpha2_tth_cat2[1.0, 0.05, 10.0];
mgg_hig_n2_tth_cat2[2.0, 0.1, 10.0];
mggHig_tth_cat2 = RooDoubleCB(mgg, mgg_hig_m0_tth_cat2, mgg_hig_sigma_tth_cat2, mgg_hig_alpha1_tth_cat2, mgg_hig_n1_tth_cat2, mgg_hig_alpha2_tth_cat2, mgg_hig_n2_tth_cat2);

mgg_hig_m0_vh_cat2[124.2, 123, 125];
mgg_hig_sigma_vh_cat2[2.0, 0.1, 3.0];
mgg_hig_alpha1_vh_cat2[1.0, 0.05, 10.0];
mgg_hig_n1_vh_cat2[2.0, 0.1, 10.0];
mgg_hig_alpha2_vh_cat2[1.0, 0.05, 10.0];
mgg_hig_n2_vh_cat2[2.0, 0.1, 10.0];
mggHig_vh_cat2 = RooDoubleCB(mgg, mgg_hig_m0_vh_cat2, mgg_hig_sigma_vh_cat2, mgg_hig_alpha1_vh_cat2, mgg_hig_n1_vh_cat2, mgg_hig_alpha2_vh_cat2, mgg_hig_n2_vh_cat2);

mgg_hig_m0_vbf_cat2[124.2, 123, 125];
mgg_hig_sigma_vbf_cat2[2.0, 0.1, 3.0];
mgg_hig_alpha1_vbf_cat2[1.0, 0.05, 10.0];
mgg_hig_n1_vbf_cat2[2.0, 0.1, 10.0];
mgg_hig_alpha2_vbf_cat2[1.0, 0.05, 10.0];
mgg_hig_n2_vbf_cat2[2.0, 0.1, 10.0];
mggHig_vbf_cat2 = RooDoubleCB(mgg, mgg_hig_m0_vbf_cat2, mgg_hig_sigma_vbf_cat2, mgg_hig_alpha1_vbf_cat2, mgg_hig_n1_vbf_cat2, mgg_hig_alpha2_vbf_cat2, mgg_hig_n2_vbf_cat2);

mgg_hig_m0_bbh_cat2[124.2, 123, 125];
mgg_hig_sigma_bbh_cat2[2.0, 0.1, 3.0];
mgg_hig_alpha1_bbh_cat2[1.0, 0.05, 10.0];
mgg_hig_n1_bbh_cat2[2.0, 0.1, 10.0];
mgg_hig_alpha2_bbh_cat2[1.0, 0.05, 10.0];
mgg_hig_n2_bbh_cat2[2.0, 0.1, 10.0];
mggHig_bbh_cat2 = RooDoubleCB(mgg, mgg_hig_m0_bbh_cat2, mgg_hig_sigma_bbh_cat2, mgg_hig_alpha1_bbh_cat2, mgg_hig_n1_bbh_cat2, mgg_hig_alpha2_bbh_cat2, mgg_hig_n2_bbh_cat2);

mjj_sig_m0_cat2[110.0, 99, 140];
mjj_sig_sigma_cat2[10.0, 1.0, 60.0];
mjj_sig_alpha1_cat2[1.0, 0.05, 10.0];
mjj_sig_n1_cat2[2.0, 0.1, 10.0];
mjj_sig_alpha2_cat2[1.0, 0.05, 10.0];
mjj_sig_n2_cat2[2.0, 0.1, 5.0];
mjjSig_cat2 = RooDoubleCB(mjj, mjj_sig_m0_cat2, mjj_sig_sigma_cat2, mjj_sig_alpha1_cat2, mjj_sig_n1_cat2, mjj_sig_alpha2_cat2, mjj_sig_n2_cat2);

mjj_sigVBF_m0_cat2[110.0, 99, 140];
mjj_sigVBF_sigma_cat2[10.0, 1.0, 60.0];
mjj_sigVBF_alpha1_cat2[1.0, 0.05, 10.0];
mjj_sigVBF_n1_cat2[2.0, 0.1, 10.0];
mjj_sigVBF_alpha2_cat2[1.0, 0.05, 10.0];
mjj_sigVBF_n2_cat2[2.0, 0.1, 5.0];
mjjSigVBF_cat2 = RooDoubleCB(mjj, mjj_sigVBF_m0_cat2, mjj_sigVBF_sigma_cat2, mjj_sigVBF_alpha1_cat2, mjj_sigVBF_n1_cat2, mjj_sigVBF_alpha2_cat2, mjj_sigVBF_n2_cat2);

mjj_hig_par1_ggh_cat2[0.1, 0, 10];
mjj_hig_par2_ggh_cat2[0.1, 0, 10];
mjj_hig_par3_ggh_cat2[0.1, 0, 10];

mjj_hig_par1_vbf_cat2[0.1, 0, 10];
mjj_hig_par2_vbf_cat2[0.1, 0, 10];
mjj_hig_par3_vbf_cat2[0.1, 0, 10];

mjj_hig_m0_tth_cat2[100, 70, 190];
mjj_hig_sigma_tth_cat2[50, 10, 100];
mjj_hig_alpha1_tth_cat2[1.0, 0.01, 10];
mjj_hig_n1_tth_cat2[1, 0.01, 10];
mjj_hig_alpha2_tth_cat2[1.0, 0.01, 10];
mjj_hig_n2_tth_cat2[1, 0.01, 10];
mjjHig_tth_cat2 = RooDoubleCB(mjj, mjj_hig_m0_tth_cat2, mjj_hig_sigma_tth_cat2, mjj_hig_alpha1_tth_cat2, mjj_hig_n1_tth_cat2, mjj_hig_alpha2_tth_cat2, mjj_hig_n2_tth_cat2);

mjj_hig_m0_vh_cat2[100, 70, 190];
mjj_hig_sigma_vh_cat2[50, 10, 100];
mjj_hig_alpha1_vh_cat2[1.0, 0.01, 10];
mjj_hig_n1_vh_cat2[1, 0.01, 10];
mjj_hig_alpha2_vh_cat2[1.0, 0.01, 10];
mjj_hig_n2_vh_cat2[1, 0.01, 10];
mjjHig_vh_cat2 = RooDoubleCB(mjj, mjj_hig_m0_vh_cat2, mjj_hig_sigma_vh_cat2, mjj_hig_alpha1_vh_cat2, mjj_hig_n1_vh_cat2, mjj_hig_alpha2_vh_cat2, mjj_hig_n2_vh_cat2);

mjj_hig_m0_bbh_cat2[100, 10, 180];
mjj_hig_sigma_bbh_cat2[50, 1.0, 100];
mjj_hig_alpha1_bbh_cat2[1.0, 0.01, 10];
mjj_hig_n1_bbh_cat2[1, 0.01, 10];
mjj_hig_alpha2_bbh_cat2[1.0, 0.01, 10];
mjj_hig_n2_bbh_cat2[1, 0.01, 10];
mjjHig_bbh_cat2 = RooDoubleCB(mjj, mjj_hig_m0_bbh_cat2, mjj_hig_sigma_bbh_cat2, mjj_hig_alpha1_bbh_cat2, mjj_hig_n1_bbh_cat2, mjj_hig_alpha2_bbh_cat2, mjj_hig_n2_bbh_cat2);
mgg_sig_m0_cat3[125., 122, 127];
mgg_sig_sigma_cat3[1.0, 0.1, 3.0];
mgg_sig_alpha1_cat3[1.0, 0.05, 10.0];
mgg_sig_n1_cat3[2.0, 0.1, 10.0];
mgg_sig_alpha2_cat3[1.0, 0.05, 10.0];
mgg_sig_n2_cat3[2.0, 0.1, 10.0];
mggSig_cat3 = RooDoubleCB(mgg, mgg_sig_m0_cat3, mgg_sig_sigma_cat3, mgg_sig_alpha1_cat3, mgg_sig_n1_cat3, mgg_sig_alpha2_cat3, mgg_sig_n2_cat3);

mgg_sigVBF_m0_cat3[125., 122, 127];
mgg_sigVBF_sigma_cat3[1.0, 0.1, 3.0];
mgg_sigVBF_alpha1_cat3[1.0, 0.05, 10.0];
mgg_sigVBF_n1_cat3[2.0, 0.1, 10.0];
mgg_sigVBF_alpha2_cat3[1.0, 0.05, 10.0];
mgg_sigVBF_n2_cat3[2.0, 0.1, 10.0];
mggSigVBF_cat3 = RooDoubleCB(mgg, mgg_sigVBF_m0_cat3, mgg_sigVBF_sigma_cat3, mgg_sigVBF_alpha1_cat3, mgg_sigVBF_n1_cat3, mgg_sigVBF_alpha2_cat3, mgg_sigVBF_n2_cat3);

mgg_hig_m0_ggh_cat3[124.2, 123, 125];
mgg_hig_sigma_ggh_cat3[2.0, 0.1, 3.0];
mgg_hig_alpha1_ggh_cat3[1.0, 0.05, 10.0];
mgg_hig_n1_ggh_cat3[2.0, 0.1, 10.0];
mgg_hig_alpha2_ggh_cat3[1.0, 0.05, 10.0];
mgg_hig_n2_ggh_cat3[2.0, 0.1, 10.0];
mggHig_ggh_cat3 = RooDoubleCB(mgg, mgg_hig_m0_ggh_cat3, mgg_hig_sigma_ggh_cat3, mgg_hig_alpha1_ggh_cat3, mgg_hig_n1_ggh_cat3, mgg_hig_alpha2_ggh_cat3, mgg_hig_n2_ggh_cat3);

mgg_hig_m0_tth_cat3[124.2, 123, 125];
mgg_hig_sigma_tth_cat3[2.0, 0.1, 3.0];
mgg_hig_alpha1_tth_cat3[1.0, 0.05, 10.0];
mgg_hig_n1_tth_cat3[2.0, 0.1, 10.0];
mgg_hig_alpha2_tth_cat3[1.0, 0.05, 10.0];
mgg_hig_n2_tth_cat3[2.0, 0.1, 10.0];
mggHig_tth_cat3 = RooDoubleCB(mgg, mgg_hig_m0_tth_cat3, mgg_hig_sigma_tth_cat3, mgg_hig_alpha1_tth_cat3, mgg_hig_n1_tth_cat3, mgg_hig_alpha2_tth_cat3, mgg_hig_n2_tth_cat3);

mgg_hig_m0_vh_cat3[124.2, 123, 125];
mgg_hig_sigma_vh_cat3[2.0, 0.1, 3.0];
mgg_hig_alpha1_vh_cat3[1.0, 0.05, 10.0];
mgg_hig_n1_vh_cat3[2.0, 0.1, 10.0];
mgg_hig_alpha2_vh_cat3[1.0, 0.05, 10.0];
mgg_hig_n2_vh_cat3[2.0, 0.1, 10.0];
mggHig_vh_cat3 = RooDoubleCB(mgg, mgg_hig_m0_vh_cat3, mgg_hig_sigma_vh_cat3, mgg_hig_alpha1_vh_cat3, mgg_hig_n1_vh_cat3, mgg_hig_alpha2_vh_cat3, mgg_hig_n2_vh_cat3);

mgg_hig_m0_vbf_cat3[124.2, 123, 125];
mgg_hig_sigma_vbf_cat3[2.0, 0.1, 3.0];
mgg_hig_alpha1_vbf_cat3[1.0, 0.05, 10.0];
mgg_hig_n1_vbf_cat3[2.0, 0.1, 10.0];
mgg_hig_alpha2_vbf_cat3[1.0, 0.05, 10.0];
mgg_hig_n2_vbf_cat3[2.0, 0.1, 10.0];
mggHig_vbf_cat3 = RooDoubleCB(mgg, mgg_hig_m0_vbf_cat3, mgg_hig_sigma_vbf_cat3, mgg_hig_alpha1_vbf_cat3, mgg_hig_n1_vbf_cat3, mgg_hig_alpha2_vbf_cat3, mgg_hig_n2_vbf_cat3);

mgg_hig_m0_bbh_cat3[124.2, 123, 125];
mgg_hig_sigma_bbh_cat3[2.0, 0.1, 3.0];
mgg_hig_alpha1_bbh_cat3[1.0, 0.05, 10.0];
mgg_hig_n1_bbh_cat3[2.0, 0.1, 10.0];
mgg_hig_alpha2_bbh_cat3[1.0, 0.05, 10.0];
mgg_hig_n2_bbh_cat3[2.0, 0.1, 10.0];
mggHig_bbh_cat3 = RooDoubleCB(mgg, mgg_hig_m0_bbh_cat3, mgg_hig_sigma_bbh_cat3, mgg_hig_alpha1_bbh_cat3, mgg_hig_n1_bbh_cat3, mgg_hig_alpha2_bbh_cat3, mgg_hig_n2_bbh_cat3);

mjj_sig_m0_cat3[110.0, 99, 140];
mjj_sig_sigma_cat3[10.0, 1.0, 60.0];
mjj_sig_alpha1_cat3[1.0, 0.05, 10.0];
mjj_sig_n1_cat3[2.0, 0.1, 10.0];
mjj_sig_alpha2_cat3[1.0, 0.05, 10.0];
mjj_sig_n2_cat3[2.0, 0.1, 5.0];
mjjSig_cat3 = RooDoubleCB(mjj, mjj_sig_m0_cat3, mjj_sig_sigma_cat3, mjj_sig_alpha1_cat3, mjj_sig_n1_cat3, mjj_sig_alpha2_cat3, mjj_sig_n2_cat3);

mjj_sigVBF_m0_cat3[110.0, 99, 140];
mjj_sigVBF_sigma_cat3[10.0, 1.0, 60.0];
mjj_sigVBF_alpha1_cat3[1.0, 0.05, 10.0];
mjj_sigVBF_n1_cat3[2.0, 0.1, 10.0];
mjj_sigVBF_alpha2_cat3[1.0, 0.05, 10.0];
mjj_sigVBF_n2_cat3[2.0, 0.1, 5.0];
mjjSigVBF_cat3 = RooDoubleCB(mjj, mjj_sigVBF_m0_cat3, mjj_sigVBF_sigma_cat3, mjj_sigVBF_alpha1_cat3, mjj_sigVBF_n1_cat3, mjj_sigVBF_alpha2_cat3, mjj_sigVBF_n2_cat3);

mjj_hig_par1_ggh_cat3[0.1, 0, 10];
mjj_hig_par2_ggh_cat3[0.1, 0, 10];
mjj_hig_par3_ggh_cat3[0.1, 0, 10];

mjj_hig_par1_vbf_cat3[0.1, 0, 10];
mjj_hig_par2_vbf_cat3[0.1, 0, 10];
mjj_hig_par3_vbf_cat3[0.1, 0, 10];

mjj_hig_m0_tth_cat3[100, 70, 190];
mjj_hig_sigma_tth_cat3[50, 10, 100];
mjj_hig_alpha1_tth_cat3[1.0, 0.01, 10];
mjj_hig_n1_tth_cat3[1, 0.01, 10];
mjj_hig_alpha2_tth_cat3[1.0, 0.01, 10];
mjj_hig_n2_tth_cat3[1, 0.01, 10];
mjjHig_tth_cat3 = RooDoubleCB(mjj, mjj_hig_m0_tth_cat3, mjj_hig_sigma_tth_cat3, mjj_hig_alpha1_tth_cat3, mjj_hig_n1_tth_cat3, mjj_hig_alpha2_tth_cat3, mjj_hig_n2_tth_cat3);

mjj_hig_m0_vh_cat3[100, 70, 190];
mjj_hig_sigma_vh_cat3[50, 10, 100];
mjj_hig_alpha1_vh_cat3[1.0, 0.01, 10];
mjj_hig_n1_vh_cat3[1, 0.01, 10];
mjj_hig_alpha2_vh_cat3[1.0, 0.01, 10];
mjj_hig_n2_vh_cat3[1, 0.01, 10];
mjjHig_vh_cat3 = RooDoubleCB(mjj, mjj_hig_m0_vh_cat3, mjj_hig_sigma_vh_cat3, mjj_hig_alpha1_vh_cat3, mjj_hig_n1_vh_cat3, mjj_hig_alpha2_vh_cat3, mjj_hig_n2_vh_cat3);

mjj_hig_m0_bbh_cat3[100, 10, 180];
mjj_hig_sigma_bbh_cat3[50, 1.0, 100];
mjj_hig_alpha1_bbh_cat3[1.0, 0.01, 10];
mjj_hig_n1_bbh_cat3[1, 0.01, 10];
mjj_hig_alpha2_bbh_cat3[1.0, 0.01, 10];
mjj_hig_n2_bbh_cat3[1, 0.01, 10];
mjjHig_bbh_cat3 = RooDoubleCB(mjj, mjj_hig_m0_bbh_cat3, mjj_hig_sigma_bbh_cat3, mjj_hig_alpha1_bbh_cat3, mjj_hig_n1_bbh_cat3, mjj_hig_alpha2_bbh_cat3, mjj_hig_n2_bbh_cat3);
mgg_sig_m0_cat4[125., 122, 127];
mgg_sig_sigma_cat4[1.0, 0.1, 3.0];
mgg_sig_alpha1_cat4[1.0, 0.05, 10.0];
mgg_sig_n1_cat4[2.0, 0.1, 10.0];
mgg_sig_alpha2_cat4[1.0, 0.05, 10.0];
mgg_sig_n2_cat4[2.0, 0.1, 10.0];
mggSig_cat4 = RooDoubleCB(mgg, mgg_sig_m0_cat4, mgg_sig_sigma_cat4, mgg_sig_alpha1_cat4, mgg_sig_n1_cat4, mgg_sig_alpha2_cat4, mgg_sig_n2_cat4);

mgg_sigVBF_m0_cat4[125., 122, 127];
mgg_sigVBF_sigma_cat4[1.0, 0.1, 3.0];
mgg_sigVBF_alpha1_cat4[1.0, 0.05, 10.0];
mgg_sigVBF_n1_cat4[2.0, 0.1, 10.0];
mgg_sigVBF_alpha2_cat4[1.0, 0.05, 10.0];
mgg_sigVBF_n2_cat4[2.0, 0.1, 10.0];
mggSigVBF_cat4 = RooDoubleCB(mgg, mgg_sigVBF_m0_cat4, mgg_sigVBF_sigma_cat4, mgg_sigVBF_alpha1_cat4, mgg_sigVBF_n1_cat4, mgg_sigVBF_alpha2_cat4, mgg_sigVBF_n2_cat4);

mgg_hig_m0_ggh_cat4[124.2, 123, 125];
mgg_hig_sigma_ggh_cat4[2.0, 0.1, 3.0];
mgg_hig_alpha1_ggh_cat4[1.0, 0.05, 10.0];
mgg_hig_n1_ggh_cat4[2.0, 0.1, 10.0];
mgg_hig_alpha2_ggh_cat4[1.0, 0.05, 10.0];
mgg_hig_n2_ggh_cat4[2.0, 0.1, 10.0];
mggHig_ggh_cat4 = RooDoubleCB(mgg, mgg_hig_m0_ggh_cat4, mgg_hig_sigma_ggh_cat4, mgg_hig_alpha1_ggh_cat4, mgg_hig_n1_ggh_cat4, mgg_hig_alpha2_ggh_cat4, mgg_hig_n2_ggh_cat4);

mgg_hig_m0_tth_cat4[124.2, 123, 125];
mgg_hig_sigma_tth_cat4[2.0, 0.1, 3.0];
mgg_hig_alpha1_tth_cat4[1.0, 0.05, 10.0];
mgg_hig_n1_tth_cat4[2.0, 0.1, 10.0];
mgg_hig_alpha2_tth_cat4[1.0, 0.05, 10.0];
mgg_hig_n2_tth_cat4[2.0, 0.1, 10.0];
mggHig_tth_cat4 = RooDoubleCB(mgg, mgg_hig_m0_tth_cat4, mgg_hig_sigma_tth_cat4, mgg_hig_alpha1_tth_cat4, mgg_hig_n1_tth_cat4, mgg_hig_alpha2_tth_cat4, mgg_hig_n2_tth_cat4);

mgg_hig_m0_vh_cat4[124.2, 123, 125];
mgg_hig_sigma_vh_cat4[2.0, 0.1, 3.0];
mgg_hig_alpha1_vh_cat4[1.0, 0.05, 10.0];
mgg_hig_n1_vh_cat4[2.0, 0.1, 10.0];
mgg_hig_alpha2_vh_cat4[1.0, 0.05, 10.0];
mgg_hig_n2_vh_cat4[2.0, 0.1, 10.0];
mggHig_vh_cat4 = RooDoubleCB(mgg, mgg_hig_m0_vh_cat4, mgg_hig_sigma_vh_cat4, mgg_hig_alpha1_vh_cat4, mgg_hig_n1_vh_cat4, mgg_hig_alpha2_vh_cat4, mgg_hig_n2_vh_cat4);

mgg_hig_m0_vbf_cat4[124.2, 123, 125];
mgg_hig_sigma_vbf_cat4[2.0, 0.1, 3.0];
mgg_hig_alpha1_vbf_cat4[1.0, 0.05, 10.0];
mgg_hig_n1_vbf_cat4[2.0, 0.1, 10.0];
mgg_hig_alpha2_vbf_cat4[1.0, 0.05, 10.0];
mgg_hig_n2_vbf_cat4[2.0, 0.1, 10.0];
mggHig_vbf_cat4 = RooDoubleCB(mgg, mgg_hig_m0_vbf_cat4, mgg_hig_sigma_vbf_cat4, mgg_hig_alpha1_vbf_cat4, mgg_hig_n1_vbf_cat4, mgg_hig_alpha2_vbf_cat4, mgg_hig_n2_vbf_cat4);

mgg_hig_m0_bbh_cat4[124.2, 123, 125];
mgg_hig_sigma_bbh_cat4[2.0, 0.1, 3.0];
mgg_hig_alpha1_bbh_cat4[1.0, 0.05, 10.0];
mgg_hig_n1_bbh_cat4[2.0, 0.1, 10.0];
mgg_hig_alpha2_bbh_cat4[1.0, 0.05, 10.0];
mgg_hig_n2_bbh_cat4[2.0, 0.1, 10.0];
mggHig_bbh_cat4 = RooDoubleCB(mgg, mgg_hig_m0_bbh_cat4, mgg_hig_sigma_bbh_cat4, mgg_hig_alpha1_bbh_cat4, mgg_hig_n1_bbh_cat4, mgg_hig_alpha2_bbh_cat4, mgg_hig_n2_bbh_cat4);


mjj_sig_m0_cat4[110.0, 99, 140];
mjj_sig_sigma_cat4[10.0, 1.0, 60.0];
mjj_sig_alpha1_cat4[1.0, 0.05, 10.0];
mjj_sig_n1_cat4[2.0, 0.1, 10.0];
mjj_sig_alpha2_cat4[1.0, 0.05, 10.0];
mjj_sig_n2_cat4[2.0, 0.1, 10.0];
mjjSig_cat4 = RooDoubleCB(mjj, mjj_sig_m0_cat4, mjj_sig_sigma_cat4, mjj_sig_alpha1_cat4, mjj_sig_n1_cat4, mjj_sig_alpha2_cat4, mjj_sig_n2_cat4);

mjj_sigVBF_m0_cat4[110.0, 99, 140];
mjj_sigVBF_sigma_cat4[10.0, 1.0, 60.0];
mjj_sigVBF_alpha1_cat4[1.0, 0.05, 10.0];
mjj_sigVBF_n1_cat4[2.0, 0.1, 10.0];
mjj_sigVBF_alpha2_cat4[1.0, 0.05, 10.0];
mjj_sigVBF_n2_cat4[2.0, 0.1, 10.0];
mjjSigVBF_cat4 = RooDoubleCB(mjj, mjj_sigVBF_m0_cat4, mjj_sigVBF_sigma_cat4, mjj_sigVBF_alpha1_cat4, mjj_sigVBF_n1_cat4, mjj_sigVBF_alpha2_cat4, mjj_sigVBF_n2_cat4);

mjj_hig_par1_ggh_cat4[0.1, 0, 10];
mjj_hig_par2_ggh_cat4[0.1, 0, 10];
mjj_hig_par3_ggh_cat4[0.1, 0, 10];

mjj_hig_par1_vbf_cat4[0.1, 0, 10];
mjj_hig_par2_vbf_cat4[0.1, 0, 10];
mjj_hig_par3_vbf_cat4[0.1, 0, 10];

mjj_hig_m0_tth_cat4[100, 70, 190];
mjj_hig_sigma_tth_cat4[50, 10, 100];
mjj_hig_alpha1_tth_cat4[1.0, 0.01, 10];
mjj_hig_n1_tth_cat4[1, 0.01, 10];
mjj_hig_alpha2_tth_cat4[1.0, 0.01, 10];
mjj_hig_n2_tth_cat4[1, 0.01, 10];
mjjHig_tth_cat4 = RooDoubleCB(mjj, mjj_hig_m0_tth_cat4, mjj_hig_sigma_tth_cat4, mjj_hig_alpha1_tth_cat4, mjj_hig_n1_tth_cat4, mjj_hig_alpha2_tth_cat4, mjj_hig_n2_tth_cat4);

mjj_hig_m0_vh_cat4[100, 70, 190];
mjj_hig_sigma_vh_cat4[50, 10, 100];
mjj_hig_alpha1_vh_cat4[1.0, 0.01, 10];
mjj_hig_n1_vh_cat4[1, 0.01, 10];
mjj_hig_alpha2_vh_cat4[1.0, 0.01, 10];
mjj_hig_n2_vh_cat4[1, 0.01, 10];
mjjHig_vh_cat4 = RooDoubleCB(mjj, mjj_hig_m0_vh_cat4, mjj_hig_sigma_vh_cat4, mjj_hig_alpha1_vh_cat4, mjj_hig_n1_vh_cat4, mjj_hig_alpha2_vh_cat4, mjj_hig_n2_vh_cat4);

mjj_hig_m0_bbh_cat4[100, 10, 180];
mjj_hig_sigma_bbh_cat4[50, 1.0, 100];
mjj_hig_alpha1_bbh_cat4[1.0, 0.01, 10];
mjj_hig_n1_bbh_cat4[1, 0.01, 10];
mjj_hig_alpha2_bbh_cat4[1.0, 0.01, 10];
mjj_hig_n2_bbh_cat4[1, 0.01, 10];
mjjHig_bbh_cat4 = RooDoubleCB(mjj, mjj_hig_m0_bbh_cat4, mjj_hig_sigma_bbh_cat4, mjj_hig_alpha1_bbh_cat4, mjj_hig_n1_bbh_cat4, mjj_hig_alpha2_bbh_cat4, mjj_hig_n2_bbh_cat4);
mgg_sig_m0_cat5[125., 122, 127];
mgg_sig_sigma_cat5[1.0, 0.1, 3.0];
mgg_sig_alpha1_cat5[1.0, 0.05, 10.0];
mgg_sig_n1_cat5[2.0, 0.1, 10.0];
mgg_sig_alpha2_cat5[1.0, 0.05, 10.0];
mgg_sig_n2_cat5[2.0, 0.1, 10.0];
mggSig_cat5 = RooDoubleCB(mgg, mgg_sig_m0_cat5, mgg_sig_sigma_cat5, mgg_sig_alpha1_cat5, mgg_sig_n1_cat5, mgg_sig_alpha2_cat5, mgg_sig_n2_cat5);

mgg_sigVBF_m0_cat5[125., 122, 127];
mgg_sigVBF_sigma_cat5[1.0, 0.1, 3.0];
mgg_sigVBF_alpha1_cat5[1.0, 0.05, 10.0];
mgg_sigVBF_n1_cat5[2.0, 0.1, 10.0];
mgg_sigVBF_alpha2_cat5[1.0, 0.05, 10.0];
mgg_sigVBF_n2_cat5[2.0, 0.1, 10.0];
mggSigVBF_cat5 = RooDoubleCB(mgg, mgg_sigVBF_m0_cat5, mgg_sigVBF_sigma_cat5, mgg_sigVBF_alpha1_cat5, mgg_sigVBF_n1_cat5, mgg_sigVBF_alpha2_cat5, mgg_sigVBF_n2_cat5);

mgg_hig_m0_ggh_cat5[124.2, 123, 125];
mgg_hig_sigma_ggh_cat5[2.0, 0.1, 3.0];
mgg_hig_alpha1_ggh_cat5[1.0, 0.05, 10.0];
mgg_hig_n1_ggh_cat5[2.0, 0.1, 10.0];
mgg_hig_alpha2_ggh_cat5[1.0, 0.05, 10.0];
mgg_hig_n2_ggh_cat5[2.0, 0.1, 10.0];
mggHig_ggh_cat5 = RooDoubleCB(mgg, mgg_hig_m0_ggh_cat5, mgg_hig_sigma_ggh_cat5, mgg_hig_alpha1_ggh_cat5, mgg_hig_n1_ggh_cat5, mgg_hig_alpha2_ggh_cat5, mgg_hig_n2_ggh_cat5);

mgg_hig_m0_tth_cat5[124.2, 123, 125];
mgg_hig_sigma_tth_cat5[2.0, 0.1, 3.0];
mgg_hig_alpha1_tth_cat5[1.0, 0.05, 10.0];
mgg_hig_n1_tth_cat5[2.0, 0.1, 10.0];
mgg_hig_alpha2_tth_cat5[1.0, 0.05, 10.0];
mgg_hig_n2_tth_cat5[2.0, 0.1, 10.0];
mggHig_tth_cat5 = RooDoubleCB(mgg, mgg_hig_m0_tth_cat5, mgg_hig_sigma_tth_cat5, mgg_hig_alpha1_tth_cat5, mgg_hig_n1_tth_cat5, mgg_hig_alpha2_tth_cat5, mgg_hig_n2_tth_cat5);

mgg_hig_m0_vh_cat5[124.2, 123, 125];
mgg_hig_sigma_vh_cat5[2.0, 0.1, 3.0];
mgg_hig_alpha1_vh_cat5[1.0, 0.05, 10.0];
mgg_hig_n1_vh_cat5[2.0, 0.1, 10.0];
mgg_hig_alpha2_vh_cat5[1.0, 0.05, 10.0];
mgg_hig_n2_vh_cat5[2.0, 0.1, 10.0];
mggHig_vh_cat5 = RooDoubleCB(mgg, mgg_hig_m0_vh_cat5, mgg_hig_sigma_vh_cat5, mgg_hig_alpha1_vh_cat5, mgg_hig_n1_vh_cat5, mgg_hig_alpha2_vh_cat5, mgg_hig_n2_vh_cat5);

mgg_hig_m0_vbf_cat5[124.2, 123, 125];
mgg_hig_sigma_vbf_cat5[2.0, 0.1, 3.0];
mgg_hig_alpha1_vbf_cat5[1.0, 0.05, 10.0];
mgg_hig_n1_vbf_cat5[2.0, 0.1, 10.0];
mgg_hig_alpha2_vbf_cat5[1.0, 0.05, 10.0];
mgg_hig_n2_vbf_cat5[2.0, 0.1, 10.0];
mggHig_vbf_cat5 = RooDoubleCB(mgg, mgg_hig_m0_vbf_cat5, mgg_hig_sigma_vbf_cat5, mgg_hig_alpha1_vbf_cat5, mgg_hig_n1_vbf_cat5, mgg_hig_alpha2_vbf_cat5, mgg_hig_n2_vbf_cat5);

mgg_hig_m0_bbh_cat5[124.2, 123, 125];
mgg_hig_sigma_bbh_cat5[2.0, 0.1, 3.0];
mgg_hig_alpha1_bbh_cat5[1.0, 0.05, 10.0];
mgg_hig_n1_bbh_cat5[2.0, 0.1, 10.0];
mgg_hig_alpha2_bbh_cat5[1.0, 0.05, 10.0];
mgg_hig_n2_bbh_cat5[2.0, 0.1, 10.0];
mggHig_bbh_cat5 = RooDoubleCB(mgg, mgg_hig_m0_bbh_cat5, mgg_hig_sigma_bbh_cat5, mgg_hig_alpha1_bbh_cat5, mgg_hig_n1_bbh_cat5, mgg_hig_alpha2_bbh_cat5, mgg_hig_n2_bbh_cat5);


mjj_sig_m0_cat5[110.0, 99, 140];
mjj_sig_sigma_cat5[10.0, 1.0, 60.0];
mjj_sig_alpha1_cat5[1.0, 0.05, 10.0];
mjj_sig_n1_cat5[2.0, 0.1, 10.0];
mjj_sig_alpha2_cat5[1.0, 0.05, 10.0];
mjj_sig_n2_cat5[2.0, 0.1, 10.0];
mjjSig_cat5 = RooDoubleCB(mjj, mjj_sig_m0_cat5, mjj_sig_sigma_cat5, mjj_sig_alpha1_cat5, mjj_sig_n1_cat5, mjj_sig_alpha2_cat5, mjj_sig_n2_cat5);

mjj_sigVBF_m0_cat5[110.0, 99, 140];
mjj_sigVBF_sigma_cat5[10.0, 1.0, 60.0];
mjj_sigVBF_alpha1_cat5[1.0, 0.05, 10.0];
mjj_sigVBF_n1_cat5[2.0, 0.1, 10.0];
mjj_sigVBF_alpha2_cat5[1.0, 0.05, 10.0];
mjj_sigVBF_n2_cat5[2.0, 0.1, 10.0];
mjjSigVBF_cat5 = RooDoubleCB(mjj, mjj_sigVBF_m0_cat5, mjj_sigVBF_sigma_cat5, mjj_sigVBF_alpha1_cat5, mjj_sigVBF_n1_cat5, mjj_sigVBF_alpha2_cat5, mjj_sigVBF_n2_cat5);

mjj_hig_par1_ggh_cat5[0.1, 0, 10];
mjj_hig_par2_ggh_cat5[0.1, 0, 10];
mjj_hig_par3_ggh_cat5[0.1, 0, 10];

mjj_hig_par1_vbf_cat5[0.1, 0, 10];
mjj_hig_par2_vbf_cat5[0.1, 0, 10];
mjj_hig_par3_vbf_cat5[0.1, 0, 10];

mjj_hig_m0_tth_cat5[100, 70, 190];
mjj_hig_sigma_tth_cat5[50, 10, 100];
mjj_hig_alpha1_tth_cat5[1.0, 0.01, 10];
mjj_hig_n1_tth_cat5[1, 0.01, 10];
mjj_hig_alpha2_tth_cat5[1.0, 0.01, 10];
mjj_hig_n2_tth_cat5[1, 0.01, 10];
mjjHig_tth_cat5 = RooDoubleCB(mjj, mjj_hig_m0_tth_cat5, mjj_hig_sigma_tth_cat5, mjj_hig_alpha1_tth_cat5, mjj_hig_n1_tth_cat5, mjj_hig_alpha2_tth_cat5, mjj_hig_n2_tth_cat5);

mjj_hig_m0_vh_cat5[100, 70, 190];
mjj_hig_sigma_vh_cat5[50, 10, 100];
mjj_hig_alpha1_vh_cat5[1.0, 0.01, 10];
mjj_hig_n1_vh_cat5[1, 0.01, 10];
mjj_hig_alpha2_vh_cat5[1.0, 0.01, 10];
mjj_hig_n2_vh_cat5[1, 0.01, 10];
mjjHig_vh_cat5 = RooDoubleCB(mjj, mjj_hig_m0_vh_cat5, mjj_hig_sigma_vh_cat5, mjj_hig_alpha1_vh_cat5, mjj_hig_n1_vh_cat5, mjj_hig_alpha2_vh_cat5, mjj_hig_n2_vh_cat5);

mjj_hig_m0_bbh_cat5[100, 10, 180];
mjj_hig_sigma_bbh_cat5[50, 1.0, 100];
mjj_hig_alpha1_bbh_cat5[1.0, 0.01, 10];
mjj_hig_n1_bbh_cat5[1, 0.01, 10];
mjj_hig_alpha2_bbh_cat5[1.0, 0.01, 10];
mjj_hig_n2_bbh_cat5[1, 0.01, 10];
mjjHig_bbh_cat5 = RooDoubleCB(mjj, mjj_hig_m0_bbh_cat5, mjj_hig_sigma_bbh_cat5, mjj_hig_alpha1_bbh_cat5, mjj_hig_n1_bbh_cat5, mjj_hig_alpha2_bbh_cat5, mjj_hig_n2_bbh_cat5);
mgg_sig_m0_cat6[125., 122, 127];
mgg_sig_sigma_cat6[1.0, 0.1, 3.0];
mgg_sig_alpha1_cat6[1.0, 0.05, 10.0];
mgg_sig_n1_cat6[2.0, 0.1, 10.0];
mgg_sig_alpha2_cat6[1.0, 0.05, 10.0];
mgg_sig_n2_cat6[2.0, 0.1, 10.0];
mggSig_cat6 = RooDoubleCB(mgg, mgg_sig_m0_cat6, mgg_sig_sigma_cat6, mgg_sig_alpha1_cat6, mgg_sig_n1_cat6, mgg_sig_alpha2_cat6, mgg_sig_n2_cat6);

mgg_sigVBF_m0_cat6[125., 122, 127];
mgg_sigVBF_sigma_cat6[1.0, 0.1, 3.0];
mgg_sigVBF_alpha1_cat6[1.0, 0.05, 10.0];
mgg_sigVBF_n1_cat6[2.0, 0.1, 10.0];
mgg_sigVBF_alpha2_cat6[1.0, 0.05, 10.0];
mgg_sigVBF_n2_cat6[2.0, 0.1, 10.0];
mggSigVBF_cat6 = RooDoubleCB(mgg, mgg_sigVBF_m0_cat6, mgg_sigVBF_sigma_cat6, mgg_sigVBF_alpha1_cat6, mgg_sigVBF_n1_cat6, mgg_sigVBF_alpha2_cat6, mgg_sigVBF_n2_cat6);

mgg_hig_m0_ggh_cat6[124.2, 123, 125];
mgg_hig_sigma_ggh_cat6[2.0, 0.1, 3.0];
mgg_hig_alpha1_ggh_cat6[1.0, 0.05, 10.0];
mgg_hig_n1_ggh_cat6[2.0, 0.1, 10.0];
mgg_hig_alpha2_ggh_cat6[1.0, 0.05, 10.0];
mgg_hig_n2_ggh_cat6[2.0, 0.1, 10.0];
mggHig_ggh_cat6 = RooDoubleCB(mgg, mgg_hig_m0_ggh_cat6, mgg_hig_sigma_ggh_cat6, mgg_hig_alpha1_ggh_cat6, mgg_hig_n1_ggh_cat6, mgg_hig_alpha2_ggh_cat6, mgg_hig_n2_ggh_cat6);

mgg_hig_m0_tth_cat6[124.2, 123, 125];
mgg_hig_sigma_tth_cat6[2.0, 0.1, 3.0];
mgg_hig_alpha1_tth_cat6[1.0, 0.05, 10.0];
mgg_hig_n1_tth_cat6[2.0, 0.1, 10.0];
mgg_hig_alpha2_tth_cat6[1.0, 0.05, 10.0];
mgg_hig_n2_tth_cat6[2.0, 0.1, 10.0];
mggHig_tth_cat6 = RooDoubleCB(mgg, mgg_hig_m0_tth_cat6, mgg_hig_sigma_tth_cat6, mgg_hig_alpha1_tth_cat6, mgg_hig_n1_tth_cat6, mgg_hig_alpha2_tth_cat6, mgg_hig_n2_tth_cat6);

mgg_hig_m0_vh_cat6[124.2, 123, 125];
mgg_hig_sigma_vh_cat6[2.0, 0.1, 3.0];
mgg_hig_alpha1_vh_cat6[1.0, 0.05, 10.0];
mgg_hig_n1_vh_cat6[2.0, 0.1, 10.0];
mgg_hig_alpha2_vh_cat6[1.0, 0.05, 10.0];
mgg_hig_n2_vh_cat6[2.0, 0.1, 10.0];
mggHig_vh_cat6 = RooDoubleCB(mgg, mgg_hig_m0_vh_cat6, mgg_hig_sigma_vh_cat6, mgg_hig_alpha1_vh_cat6, mgg_hig_n1_vh_cat6, mgg_hig_alpha2_vh_cat6, mgg_hig_n2_vh_cat6);

mgg_hig_m0_vbf_cat6[124.2, 123, 125];
mgg_hig_sigma_vbf_cat6[2.0, 0.1, 3.0];
mgg_hig_alpha1_vbf_cat6[1.0, 0.05, 10.0];
mgg_hig_n1_vbf_cat6[2.0, 0.1, 10.0];
mgg_hig_alpha2_vbf_cat6[1.0, 0.05, 10.0];
mgg_hig_n2_vbf_cat6[2.0, 0.1, 10.0];
mggHig_vbf_cat6 = RooDoubleCB(mgg, mgg_hig_m0_vbf_cat6, mgg_hig_sigma_vbf_cat6, mgg_hig_alpha1_vbf_cat6, mgg_hig_n1_vbf_cat6, mgg_hig_alpha2_vbf_cat6, mgg_hig_n2_vbf_cat6);

mgg_hig_m0_bbh_cat6[124.2, 123, 125];
mgg_hig_sigma_bbh_cat6[2.0, 0.1, 3.0];
mgg_hig_alpha1_bbh_cat6[1.0, 0.05, 10.0];
mgg_hig_n1_bbh_cat6[2.0, 0.1, 10.0];
mgg_hig_alpha2_bbh_cat6[1.0, 0.05, 10.0];
mgg_hig_n2_bbh_cat6[2.0, 0.1, 10.0];
mggHig_bbh_cat6 = RooDoubleCB(mgg, mgg_hig_m0_bbh_cat6, mgg_hig_sigma_bbh_cat6, mgg_hig_alpha1_bbh_cat6, mgg_hig_n1_bbh_cat6, mgg_hig_alpha2_bbh_cat6, mgg_hig_n2_bbh_cat6);


mjj_sig_m0_cat6[110.0, 99, 140];
mjj_sig_sigma_cat6[10.0, 1.0, 60.0];
mjj_sig_alpha1_cat6[1.0, 0.05, 10.0];
mjj_sig_n1_cat6[2.0, 0.1, 10.0];
mjj_sig_alpha2_cat6[1.0, 0.05, 10.0];
mjj_sig_n2_cat6[2.0, 0.1, 10.0];
mjjSig_cat6 = RooDoubleCB(mjj, mjj_sig_m0_cat6, mjj_sig_sigma_cat6, mjj_sig_alpha1_cat6, mjj_sig_n1_cat6, mjj_sig_alpha2_cat6, mjj_sig_n2_cat6);

mjj_sigVBF_m0_cat6[110.0, 99, 140];
mjj_sigVBF_sigma_cat6[10.0, 1.0, 60.0];
mjj_sigVBF_alpha1_cat6[1.0, 0.05, 10.0];
mjj_sigVBF_n1_cat6[2.0, 0.1, 10.0];
mjj_sigVBF_alpha2_cat6[1.0, 0.05, 10.0];
mjj_sigVBF_n2_cat6[2.0, 0.1, 10.0];
mjjSigVBF_cat6 = RooDoubleCB(mjj, mjj_sigVBF_m0_cat6, mjj_sigVBF_sigma_cat6, mjj_sigVBF_alpha1_cat6, mjj_sigVBF_n1_cat6, mjj_sigVBF_alpha2_cat6, mjj_sigVBF_n2_cat6);

mjj_hig_par1_ggh_cat6[0.1, 0, 10];
mjj_hig_par2_ggh_cat6[0.1, 0, 10];
mjj_hig_par3_ggh_cat6[0.1, 0, 10];

mjj_hig_par1_vbf_cat6[0.1, 0, 10];
mjj_hig_par2_vbf_cat6[0.1, 0, 10];
mjj_hig_par3_vbf_cat6[0.1, 0, 10];

mjj_hig_m0_tth_cat6[100, 70, 190];
mjj_hig_sigma_tth_cat6[50, 10, 100];
mjj_hig_alpha1_tth_cat6[1.0, 0.01, 10];
mjj_hig_n1_tth_cat6[1, 0.01, 10];
mjj_hig_alpha2_tth_cat6[1.0, 0.01, 10];
mjj_hig_n2_tth_cat6[1, 0.01, 10];
mjjHig_tth_cat6 = RooDoubleCB(mjj, mjj_hig_m0_tth_cat6, mjj_hig_sigma_tth_cat6, mjj_hig_alpha1_tth_cat6, mjj_hig_n1_tth_cat6, mjj_hig_alpha2_tth_cat6, mjj_hig_n2_tth_cat6);

mjj_hig_m0_vh_cat6[100, 70, 190];
mjj_hig_sigma_vh_cat6[50, 10, 100];
mjj_hig_alpha1_vh_cat6[1.0, 0.01, 10];
mjj_hig_n1_vh_cat6[1, 0.01, 10];
mjj_hig_alpha2_vh_cat6[1.0, 0.01, 10];
mjj_hig_n2_vh_cat6[1, 0.01, 10];
mjjHig_vh_cat6 = RooDoubleCB(mjj, mjj_hig_m0_vh_cat6, mjj_hig_sigma_vh_cat6, mjj_hig_alpha1_vh_cat6, mjj_hig_n1_vh_cat6, mjj_hig_alpha2_vh_cat6, mjj_hig_n2_vh_cat6);

mjj_hig_m0_bbh_cat6[100, 10, 180];
mjj_hig_sigma_bbh_cat6[50, 1.0, 100];
mjj_hig_alpha1_bbh_cat6[1.0, 0.01, 10];
mjj_hig_n1_bbh_cat6[1, 0.01, 10];
mjj_hig_alpha2_bbh_cat6[1.0, 0.01, 10];
mjj_hig_n2_bbh_cat6[1, 0.01, 10];
mjjHig_bbh_cat6 = RooDoubleCB(mjj, mjj_hig_m0_bbh_cat6, mjj_hig_sigma_bbh_cat6, mjj_hig_alpha1_bbh_cat6, mjj_hig_n1_bbh_cat6, mjj_hig_alpha2_bbh_cat6, mjj_hig_n2_bbh_cat6);
mgg_sig_m0_cat7[125., 122, 127];
mgg_sig_sigma_cat7[1.0, 0.1, 3.0];
mgg_sig_alpha1_cat7[1.0, 0.05, 10.0];
mgg_sig_n1_cat7[2.0, 0.1, 10.0];
mgg_sig_alpha2_cat7[1.0, 0.05, 10.0];
mgg_sig_n2_cat7[2.0, 0.1, 10.0];
mggSig_cat7 = RooDoubleCB(mgg, mgg_sig_m0_cat7, mgg_sig_sigma_cat7, mgg_sig_alpha1_cat7, mgg_sig_n1_cat7, mgg_sig_alpha2_cat7, mgg_sig_n2_cat7);

mgg_sigVBF_m0_cat7[125., 122, 127];
mgg_sigVBF_sigma_cat7[1.0, 0.1, 3.0];
mgg_sigVBF_alpha1_cat7[1.0, 0.05, 10.0];
mgg_sigVBF_n1_cat7[2.0, 0.1, 10.0];
mgg_sigVBF_alpha2_cat7[1.0, 0.05, 10.0];
mgg_sigVBF_n2_cat7[2.0, 0.1, 10.0];
mggSigVBF_cat7 = RooDoubleCB(mgg, mgg_sigVBF_m0_cat7, mgg_sigVBF_sigma_cat7, mgg_sigVBF_alpha1_cat7, mgg_sigVBF_n1_cat7, mgg_sigVBF_alpha2_cat7, mgg_sigVBF_n2_cat7);

mgg_hig_m0_ggh_cat7[124.2, 123, 125];
mgg_hig_sigma_ggh_cat7[2.0, 0.1, 3.0];
mgg_hig_alpha1_ggh_cat7[1.0, 0.05, 10.0];
mgg_hig_n1_ggh_cat7[2.0, 0.1, 10.0];
mgg_hig_alpha2_ggh_cat7[1.0, 0.05, 10.0];
mgg_hig_n2_ggh_cat7[2.0, 0.1, 10.0];
mggHig_ggh_cat7 = RooDoubleCB(mgg, mgg_hig_m0_ggh_cat7, mgg_hig_sigma_ggh_cat7, mgg_hig_alpha1_ggh_cat7, mgg_hig_n1_ggh_cat7, mgg_hig_alpha2_ggh_cat7, mgg_hig_n2_ggh_cat7);

mgg_hig_m0_tth_cat7[124.2, 123, 125];
mgg_hig_sigma_tth_cat7[2.0, 0.1, 3.0];
mgg_hig_alpha1_tth_cat7[1.0, 0.05, 10.0];
mgg_hig_n1_tth_cat7[2.0, 0.1, 10.0];
mgg_hig_alpha2_tth_cat7[1.0, 0.05, 10.0];
mgg_hig_n2_tth_cat7[2.0, 0.1, 10.0];
mggHig_tth_cat7 = RooDoubleCB(mgg, mgg_hig_m0_tth_cat7, mgg_hig_sigma_tth_cat7, mgg_hig_alpha1_tth_cat7, mgg_hig_n1_tth_cat7, mgg_hig_alpha2_tth_cat7, mgg_hig_n2_tth_cat7);

mgg_hig_m0_vh_cat7[124.2, 123, 125];
mgg_hig_sigma_vh_cat7[2.0, 0.1, 3.0];
mgg_hig_alpha1_vh_cat7[1.0, 0.05, 10.0];
mgg_hig_n1_vh_cat7[2.0, 0.1, 10.0];
mgg_hig_alpha2_vh_cat7[1.0, 0.05, 10.0];
mgg_hig_n2_vh_cat7[2.0, 0.1, 10.0];
mggHig_vh_cat7 = RooDoubleCB(mgg, mgg_hig_m0_vh_cat7, mgg_hig_sigma_vh_cat7, mgg_hig_alpha1_vh_cat7, mgg_hig_n1_vh_cat7, mgg_hig_alpha2_vh_cat7, mgg_hig_n2_vh_cat7);

mgg_hig_m0_vbf_cat7[124.2, 123, 125];
mgg_hig_sigma_vbf_cat7[2.0, 0.1, 3.0];
mgg_hig_alpha1_vbf_cat7[1.0, 0.05, 10.0];
mgg_hig_n1_vbf_cat7[2.0, 0.1, 10.0];
mgg_hig_alpha2_vbf_cat7[1.0, 0.05, 10.0];
mgg_hig_n2_vbf_cat7[2.0, 0.1, 10.0];
mggHig_vbf_cat7 = RooDoubleCB(mgg, mgg_hig_m0_vbf_cat7, mgg_hig_sigma_vbf_cat7, mgg_hig_alpha1_vbf_cat7, mgg_hig_n1_vbf_cat7, mgg_hig_alpha2_vbf_cat7, mgg_hig_n2_vbf_cat7);

mgg_hig_m0_bbh_cat7[124.2, 123, 125];
mgg_hig_sigma_bbh_cat7[2.0, 0.1, 3.0];
mgg_hig_alpha1_bbh_cat7[1.0, 0.05, 10.0];
mgg_hig_n1_bbh_cat7[2.0, 0.1, 10.0];
mgg_hig_alpha2_bbh_cat7[1.0, 0.05, 10.0];
mgg_hig_n2_bbh_cat7[2.0, 0.1, 10.0];
mggHig_bbh_cat7 = RooDoubleCB(mgg, mgg_hig_m0_bbh_cat7, mgg_hig_sigma_bbh_cat7, mgg_hig_alpha1_bbh_cat7, mgg_hig_n1_bbh_cat7, mgg_hig_alpha2_bbh_cat7, mgg_hig_n2_bbh_cat7);


mjj_sig_m0_cat7[110.0, 99, 140];
mjj_sig_sigma_cat7[10.0, 1.0, 60.0];
mjj_sig_alpha1_cat7[1.0, 0.05, 10.0];
mjj_sig_n1_cat7[2.0, 0.1, 10.0];
mjj_sig_alpha2_cat7[1.0, 0.05, 10.0];
mjj_sig_n2_cat7[2.0, 0.1, 10.0];
mjjSig_cat7 = RooDoubleCB(mjj, mjj_sig_m0_cat7, mjj_sig_sigma_cat7, mjj_sig_alpha1_cat7, mjj_sig_n1_cat7, mjj_sig_alpha2_cat7, mjj_sig_n2_cat7);

mjj_sigVBF_m0_cat7[110.0, 99, 140];
mjj_sigVBF_sigma_cat7[10.0, 1.0, 60.0];
mjj_sigVBF_alpha1_cat7[1.0, 0.05, 10.0];
mjj_sigVBF_n1_cat7[2.0, 0.1, 10.0];
mjj_sigVBF_alpha2_cat7[1.0, 0.05, 10.0];
mjj_sigVBF_n2_cat7[2.0, 0.1, 10.0];
mjjSigVBF_cat7 = RooDoubleCB(mjj, mjj_sigVBF_m0_cat7, mjj_sigVBF_sigma_cat7, mjj_sigVBF_alpha1_cat7, mjj_sigVBF_n1_cat7, mjj_sigVBF_alpha2_cat7, mjj_sigVBF_n2_cat7);

mjj_hig_par1_ggh_cat7[0.1, 0, 10];
mjj_hig_par2_ggh_cat7[0.1, 0, 10];
mjj_hig_par3_ggh_cat7[0.1, 0, 10];

mjj_hig_par1_vbf_cat7[0.1, 0, 10];
mjj_hig_par2_vbf_cat7[0.1, 0, 10];
mjj_hig_par3_vbf_cat7[0.1, 0, 10];

mjj_hig_m0_tth_cat7[100, 70, 190];
mjj_hig_sigma_tth_cat7[50, 10, 100];
mjj_hig_alpha1_tth_cat7[1.0, 0.01, 10];
mjj_hig_n1_tth_cat7[1, 0.01, 10];
mjj_hig_alpha2_tth_cat7[1.0, 0.01, 10];
mjj_hig_n2_tth_cat7[1, 0.01, 10];
mjjHig_tth_cat7 = RooDoubleCB(mjj, mjj_hig_m0_tth_cat7, mjj_hig_sigma_tth_cat7, mjj_hig_alpha1_tth_cat7, mjj_hig_n1_tth_cat7, mjj_hig_alpha2_tth_cat7, mjj_hig_n2_tth_cat7);

mjj_hig_m0_vh_cat7[100, 70, 190];
mjj_hig_sigma_vh_cat7[50, 10, 100];
mjj_hig_alpha1_vh_cat7[1.0, 0.01, 10];
mjj_hig_n1_vh_cat7[1, 0.01, 10];
mjj_hig_alpha2_vh_cat7[1.0, 0.01, 10];
mjj_hig_n2_vh_cat7[1, 0.01, 10];
mjjHig_vh_cat7 = RooDoubleCB(mjj, mjj_hig_m0_vh_cat7, mjj_hig_sigma_vh_cat7, mjj_hig_alpha1_vh_cat7, mjj_hig_n1_vh_cat7, mjj_hig_alpha2_vh_cat7, mjj_hig_n2_vh_cat7);

mjj_hig_m0_bbh_cat7[100, 10, 180];
mjj_hig_sigma_bbh_cat7[50, 1.0, 100];
mjj_hig_alpha1_bbh_cat7[1.0, 0.01, 10];
mjj_hig_n1_bbh_cat7[1, 0.01, 10];
mjj_hig_alpha2_bbh_cat7[1.0, 0.01, 10];
mjj_hig_n2_bbh_cat7[1, 0.01, 10];
mjjHig_bbh_cat7 = RooDoubleCB(mjj, mjj_hig_m0_bbh_cat7, mjj_hig_sigma_bbh_cat7, mjj_hig_alpha1_bbh_cat7, mjj_hig_n1_bbh_cat7, mjj_hig_alpha2_bbh_cat7, mjj_hig_n2_bbh_cat7);
mgg_sig_m0_cat8[125., 122, 127];
mgg_sig_sigma_cat8[1.0, 0.1, 3.0];
mgg_sig_alpha1_cat8[1.0, 0.05, 10.0];
mgg_sig_n1_cat8[2.0, 0.1, 10.0];
mgg_sig_alpha2_cat8[1.0, 0.05, 10.0];
mgg_sig_n2_cat8[2.0, 0.1, 10.0];
mggSig_cat8 = RooDoubleCB(mgg, mgg_sig_m0_cat8, mgg_sig_sigma_cat8, mgg_sig_alpha1_cat8, mgg_sig_n1_cat8, mgg_sig_alpha2_cat8, mgg_sig_n2_cat8);

mgg_sigVBF_m0_cat8[125., 122, 127];
mgg_sigVBF_sigma_cat8[1.0, 0.1, 3.0];
mgg_sigVBF_alpha1_cat8[1.0, 0.05, 10.0];
mgg_sigVBF_n1_cat8[2.0, 0.1, 10.0];
mgg_sigVBF_alpha2_cat8[1.0, 0.05, 10.0];
mgg_sigVBF_n2_cat8[2.0, 0.1, 10.0];
mggSigVBF_cat8 = RooDoubleCB(mgg, mgg_sigVBF_m0_cat8, mgg_sigVBF_sigma_cat8, mgg_sigVBF_alpha1_cat8, mgg_sigVBF_n1_cat8, mgg_sigVBF_alpha2_cat8, mgg_sigVBF_n2_cat8);

mgg_hig_m0_ggh_cat8[124.2, 123, 125];
mgg_hig_sigma_ggh_cat8[2.0, 0.1, 3.0];
mgg_hig_alpha1_ggh_cat8[1.0, 0.05, 10.0];
mgg_hig_n1_ggh_cat8[2.0, 0.1, 10.0];
mgg_hig_alpha2_ggh_cat8[1.0, 0.05, 10.0];
mgg_hig_n2_ggh_cat8[2.0, 0.1, 10.0];
mggHig_ggh_cat8 = RooDoubleCB(mgg, mgg_hig_m0_ggh_cat8, mgg_hig_sigma_ggh_cat8, mgg_hig_alpha1_ggh_cat8, mgg_hig_n1_ggh_cat8, mgg_hig_alpha2_ggh_cat8, mgg_hig_n2_ggh_cat8);

mgg_hig_m0_tth_cat8[124.2, 123, 125];
mgg_hig_sigma_tth_cat8[2.0, 0.1, 3.0];
mgg_hig_alpha1_tth_cat8[1.0, 0.05, 10.0];
mgg_hig_n1_tth_cat8[2.0, 0.1, 10.0];
mgg_hig_alpha2_tth_cat8[1.0, 0.05, 10.0];
mgg_hig_n2_tth_cat8[2.0, 0.1, 10.0];
mggHig_tth_cat8 = RooDoubleCB(mgg, mgg_hig_m0_tth_cat8, mgg_hig_sigma_tth_cat8, mgg_hig_alpha1_tth_cat8, mgg_hig_n1_tth_cat8, mgg_hig_alpha2_tth_cat8, mgg_hig_n2_tth_cat8);

mgg_hig_m0_vh_cat8[124.2, 123, 125];
mgg_hig_sigma_vh_cat8[2.0, 0.1, 3.0];
mgg_hig_alpha1_vh_cat8[1.0, 0.05, 10.0];
mgg_hig_n1_vh_cat8[2.0, 0.1, 10.0];
mgg_hig_alpha2_vh_cat8[1.0, 0.05, 10.0];
mgg_hig_n2_vh_cat8[2.0, 0.1, 10.0];
mggHig_vh_cat8 = RooDoubleCB(mgg, mgg_hig_m0_vh_cat8, mgg_hig_sigma_vh_cat8, mgg_hig_alpha1_vh_cat8, mgg_hig_n1_vh_cat8, mgg_hig_alpha2_vh_cat8, mgg_hig_n2_vh_cat8);

mgg_hig_m0_vbf_cat8[124.2, 123, 125];
mgg_hig_sigma_vbf_cat8[2.0, 0.1, 3.0];
mgg_hig_alpha1_vbf_cat8[1.0, 0.05, 10.0];
mgg_hig_n1_vbf_cat8[2.0, 0.1, 10.0];
mgg_hig_alpha2_vbf_cat8[1.0, 0.05, 10.0];
mgg_hig_n2_vbf_cat8[2.0, 0.1, 10.0];
mggHig_vbf_cat8 = RooDoubleCB(mgg, mgg_hig_m0_vbf_cat8, mgg_hig_sigma_vbf_cat8, mgg_hig_alpha1_vbf_cat8, mgg_hig_n1_vbf_cat8, mgg_hig_alpha2_vbf_cat8, mgg_hig_n2_vbf_cat8);

mgg_hig_m0_bbh_cat8[124.2, 123, 125];
mgg_hig_sigma_bbh_cat8[2.0, 0.1, 3.0];
mgg_hig_alpha1_bbh_cat8[1.0, 0.05, 10.0];
mgg_hig_n1_bbh_cat8[2.0, 0.1, 10.0];
mgg_hig_alpha2_bbh_cat8[1.0, 0.05, 10.0];
mgg_hig_n2_bbh_cat8[2.0, 0.1, 10.0];
mggHig_bbh_cat8 = RooDoubleCB(mgg, mgg_hig_m0_bbh_cat8, mgg_hig_sigma_bbh_cat8, mgg_hig_alpha1_bbh_cat8, mgg_hig_n1_bbh_cat8, mgg_hig_alpha2_bbh_cat8, mgg_hig_n2_bbh_cat8);


mjj_sig_m0_cat8[110.0, 99, 140];
mjj_sig_sigma_cat8[10.0, 1.0, 60.0];
mjj_sig_alpha1_cat8[1.0, 0.05, 10.0];
mjj_sig_n1_cat8[2.0, 0.1, 10.0];
mjj_sig_alpha2_cat8[1.0, 0.05, 10.0];
mjj_sig_n2_cat8[2.0, 0.1, 10.0];
mjjSig_cat8 = RooDoubleCB(mjj, mjj_sig_m0_cat8, mjj_sig_sigma_cat8, mjj_sig_alpha1_cat8, mjj_sig_n1_cat8, mjj_sig_alpha2_cat8, mjj_sig_n2_cat8);

mjj_sigVBF_m0_cat8[110.0, 99, 140];
mjj_sigVBF_sigma_cat8[10.0, 1.0, 60.0];
mjj_sigVBF_alpha1_cat8[1.0, 0.05, 10.0];
mjj_sigVBF_n1_cat8[2.0, 0.1, 10.0];
mjj_sigVBF_alpha2_cat8[1.0, 0.05, 10.0];
mjj_sigVBF_n2_cat8[2.0, 0.1, 10.0];
mjjSigVBF_cat8 = RooDoubleCB(mjj, mjj_sigVBF_m0_cat8, mjj_sigVBF_sigma_cat8, mjj_sigVBF_alpha1_cat8, mjj_sigVBF_n1_cat8, mjj_sigVBF_alpha2_cat8, mjj_sigVBF_n2_cat8);

mjj_hig_par1_ggh_cat8[0.1, 0, 10];
mjj_hig_par2_ggh_cat8[0.1, 0, 10];
mjj_hig_par3_ggh_cat8[0.1, 0, 10];

mjj_hig_par1_vbf_cat8[0.1, 0, 10];
mjj_hig_par2_vbf_cat8[0.1, 0, 10];
mjj_hig_par3_vbf_cat8[0.1, 0, 10];

mjj_hig_m0_tth_cat8[100, 70, 190];
mjj_hig_sigma_tth_cat8[50, 10, 100];
mjj_hig_alpha1_tth_cat8[1.0, 0.01, 10];
mjj_hig_n1_tth_cat8[1, 0.01, 10];
mjj_hig_alpha2_tth_cat8[1.0, 0.01, 10];
mjj_hig_n2_tth_cat8[1, 0.01, 10];
mjjHig_tth_cat8 = RooDoubleCB(mjj, mjj_hig_m0_tth_cat8, mjj_hig_sigma_tth_cat8, mjj_hig_alpha1_tth_cat8, mjj_hig_n1_tth_cat8, mjj_hig_alpha2_tth_cat8, mjj_hig_n2_tth_cat8);

mjj_hig_m0_vh_cat8[100, 70, 190];
mjj_hig_sigma_vh_cat8[50, 10, 100];
mjj_hig_alpha1_vh_cat8[1.0, 0.01, 10];
mjj_hig_n1_vh_cat8[1, 0.01, 10];
mjj_hig_alpha2_vh_cat8[1.0, 0.01, 10];
mjj_hig_n2_vh_cat8[1, 0.01, 10];
mjjHig_vh_cat8 = RooDoubleCB(mjj, mjj_hig_m0_vh_cat8, mjj_hig_sigma_vh_cat8, mjj_hig_alpha1_vh_cat8, mjj_hig_n1_vh_cat8, mjj_hig_alpha2_vh_cat8, mjj_hig_n2_vh_cat8);

mjj_hig_m0_bbh_cat8[100, 10, 180];
mjj_hig_sigma_bbh_cat8[50, 1.0, 100];
mjj_hig_alpha1_bbh_cat8[1.0, 0.01, 10];
mjj_hig_n1_bbh_cat8[1, 0.01, 10];
mjj_hig_alpha2_bbh_cat8[1.0, 0.01, 10];
mjj_hig_n2_bbh_cat8[1, 0.01, 10];
mjjHig_bbh_cat8 = RooDoubleCB(mjj, mjj_hig_m0_bbh_cat8, mjj_hig_sigma_bbh_cat8, mjj_hig_alpha1_bbh_cat8, mjj_hig_n1_bbh_cat8, mjj_hig_alpha2_bbh_cat8, mjj_hig_n2_bbh_cat8);
mgg_sig_m0_cat9[125., 122, 127];
mgg_sig_sigma_cat9[1.0, 0.1, 3.0];
mgg_sig_alpha1_cat9[1.0, 0.05, 10.0];
mgg_sig_n1_cat9[2.0, 0.1, 10.0];
mgg_sig_alpha2_cat9[1.0, 0.05, 10.0];
mgg_sig_n2_cat9[2.0, 0.1, 10.0];
mggSig_cat9 = RooDoubleCB(mgg, mgg_sig_m0_cat9, mgg_sig_sigma_cat9, mgg_sig_alpha1_cat9, mgg_sig_n1_cat9, mgg_sig_alpha2_cat9, mgg_sig_n2_cat9);

mgg_sigVBF_m0_cat9[125., 122, 127];
mgg_sigVBF_sigma_cat9[1.0, 0.1, 3.0];
mgg_sigVBF_alpha1_cat9[1.0, 0.05, 10.0];
mgg_sigVBF_n1_cat9[2.0, 0.1, 10.0];
mgg_sigVBF_alpha2_cat9[1.0, 0.05, 10.0];
mgg_sigVBF_n2_cat9[2.0, 0.1, 10.0];
mggSigVBF_cat9 = RooDoubleCB(mgg, mgg_sigVBF_m0_cat9, mgg_sigVBF_sigma_cat9, mgg_sigVBF_alpha1_cat9, mgg_sigVBF_n1_cat9, mgg_sigVBF_alpha2_cat9, mgg_sigVBF_n2_cat9);

mgg_hig_m0_ggh_cat9[124.2, 123, 125];
mgg_hig_sigma_ggh_cat9[2.0, 0.1, 3.0];
mgg_hig_alpha1_ggh_cat9[1.0, 0.05, 10.0];
mgg_hig_n1_ggh_cat9[2.0, 0.1, 10.0];
mgg_hig_alpha2_ggh_cat9[1.0, 0.05, 10.0];
mgg_hig_n2_ggh_cat9[2.0, 0.1, 10.0];
mggHig_ggh_cat9 = RooDoubleCB(mgg, mgg_hig_m0_ggh_cat9, mgg_hig_sigma_ggh_cat9, mgg_hig_alpha1_ggh_cat9, mgg_hig_n1_ggh_cat9, mgg_hig_alpha2_ggh_cat9, mgg_hig_n2_ggh_cat9);

mgg_hig_m0_tth_cat9[124.2, 123, 125];
mgg_hig_sigma_tth_cat9[2.0, 0.1, 3.0];
mgg_hig_alpha1_tth_cat9[1.0, 0.05, 10.0];
mgg_hig_n1_tth_cat9[2.0, 0.1, 10.0];
mgg_hig_alpha2_tth_cat9[1.0, 0.05, 10.0];
mgg_hig_n2_tth_cat9[2.0, 0.1, 10.0];
mggHig_tth_cat9 = RooDoubleCB(mgg, mgg_hig_m0_tth_cat9, mgg_hig_sigma_tth_cat9, mgg_hig_alpha1_tth_cat9, mgg_hig_n1_tth_cat9, mgg_hig_alpha2_tth_cat9, mgg_hig_n2_tth_cat9);

mgg_hig_m0_vh_cat9[124.2, 123, 125];
mgg_hig_sigma_vh_cat9[2.0, 0.1, 3.0];
mgg_hig_alpha1_vh_cat9[1.0, 0.05, 10.0];
mgg_hig_n1_vh_cat9[2.0, 0.1, 10.0];
mgg_hig_alpha2_vh_cat9[1.0, 0.05, 10.0];
mgg_hig_n2_vh_cat9[2.0, 0.1, 10.0];
mggHig_vh_cat9 = RooDoubleCB(mgg, mgg_hig_m0_vh_cat9, mgg_hig_sigma_vh_cat9, mgg_hig_alpha1_vh_cat9, mgg_hig_n1_vh_cat9, mgg_hig_alpha2_vh_cat9, mgg_hig_n2_vh_cat9);

mgg_hig_m0_vbf_cat9[124.2, 123, 125];
mgg_hig_sigma_vbf_cat9[2.0, 0.1, 3.0];
mgg_hig_alpha1_vbf_cat9[1.0, 0.05, 10.0];
mgg_hig_n1_vbf_cat9[2.0, 0.1, 10.0];
mgg_hig_alpha2_vbf_cat9[1.0, 0.05, 10.0];
mgg_hig_n2_vbf_cat9[2.0, 0.1, 10.0];
mggHig_vbf_cat9 = RooDoubleCB(mgg, mgg_hig_m0_vbf_cat9, mgg_hig_sigma_vbf_cat9, mgg_hig_alpha1_vbf_cat9, mgg_hig_n1_vbf_cat9, mgg_hig_alpha2_vbf_cat9, mgg_hig_n2_vbf_cat9);

mgg_hig_m0_bbh_cat9[124.2, 123, 125];
mgg_hig_sigma_bbh_cat9[2.0, 0.1, 3.0];
mgg_hig_alpha1_bbh_cat9[1.0, 0.05, 10.0];
mgg_hig_n1_bbh_cat9[2.0, 0.1, 10.0];
mgg_hig_alpha2_bbh_cat9[1.0, 0.05, 10.0];
mgg_hig_n2_bbh_cat9[2.0, 0.1, 10.0];
mggHig_bbh_cat9 = RooDoubleCB(mgg, mgg_hig_m0_bbh_cat9, mgg_hig_sigma_bbh_cat9, mgg_hig_alpha1_bbh_cat9, mgg_hig_n1_bbh_cat9, mgg_hig_alpha2_bbh_cat9, mgg_hig_n2_bbh_cat9);


mjj_sig_m0_cat9[110.0, 99, 140];
mjj_sig_sigma_cat9[10.0, 1.0, 60.0];
mjj_sig_alpha1_cat9[1.0, 0.05, 10.0];
mjj_sig_n1_cat9[2.0, 0.1, 10.0];
mjj_sig_alpha2_cat9[1.0, 0.05, 10.0];
mjj_sig_n2_cat9[2.0, 0.1, 10.0];
mjjSig_cat9 = RooDoubleCB(mjj, mjj_sig_m0_cat9, mjj_sig_sigma_cat9, mjj_sig_alpha1_cat9, mjj_sig_n1_cat9, mjj_sig_alpha2_cat9, mjj_sig_n2_cat9);

mjj_sigVBF_m0_cat9[110.0, 99, 140];
mjj_sigVBF_sigma_cat9[10.0, 1.0, 60.0];
mjj_sigVBF_alpha1_cat9[1.0, 0.05, 10.0];
mjj_sigVBF_n1_cat9[2.0, 0.1, 10.0];
mjj_sigVBF_alpha2_cat9[1.0, 0.05, 10.0];
mjj_sigVBF_n2_cat9[2.0, 0.1, 10.0];
mjjSigVBF_cat9 = RooDoubleCB(mjj, mjj_sigVBF_m0_cat9, mjj_sigVBF_sigma_cat9, mjj_sigVBF_alpha1_cat9, mjj_sigVBF_n1_cat9, mjj_sigVBF_alpha2_cat9, mjj_sigVBF_n2_cat9);

mjj_hig_par1_ggh_cat9[0.1, 0, 10];
mjj_hig_par2_ggh_cat9[0.1, 0, 10];
mjj_hig_par3_ggh_cat9[0.1, 0, 10];

mjj_hig_par1_vbf_cat9[0.1, 0, 10];
mjj_hig_par2_vbf_cat9[0.1, 0, 10];
mjj_hig_par3_vbf_cat9[0.1, 0, 10];

mjj_hig_m0_tth_cat9[100, 70, 190];
mjj_hig_sigma_tth_cat9[50, 10, 100];
mjj_hig_alpha1_tth_cat9[1.0, 0.01, 10];
mjj_hig_n1_tth_cat9[1, 0.01, 10];
mjj_hig_alpha2_tth_cat9[1.0, 0.01, 10];
mjj_hig_n2_tth_cat9[1, 0.01, 10];
mjjHig_tth_cat9 = RooDoubleCB(mjj, mjj_hig_m0_tth_cat9, mjj_hig_sigma_tth_cat9, mjj_hig_alpha1_tth_cat9, mjj_hig_n1_tth_cat9, mjj_hig_alpha2_tth_cat9, mjj_hig_n2_tth_cat9);

mjj_hig_m0_vh_cat9[100, 70, 190];
mjj_hig_sigma_vh_cat9[50, 10, 100];
mjj_hig_alpha1_vh_cat9[1.0, 0.01, 10];
mjj_hig_n1_vh_cat9[1, 0.01, 10];
mjj_hig_alpha2_vh_cat9[1.0, 0.01, 10];
mjj_hig_n2_vh_cat9[1, 0.01, 10];
mjjHig_vh_cat9 = RooDoubleCB(mjj, mjj_hig_m0_vh_cat9, mjj_hig_sigma_vh_cat9, mjj_hig_alpha1_vh_cat9, mjj_hig_n1_vh_cat9, mjj_hig_alpha2_vh_cat9, mjj_hig_n2_vh_cat9);

mjj_hig_m0_bbh_cat9[100, 10, 180];
mjj_hig_sigma_bbh_cat9[50, 1.0, 100];
mjj_hig_alpha1_bbh_cat9[1.0, 0.01, 10];
mjj_hig_n1_bbh_cat9[1, 0.01, 10];
mjj_hig_alpha2_bbh_cat9[1.0, 0.01, 10];
mjj_hig_n2_bbh_cat9[1, 0.01, 10];
mjjHig_bbh_cat9 = RooDoubleCB(mjj, mjj_hig_m0_bbh_cat9, mjj_hig_sigma_bbh_cat9, mjj_hig_alpha1_bbh_cat9, mjj_hig_n1_bbh_cat9, mjj_hig_alpha2_bbh_cat9, mjj_hig_n2_bbh_cat9);
