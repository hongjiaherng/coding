from neetcode150_py.stack.q155_min_stack import MinStack


def test_min_stack() -> None:
    obj = MinStack()
    obj.push(-2)
    obj.push(0)
    obj.push(-3)
    assert obj.getMin() == -3
    obj.pop()
    assert obj.top() == 0
    assert obj.getMin() == -2


# obj = MinStack()
# obj.push(val)
# obj.pop()
# param_3 = obj.top()
# param_4 = obj.getMin()
