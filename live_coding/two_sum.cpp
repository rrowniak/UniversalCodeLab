#include <iostream>
#include <algorithm>
#include <vector>

bool exists_bs(const std::vector<int>& v, size_t from, size_t to, int val) {
    if (to - from < 2) {
        if (v[from] == val) {
            return true;
        } else if (v[to] == val) {
            return true;
        } else {
            return false;
        }
    }

    auto mid = from + (to - from) / 2;
    if (v[mid] == val) {
        return true;
    }

    if (v[mid] > val) {
        return exists_bs(v, from, mid - 1, val);
    } else {
        return exists_bs(v, mid + 1, to, val);
    }
}

int main() {
    std::vector<int> array = { 1, 5, -3, 8, 6, 10, 2, 15 };
    const int sum = 12;

    std::sort(array.begin(), array.end());
    for (int i = 0; i < array.size() - 1; ++i) {
        auto a = array[i];
        auto missing = sum - a;

        // check if missing is in the array
        // use binary search
        if (exists_bs(array, i+1, array.size() - 1, missing)) {
            std::cout << "We have mach: " << a << " + " << missing << " = " << sum;
            std::cout << std::endl;
        }
    }
}
