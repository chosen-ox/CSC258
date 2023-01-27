use std::hash::Hash;
use std::collections::HashMap;
// 2.1 HashMap Tests
#[cfg(test)]
mod hashmap_tests {
    use std::collections::HashMap;
    #[test]
    fn test_drain() {
        let mut map = HashMap::new();
        map.insert(1, 2);
        map.insert(3, 4);

        for (k, v) in map.drain() {
            assert!((k, v) == (1, 2) || (k, v) == (3, 4));
        }

        assert!(map.is_empty());

        map.insert(5, 6);
        map.insert(7, 8);

        // Iter is dropped before fully consumed because of 'take(1)', but it still drops the remaining
        // key/value pairs.
        for (k, v) in map.drain().take(1) {
            assert!((k, v) == (5, 6) || (k, v) == (7, 8));
        }
        assert!(map.is_empty());

    }

    #[test]
    fn test_iter_mut() {
        let mut map = HashMap::new();
        map.insert(1, 2);
        map.insert(3, 4);
        map.insert(4, 5);

        for (k, v) in map.iter_mut() {
            if (k % 2) == 0 {
                *v = 10;
                assert_eq!(*k, 4);
            }
            else {
                *v = 20;
                assert!(*k == 1 || *k == 3);
            }
        }
        assert_eq!(map.get(&1), Some(&20));
        assert_eq!(map.get(&3), Some(&20));
        assert_eq!(map.get(&4), Some(&10));

        for (k, v) in map.iter_mut() {
            if (k % 2) == 0 {
                *v = 20;
                assert_eq!(*k, 4);
            }
            else {
                *v = 10;
                assert!(*k == 1 || *k == 3);
            }
        }

        assert_eq!(map.get(&1), Some(&10));
        assert_eq!(map.get(&3), Some(&10));
        assert_eq!(map.get(&4), Some(&20));
    }

    #[test]
    fn test_get_key_value() {
        let mut map = HashMap::new();
        map.insert(1, 2);
        map.insert(3, 4);
        map.insert(4, 5);

        assert_eq!(map.get_key_value(&1), Some((&1, &2)));
        assert_eq!(map.get_key_value(&3), Some((&3, &4)));
        assert_eq!(map.get_key_value(&4), Some((&4, &5)));
        assert_eq!(map.get_key_value(&5), None);
    }

    #[test]
    fn test_get_mut() {
        let mut map = HashMap::new();
        map.insert(1, 2);
        map.insert(3, 4);

        if let Some(x) = map.get_mut(&1) {
            assert_eq!(x, &mut 2);
            *x = 20;
        }
        assert_eq!(map.get(&1), Some(&20));

        if let Some(_x) = map.get_mut(&5) {
            // no key 5 in the map
            panic!("Should not be here");
        }
    }

    #[test]
    fn test_remove_entry() {
        let mut map = HashMap::new();
        map.insert(1, 2);
        map.insert(3, 4);

        assert_eq!(map.remove_entry(&1), Some((1, 2)));
        assert_eq!(map.get(&1), None);
        assert_eq!(map.remove_entry(&5), None);
        assert_eq!(map.remove_entry(&5), None);
    }
}

#[cfg(test)]
mod hashmap_entry_tests{
    use std::collections::HashMap;

    #[test]
    fn test_and_modify() {
        let mut map = HashMap::new();
        map.insert(1, 2);
        map.insert(3, 4);

        map.entry(1).and_modify(|x| *x += 1);
        assert_eq!(map.get(&1), Some(&3));

        map.entry(5).and_modify(|x| *x += 1);
        assert_eq!(map.get(&5), None);
    }

    #[test]
    fn test_or_insert() {
        let mut map = HashMap::new();
        map.insert(1, 2);
        map.insert(3, 4);

        map.entry(1).or_insert(5);
        assert_eq!(map.get(&1), Some(&2));

        map.entry(5).or_insert(6);
        assert_eq!(map.get(&5), Some(&6));
    }

    #[test]
    fn test_or_insert_default() {
        let mut map = HashMap::new();
        map.insert(1, 2);
        map.insert(3, 4);

        map.entry(1).or_default();
        assert_eq!(map.get(&1), Some(&2));

        map.entry(5).or_default();
        assert_eq!(map.get(&5), Some(&0));
    }

    #[test]
    fn test_or_insert_with() {
        let mut map = HashMap::new();
        map.insert(1, 2);
        map.insert(3, 4);

        map.entry(1).or_insert_with(|| 5);
        assert_eq!(map.get(&1), Some(&2));

        map.entry(5).or_insert_with(|| 6);
        assert_eq!(map.get(&5), Some(&6));
    }

    #[test]
    fn test_or_insert_with_key() {
        let mut map = HashMap::new();
        map.insert(1, 2);
        map.insert(3, 4);

        map.entry(1).or_insert_with_key(|k| *k + 1);
        assert_eq!(map.get(&1), Some(&2));

        map.entry(5).or_insert_with_key(|k| *k + 10);
        assert_eq!(map.get(&5), Some(&15));
    }
}

// 2.2 Markov Chain
pub struct Chain<T> where T: Eq+Hash+Clone {
    graph : HashMap<T, HashMap<T, u32>>,
}

impl <T> Chain<T> where T: Eq+Hash+Clone {
    pub fn new() -> Chain<T> {
        Chain { graph: HashMap::new() }
    }

    pub fn add(&mut self, from: T, to: T) {
        let mut from_map = self.graph.entry(from).or_insert(HashMap::new());
        let count = from_map.entry(to).or_default();
        *count += 1;
    }

    pub fn train(&mut self, sequence: &[T]) -> &mut Chain<T> {
        for (from, to) in sequence.iter().zip(sequence.iter().skip(1)) {
            self.add(from.clone(), to.clone());
        }
        self
    }

    pub fn most_likely_after(&self, from: &T) -> Option<&T> {
        self.graph.get(from).and_then(|map| {
            map.iter().max_by_key(|&(_, weight)| weight).map(|(to, _)| to)
        })
    }

    pub fn get(&self, from: &T) -> Option<&HashMap<T, u32>> {
        self.graph.get(from)
    }

    pub fn generate_from_seed(&self, seed: &T, length: usize) -> Vec<T> {
        let mut result = vec![seed.clone()];
        for _ in 0..length-1 {
            if let Some(next) = self.most_likely_after(&result[result.len()-1]) {
                result.push(next.clone());
            }
        }
        result
    }

    pub fn generate(&self, length: usize) -> Vec<T> {
        if let Some(seed) = self.graph.keys().next() {
            self.generate_from_seed(seed, length )
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod tt {
    use std::collections::HashMap;
    use crate::Chain;
    #[test]
    fn main() {
        let mut chain = Chain::<i32>::new();
        chain.train(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1 ,3]).train(&[5, 6, 10]);
        let array = chain.generate_from_seed( &3, 10);
        println!("{:?}", array);

        let mut chain = Chain::<&str>::new();
        chain.train(&["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "a", "c"]);
        let array = chain.generate_from_seed( &"a", 10);
        println!("{:?}", array);

        let mut chain = Chain::<String>::new();
        chain.train(&[String::from("I"), String::from("love"), String::from("you")]);
        let array = chain.generate_from_seed( &String::from("I"), 10);
        assert_eq!(array, vec![String::from("I"), String::from("love"), String::from("you")]);

    }
}