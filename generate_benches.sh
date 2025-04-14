#!/bin/bash

set -e

BINARY="./target/release/plonky2-fibonacci-bench"
start=2
end=23

for degree in $(seq $start $end); do
  for compression in "compressed" "uncompressed"; do
    for hash in "keccak" "poseidon"; do
      echo "Running degree=$degree, compression=$compression, hash=$hash"
      TARGET_DIR="benchmark_data/degree_$degree/$compression/$hash"
      mkdir -p $TARGET_DIR
      if [ "$compression" = "compressed" ]; then
        compression_flag="--compress"
      else
        compression_flag=""
      fi
      "$BINARY" \
        --power $degree \
        --hash $hash \
        $compression_flag
      mv proof.bin pubs.bin vk.bin $TARGET_DIR
    done
  done
done

echo "Benchmark data generation complete."