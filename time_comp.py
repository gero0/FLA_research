import subprocess
import os

files = [
    "./test_cases/uniform/uniform_7/matrix.txt",
    "./test_cases/uniform/uniform_8/matrix.txt",
    "./test_cases/uniform/uniform_9/matrix.txt",
    "./test_cases/uniform/uniform_10/matrix.txt",
    "./test_cases/uniform/uniform_11/matrix.txt",
    "./test_cases/grid/grid_7/matrix.txt",
    "./test_cases/grid/grid_8/matrix.txt",
    "./test_cases/grid/grid_9/matrix.txt",
    "./test_cases/grid/grid_10/matrix.txt",
    "./test_cases/grid/grid_11/matrix.txt",
    "./test_cases/cliques/cliques_7/matrix.txt",
    "./test_cases/cliques/cliques_8/matrix.txt",
    "./test_cases/cliques/cliques_9/matrix.txt",
    "./test_cases/cliques/cliques_10/matrix.txt",
    "./test_cases/cliques/cliques_11/matrix.txt",
]

snowball_settings = [
    (1000, 100, 3),
    (100, 100, 3),
    (10, 100, 3),
    (1, 100, 3),
]

tp_settings = [
    (1, 10000, 10000),
    (1, 1000, 1000),
    (1, 100, 100),
    (1, 10, 10),
]

SAMPLE_TRESHOLD = 10000
D = 2
LIMIT = 100

OUT_DIR = "results/time_comp/"
STAT_OUT_DIR = "stat_results/time_comp/"

for file in files:
    dir = os.path.dirname(file)
    name = dir.split('/')[-1]
    if name == "matrix.txt":
        name = file.split('/')[-1]
    else:
        name = name.split(".")[0]

    for (ITERS, N_MAX, E_ATT) in tp_settings:
        N_ATT = N_MAX
        dir = f"{OUT_DIR}{name}_twophase_{N_MAX}"
        command = [
            "./tsp_samplers/target/release/tsp_samplers", file, dir, "tp",
            str(ITERS),
            str(N_MAX),
            str(N_ATT),
            str(E_ATT),
            str(D)
        ]
        print(command)
        subprocess.run(command)
        output = f"{STAT_OUT_DIR}{name}_twophase_{N_MAX}.csv"
        command = [
            "python", "./stat_calculator/main.py", dir, "-l",
            str(LIMIT), "-o", output
        ]
        subprocess.run(command)

    for (WALK_LEN, N_EDGES, DEPTH) in snowball_settings:
        dir = f"{OUT_DIR}{name}_snowball_{WALK_LEN}"
        command = [
            "./tsp_samplers/target/release/tsp_samplers",
            file,
            dir,
            "snowball",
            str(WALK_LEN),
            str(N_EDGES),
            str(DEPTH),
            str(D),
            str(SAMPLE_TRESHOLD),
        ]

        print(command)
        subprocess.run(command)
        output = f"{STAT_OUT_DIR}{name}_snowball_{WALK_LEN}.csv"
        command = [
            "python", "./stat_calculator/main.py", dir, "-l",
            str(LIMIT), "-o", output
        ]
        subprocess.run(command)