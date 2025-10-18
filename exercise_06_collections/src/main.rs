use std::collections::HashMap;
use std::collections::HashSet;

fn sum_vec(numbers: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for num in numbers {
        sum += num;
    }
    sum
}

fn double_vec(numbers: &mut Vec<i32>) {
    for num in numbers {
        *num *= 2;
    }
}

fn find_max(numbers: &Vec<i32>) -> Option<i32> {
    let mut max = None;
    for num in numbers {
        if let Some(compare) = max {
            if *num > compare {
                max = Some(*num);
            }
        } else {
            max = Some(*num);
        }
    }
    max
}

fn word_count(text: &str) -> HashMap<String, u32> {
    let mut word_count: HashMap<String, u32> = HashMap::new();

    for word in text.split_whitespace() {
        *word_count.entry(word.to_string()).or_insert(0) += 1;
    }

    word_count
}

fn merge_maps(map1: &HashMap<String, i32>, map2: &HashMap<String, i32>) -> HashMap<String, i32> {
    let mut merged_map: HashMap<String, i32> = HashMap::new();

    for (key, value) in map1 {
        merged_map.insert(key.to_string(), *value);
    }

    for (key, value) in map2 {
        if let Some(old_value) = merged_map.get(key) {
            merged_map.insert(key.to_string(), old_value + *value);
        } else {
            merged_map.insert(key.to_string(), *value);
        }
    }

    merged_map
}

fn unique_elements(numbers: &Vec<i32>) -> HashSet<i32> {
    let mut new_set: HashSet<i32> = HashSet::new();
    for number in numbers {
        new_set.insert(*number);
    }
    new_set
}

fn has_duplicates(numbers: &Vec<i32>) -> bool {
    let mut new_set: HashSet<i32> = HashSet::new();

    for number in numbers {
        if new_set.contains(number) {
            return true;
        }
        new_set.insert(*number);
    }
    false
}

fn common_elements(v1: &Vec<i32>, v2: &Vec<i32>) -> Vec<i32> {
    let mut v1_set: HashSet<i32> = HashSet::new();
    let mut v2_set: HashSet<i32> = HashSet::new();

    for item in v1 {
        v1_set.insert(*item);
    }

    for item in v2 {
        v2_set.insert(*item);
    }

    v1_set.intersection(&v2_set).cloned().collect()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_sum_vec() {
        assert_eq!(sum_vec(&vec![1, 2, 3, 4, 5]), 15);
        assert_eq!(sum_vec(&vec![]), 0);
        assert_eq!(sum_vec(&vec![-1, 1]), 0);
    }

    #[test]
    fn test_double_vec() {
        let mut v = vec![1, 2, 3];
        double_vec(&mut v);
        assert_eq!(v, vec![2, 4, 6]);
    }

    #[test]
    fn test_find_max() {
        assert_eq!(find_max(&vec![1, 5, 3, 2]), Some(5));
        assert_eq!(find_max(&vec![-10, -5, -20]), Some(-5));
        assert_eq!(find_max(&vec![]), None);
    }

    #[test]
    fn test_word_count() {
        let counts = word_count("hello world hello rust");
        assert_eq!(counts.get("hello"), Some(&2));
        assert_eq!(counts.get("world"), Some(&1));
        assert_eq!(counts.get("rust"), Some(&1));
    }

    #[test]
    fn test_merge_maps() {
        let mut map1 = HashMap::new();
        map1.insert(String::from("a"), 1);
        map1.insert(String::from("b"), 2);

        let mut map2 = HashMap::new();
        map2.insert(String::from("b"), 3);
        map2.insert(String::from("c"), 4);

        let merged = merge_maps(&map1, &map2);
        assert_eq!(merged.get("a"), Some(&1));
        assert_eq!(merged.get("b"), Some(&5)); // 2 + 3
        assert_eq!(merged.get("c"), Some(&4));
    }

    #[test]
    fn test_unique_elements() {
        let result = unique_elements(&vec![1, 2, 2, 3, 3, 3]);
        let expected: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_has_duplicates() {
        assert_eq!(has_duplicates(&vec![1, 2, 3, 4]), false);
        assert_eq!(has_duplicates(&vec![1, 2, 2, 3]), true);
        assert_eq!(has_duplicates(&vec![]), false);
    }

    #[test]
    fn test_common_elements() {
        let v1 = vec![1, 2, 3, 4];
        let v2 = vec![3, 4, 5, 6];
        let mut result = common_elements(&v1, &v2);
        result.sort(); // HashSet order is not guaranteed
        assert_eq!(result, vec![3, 4]);
    }
}
