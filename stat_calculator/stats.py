import rust_stat_tools as rst

def e2n_ratio(nodes, edges):
    return len(edges) / len(nodes)


def subsink_count(nodes, edges):
    nodes_t = [tuple(x) for x in nodes]
    edges_t = [tuple(x) for x in edges]
    return rst.num_subsinks(nodes_t, edges_t)

def sink_count(nodes, edges):
    nodes_t = [tuple(x) for x in nodes]
    edges_t = [tuple(x) for x in edges]
    return rst.num_sinks(nodes_t, edges_t)

def source_count(nodes, edges):
    nodes_t = [tuple(x) for x in nodes]
    edges_t = [tuple(x) for x in edges]
    return rst.num_sources(nodes_t, edges_t)

def distLO(nodes, edges):
    best = nodes[0]
    best_id = best[0]

    distances = []

    for edge in edges:
        (src, dst, weight) = edge
        if dst == best_id:
            distances.append(1 / weight)

    if(len(distances) == 0):
        return 0

    return sum(distances) / len(distances)

def conrel(nodes, edges):
    best = nodes[0]
    best_id = best[0]

    connected = set()

    for edge in edges:
        (src, dst, wgt) = edge
        if(dst == best_id and src != best_id):
            connected.add(src)
        elif(src == best_id and dst != best_id):
            connected.add(dst)

    nc_counter = 0
    for node in nodes:
        if node[0] not in connected:
            nc_counter += 1

    return len(connected) / nc_counter

