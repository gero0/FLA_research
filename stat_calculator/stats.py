import igraph
import numpy as np
from matplotlib import pyplot as plt
import rust_stat_tools as rst
from copy import deepcopy

def avg_fitness(nodes):
    return np.mean([n[2] for n in nodes])

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

def distLO(nodes, edges, best_node):
    best_id = best_node[0]

    distances = []

    for edge in edges:
        (src, dst, weight) = edge
        if dst == best_id:
            distances.append(1 / weight)

    if(len(distances) == 0):
        return 0

    return sum(distances) / len(distances)

def conrel(nodes, edges, best_node):
    best_id = best_node[0]

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

def find_funnels(g, filter, best_node):
    g = deepcopy(g)

    #Prune off non-improving edges
    edges_to_remove = []
    for e in g.es:
        src = g.vs[e.source]
        dst = g.vs[e.target]
        if(dst['weight'] >= src['weight']):
            edges_to_remove.append(e.index)

    g.delete_edges(edges_to_remove)

    isolated = []
    sources = []
    sinks = []
    for v in g.vs:
        if v.degree(mode="in") == 0:
            sources.append(v.index)
        if(v.degree(mode="out") == 0):
            sinks.append(v.index)
            if(v.degree(mode="in") == 0):
                isolated.append(v.index)

    if(filter):
        sinks = [x for x in sinks if x not in isolated]

    go_funnel_size = 0

    funnel_sizes = []
    for sink in sinks:
        [vertices, parents] = g.dfs(vid=sink, mode="in")
        funnel_sizes.append(len(vertices))
        if sink == best_node[0]:
            go_funnel_size = len(vertices)

    rel_go_funnel_size = go_funnel_size / max(funnel_sizes)

    mean_fs = sum(funnel_sizes) / len(funnel_sizes)

    return(len(funnel_sizes), mean_fs, max(funnel_sizes), min(funnel_sizes), rel_go_funnel_size)


