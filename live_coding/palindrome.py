def palindrome(input):
    left = 0
    right = len(input) - 1
    is_palindrome = True
    while left < right:
        if input[left] != input[right]:
            is_palindrome = False
            break
        left += 1
        right -= 1

    if is_palindrome:
        print(f'{input} is palindrome')
    else:
        print(f'{input} is not palindrome')


def palindrome2(s):
    # Remove spaces and convert the string to lowercase
    # for case-insensitive comparison
    s = s.replace(" ", "").lower()
    if s == s[::-1]:
        print(f'{s} is palindrome')
    else:
        print(f'{s} is not palindrome')


palindrome('madam')
palindrome('madam1')

palindrome2('madam')
palindrome2('madam1')
