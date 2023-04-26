import subprocess
import os

dirs = [
    "./results/snowball/cliques_10_snowball",
    "./results/snowball/cliques_11_snowball",
    "./results/snowball/cliques_12_snowball",
    "./results/snowball/cliques_20_snowball",
    "./results/snowball/cliques_50_snowball",
    "./results/snowball/cliques_100_snowball",

    "./results/snowball/uniform_10_snowball",
    "./results/snowball/uniform_11_snowball",
    "./results/snowball/uniform_12_snowball",
    "./results/snowball/uniform_20_snowball",
    "./results/snowball/uniform_50_snowball",
    "./results/snowball/uniform_100_snowball",

    "./results/snowball/grid_10_snowball",
    "./results/snowball/grid_11_snowball",
    "./results/snowball/grid_12_snowball",
    "./results/snowball/grid_20_snowball",
    "./results/snowball/grid_50_snowball",
    "./results/snowball/grid_100_snowball",
]

OUT_DIR = "./stat_results/"
LIMIT = 100

for dir in dirs:
    output = OUT_DIR + dir.split("/")[-1] + "_stats.csv"
    print("Now running :", dir)
    command = ["python", "./stat_calculator/main.py", dir, "-l", str(LIMIT), "-o", output]
    print(command)
    subprocess.run(command)
