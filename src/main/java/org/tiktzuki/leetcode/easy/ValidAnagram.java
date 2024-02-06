package org.tiktzuki.leetcode.easy;

import java.util.HashMap;
import java.util.HashSet;
import java.util.Map;
import java.util.Set;

public class ValidAnagram {
    Set<Map<Integer, Integer>> sets = new HashSet<>();

    public boolean isAnagram(String s, String t) {
        Map<Integer, Integer> m = new HashMap<>();
        s.chars().forEach(c -> m.merge(c, 1, Integer::sum));
        sets.add(m);

        Map<Integer, Integer> m2 = new HashMap<>();
        t.chars().forEach(c -> m2.merge(c, 1, Integer::sum));
        return sets.contains(m2);
    }

    public static void main(String[] args) {
        String s = "rac";
        String t = "car";
        boolean rs = new ValidAnagram().isAnagram(s, t);
        System.out.println(rs);
    }
}
