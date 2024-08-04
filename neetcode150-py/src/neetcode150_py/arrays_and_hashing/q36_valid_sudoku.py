from typing import List, Dict, Set, Tuple
from collections import defaultdict


class Solution:
    def isValidSudoku(self, board: List[List[str]]) -> bool:
        """
        T(n) = O(n^2) = O(9*9) = O(81) ; 81 checks
        S(n) = O(n*3*n) = O(27*9) ; 27 HashSet
        """
        nrows, ncols = len(board), len(board[0])
        rows: Dict[int, Set[str]] = defaultdict(set)
        cols: Dict[int, Set[str]] = defaultdict(set)
        sboxes: Dict[Tuple[int, int], Set[str]] = defaultdict(set)

        for i in range(nrows):
            for j in range(ncols):
                if board[i][j] == ".":
                    continue
                if (
                    board[i][j] in rows[i]
                    or board[i][j] in cols[j]
                    or board[i][j] in sboxes[(i // 3, j // 3)]
                ):
                    return False
                rows[i].add(board[i][j])
                cols[j].add(board[i][j])
                sboxes[(i // 3, j // 3)].add(board[i][j])

        return True

    def isValidSudoku1(self, board: List[List[str]]) -> bool:
        """
        T(n) = O(3n^2) = O(n^2)
        S(n) = O(27n) = O(n)
        """
        nrows, ncols = len(board), len(board[0])

        # Scan through rows, check for duplicates
        for rix in range(nrows):
            nums = set()
            seen = 0
            for cix in range(ncols):
                if not board[rix][cix].isdigit():
                    continue
                seen += 1
                nums.add(board[rix][cix])
            if seen != len(nums):
                return False

        # Scan through cols, check for duplicates
        for cix in range(ncols):
            nums = set()
            seen = 0
            for rix in range(nrows):
                if not board[rix][cix].isdigit():
                    continue
                seen += 1
                nums.add(board[rix][cix])
            if seen != len(nums):
                return False

        # Scan through 3x3 sub-boxes, check for duplicates
        for brix in range(3):
            for bcix in range(3):
                nums = set()
                seen = 0
                for rix in range(brix * 3, brix * 3 + 3):
                    for cix in range(bcix * 3, bcix * 3 + 3):
                        if not board[rix][cix].isdigit():
                            continue
                        seen += 1
                        nums.add(board[rix][cix])
                if seen != len(nums):
                    return False

        return True
