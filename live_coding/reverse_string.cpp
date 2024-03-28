#include <iostream>
#include <string>
#include <algorithm>

int main() {
    std::string str = "this is string to be reversed";
    std::cout << "Before: " << str << std::endl;

    for (int i = 0; i < str.size() / 2; i++) {
        auto c = str[i];
        auto k = str.size() - i - 1;
        str[i] = str[k];
        str[k] = c;
    }

    std::cout << "After: " << str << std::endl;
}
