package org.tiktzuki.leetcode.easy;

import java.util.HashSet;
import java.util.Set;

public class Pangram {
    public boolean checkIfPangram(String sentence) {
        Set<Character> characters = new HashSet<>();
        for (char c : sentence.toLowerCase().toCharArray()) {
            if (97 <= c && c <= 122)
                characters.add(c);
        }
        return characters.size() == 26;
    }


    public static void main(String[] args) {
        String input = "Cwm fjord bank glyphs vext quiz";
        boolean result = new Pangram().checkIfPangram(input);
        assert result;
    }
}
