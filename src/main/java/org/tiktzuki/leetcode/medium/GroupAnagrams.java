package org.tiktzuki.leetcode.medium;

import java.util.*;
import java.util.function.IntConsumer;
import java.util.stream.IntStream;

public class GroupAnagrams {
    private int hasCode(String s){
        return s.chars().sum();
    }
    public List<List<String>> groupAnagrams(String[] strs) {
        List<List<String>> rs = new ArrayList<>();
        Map<Integer, List<String>> map = new HashMap<>();
        for(String s: strs){
            int key = s.chars().reduce(1, (left, right) -> {
               return left*right;
            });
            List<String> value = map.getOrDefault(key, new ArrayList<>());
            value.add(s);
            map.put(key, value);
        }
        return map.values().stream().toList();
    }

    public static void main(String[] args) {
        var algorithm  = new GroupAnagrams();
        var input = new String[]{"eat","tea","tan","ate","nat","bat"};
        var output = algorithm.groupAnagrams(input);
        output.forEach(System.out::println);
    }
}
