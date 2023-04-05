import subprocess
import os

files = [
    "./test_cases/cliques/cliques_6/matrix.txt",
    "./test_cases/cliques/cliques_7/matrix.txt",
    "./test_cases/cliques/cliques_8/matrix.txt",
    "./test_cases/cliques/cliques_9/matrix.txt",
    "./test_cases/cliques/cliques_10/matrix.txt",
    "./test_cases/cliques/cliques_11/matrix.txt",

    "./test_cases/uniform/uniform_6/matrix.txt",
    "./test_cases/uniform/uniform_7/matrix.txt",
    "./test_cases/uniform/uniform_8/matrix.txt",
    "./test_cases/uniform/uniform_9/matrix.txt",
    "./test_cases/uniform/uniform_10/matrix.txt",
    "./test_cases/uniform/uniform_11/matrix.txt",

    "./test_cases/grid/grid_6/matrix.txt",
    "./test_cases/grid/grid_7/matrix.txt",
    "./test_cases/grid/grid_8/matrix.txt",
    "./test_cases/grid/grid_9/matrix.txt",
    "./test_cases/grid/grid_10/matrix.txt",
    "./test_cases/grid/grid_11/matrix.txt",
]

OUT_DIR = "./results/"

for file in files:
    dir = os.path.dirname(file)
    name = dir.split('/')[-1]
    output = OUT_DIR + name + "_exhaustive"
    print("Now running :", name)
    subprocess.run(["./tsp_samplers/target/release/tsp_samplers", file, "0", output, "exhaustive"])