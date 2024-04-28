#!/bin/bash
PARALLEL_VALUES=(true false)
for rustFlags in "${PARALLEL_VALUES[@]}"; do
    echo "Running with PARALLEL=$rustFlags"	
    if [ "$rustFlags" = "true" ]; then
       cfg_flag="--cfg parallel"
    else
        cfg_flag=""
    fi
    cargo $cfg_flag bench
done