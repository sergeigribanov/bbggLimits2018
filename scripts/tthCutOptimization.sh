#!/bin/bash
nodes="
SM
"

num=0
max=25
cut=0.3
bin=0.02

for arg in $nodes:
do
    node=$arg
    echo running limits for node $node
    while [ $num -lt $max ]
    do 
    cut=`echo $cut + $bin | bc`

    echo ttH_cut $cut

    echo === step : creating workspace and datacards ===
    ./runLimit.py -f conf_default_local.json --node=$node -o outDir_LIM_node_${node}_ttHcut_$num --analyticalRW --ttHTaggerCut=$cut

    echo === step : running combine for limits ===
    sh scripts/Analyzer.sh outDir_LIM_node_${node}_ttHcut_$num    

    num=$(( $num + 1 ))
    done

    echo done with node $node
done
 
