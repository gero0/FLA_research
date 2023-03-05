def e2n_ratio(nodes, edges):
    return len(edges) / len(nodes)


def subsinks(nodes, edges):
    subsinks = []
    for current_node in nodes:
        (id, perm, path_len) = current_node
        #Find all outgoing edges of this node
        outgoing_edges = []
        for edge in edges:
            (src, dst, weight) = edge
            if src == id:
                outgoing_edges.append(edge)

        counter = 0
        #Count all nodes with shorter path that are destination of edges
        for edge in outgoing_edges:
            (src, dst, weight) = edge
            if dst == id:
                continue

            for node in nodes:
                (id, perm, p_len) = node
                if id == dst and p_len < path_len:
                    counter += 1

        if counter == 0:
            subsinks.append(current_node)

    return subsinks


def subsinks_count(nodes, edges):
    sinks = subsinks(nodes, edges)
    return len(sinks)

def distLO(nodes, edges):
    best = nodes[0]
    best_id = best[0]

    distances = []

    for edge in edges:
        (src, dst, weight) = edge
        if dst == best_id:
            distances.append(1 / weight)

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

