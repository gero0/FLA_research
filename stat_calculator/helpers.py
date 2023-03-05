all_stats = [
    "num_subsinks", "edge_to_node", "distLO", "conrel", "assortativity",
    "clustering", "cliques", "density"
]


def split_edge_data(edges):
    edge_list = [[x[0], x[1]] for x in edges]
    weight_list = [x[2] for x in edges]
    return edge_list, weight_list