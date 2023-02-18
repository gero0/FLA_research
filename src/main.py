from os import listdir
from os.path import isfile, join

import argparse
import natsort
import pandas as pd
import json
from igraph import Graph
from helpers import split_edge_data

from stats import *


def load(path):
    files = [f for f in listdir(path) if isfile(join(path, f))]
    files = natsort.natsorted(files)
    return files


def process_file(path):
    with open(path) as f:
        print(f"Now procesing file {path}")
        f = json.load(f)

        nodes = f["nodes"]
        nodes.sort(key=lambda x: x[2])

        edges = f["edges"]

        return (nodes, edges, f["time_ms"], f["hc_count"])


def calculate_stats(nodes, edges, stats):
    results = {}
    results["node_count"] = len(nodes)
    results["edge_count"] = len(edges)

    if "num_subsinks" in stats:
        results["num_subsinks"] = subsinks_count(nodes, edges)

    if "edge_to_node" in stats:
        results["edge_to_node"] = e2n_ratio(nodes, edges)

    if "distLO" in stats:
        results["distLO"] = distLO(nodes, edges)

    if "conrel" in stats:
        results["conrel"] = conrel(nodes, edges)

    #Graph stats
    (edge_list, weight_list) = split_edge_data(edges)
    g = Graph(n=len(nodes),
              edges=edge_list,
              edge_attrs={"weight": weight_list})

    if "assortativity" in stats:
        results["assortativity"] = g.assortativity_degree()

    if "clustering" in stats:
        results["clustering"] = g.transitivity_undirected()

    if "cliques" in stats:
        results["cliques"] = g.clique_number()

    if "density" in stats:
        results["density"] = g.density()

    return results


def main():
    parser = argparse.ArgumentParser(prog="lonstats",
                                     description="What the program does")
    parser.add_argument("dirname",
                        help="Name of directory containig input files")
    parser.add_argument(
        "-s",
        "--stats",
        nargs="+",
        help="Stats to calculate. Leave empty to calculate all",
        required=False)
    parser.add_argument("-o",
                        "--output",
                        help="Name of output file",
                        required=False)
    args = parser.parse_args()

    all_stats = [
        "num_subsinks", "edge_to_node", "distLO", "conrel", "assortativity",
        "clustering", "cliques", "density"
    ]

    path = args.dirname
    stats = args.stats
    output = args.output

    if output is None:
        output = "results.csv"

    if stats is None:
        stats = all_stats

    df = pd.DataFrame()
    files = load(path)
    for index, file in enumerate(files):
        nodes, edges, time, hc_count = process_file(join(path, file))
        row = {"time_ms": time, "hc_count": hc_count}
        results = calculate_stats(nodes, edges, stats)
        row = row | results
        df = pd.concat([df, pd.DataFrame([row], index=[index])])

    df.to_csv(output, sep=";")


if __name__ == "__main__":
    main()