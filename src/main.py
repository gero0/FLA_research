from numpy import append
import igraph as ig
from igraph import Graph
from helpers import create_distance_matrix, split_edge_data
import tsplib95 as tsp
import matplotlib.pyplot as plt
import json
from os import listdir
from os.path import isfile, join
import natsort

from stats import subsinks_count


def main():
    path = "snowball_latest"
    files = [f for f in listdir(path) if isfile(join(path, f))]
    files = natsort.natsorted(files)

    ss_x = []
    ss_y = []

    for file in files[98:]:
        with open(join(path, file)) as f:
            print(f"Now procesing file {file}")
            f = json.load(f)
            ss_x.append(f["hc_count"])
            v = subsinks_count(f['nodes'], f['edges'])
            ss_y.append(v)

    fig = plt.figure()
    ax1 = fig.add_subplot(111)

    ax1.scatter(ss_x, ss_y, s=10, c='b', marker="s", label='snowball')
    plt.savefig("fig.png")

if __name__ == "__main__":
    main()