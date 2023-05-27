import subprocess
import os

dirs = [
    # "./results/snowball/cliques_10_snowball",
    # "./results/snowball/cliques_11_snowball",
    # "./results/snowball/cliques_12_snowball",
    # "./results/snowball/cliques_20_snowball",
    # "./results/snowball/cliques_50_snowball",
    # "./results/snowball/cliques_100_snowball",

    # "./results/snowball/uniform_10_snowball",
    # "./results/snowball/uniform_11_snowball",
    # "./results/snowball/uniform_12_snowball",
    # "./results/snowball/uniform_20_snowball",
    # "./results/snowball/uniform_50_snowball",
    # "./results/snowball/uniform_100_snowball",

    # "./results/snowball/grid_10_snowball",
    # "./results/snowball/grid_11_snowball",
    # "./results/snowball/grid_12_snowball",
    # "./results/snowball/grid_20_snowball",
    # "./results/snowball/grid_50_snowball",
    # "./results/snowball/grid_100_snowball",

    # "./results/snowball/conv_burma14_snowball"
    # "./results/snowball/conv_ulysses22_snowball"
    # "./results/snowball/conv_att48_snowball",
    # "./results/snowball/conv_berlin52_snowball",
    # "./results/snowball/conv_rat99_snowball",
    # "./results/snowball/conv_bier127_snowball",

    # "./results/snowball/per100/cliques_6_snowball",
    # "./results/snowball/per100/cliques_7_snowball",
    # "./results/snowball/per100/cliques_8_snowball",
    # "./results/snowball/per100/cliques_9_snowball",
    # "./results/snowball/per100/cliques_10_snowball",
    # "./results/snowball/per100/cliques_11_snowball",
    # "./results/snowball/per100/cliques_12_snowball",
    # "./results/snowball/per100/cliques_20_snowball",
    # "./results/snowball/per100/cliques_50_snowball",

    # "./results/snowball/per100/grid_6_snowball",
    # "./results/snowball/per100/grid_7_snowball",
    # "./results/snowball/per100/grid_8_snowball",
    # "./results/snowball/per100/grid_9_snowball",
    # "./results/snowball/per100/grid_10_snowball",
    # "./results/snowball/per100/grid_11_snowball",
    # "./results/snowball/per100/grid_12_snowball",
    # "./results/snowball/per100/grid_20_snowball",

    # "./results/snowball/per100/uniform_6_snowball",
    # "./results/snowball/per100/uniform_7_snowball",
    # "./results/snowball/per100/uniform_8_snowball",
    # "./results/snowball/per100/uniform_9_snowball",
    # "./results/snowball/per100/uniform_10_snowball",
    # "./results/snowball/per100/uniform_11_snowball",
    # "./results/snowball/per100/uniform_12_snowball",
    # "./results/snowball/per100/uniform_20_snowball",
    # "./results/snowball/per100/conv_ulysses22_snowball",

    # "./results/exhaustive/cliques_6_exhaustive",
    # "./results/exhaustive/cliques_7_exhaustive",
    # "./results/exhaustive/cliques_8_exhaustive",
    # "./results/exhaustive/cliques_9_exhaustive",
    # "./results/exhaustive/cliques_10_exhaustive",
    # "./results/exhaustive/cliques_11_exhaustive",
    # "./results/exhaustive/grid_6_exhaustive",
    # "./results/exhaustive/grid_7_exhaustive",
    # "./results/exhaustive/grid_8_exhaustive",
    # "./results/exhaustive/grid_9_exhaustive",
    # "./results/exhaustive/grid_10_exhaustive",
    # "./results/exhaustive/grid_11_exhaustive",
    # "./results/exhaustive/uniform_6_exhaustive",
    # "./results/exhaustive/uniform_7_exhaustive",
    # "./results/exhaustive/uniform_8_exhaustive",
    # "./results/exhaustive/uniform_9_exhaustive",
    # "./results/exhaustive/uniform_10_exhaustive",
    # "./results/exhaustive/uniform_11_exhaustive",

    # "./results/snowball/per1/cliques_6_snowball",
    # "./results/snowball/per1/cliques_7_snowball",
    # "./results/snowball/per1/cliques_8_snowball",
    # "./results/snowball/per1/cliques_9_snowball",
    # "./results/snowball/per1/cliques_10_snowball",
    # "./results/snowball/per1/cliques_11_snowball",
    # "./results/snowball/per1/cliques_12_snowball",
    # "./results/snowball/per1/uniform_6_snowball",
    # "./results/snowball/per1/uniform_7_snowball",
    # "./results/snowball/per1/uniform_8_snowball",
    # "./results/snowball/per1/uniform_9_snowball",
    # "./results/snowball/per1/uniform_10_snowball",
    # "./results/snowball/per1/uniform_11_snowball",
    # "./results/snowball/per1/uniform_12_snowball",
    # "./results/snowball/per1/grid_6_snowball",
    # "./results/snowball/per1/grid_7_snowball",
    # "./results/snowball/per1/grid_8_snowball",
    # "./results/snowball/per1/grid_9_snowball",
    # "./results/snowball/per1/grid_10_snowball",
    # "./results/snowball/per1/grid_11_snowball",
    # "./results/snowball/per1/conv_burma14_snowball",
    # "./results/snowball/per1/conv_ulysses22_snowball",
    # "./results/snowball/per1/grid_12_snowball",
    # "./results/snowball/grid_80_snowball",
    # "./results/snowball/uniform_80_snowball",
    # "./results/snowball/cliques_80_snowball",
    # "./results/snowball/conv_eil76_snowball",
    # "./results/snowball/conv_pr76_snowball",


    # "./results/twophase/grid_80_twophase",
    # "./results/twophase/uniform_80_twophase",
    # "./results/twophase/cliques_80_twophase",
    # "./results/twophase/conv_eil76_twophase",
    # "./results/twophase/conv_pr76_twophase",

    # "./results/twophase/per1/cliques_7_twophase",
    # "./results/twophase/per1/cliques_8_twophase",
    # "./results/twophase/per1/cliques_9_twophase",
    # "./results/twophase/per1/cliques_10_twophase",
    # "./results/twophase/per1/cliques_11_twophase",
    # "./results/twophase/per1/uniform_7_twophase",
    # "./results/twophase/per1/uniform_8_twophase",
    # "./results/twophase/per1/uniform_9_twophase",
    # "./results/twophase/per1/uniform_10_twophase",
    # "./results/twophase/per1/uniform_11_twophase",
    # "./results/twophase/per1/grid_7_twophase",
    # "./results/twophase/per1/grid_8_twophase",
    # "./results/twophase/per1/grid_9_twophase",
    # "./results/twophase/per1/grid_10_twophase",
    # "./results/twophase/per1/grid_11_twophase",

    # "./results/twophase/per1/grid_20_twophase",
    # "./results/twophase/per1/conv_burma14_twophase",
    # "./results/twophase/per1/conv_ulysses22_twophase",
    # "./results/twophase/per1/cliques_20_twophase",
    # "./results/twophase/per1/uniform_20_twophase",

    "./results/snowball/per1/grid_20_snowball",
    "./results/snowball/per1/conv_burma14_snowball",
    "./results/snowball/per1/conv_ulysses22_snowball",
    "./results/snowball/per1/cliques_20_snowball",
    "./results/snowball/per1/uniform_20_snowball",

    # "./results/twophase/per100/conv_burma14_twophase",
    # "./results/twophase/per100/conv_ulysses22_twophase",
    # "./results/twophase/per100/cliques_20_twophase",
    # "./results/twophase/per100/uniform_20_twophase",
]

# OUT_DIR = "./stat_results/"
OUT_DIR = "./stat_results/snowball/per1/"
LIMIT = 1000

# processes = []

for dir in dirs:
    output = OUT_DIR + dir.split("/")[-1] + "_stats.csv"
    # print("Now running :", dir)
    command = [
        "python", "./stat_calculator/stat_calc_multi.py", dir, "-l",
        str(LIMIT), "-o", output
    ]
    print(" ".join(command))
    p = subprocess.run(command)
    # processes.append(p)

# for p in processes:
#     p.wait()