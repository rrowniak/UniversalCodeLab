#include <iostream>
#include <algorithm>
#include <vector>

void merge_arrays() {
    std::vector<int> arr1 = {1, 3, 5, -1, -6};
    std::vector<int> arr2 = {11, -11, 8};

    std::sort(arr1.begin(), arr1.end());
    std::sort(arr2.begin(), arr2.end());

    std::vector<int> res;
    size_t i = 0, j = 0;
    while (true) {
        if (i < arr1.size() && j < arr2.size()) {
            if (arr1[i] < arr2[j]) {
                res.push_back(arr1[i]);
                i++;
                continue;
            } else {
                res.push_back(arr2[j]);
                j++;
                continue;
            }
        }

        if (i < arr1.size()) {
            res.push_back(arr1[i]);
            i++;
            continue;
        }

        if (j < arr2.size()) {
            res.push_back(arr2[j]);
            j++;
            continue;
        }

        break;
    }

    for (auto v: res) {
        std::cout << v << " ";
    }
    std::cout << std::endl;
}

int main() {
    std::cout << "Hello" << std::endl;
    merge_arrays();
}
