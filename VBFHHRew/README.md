# Prepare KL and C2V weights in VBFHH signal samples
1. Indicate inside file add_branch_KL.C path to input directory with samples and choice variant for create weight (KL or C2V) 

2. Create weights:
```
root -l -q add_branch_KL.C
```

# Prepare KL weights in ggHH signal samples
1. Indicate inside file add_branch_ggHHKL.C path to input directory with samples

2. Create weights:
```
root -l -q add_branch_ggHHKL.C
```

