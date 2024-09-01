from typing import List, Tuple


class Solution:
    def carFleet(self, target: int, position: List[int], speed: List[int]) -> int:
        """
        T(n) = O(n) + O(nlogn) = O(nlogn)
        S(n) = O(n)

        If a car's time to reach target <= another car in front of it
        => they'll collide and move as a fleet
        Otherwise
        => they move seperately (car behind is slower than car in front, they'll never collide)
        """
        fleet_stack: List[float] = []
        cars: List[Tuple[int, int]] = [(pos, spd) for pos, spd in zip(position, speed)]

        # Sort based on position
        cars.sort(key=lambda pos_spd: pos_spd[0], reverse=True)  # O(nlogn)

        for pos, spd in cars:
            t_target = (target - pos) / spd  # Time to reach target at position pos
            if fleet_stack and t_target <= fleet_stack[-1]:
                # current car collide with the car infront, so they become a fleet
                continue
            # Empty, or
            # current car won't collide with the car infront of it
            fleet_stack.append(t_target)

        return len(fleet_stack)
