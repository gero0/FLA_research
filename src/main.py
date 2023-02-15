from ctypes import resize
from os import listdir
from os.path import isfile, join

import natsort
import pandas as pd
import numpy as np
import json
import matplotlib.pyplot as plt
import igraph as ig
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

        nodes = f['nodes']
        nodes.sort(key=lambda x: x[2])

        edges = f['edges']

        return (nodes, edges, f['time_ms'], f['hc_count'])


def calculate_stats(nodes, edges, stats):
    results = {}
    results['node_count'] = len(nodes)
    results['edge_count'] = len(edges)

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
              edge_attrs={'weight': weight_list})

    if "assortativity" in stats:
        results["assortativity"] = g.assortativity_degree()

    if "clustering" in stats:
        results['clustering'] = g.transitivity_undirected()

    if "cliques" in stats:
        results["cliques"] = g.clique_number()

    if "density" in stats:
        results["density"] = g.density()

    return results


def main():
    df = pd.DataFrame()
    path = "snowball_latest"
    files = load(path)
    for index, file in enumerate(files):
        nodes, edges, time, hc_count = process_file(join(path, file))
        row = {'time_ms': time, 'hc_count': hc_count}
        results = calculate_stats(nodes, edges, [
            'edge_to_node', 'distLO', 'conrel', 'assortativity', 'clustering',
            'cliques', 'density'
        ])
        row = row | results
        df = pd.concat([df, pd.DataFrame([row], index=[index])])

    df.to_csv("results.csv", sep=';')


if __name__ == "__main__":
    main()