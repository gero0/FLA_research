all_stats = [
    "num_subsinks", "num_sinks", "num_sources", "edge_to_node", "distLO", "conrel", "assortativity_deg",
    "clustering", "density", "girth", "radius", "avg_path_len", "cliques_num",
    "maximal_cliques_num", "largest_clique_size", "motifs_randesu_no",
    "reciprocity"
]


def split_edge_data(edges):
    edge_list = [[x[0], x[1]] for x in edges]
    weight_list = [x[2] for x in edges]
    return edge_list, weight_list