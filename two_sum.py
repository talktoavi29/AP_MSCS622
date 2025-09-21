def two_sum(nums, target):
    seen = {} 
    for i, x in enumerate(nums):
        y = target - x
        if y in seen:
            return [seen[y], i]
        seen[x] = i
    return None

print(two_sum([2, 7, 11, 15], 9))