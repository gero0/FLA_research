from typing import List, Tuple
from .tsp_samplers import *

class SnowballSampler:
    """
    Sampler using the snowballing technique from
    "Sampling Local Optima Networks of Large Combinatorial Search Spaces: The QAP Case" by Verel, Daolio, Ochoa, Tomassini.
    Implementation allows resuming walk by calling sample again.
    Tracks the number of calls to hillclimb function.
    """
    def __init__(
        self,
        walk_len: int,
        n_edges: int,
        depth: int,
        mut_d: int,
        distance_matrix: List[List[int]],
        hillclimb_function: str,
        seed: int|None 
    ): ...

    """
    Runs sampling function
    """
    def sample(self): ...

    """
    Get results - tuple containing nodes and edges in format:
    ( (permutation, solution_id, path_len), (start, end, weight) ) 
    """
    def get_results(self) -> Tuple[List[ Tuple[List[int], int, int] ], List[Tuple[int, int, int]]]: ...

    """
    Get number of calls to the hillclimb function that had been made
    """
    def get_hc_calls(self) -> int : ...


class PwrSampler:
    """
    Sampler using the technique from
    "Local Optima Networks in Solving Algorithm Selection Problem for TSP" by Bożejko, Gnatowski et al.
    Modified to allow to plug in any hc function as parameter.
    Tracks the number of calls to hillclimb function.
    """
    def __init__(
        self,
        distance_matrix: List[List[int]],
        hillclimb_function: str,
        seed: int|None 
    ): ...

    """
    Runs sampling function
    n_max - desired number of nodes
    n_att - attempts to generate unique node
    e_att - attempts to find edges coming from node
    """
    def sample(self, n_max: int, n_att: int, e_att: int): ...

    """
    Get results - tuple containing nodes and edges in format:
    ( (permutation, solution_id, path_len), (start, end, weight) ) 
    """
    def get_results(self) -> Tuple[List[ Tuple[List[int], int, int] ], List[Tuple[int, int, int]]]: ...

    """
    Get number of calls to the hillclimb function that had been made
    """
    def get_hc_calls(self) -> int : ...