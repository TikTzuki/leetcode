package org.tiktzuki.leetcode.easy;

import java.util.HashMap;
import java.util.Map;

public class ValidAnagram {
    public boolean isAnagram(String s, String t) {
        Map<Integer, Integer> map = new HashMap<>();
        StringBuilder tBuilder = new StringBuilder(t);
        s.chars().forEach(c -> {
            tBuilder.deleteCharAt(t.indexOf(c));
        });
        System.out.println(tBuilder.toString());
        return map.values().stream().allMatch(x -> x.equals(0));
    }

    public static void main(String[] args) {
        String s = "rat";
        String t = "car";
        boolean rs = new ValidAnagram().isAnagram(s, t);
        System.out.println(rs);
    }
}
