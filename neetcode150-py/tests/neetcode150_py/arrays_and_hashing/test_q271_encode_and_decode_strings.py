from typing import List
from neetcode150_py.arrays_and_hashing.q271_encode_and_decode_strings import Solution


def test_encode_and_decode_strings() -> None:
    solution = Solution()
    list1 = ["neet", "code", "love", "you"]
    list2 = ["we", "say", ":", "yes"]
    list3 = ["we", "say", ":", "yes", "!@#$%^&*()"]
    list4 = [""]
    list5 = ["1,23", "45,6", "7,8,9"]
    list6 = ["#neet", "code", "love", "you"]
    list7: List[str] = []

    assert solution.decode(solution.encode(list1)) == list1
    assert solution.decode(solution.encode(list2)) == list2
    assert solution.decode(solution.encode(list3)) == list3
    assert solution.decode(solution.encode(list4)) == list4
    assert solution.decode(solution.encode(list5)) == list5
    assert solution.decode(solution.encode(list6)) == list6
    assert solution.decode(solution.encode(list7)) == list7
