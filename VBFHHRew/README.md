# Prepare KL and C2V weights in VBFHH signal samples
1. Indicate inside file add_branch_KL.C path to input directory with samples and choice variant for create weight (KL or C2V) 

2. Create weights:
```
root -l -q add_branch_KL.C
```

# Prepare KL weights in ggHH signal samples
1. Check and if not corrected delete from signal ggHH file (2018 year) event with bad weight
```
root -l -q delete_event.C
```

2. Indicate inside file add_branch_ggHHKL.C path to input directory with samples

3. Create weights:
```
root -l -q add_branch_ggHHKL.C
```

