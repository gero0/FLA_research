all_stats = [
    "num_subsinks", "edge_to_node", "distLO", "conrel", "assortativity",
    "clustering", "cliques", "density"
]


def create_distance_matrix(problem):
    n = problem.dimension
    matrix = []
    for row_i in range(1, n + 1):
        row = []
        for col_i in range(1, n + 1):
            row.append(problem.get_weight(row_i, col_i))
        matrix.append(row)

    return matrix


def split_edge_data(edges):
    edge_list = [[x[0], x[1]] for x in edges]
    weight_list = [x[2] for x in edges]
    return edge_list, weight_list