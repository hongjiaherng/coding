from neetcode150_py.binary_search.q4_median_of_two_sorted_arrays import Solution


def test_find_median_sorted_arrays() -> None:
    sol = Solution()
    assert sol.findMedianSortedArrays(nums1=[1, 3], nums2=[2]) == 2.0
    assert sol.findMedianSortedArrays(nums1=[1, 2], nums2=[3, 4]) == 2.5
    assert sol.findMedianSortedArrays(nums1=[], nums2=[2]) == 2
    assert sol.findMedianSortedArrays(nums1=[2], nums2=[]) == 2
