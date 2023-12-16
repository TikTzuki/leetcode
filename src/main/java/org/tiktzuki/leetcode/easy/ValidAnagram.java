package org.tiktzuki.leetcode.easy;

import java.util.HashMap;
import java.util.Map;

public class ValidAnagram {
    public boolean isAnagram(String s, String t) {
        Map<Integer, Integer> map = new HashMap<>();
        s.chars().forEach(c -> {
            map.merge(c, 1, Integer::sum);
        });
        for (int tChar : t.chars().toArray()) {
            if (map.containsKey(tChar)) {
                int value = map.get(tChar) - 1;
                if (value == 0) {
                    map.remove(tChar);
                }else{
                    map.put(tChar, value);
                }
            } else {
                return false;
            }
        }
        return map.isEmpty();
    }

    public static void main(String[] args) {
        String s = "rat";
        String t = "car";
        boolean rs = new ValidAnagram().isAnagram(s, t);
        System.out.println(rs);
    }
}
