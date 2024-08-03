from collections import defaultdict
from typing import List, Dict, Tuple


class Solution:
    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:
        """
        T(n) = O(nm)
        S(n) = O(ng)
        """
        group_map: Dict[Tuple[int, ...], List[str]] = defaultdict(list)
        for s in strs:  # O(n)
            counts = [0] * 26
            for c in s:  # O(m)
                counts[ord(c) - ord("a")] += 1
            group_map[tuple(counts)].append(s)
        group_list = list(group_map.values())
        return group_list

    def groupAnagrams1(self, strs: List[str]) -> List[List[str]]:
        """
        T(n) = O(n * m log m)
        S(n) = O(n)
        """
        group_map: Dict[str, List[str]] = {}
        for s in strs:  # O(n)
            sorted_ana = "".join(sorted(s))  # O(m log m)
            group_map[sorted_ana] = group_map.get(sorted_ana, []) + [s]
        group_list = list(group_map.values())
        return group_list
