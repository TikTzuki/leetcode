package org.tiktzuki.leetcode.medium;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class GroupAnagrams {
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
                } else {
                    map.put(tChar, value);
                }
            } else {
                return false;
            }
        }
        return map.isEmpty();
    }

    public List<List<String>> groupAnagrams(String[] strs) {
        List<List<String>> groups = new ArrayList<>();
        for (int i = 0; i < strs.length; i++) {
            if (strs[i] == null)
                continue;
            List<String> group = new ArrayList<>();
            for (int j = i + 1; j < strs.length; j++) {
                if (strs[j] == null)
                    continue;
                if (isAnagram(strs[i], strs[j])) {
                    group.add(strs[j]);
                    strs[j] = null;
                }
            }
            group.add(strs[i]);
            strs[i] = null;
            groups.add(group);
        }
        return groups;
    }

    public static void main(String[] args) {
        var algorithm = new GroupAnagrams();
        var input = new String[]{"eat", "tea", "tan", "ate", "nat", "bat"};
        var output = algorithm.groupAnagrams(input);
        output.forEach(System.out::println);
        System.out.println("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" == "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    }
}
