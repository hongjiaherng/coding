from typing import Dict, List, Tuple


class TimeMap:
    """
    All the timestamps timestamp of set are strictly increasing.
    """

    def __init__(self) -> None:
        self.map: Dict[str, List[Tuple[int, str]]] = {}

    def set(self, key: str, value: str, timestamp: int) -> None:
        if key not in self.map:
            self.map[key] = []
        self.map[key].append((timestamp, value))

    def get(self, key: str, timestamp: int) -> str:
        """
        T(n) = O(log n)
        S(n) = O(1)

        Example map: {'foo': [(1, 'bar'), (4, 'bar2')]}

        timestamp = 3

        [1, 4, 5, 10]
         |  |      |
         L  M      R

        M <= timestamp: No
        M > timestamp: Yes, right = mid - 1

        [1, 4, 5, 10]
         |
         L
         R
         M

        M <= timestamp: Yes, set M to res, left = mid + 1

        [1, 4, 5, 10]
         |  |
         R  L

        L > R: break loop

        timestamp = 11
        [1, 4, 5, 10]
         |  |      |
         L  M      R

        [1, 4, 5, 10]
               |   |
               L   R
               M

        [1, 4, 5, 10]
                   |
                   R
                   L
                   M
        """

        res = ""
        values = self.map.get(key, [])

        left, right = 0, len(values) - 1
        while left <= right:
            mid = left + (right - left) // 2
            if values[mid][0] <= timestamp:
                res = values[mid][1]
                left = mid + 1
            else:
                right = mid - 1

        return res


# Your TimeMap object will be instantiated and called as such:
# obj = TimeMap()
# obj.set(key,value,timestamp)
# param_2 = obj.get(key,timestamp)
