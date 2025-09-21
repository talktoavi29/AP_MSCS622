function twoSum(nums, target) {
  const seen = new Map();
  for (let i = 0; i < nums.length; i++) {
    const x = nums[i];
    const y = target - x;
    if (seen.has(y)) return [seen.get(y), i];
    seen.set(x, i);
  }
  return null;
}

console.log(twoSum([2, 7, 11, 15], 9));