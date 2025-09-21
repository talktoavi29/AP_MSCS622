#include <vector>
#include <unordered_map>
#include <cstdio>

std::vector<int> twoSum(const std::vector<int>& nums, int target) {
    std::unordered_map<int, int> seen;      
    seen.reserve(nums.size());  
    for (int i = 0; i < static_cast<int>(nums.size()); ++i) {
        int x = nums[i];
        int y = target - x;
        auto it = seen.find(y);
        if (it != seen.end()) return {it->second, i};
        seen[x] = i;
    }
    return {};
}

int main() {
    std::vector<int> result = twoSum({2, 7, 11, 15}, 9);
    for (std::size_t i = 0; i < result.size(); ++i) {
        std::printf("%d", result[i]);
        if (i + 1 < result.size()) std::printf(" ");
    }
    std::printf("\n");
    return 0;
}