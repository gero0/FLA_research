all_stats = [
    "num_subsinks", "num_sinks", "num_sources", "edge_to_node", "distLO",
    "conrel", "assortativity_deg", "clustering", "density", "components",
    "avg_path_len", "cliques_num", "maximal_cliques_num",
    "largest_clique_size", "motifs_randesu_no", "reciprocity", "funnels",
    "funnels_filtered", "avg_fitness", "out_degree", "in_degree",
    "paths_to_go", "strength"
]


def split_edge_data(edges):
    edge_list = [[x[0], x[1]] for x in edges]
    weight_list = [x[2] for x in edges]
    return edge_list, weight_list


def split_node_data(nodes):
    node_ids = [x[0] for x in nodes]
    node_perms = [x[1] for x in nodes]
    weight_list = [x[2] for x in nodes]
    return node_ids, node_perms, weight_list