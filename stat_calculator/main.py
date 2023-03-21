from os import listdir
from os.path import isfile, join

import argparse
import natsort
import pandas as pd
import json
from igraph import Graph
from helpers import split_edge_data, all_stats

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

        return (nodes, edges, f["time_ms"], f["hc_count"], f["oracle_count"])


def calculate_stats(nodes, edges, stats):
    results = {}
    results["node_count"] = len(nodes)
    results["edge_count"] = len(edges)

    # if "num_subsinks" in stats:
    #     results["num_subsinks"] = subsinks_count(nodes, edges)

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
              edge_attrs={"weight": weight_list},
              directed=True)

    if "assortativity_deg" in stats:
        results["assortativity_deg"] = g.assortativity_degree()

    if "clustering" in stats:
        results["clustering"] = g.transitivity_undirected()

    if "density" in stats:
        results["density"] = g.density()

    if "girth" in stats:
        results["girth"] = g.girth()

    if "radius" in stats:
        results["radius"] = g.radius()

    if "avg_path_len" in stats:
        results["avg_path_len"] = g.average_path_length()

    if "cliques_num" in stats:
        results["cliques_num"] = g.clique_number()

    if "maximal_cliques_num" in stats:
        results["maximal_cliques_num"] = len(g.maximal_cliques())

    if "largest_clique_size" in stats:
        results["largest_clique_size"] = len(g.largest_cliques()[0])

    if "motifs_randesu_no" in stats:
        results["motifs_randesu_no"] = g.motifs_randesu_no()

    #mincut_value - long time, keeps 1.0 value

    if "reciprocity" in stats:
        results["reciprocity"] = g.reciprocity()

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
        nodes, edges, time, hc_count, oracle_count = process_file(join(path, file))
        row = {"time_ms": time, "hc_count": hc_count, "oracle_count": oracle_count}
        results = calculate_stats(nodes, edges, stats)
        row = row | results
        df = pd.concat([df, pd.DataFrame([row], index=[index])])

    df.to_csv(output, sep=";")
    corr = df.corr()
    corr.to_csv("corr_" + output, sep=";")


if __name__ == "__main__":
    main()