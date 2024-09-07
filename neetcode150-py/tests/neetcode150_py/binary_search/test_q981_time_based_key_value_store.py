from neetcode150_py.binary_search.q981_time_based_key_value_store import TimeMap


def test_timemap() -> None:
    obj = TimeMap()
    print(obj.map)
    obj.set(key="foo", value="bar", timestamp=1)
    print(obj.map)
    assert obj.get(key="foo", timestamp=1) == "bar"
    assert obj.get(key="foo", timestamp=3) == "bar"
    obj.set(key="foo", value="bar2", timestamp=4)
    print(obj.map)
    assert obj.get(key="foo", timestamp=4) == "bar2"
    assert obj.get(key="foo", timestamp=5) == "bar2"
