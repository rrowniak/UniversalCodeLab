#include <iostream>
#include <string>
#include <algorithm>

using namespace std;

void palindrome(const std::string& in) {
    bool is_palindrome = true;
    int left = 0;
    int right = in.size() - 1;
    for (; left < right; left++, right--) {
        if (in[left] != in[right]) {
            is_palindrome = false;
            break;
        }
    }
    
    if (is_palindrome) {
        cout << in << " is palindrome" << endl;
    } else {
        cout << in << " in not palindrome" << endl;
    }
}

void palindrome2(const std::string& in) {
    if (equal(in.begin(), in.end(), in.rbegin())) {
        cout << in << " is palindrome" << endl;
    } else {
        cout << in << " is not palindrome" << endl;
    }
}

int main()
{
    palindrome("madam");
    palindrome("madam2");

    palindrome2("madam");
    palindrome2("madam2");

    return 0;
}
