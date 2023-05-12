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

        try:
            missed = f["missed"]
        except:
            missed = None
        return (nodes, edges, f["time_ms"], f["opt_count"], f["oracle_count"], missed)


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

    if "paths_to_go" in stats:
        print("-----------Paths to g.o.-------------")
        paths = g.get_shortest_paths(best_node[0], weights=None, mode="in")
        existing_paths = [p for p in paths if len(p) > 1]
        path_lens = [len(p) - 1 for p in existing_paths]

        results['go_path_ratio'] = len(existing_paths) / len(paths)
        results['avg_go_path_len'] = np.mean(path_lens)
        results['max_go_path_len'] = np.max(path_lens, initial=0.0)

    # print("Strength-------------------------------")
    # if "strength" in stats:
    #     in_strength = g.strength(mode="in", loops=False)
    #     out_strength = g.strength(mode="out", loops=False)
    #     results['avg_in_strength'] = np.mean(in_strength)
    #     results['max_in_strength'] = np.max(in_strength, initial=0.0)
    #     results['avg_out_strength'] = np.mean(out_strength)
    #     results['max_out_strength'] = np.max(out_strength, initial=0.0)

    if "funnels" in stats:
        print("-----------Funnels.-------------")
        results['num_sinks'], results['num_sources'], results['funnel_num'], results['mean_funnel_size'], results[
            'max_funnel_size'], mfs, x = find_funnels(g, False, best_node)

    # if "funnels_filtered" in stats:
    #     results['funnel_num_f'], results['mean_funnel_size_f'], results[
    #         'max_funnel_size_f'], mfsf, results[
    #             'rel_go_funnel_size'] = find_funnels(g, True, best_node)

    if "out_degree" in stats:
        print("-----------Out Degree.-------------")
        d = g.degree(mode="out", loops=False)
        results["max_out_degree"] = np.max(d, initial=0.0)
        results["avg_out_degree"] = np.mean(d)

    if "in_degree" in stats:
        print("-----------In Degree.-------------")
        d = g.degree(mode="in", loops=False)
        results["max_in_degree"] = np.max(d, initial=0.0)
        results["avg_in_degree"] = np.mean(d)

    if "assortativity_deg" in stats:
        print("-----------Assortativity-------------")
        results["assortativity_deg"] = g.assortativity_degree()

    if "clustering_coeff" in stats:
        print("-----------Clustering_coeff-------------")
        results["clustering_coeff"] = g.transitivity_undirected()

    if "density" in stats:
        print("-----------Density.-------------")
        results["density"] = g.density()

    if "components" in stats:
        print("-----------Components.-------------")
        components = g.components(mode="weak")
        if len(components) > 0:
            comp_sizes = components.sizes()
            largest_cc_size = np.max(comp_sizes)
            largest_cc_i = comp_sizes.index(largest_cc_size)

            largest_cc_subgraph = components.subgraphs()[largest_cc_i]
            largest_cc_subgraph.to_undirected()
            results["num_cc"] = len(components)
            results["largest_cc"] = largest_cc_size
            results["largest_cc_radius"] = largest_cc_subgraph.radius()
        else:
            results["num_cc"] = 0
            results["largest_cc"] = 0
            results["largest_cc_radius"] = 0

    if "avg_path_len" in stats:
        print("-----------Avg path len-------------")
        results["avg_path_len"] = g.average_path_length()

    # if "cliques_num" in stats:
    #     cliques = g.cliques()
    #     results["cliques_num"] = len(cliques)

    # if "maximal_cliques_num" in stats:
    #     results["maximal_cliques_num"] = len(g.maximal_cliques())

    if "largest_clique_size" in stats:
        print("-----------largest clique size.-----------")
        results["largest_clique_size"] = g.clique_number()

    # print("MOTIFS -------------------------------")
    # if "motifs_randesu_no" in stats:
    #     results["motifs_randesu_no"] = g.motifs_randesu_no()

    if "reciprocity" in stats:
        print("-----------Reciprocity.-------------")
        results["reciprocity"] = g.reciprocity()

    return results


def calculate_stats(nodes, edges, stats):
    results = {}
    results["node_count"] = len(nodes)
    results["edge_count"] = len(edges)

    best_node = min(nodes, key=lambda x: x[2])
    graph_results = calculate_graph_stats(nodes, edges, stats, best_node)

    if "num_subsinks" in stats:
        print("-----------Num. Subsinks-------------")
        results["num_subsinks"] = subsink_count(nodes, edges)

    # print("SINKS-------------------------------")
    # if "num_sinks" in stats:
    #     results["num_sinks"] = sink_count(nodes, edges)

    # print("SOURCES-------------------------------")
    # if "num_sources" in stats:
    #     results["num_sources"] = source_count(nodes, edges)

    if "edge_to_node" in stats:
        print("-----------E2N.-------------")
        results["edge_to_node"] = e2n_ratio(nodes, edges)

    if "avg_fitness" in stats:
        print("-----------Avg fitness-------------")
        results["avg_fitness"] = avg_fitness(nodes)

    if "distLO" in stats:
        print("-----------distLO.-------------")
        results["distLO"] = distLO(nodes, edges, best_node)

    if "conrel" in stats:
        print("-----------conrel.-------------")
        results["conrel"] = conrel(nodes, edges, best_node)

    if "avg_loop_weight" in stats:
        print("-----------avg_loop_weight-------------")
        results["avg_loop_weight"] = avg_loop_weight(nodes, edges)

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

    parser.add_argument("-l",
                        "--limit",
                        help="limit input files to process",
                        required=False)

    args = parser.parse_args()

    path = args.dirname
    stats = args.stats
    output = args.output

    limit = args.limit
    if limit is None:
        limit = 999999999999999
    else:
        limit = int(args.limit)

    if output is None:
        output = "results.csv"

    if stats is None:
        stats = all_stats

    df = pd.DataFrame()
    files = load(path)
    for index, file in enumerate(files):
        print(f"NOW PROCESSING: {file}===================")
        if index >= limit:
            break
        nodes, edges, time, opt_count, oracle_count, missed = process_file(
            join(path, file))
        
        row = {
            "time_ms": time,
            "opt_count": opt_count,
            "oracle_count": oracle_count,
        }

        results = calculate_stats(nodes, edges, stats)
        row = row | results
        if missed is not None:
            row = row | {"missed" : missed}
        df = pd.concat([df, pd.DataFrame([row], index=[index])])

    df.to_csv(output, sep=";")
    corr = df.corr()
    corr.to_csv(output.replace(".csv", "_corr.csv"), sep=";")


if __name__ == "__main__":
    main()