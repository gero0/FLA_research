from numpy import append
from tsp_samplers import SnowballSampler, PwrSampler
import igraph as ig
from igraph import Graph
from helpers import create_distance_matrix, split_edge_data
import tsplib95 as tsp
import matplotlib.pyplot as plt

from stats import subsinks_count

problem = tsp.load('./data/burma14.tsp')

m = create_distance_matrix(problem)

s = SnowballSampler(100, 5, 3, 2, m, "twooptfi", 2000)

# s = PwrSampler(m, "twooptfi", 2000)

x = []
y = []

for i in range (0, 100):
    # s.sample(100, 100, 100)
    s.sample()
    nodes, edges = s.get_results()
    # (edge_list, weight_list) = split_edge_data(edges)
    # g = Graph(n=len(nodes), edges=edge_list, edge_attrs={'weight': weight_list, "label" : weight_list})
    hc = s.get_hc_calls()
    x.append(hc)
    # y.append(g.radius())
    y.append(subsinks_count(nodes, edges))

plt.scatter(x, y)
plt.savefig("fig.png")

# s = SnowballSampler(100, 50, 3, 2, m, "hc", 2000)
# s.sample()
# nodes, edges = s.get_results()


# g = Graph(n=len(nodes), edges=edge_list, edge_attrs={'weight': weight_list, "label" : weight_list})
# ig.plot(g, target="graph.pdf")

# print(g)
