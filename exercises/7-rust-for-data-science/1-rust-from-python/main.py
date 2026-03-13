import pprint
from collections import defaultdict
from dataclasses import dataclass
from itertools import batched
from typing import Dict, Iterator

from data import CRAB, DNA_TO_AMINO
from rust_dna import count_kmers, assert_valid_dna, Sequence, OpenReadingFrame, decode_orf

# 0. Setup
#
# $ cd rust-dna
# $ uv run maturin develop --uv
# $ cd ..
# $ uv run pytest



def all_orfs(sequence: Sequence) -> Iterator[OpenReadingFrame]:
    i = 0
    while i < len(sequence):
        if str(sequence)[i:].startswith("ATG"):
            orf = decode_orf(sequence, i)
            i = orf.end
            yield orf
            continue
        i += 1


def main():
    cnt = count_kmers(CRAB)

    print("This crab sequence contains the following three-mers:")
    pprint.pp(cnt, sort_dicts=True)

    print("This crab has the following longer proteins encoded:")
    seq = Sequence(CRAB)
    for orf in all_orfs(seq):
        if len(orf.decoded) > 10:
            print(orf.decoded)


if __name__ == "__main__":
    main()
