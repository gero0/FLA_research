import os
from os.path import isfile, join
import natsort
import pandas as pd

dirs = [
    "./results/twophase/uniform_50_twophase",
    "./results/twophase/cliques_50_twophase",
    "./results/twophase/conv_att48_twophase",
    "./results/twophase/conv_berlin52_twophase",
    "./results/twophase/cliques_80_twophase",
    "./results/twophase/conv_pr76_twophase",
]

for path in dirs:
    print(f"Now processing {path}")
    missed = []
    files = [f for f in os.listdir(path) if isfile(join(path, f))]
    files = natsort.natsorted(files)
    for f in files:
        with open(join(path, f)) as file:
            for i, line in enumerate(file):
                if i == 5:
                    value = line.split(":")[1][:-2]
                    missed.append(int(value))
                    break

    stat_path = path.replace("/results/", "/stat_results/").replace(
        "/twophase/", "/twophase/reduced/") + "_stats.csv"
    df = pd.read_csv(stat_path, delimiter=";", index_col=0)
    df["missed"] = missed
    out_path = stat_path.replace("reduced", "reduced/ms")

    df.to_csv(out_path, sep=";")
    corr = df.corr()
    corr.to_csv(out_path.replace(".csv", "_corr.csv"), sep=";")