from tsp_samplers import SnowballSampler
import igraph as ig
from igraph import Graph
import tsplib95 as tsp
import matplotlib.pyplot as plt

problem = tsp.load('./data/bays29.tsp')

def create_distance_matrix(problem):
    n = problem.dimension
    matrix = []
    for row_i in range (1, n+1):
        row = []
        for col_i in range(1, n+1):
            row.append(problem.get_weight(row_i, col_i))
        matrix.append(row)

    return matrix

def split_edge_data(edges):
    edge_list = [ [x[0], x[1]] for x in edges]
    weight_list = [x[2] for x in edges]
    return edge_list, weight_list

m = create_distance_matrix(problem)

s = SnowballSampler(1, 20, 3, 2, m, "twoopt", 2000)

x = []
y = []

for i in range (0, 100):
    s.sample()
    nodes, edges = s.get_results()
    (edge_list, weight_list) = split_edge_data(edges)
    g = Graph(n=len(nodes), edges=edge_list, edge_attrs={'weight': weight_list, "label" : weight_list})
    hc = s.get_hc_calls()
    x.append(i)
    y.append(g.radius())

plt.scatter(x, y)
plt.savefig("fig.png")

# s = SnowballSampler(100, 50, 3, 2, m, "hc", 2000)
# s.sample()
# nodes, edges = s.get_results()


# g = Graph(n=len(nodes), edges=edge_list, edge_attrs={'weight': weight_list, "label" : weight_list})
# ig.plot(g, target="graph.pdf")

# print(g)
