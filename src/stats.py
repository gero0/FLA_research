def e2n_ratio(nodes, edges):
    return len(edges) / len(nodes)

def subsinks(nodes, edges):
    subsinks = []
    for current_node in nodes:
        (perm, id, path_len) = current_node
        #Find all outgoing edges of this node
        outgoing_edges = []
        for edge in edges:
            if edge[0] == id :
                outgoing_edges.append(edge)

        counter = 0
        #Count all nodes with shorter path that are destination of edges
        for edge in outgoing_edges:
            dst = edge[1]
            if dst == id:
                continue

            for node in nodes:
                if node[1] == dst and node[2] < path_len:
                    counter+=1

        if counter == 0:
            subsinks.append(current_node)

    return subsinks

def subsinks_count(nodes, edges):
    sinks = subsinks(nodes, edges)
    return len(sinks)

