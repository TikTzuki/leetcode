package org.tiktzuki.leetcode.easy;

import java.util.HashMap;
import java.util.Map;

public class ValidAnagram {
    public boolean isAnagram(String s, String t) {
        Map<Integer, Integer> map = new HashMap<>();
        s.chars().forEach(c -> {
            map.merge(c, 1, Integer::sum);
        });
        t.chars().forEach(c -> {
            int count = map.getOrDefault(c, 0) - 1;
            map.put(c, count);
        });
        return map.values().stream().allMatch(x->x.equals(0));
    }
    public static void main(String[] args) {
        String s = "rat";
        String t = "car";
        boolean rs = new ValidAnagram().isAnagram(s, t);
        System.out.println(rs);
    }
}
