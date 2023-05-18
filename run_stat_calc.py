import subprocess

dirs = [
    "./results/twophase/grid_20_twophase",
    "./results/twophase/uniform_50_twophase",
    "./results/twophase/cliques_50_twophase",
    "./results/twophase/conv_att48_twophase",
    "./results/twophase/conv_berlin52_twophase",
    "./results/twophase/cliques_80_twophase",
    "./results/twophase/conv_pr76_twophase",
]

OUT_DIR = "./stat_results/twophase/"
LIMIT = 100

for dir in dirs:
    output = OUT_DIR + dir.split("/")[-1] + "_stats.csv"
    print("Now running :", dir)
    command = [
        "python", "./stat_calculator/stat_calc_multi.py", dir, "-l",
        str(LIMIT), "-o", output
    ]
    # print(" ".join(command))
    subprocess.run(command)
