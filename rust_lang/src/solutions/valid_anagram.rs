// Set<Map<Integer, Integer>> SETS = new HashSet<>();
//
// public boolean isAnagram(String s, String t) {
// Map<Integer, Integer> m = new HashMap<>();
// s.chars().forEach(c -> m.merge(c, 1, Integer::sum));
// SETS.add(m);
//
// Map<Integer, Integer> m2 = new HashMap<>();
// t.chars().forEach(c -> m2.merge(c, 1, Integer::sum));
// return SETS.contains(m2);
// }
//
// public static void main(String[] args) {
// String s = "rac";
// String t = "car";
// boolean rs = new ValidAnagram().isAnagram(s, t);
// System.out.println(rs);
// }

use std::collections::HashMap;

#[derive(Debug)]
struct ValidAnagram {}


impl ValidAnagram {
    pub fn is_anagram(s: String, t: String) -> bool {
        if (s.len() != t.len()) {
            return false;
        }

        let mut m: HashMap<char, usize> = HashMap::new();
        for k in s.chars() {
            *m.entry(k).or_insert(0) += 1;
        }

        let mut m2: HashMap<char, usize> = HashMap::new();
        for k in t.chars() {
            *m2.entry(k).or_insert(0) += 1;
        }

        return m == m2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run() {
        let s = "car".parse().unwrap();
        let t = "rat".parse().unwrap();
        let result = ValidAnagram::is_anagram(s, t);
        println!("{:?}", result);
    }
}
