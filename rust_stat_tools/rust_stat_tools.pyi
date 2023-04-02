from typing import List, Tuple

"""
Calculates number of subsinks (nodes with no outgoing edges going to better solutions)
"""
def num_subsinks(nodes: List[Tuple[int, List[int], int]], edges: List[Tuple[int, int, int]]) -> int:
    ...

"""
Calculates number of sinks (nodes with no outgoing edges)
"""
def num_sinks(nodes: List[Tuple[int, List[int], int]], edges: List[Tuple[int, int, int]]) -> int:
    ...

"""
Calculates number of sources (nodes with no outgoing edges)
"""
def num_sources(nodes: List[Tuple[int, List[int], int]], edges: List[Tuple[int, int, int]]) -> int:
    ...