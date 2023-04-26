import subprocess
import os

files = [
    "./test_cases/tsplib/conv_rat99.txt",
    # "./test_cases/tsplib/conv_bier127.txt",
    # "./test_cases/cliques/cliques_100/matrix.txt",
    # "./test_cases/uniform/uniform_100/matrix.txt",
    # "./test_cases/grid/grid_100/matrix.txt",
]

OUT_DIR = "./results/"

ITERS = 60
N_MAX = 1000
N_ATT = 1000
E_ATT = 1000
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
