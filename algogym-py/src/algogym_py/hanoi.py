from typing import Literal, cast

TowerName = Literal["src", "aux", "dest"]


class Solution:
    def __init__(self) -> None:
        self.towers: dict[TowerName, list[int]] = {
            "src": [],
            "aux": [],
            "dest": [],
        }
        self.n_moves = 0
        self._logs: list[str] = []

    def solve_hanoi(self, n: int) -> int:
        self.towers["src"] = list(range(n, 0, -1))
        self.hanoi(n, start="src", end="dest")

        for line in self._logs:
            print(line)

        return self.n_moves

    def hanoi(self, n: int, start: TowerName, end: TowerName) -> None:
        if n == 1:
            # Move a disk from start to end
            self.towers[end].append(self.towers[start].pop())
            self.n_moves += 1
            self._logs.append(
                f"[Base] Move disk {n} from {start} -> {end}\t: {self.towers}"
            )

        else:
            # Find the current aux for the subproblem
            aux = cast(
                TowerName,
                [key for key in self.towers.keys() if key not in [start, end]][0],
            )

            # Move top n-1 disks from src to aux
            # For n-1, move from start to aux, end as the aux
            self.hanoi(n - 1, start=start, end=aux)

            # Move nth disk from src to dest
            self.towers[end].append(self.towers[start].pop())
            self.n_moves += 1
            self._logs.append(
                f"[Move] Move disk {n} from {start} -> {end}\t: {self.towers}"
            )

            # Move top n-1 disks from aux to dest
            self.hanoi(n - 1, start=aux, end=end)
