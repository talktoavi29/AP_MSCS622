// Run: g++ -std=c++17 -O2 main.cpp && ./a.out
#include <iostream>
#include <memory>

int sum(const int* a, int n) {
    int t = 0;
    for (int i = 0; i < n; ++i) t += a[i];
    return t;
}

int main() {
    int n = 5;

    int* raw = new int[n]{1,2,3,4,5};
    std::cout << "C++ raw sum = " << sum(raw, n) << "\n";

    auto buf = std::make_unique<int[]>(n);
    for (int i = 0; i < n; ++i) buf[i] = i + 1;
    std::cout << "C++ RAII sum = " << sum(buf.get(), n) << "\n";

    int* dangling;
    { int x = 42; dangling = &x; }
    std::cout << *dangling << "\n";

    return 0;
}
