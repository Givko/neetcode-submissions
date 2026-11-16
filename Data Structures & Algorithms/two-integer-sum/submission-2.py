# I DONT WANT HELP FROM AI HERE
class Solution:
  def twoSum(self, nums: list[int], target: int) -> list[int]:
    for index, num in enumerate(nums):
      x = target - num;
      try:
        i = nums.index(x)
        if i == index:
          continue

        return sorted([index, i])
      except Exception as e:
        print("not found {e}")
        continue

    return []
    
        