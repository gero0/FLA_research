import subprocess
import os

dirs = [
    "./stat_results/snowball/cliques_50_snowball",
    "./stat_results/snowball/cliques_80_snowball",
    "./stat_results/snowball/cliques_100_snowball",

    "./stat_results/snowball/uniform_50_snowball",
    "./stat_results/snowball/uniform_80_snowball",
    "./stat_results/snowball/uniform_100_snowball",

    "./stat_results/snowball/grid_50_snowball",
    "./stat_results/snowball/grid_80_snowball",
    "./stat_results/snowball/grid_100_snowball",

    "./stat_results/snowball/conv_att48_snowball",
    "./stat_results/snowball/conv_berlin52_snowball",
    "./stat_results/snowball/conv_rat99_snowball",
    "./stat_results/snowball/conv_bier127_snowball",

    "./stat_results/snowball/per100/grid_20_snowball",
    "./stat_results/snowball/per100/cliques_20_snowball",
    "./stat_results/snowball/per100/cliques_50_snowball",
]

OUT_DIR = "./plots/snowball/"

for dir in dirs:
    output = OUT_DIR + dir.split("/")[-1]
    command = ["python", "./stat_calculator/plotting.py", dir+"_stats.csv", dir+"_stats_corr.csv", "-o", output]
    print(" ".join(command))
    subprocess.run(command)
