import subprocess
import os

files = [
    # "./test_cases/grid/grid_80/matrix.txt",
    # "./test_cases/uniform/uniform_80/matrix.txt",
    # "./test_cases/cliques/cliques_80/matrix.txt",
    # "./test_cases/tsplib/conv_pr76.txt",
    # "./test_cases/tsplib/conv_eil76.txt",
    # "./test_cases/tsplib/conv_rat99.txt",
    # "./test_cases/tsplib/conv_bier127.txt",
    # "./test_cases/cliques/cliques_100/matrix.txt",
    # "./test_cases/uniform/uniform_100/matrix.txt",
    # "./test_cases/grid/grid_100/matrix.txt",

    # "./test_cases/uniform/uniform_7/matrix.txt",
    # "./test_cases/uniform/uniform_8/matrix.txt",
    # "./test_cases/uniform/uniform_9/matrix.txt",
    # "./test_cases/uniform/uniform_10/matrix.txt",
    # "./test_cases/uniform/uniform_11/matrix.txt",
    # "./test_cases/grid/grid_7/matrix.txt",
    # "./test_cases/grid/grid_8/matrix.txt",
    # "./test_cases/grid/grid_9/matrix.txt",
    # "./test_cases/grid/grid_10/matrix.txt",
    # "./test_cases/grid/grid_11/matrix.txt",
    # "./test_cases/cliques/cliques_7/matrix.txt",
    # "./test_cases/cliques/cliques_8/matrix.txt",
    # "./test_cases/cliques/cliques_9/matrix.txt",
    # "./test_cases/cliques/cliques_10/matrix.txt",
    # "./test_cases/cliques/cliques_11/matrix.txt",
    "./test_cases/tsplib/conv_burma14.txt",
    "./test_cases/tsplib/conv_ulysses22.txt",

    "./test_cases/cliques/cliques_20/matrix.txt",
    "./test_cases/uniform/uniform_20/matrix.txt",
    "./test_cases/grid/grid_20/matrix.txt",
]

# OUT_DIR = "./results/"
OUT_DIR = "./results/twophase/per1/"

# ITERS = 100
# N_MAX = 1000
# N_ATT = 1000
# E_ATT = 1000
ITERS = 1000
N_MAX = 1
N_ATT = 1000
E_ATT = 10

# ITERS = 100
# N_MAX = 100
# N_ATT = 1000
# E_ATT = 100
D = 2

for file in files:
    dir = os.path.dirname(file)

    name = file.split('/')[-1]
    if name == "matrix.txt":
        name = dir.split('/')[-1]
    else:
        name = name.split(".")[0]

    output = OUT_DIR + name + "_twophase"
    print("Now running :", file)
    command = [
        "./tsp_samplers/target/release/tsp_samplers", file, output, "tp",
        str(ITERS),
        str(N_MAX),
        str(N_ATT),
        str(E_ATT),
        str(D)
    ]
    print(command)
    subprocess.run(command)
