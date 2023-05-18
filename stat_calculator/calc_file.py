import os
import sys
import json
from igraph import Graph
from helpers import split_edge_data, split_node_data

from stats import *


def process_file(path):
    with open(path) as f:
        f = json.load(f)

        # sort by id
        nodes = f["nodes"]
        nodes.sort(key=lambda x: x[0])

        edges = f["edges"]

        try:
            missed = f["missed"]
        except:
            missed = None
        return (nodes, edges, f["time_ms"], f["opt_count"], f["oracle_count"],
                missed)


def calculate_graph_stats(nodes, edges, stats, best_node, threadID):
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
        paths = g.get_shortest_paths(best_node[0], weights=None, mode="in")
        existing_paths = [p for p in paths if len(p) > 1]
        path_lens = [len(p) - 1 for p in existing_paths]

        results['go_path_ratio'] = len(existing_paths) / len(paths)
        results['avg_go_path_len'] = np.mean(path_lens)
        results['max_go_path_len'] = np.max(path_lens, initial=0.0)

    if "funnels" in stats:
        results['num_sinks'], results['num_sources'], results[
            'funnel_num'], results['mean_funnel_size'], results[
                'max_funnel_size'], mfs, x = find_funnels(g, False, best_node)

    if "out_degree" in stats:
        d = g.degree(mode="out", loops=False)
        results["max_out_degree"] = np.max(d, initial=0.0)
        results["avg_out_degree"] = np.mean(d)

    if "in_degree" in stats:
        d = g.degree(mode="in", loops=False)
        results["max_in_degree"] = np.max(d, initial=0.0)
        results["avg_in_degree"] = np.mean(d)

    if "assortativity_deg" in stats:
        results["assortativity_deg"] = g.assortativity_degree()

    if "clustering_coeff" in stats:
        results["clustering_coeff"] = g.transitivity_undirected()

    if "density" in stats:
        results["density"] = g.density()

    if "components" in stats:
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
        results["avg_path_len"] = g.average_path_length()

    if "largest_clique_size" in stats:
        results["largest_clique_size"] = g.clique_number()

    if "reciprocity" in stats:
        results["reciprocity"] = g.reciprocity()

    return results


def calculate_stats(nodes, edges, stats, threadID):
    results = {}
    results["node_count"] = len(nodes)
    results["edge_count"] = len(edges)

    best_node = min(nodes, key=lambda x: x[2])
    graph_results = calculate_graph_stats(nodes, edges, stats, best_node,
                                          threadID)

    if "num_subsinks" in stats:
        results["num_subsinks"] = subsink_count(nodes, edges)

    if "edge_to_node" in stats:
        results["edge_to_node"] = e2n_ratio(nodes, edges)

    if "avg_fitness" in stats:
        results["avg_fitness"] = avg_fitness(nodes)

    if "distLO" in stats:
        results["distLO"] = distLO(nodes, edges, best_node)

    if "conrel" in stats:
        results["conrel"] = conrel(nodes, edges, best_node)

    if "avg_loop_weight" in stats:
        results["avg_loop_weight"] = avg_loop_weight(nodes, edges)

    return results | graph_results


# Igraph runtime errors are annoying and make it harder to read progress
def blockPrint():
    sys.stdout = open(os.devnull, 'w')
    sys.stderr = open(os.devnull, 'w')


# Restore
def enablePrint():
    sys.stdout = sys.__stdout__
    sys.stderr = sys.__stderr__


def main():
    blockPrint()
    i, filepath, stats = sys.argv[1], sys.argv[2], sys.argv[3:]
    nodes, edges, time, opt_count, oracle_count, missed = process_file(
        filepath)

    row = {
        "time_ms": time,
        "opt_count": opt_count,
        "oracle_count": oracle_count,
    }

    results = calculate_stats(nodes, edges, stats, i)
    row = row | results
    if missed is not None:
        row = row | {"missed": missed}
    enablePrint()
    print(row)
    return True


if __name__ == "__main__":
    main()