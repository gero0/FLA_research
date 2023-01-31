from tsp_samplers import SnowballSampler
import tsplib95 as tsp

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

m = create_distance_matrix(problem)


s = SnowballSampler(1, 5, 3, 2, m, "twoopt", 2000)
s.sample()
# s.sample()
nodes, edges = s.get_results()
hc_calls = s.get_hc_calls()
print(len(nodes))
print(len(edges))
print(hc_calls)