package org.tiktzuki.leetcode.medium;

import java.util.ArrayList;
import java.util.List;

public class LongestPalindromicSubstring {
    public String longestPalindrome(String s) {
        if (s.length() == 1) {
            return s;
        }
        List<String> palindrome = new ArrayList<>();
        for (int k = 1; k < s.length(); k++) {
            StringBuilder kPalindrome = new StringBuilder();
            kPalindrome.append(s.charAt(k));
            int i = k - 1;
            int j = k + 1;
            if (k <= s.length() - 2 && s.charAt(k + 1) == s.charAt(k)) {
                j = k + 2;
                kPalindrome.append(s.charAt(k));
            }
            while (i >= 0 && j < s.length()) {
                char iChar = s.charAt(i);
                char jChar = s.charAt(j);
                if (iChar == jChar) {
                    kPalindrome.insert(0, iChar);
                    kPalindrome.append(jChar);
                }
                i--;
                j++;
            }
            palindrome.add(kPalindrome.toString());
        }
        return palindrome.stream().max((s1, s2) -> s1.length() - s2.length()).get();
    }


    public static void main(String[] args) {
        String s = "bb";
        System.out.printf(new LongestPalindromicSubstring().longestPalindrome(s));
    }
}
