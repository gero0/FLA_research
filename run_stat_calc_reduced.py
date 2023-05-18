import subprocess
from pathlib import Path

dirs = [
    "./results/twophase/grid_20_twophase",
    "./results/twophase/uniform_50_twophase",
    "./results/twophase/cliques_50_twophase",
    "./results/twophase/conv_att48_twophase",
    "./results/twophase/conv_berlin52_twophase",
    "./results/twophase/cliques_80_twophase",
    "./results/twophase/conv_pr76_twophase",
]

OUT_DIR = "./stat_results/twophase/reduced/"
LIMIT = 100

p = Path(OUT_DIR).mkdir(parents=True, exist_ok=True)

selected_stats = [
    "num_sinks", "num_sources", "edge_to_node", "distLO", "conrel",
    "assortativity_deg", "clustering", "density", "avg_path_len",
    "reciprocity", "avg_fitness", "funnels", "out_degree", "in_degree",
    "paths_to_go", "avg_loop_weight"
]

for dir in dirs:
    output = OUT_DIR + dir.split("/")[-1] + "_stats.csv"
    print("Now running :", dir)
    command = [
        "python", "./stat_calculator/stat_calc_multi.py", dir, "-l",
        str(LIMIT), "-o", output, "--stats"
    ]
    for stat in selected_stats:
        command.append(stat)
    print(" ".join(command))
    subprocess.run(command)
