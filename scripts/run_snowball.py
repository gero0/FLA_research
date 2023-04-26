import subprocess
import os

files = [
    "./test_cases/tsplib/conv_burma14.txt",
    "./test_cases/tsplib/conv_ulysses22.txt",
    "./test_cases/tsplib/conv_att48.txt",
    "./test_cases/tsplib/conv_berlin52.txt",
    "./test_cases/tsplib/conv_rat99.txt",
    "./test_cases/tsplib/conv_bier127.txt",

    "./test_cases/cliques/cliques_10/matrix.txt",
    "./test_cases/cliques/cliques_11/matrix.txt",
    "./test_cases/cliques/cliques_12/matrix.txt",
    "./test_cases/cliques/cliques_20/matrix.txt",
    "./test_cases/cliques/cliques_50/matrix.txt",
    "./test_cases/cliques/cliques_100/matrix.txt",

    "./test_cases/uniform/uniform_10/matrix.txt",
    "./test_cases/uniform/uniform_11/matrix.txt",
    "./test_cases/uniform/uniform_12/matrix.txt",
    "./test_cases/uniform/uniform_20/matrix.txt",
    "./test_cases/uniform/uniform_50/matrix.txt",
    "./test_cases/uniform/uniform_100/matrix.txt",
    
    "./test_cases/grid/grid_10/matrix.txt",
    "./test_cases/grid/grid_11/matrix.txt",
    "./test_cases/grid/grid_12/matrix.txt",
    "./test_cases/grid/grid_20/matrix.txt",
    "./test_cases/grid/grid_50/matrix.txt",
    #Warning: this one generates almost 250Gb of output data!
    #"./test_cases/grid/grid_100/matrix.txt",
]

OUT_DIR = "./results/"

WALK_LEN = 10000
N_EDGES = 100
DEPTH = 3
D = 2
SAMPLE_TRESHOLD = 1000
SEED = 1973

for file in files:
    dir = os.path.dirname(file)

    name = file.split('/')[-1]
    if name == "matrix.txt":
        name = dir.split('/')[-1]
    else:
        name = name.split(".")[0]

    output = OUT_DIR + name + "_snowball"
    print("Now running :", file)
    command = [
        "./tsp_samplers/target/release/tsp_samplers", file, output, "snowball",
        str(WALK_LEN),
        str(N_EDGES),
        str(DEPTH),
        str(D),
        str(SAMPLE_TRESHOLD),
        str(SEED)
    ]
    print(command)
    subprocess.run(command)
