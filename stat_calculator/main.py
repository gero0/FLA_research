from os import listdir
from os.path import isfile, join

import argparse
import natsort
import pandas as pd
import json
from igraph import Graph
from helpers import split_edge_data, all_stats, split_node_data

from stats import *


def load(path):
    files = [f for f in listdir(path) if isfile(join(path, f))]
    files = natsort.natsorted(files)
    return files


def process_file(path):
    with open(path) as f:
        print(f"Now procesing file {path}")
        f = json.load(f)

        # sort by id
        nodes = f["nodes"]
        nodes.sort(key=lambda x: x[0])

        edges = f["edges"]

        return (nodes, edges, f["time_ms"], f["hc_count"], f["oracle_count"])


def calculate_graph_stats(nodes, edges, stats, best_node):
    results = {}

    # Nodes must be sorted by id in order for correct weights to be assigned!
    (node_ids, node_perms, node_weights) = split_node_data(nodes)
    (edge_list, weight_list) = split_edge_data(edges)

    g = Graph(n=len(node_ids),
              edges=edge_list,
              edge_attrs={"weight": weight_list},
              vertex_attrs={
                  "weight": node_weights,
                  "name": node_perms
              },
              directed=True)

    if "paths_to_go":
        paths = g.get_shortest_paths(best_node[0], weights=None, mode="in")
        existing_paths = [p for p in paths if len(p) > 1]
        path_lens = [len(p) - 1 for p in existing_paths]

        results['go_path_ratio'] = len(existing_paths) / len(paths)
        results['avg_go_path_len'] = np.mean(path_lens)
        results['max_go_path_len'] = np.max(path_lens)

    if "strength":
        in_strength = g.strength(mode="in", loops=False)
        out_strength = g.strength(mode="out", loops=False)
        results['avg_in_strength'] = np.mean(in_strength)
        results['max_in_strength'] = np.max(in_strength)
        results['avg_out_strength'] = np.mean(out_strength)
        results['max_out_strength'] = np.max(out_strength)

    if "funnels" in stats:
        results['funnel_num'], results['mean_funnel_size'], results[
            'max_funnel_size'], results['min_funnel_size'], x = find_funnels(
                g, False, best_node)

    if "funnels_filtered" in stats:
        results['funnel_num_f'], results['mean_funnel_size_f'], results[
            'max_funnel_size_f'], results['min_funnel_size_f'], results[
                'rel_go_funnel_size'] = find_funnels(g, True, best_node)

    if "out_degree":
        d = g.degree(mode="out", loops=False)
        results["max_out_degree"] = np.max(d)
        results["avg_out_degree"] = np.mean(d)

    if "in_degree":
        d = g.degree(mode="in", loops=False)
        results["max_in_degree"] = np.max(d)
        results["avg_in_degree"] = np.mean(d)

    if "assortativity_deg" in stats:
        results["assortativity_deg"] = g.assortativity_degree()

    if "clustering_coeff" in stats:
        results["clustering_coeff"] = g.transitivity_undirected()

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

    if "reciprocity" in stats:
        results["reciprocity"] = g.reciprocity()

    return results


def calculate_stats(nodes, edges, stats):
    results = {}
    results["node_count"] = len(nodes)
    results["edge_count"] = len(edges)

    best_node = min(nodes, key=lambda x: x[2])
    graph_results = calculate_graph_stats(nodes, edges, stats, best_node)

    if "num_subsinks" in stats:
        results["num_subsinks"] = subsink_count(nodes, edges)

    if "num_sinks" in stats:
        results["num_sinks"] = sink_count(nodes, edges)

    if "num_sources" in stats:
        results["num_sources"] = source_count(nodes, edges)

    if "edge_to_node" in stats:
        results["edge_to_node"] = e2n_ratio(nodes, edges)

    if "avg_fitness" in stats:
        results["avg_fitness"] = avg_fitness(nodes)

    if "distLO" in stats:
        results["distLO"] = distLO(nodes, edges, best_node)

    if "conrel" in stats:
        results["conrel"] = conrel(nodes, edges, best_node)

    return results | graph_results


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
        nodes, edges, time, hc_count, oracle_count = process_file(
            join(path, file))
        row = {
            "time_ms": time,
            "hc_count": hc_count,
            "oracle_count": oracle_count
        }
        results = calculate_stats(nodes, edges, stats)
        row = row | results
        df = pd.concat([df, pd.DataFrame([row], index=[index])])

    df.to_csv(output, sep=";")
    corr = df.corr()
    corr.to_csv("corr_" + output, sep=";")


if __name__ == "__main__":
    main()