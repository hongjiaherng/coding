"""
# Grab MLE: technical round
Problem: Greedily remove all adjacent duplicates until there is none

## Recursive Method:
Test cases:
1. f("abcd") == "abcd"
- "abcd" -> "abcd"

2. f("abcccbabc") == "bc"
- "ab(ccc)babc" -> "a(bb)abc" -> "aabc" -> "(aa)bc" -> "bc"

3. f("deeedbbcccbdaa") == "bd"
- "d(eee)d(bb)(ccc)bd(aa)" -> "ddbd" -> "(dd)bd" -> "bd"

4. f("caaabbbaacdddd") == ""
- "c(aaa)(bbb)(aa)c(dddd)" -> "cc" -> "(cc)" -> ""

5. f("acaaabbbacdddd") == "acac"
- "ac(aaa)(bbb)ac(dddd)" -> "acac"

6. f("abcdefggfedcba") == ""; worst case n/2 recursive calls
- "abcdef(gg)fedcba" -> "abcde(ff)edcba" -> "abcd(ee)dcba" -> "abc(dd)cba" -> "ab(cc)ba" -> "a(bb)a" -> "(aa)" -> ""

## Stack Method:
3 possible actions:
- push s[i] to stack
    - condition: stack[-1] != s[i], last_rm != s[i]
- pop last item from stack, update last_rm
    - condition: stack[-1] == s[i]
- do nothing
    - condition: last_rm == s[i]

Test cases:
1. f("abcd") == "abcd"
- "abcd" -> "abcd"

2. f("abcccbabc") == "bc"
- "a" -(b)-> "ab"
- "ab" -(c)-> "abc"
- "abc" -(c)-> "ab"; last_rm = c
- "ab" -(c)-> "ab"
- "ab" -(b)-> "a"; last_rm = b
- "a" -(a)-> ""; last_rm = a
- "" -(b)-> "b"
- "b" -(c)-> "bc"

3. f("abcccccbabc") == "bc"
- "a" -(b)-> "ab"
- "ab" -(c)-> "abc"
- "abc" -(c)-> "ab"; last_rm = c
- "ab" -(c)-> "ab"
- "ab" -(c)-> "ab"
- "ab" -(c)-> "ab"
- "ab" -(b)-> "a"; last_rm = b
- "a" -(a)-> ""; last_rm = a
- "" -(b)-> "b"
- "b" -(c)-> "bc"

4. f("deeedbbcccbdaa") == "bd"
- "d" -(e)-> "de"
- "de" -(e)-> "d"; last_rm = e
- "d" -(e)-> "d"
- "d" -(d)-> ""; last_rm = d
- "" -(b)-> "b"
- "b" -(b)-> "b"
- "b" -(c)-> "bc"
- "bc" -(c)-> "b"; last_rm = c
- "b" -(d)-> "bd"

5. f("caaabbbaacdddd") == ""
- "c" -(a)-> "ca"
- "ca" -(a)-> "c"; last_rm = a
- "c" -(a)-> "c"
- "c" -(a)-> "c"
- "c" -(b)-> "cb"
- "cb" -(b)-> "c"; last_rm = b
- "c" -(b)-> "c"
- "c" -(a)-> "ca"
- "ca" -(a)-> "c"; last_rm = a
- "c" -(c)-> ""; last_rm = c
- "" -(d)-> "d"
- "d" -(d)-> "", last_rm = d
- "" -(d)-> ""
- "" -(d)-> ""

6. f("acaaabbbacdddd") == "acac"
- "a" -(c)-> "ac"
- "ac" -(a)-> "aca"
- "aca" -(a)-> "ac", last_rm = a
- "ac" -(a)-> "ac"
- "ac" -(b)-> "acb"
- "acb" -(b)-> "ac", last_rm = b
- "ac" -(b)-> "ac"
- "ac" -(a)-> "aca"
- "aca" -(c)-> "acac"
- "acac" -(d)-> "acacd"
- "acacd" -(d)-> "acac"; last_rm = d
- "acac" -(d)-> "acac"
- "acac" -(d)-> "acac"

7. f("abcdefggfedcba") == ""
- "a" -(b)-> "ab"
- "ab" -(c)-> "abc"
- "abc" -(d)-> "abcd"
- "abcd" -(e)-> "abcde"
- "abcde" -(f)-> "abcdef"
- "abcdef" -(g)-> "abcdefg"
- "abcdefg" -(g)-> "abcdef"; last_rm = g
- "abcdef" -(f)-> "abcde"; last_rm = f
- "abcde" -(e)-> "abcd"; last_rm = e
- "abcd" -(d)-> "abc"; last_rm = d
- "abc" -(c)-> "ab"; last_rm = c
- "ab" -(b)-> "a"; last_rm = b
- "a" -(a)-> ""; last_rm = a

"""

from typing import Set, List


class Solution:
    def recursive_remove_duplicates(self, s: str) -> str:
        """
        T(n) = O(n / 2) * (O(n) + O(n)) = O(n^2); n / 2 recursive calls, each call cause O(n)
        S(n) = O(n / 2) * (O(n) + O(n)) = O(n^2); n / 2 recursive calls, each call has a set of n length and a string of n length
        """
        # base case: no adjacent duplicates
        # how to simply check? no way
        # if things to removed is empty/null, return current string
        # otherwise, remove adjacent duplicates and continue the recursion

        # Get indices of adjacent duplicates in this scan
        stripped_ixs: Set[int] = set()
        for i in range(1, len(s)):  # O(n)
            if s[i] != s[i - 1]:
                continue
            stripped_ixs.update([i - 1, i])  # O(1)

        # Base case: no more adjacent duplicates
        if len(stripped_ixs) == 0:
            return s

        # Strip out the adjacent duplicate chars in the string based on stripped_ixs, O(n)
        stripped_s = "".join([s[i] for i in range(len(s)) if i not in stripped_ixs])

        return self.recursive_remove_duplicates(stripped_s)

    def stack_remove_duplicates(self, s: str) -> str:
        """
        T(n) = O(n)
        S(n) = O(n)

        WTF ... this looks so simple but actually so hard to come up with ...
        """
        stack: List[str] = []
        last_rm: str = ""
        # print(f"{s=}")
        for c in s:
            # print(f"{stack=}, {c=}, {last_rm=}")
            if c == last_rm:
                continue

            if stack and stack[-1] == c:
                last_rm = stack.pop()
            else:
                stack.append(c)
                last_rm = ""

        # print(f"{stack=}", end="\n\n")
        return "".join(stack)

    def crazy_remove_duplicates(self, s: str) -> str:
        prev_s = s
        while True:
            cur_s = ""
            i = 0
            while i < len(prev_s):
                if i < len(prev_s) - 1 and prev_s[i] == prev_s[i + 1]:
                    while i < len(prev_s) - 1 and prev_s[i] == prev_s[i + 1]:
                        i += 1
                else:
                    cur_s += prev_s[i]
                i += 1
            if cur_s == prev_s:
                break
            prev_s = cur_s
        return prev_s
