mod add_two_numbers;
mod longest_substring_without_repeating_characters;
mod median_of_two_sorted_arrays;
/// [5. Longest Palindromic Substring](https://leetcode.com/problems/longest-palindromic-substring/)
mod longest_palindromic_substring;
/// [6. Zigzag Conversion](https://leetcode.com/problems/zigzag-conversion/)
mod zigzag_conversion;

///[7. Reverse Integer](https://leetcode.com/problems/reverse-integer/)  
/// - Given a signed `32-bit` integer `x`, return `x` with its digits reversed.  
/// -If reversing `x` causes the value to go outside the signed 32-bit  
/// integer range [-2E31, 2E31 - 1], then return 0.  
mod reverse_integer;
/// [8. String to Integer (atoi)](https://leetcode.com/problems/string-to-integer-atoi/)
/// 
/// Implement the `myAtoi(string s)` function,   
/// which converts a string to a 32-bit signed integer (similar to C/C++'s atoi function).
mod string_to_integer_atoi;

/// [9. Palindrome Number](https://leetcode.com/problems/palindrome-number/)
/// Given an integer `x`, return `true` if `x` is palindrome integer.
/// An integer is a ***palindrome*** when it reads the same backward as forward.
mod palindrome_number;

/// [10. Regular Expression Matching](https://leetcode.com/problems/regular-expression-matching/)
/// Given an input string `s` and a pattern `p`,   
/// implement regular expression matching with support for '.' and '*'   
/// where:
/// * `'.'` Matches any single character.​​​​
/// * `'*'` Matches zero or more of the preceding element.
mod regular_expression_matching;

/// [11. Container With Most Water](https://leetcode.com/problems/container-with-most-water/)
/// You are given an integer array height of length n.  
/// There are n vertical lines drawn such that the two endpoints of the ith line are (i, 0) and (i, height[i]).  
///**Find** two lines that together with the x-axis form a container, such that the container contains the most water.  
///**Return** the maximum amount of water a container can store.  
mod container_with_most_water;

///[12. Integer to Roman](https://leetcode.com/problems/integer-to-roman/)
/// 
/// Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.
/// | Symbol | Value |
/// | :----: | :---: |
/// | I     | 1      |
/// | V     | 5     |
/// | X | 10|
/// | L | 50 |
/// | C | 100 |
/// | D | 500 |
/// | M | 1000 |
mod integer_to_roman;
/// [13. Roman to Integer](https://leetcode.com/problems/roman-to-integer/)
mod roman_to_integer;

/// [14. Longest Common Prefix](https://leetcode.com/problems/longest-common-prefix)
/// Write a function to find the longest common prefix string amongst an array of strings.  
/// If there is no common prefix, return an empty string "".
mod longest_common_prefix;
///[15. 3Sum](https://leetcode.com/problems/3sum/)
/// Given an integer array nums, return all the triplets `[nums[i], nums[j], nums[k]]`  
/// such that `i != j`, `i != k`, and `j != k`, and `nums[i] + nums[j] + nums[k] == 0`.
mod three_sum;
pub struct Solution;