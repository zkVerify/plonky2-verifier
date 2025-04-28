#!/bin/bash

set -e

BINARY="./target/release/plonky2-fibonacci-bench"
start=2
end=23

for power in $(seq $start $end); do
  for compression in "compressed" "uncompressed"; do
    for hash in "keccak" "poseidon"; do
      echo "Power=$power, compression=$compression, hash=$hash"
      if [ "$compression" = "compressed" ]; then
        compression_flag="--compress"
      else
        compression_flag=""
      fi
      output=$("$BINARY" \
        --power $power \
        --hash $hash \
        $compression_flag)
      degree_bits=$(echo "$output" | grep "degree_bits" | cut -d'=' -f2 | tr -d ' ')
      echo "Extracted degree_bits: $degree_bits"
      TARGET_DIR="benchmark_data/degree_$degree_bits/$compression/$hash"
      mkdir -p $TARGET_DIR
      mv proof.bin pubs.bin vk.bin $TARGET_DIR
    done
  done
done

echo "Benchmark data generation complete."