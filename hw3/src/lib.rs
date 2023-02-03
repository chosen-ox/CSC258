pub mod markov {
    use std::collections::HashMap;
    use std::hash::Hash;

    pub struct Chain<T: Eq + Hash + Clone>(HashMap<T, HashMap<T, f64>>);

    impl<T: Eq + Hash + Clone + std::fmt::Debug> Chain<T> {
        pub fn new() -> Self {
            Self(HashMap::new())
        }

        pub fn train(&mut self, sequence: &[T]) -> &mut Self {
            // Count transitions
            (0..sequence.len() - 1).for_each(|i| {
                *self
                    .0
                    .entry(sequence[i].clone())
                    .or_insert(HashMap::new())
                    .entry(sequence[i + 1].clone())
                    .or_insert(0.0) += 1.0
            });

            // Scale
            let mut freq = HashMap::<&T, u32>::new();
            sequence[0..sequence.len() - 1]
                .iter()
                .for_each(|t| *freq.entry(t).or_insert(0) += 1);
            self.0
                .iter_mut()
                .for_each(|(k, v)| v.iter_mut().for_each(|(_, p)| *p /= freq[k] as f64));
            //self.verify();
            self
        }

        pub fn verify(&self) {
            println!("{:?}", self.0);
            self.0.iter().for_each(|(_, v)| {
                let sum = v.values().fold(0.0, |s, p| s + p);
                assert!(sum == 1.0 || sum == 0.9999999999999999);
            });
        }

        /// Real markov chains will sample the distribution, not just pick the max
        pub fn most_likely_after(&self, token: T) -> Option<T> {
            self.0
                .get(&token)
                .map(|map| {
                    map.iter()
                        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                        .unwrap()
                        .0
                })
                .cloned()
        }

        /// Generate a sequence of tokens of a certain length using the most_likely_after
        /// prediction function. A better version would be one that samples the distribution
        /// properly, not just picking the max probability.
        pub fn generate(&self, start: T, len: usize) -> Vec<T> {
            (0..len).fold(vec![start], |mut v, _| {
                match self.most_likely_after(v.last().unwrap().clone()) {
                    Some(t) => v.push(t),
                    None => (),
                }
                v
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::markov;
    use std::collections::HashMap;
    use std::collections::HashSet;

    /* -- HASHMAP TESTS -- */

    fn testmap() -> HashMap<String, u32> {
        let keys = vec!["Rust", "Go", "C"];
        let mut map: HashMap<String, u32> = HashMap::new();
        keys.into_iter().enumerate().for_each(|(i, x)| {
            map.insert(x.to_string(), i as u32 + 2);
        });
        map
    }

    #[test]
    fn hashmap_drain() {
        let mut map = testmap();

        assert_eq!(
            map.drain().fold(1, |t, (k, v)| {
                t + k
                    .chars()
                    .fold(5381, |h, c| ((h << 5) % 5381 + h) + c as i32)
                    * v as i32
            }),
            78036
        );

        assert_eq!(map.keys().collect::<Vec<&String>>(), Vec::<&String>::new());
    }

    #[test]
    fn hashmap_itermut() {
        let mut map = testmap();
        map.iter_mut().for_each(|(_, v)| *v *= 2);

        assert_eq!(
            map.values().map(|x| *x).collect::<HashSet<u32>>(),
            (4..8 + 1).step_by(2).collect::<HashSet<u32>>()
        );
    }

    #[test]
    fn hashmap_get_key_value() {
        let map = testmap();
        assert_eq!(map.get_key_value("oops"), None);
        assert_eq!(map.get_key_value("Rust"), Some((&"Rust".to_string(), &2)));
        assert_eq!(map.get_key_value("Go"), Some((&"Go".to_string(), &3)));
        assert_eq!(map.get_key_value("C"), Some((&"C".to_string(), &4)));
    }

    #[test]
    fn hashmap_get_mut() {
        let mut map = testmap();
        assert_eq!(map.get("Rust"), Some(&2));
        map.get_mut("Rust").map(|x| *x = 1000);
        assert_eq!(map.get("Rust"), Some(&1000));
    }

    #[test]
    fn hashmap_remove_entry() {
        let mut map = testmap();
        assert_eq!(map.remove_entry("Rust"), Some(("Rust".to_string(), 2)));
        assert_eq!(
            map.keys().map(|s| s.as_ref()).collect::<HashSet<&str>>(),
            HashSet::from_iter(vec!["C", "Go"].into_iter())
        );
    }

    /* -- ENTRY TESTS -- */

    #[test]
    fn entry_and_modify_or_insert() {
        let mut map = testmap();

        // Test some
        assert_eq!(map.get("Rust"), Some(&2));
        map.entry("Rust".to_string())
            .and_modify(|v| *v /= 2)
            .or_insert(0);
        assert_eq!(map.get("Rust"), Some(&1));

        // Test none
        assert_eq!(map.get("OCaml"), None);
        map.entry("OCaml".to_string())
            .and_modify(|v| *v /= 2)
            .or_insert(0);
        assert_eq!(map.get("OCaml"), Some(&0));
    }

    #[test]
    fn entry_or_default() {
        let mut map = testmap();

        // Some
        assert_eq!(map.get("Rust"), Some(&2));
        map.entry("Rust".to_string()).or_default();
        assert_eq!(map.get("Rust"), Some(&2));

        // None
        assert_eq!(map.get("Scala"), None);
        map.entry("Scala".to_string()).or_default();
        assert_eq!(map.get("Scala"), Some(&0));
    }

    fn fact(n: u32) -> u32 {
        if n == 0 || n == 1 {
            1
        } else {
            n * fact(n - 1)
        }
    }

    #[test]
    fn entry_or_insert_with() {
        let mut map = testmap();

        // Some case
        assert_eq!(map.get("Rust"), Some(&2));
        map.entry("Rust".to_string()).or_insert_with(|| 10);
        assert_eq!(map.get("Rust"), Some(&2));

        // None case
        assert_eq!(map.get("Haskell"), None);
        map.entry("Haskell".to_string()).or_insert_with(|| 10);
        assert_eq!(map.get("Haskell"), Some(&10));
    }

    #[test]
    fn entry_or_insert_with_key() {
        let mut map = testmap();

        // Some case
        assert_eq!(map.get("Rust"), Some(&2));
        map.entry("Rust".to_string())
            .or_insert_with_key(|k| fact(k.len() as u32));
        assert_eq!(map.get("Rust"), Some(&2));

        // None case
        assert_eq!(map.get("Haskell"), None);
        map.entry("Haskell".to_string())
            .or_insert_with_key(|k| fact(k.len() as u32)); // Something expensive
        assert_eq!(map.get("Haskell"), Some(&5040));
    }

    /* --  MARKOV CHAIN TESTS -- */

    #[test]
    fn test_markov_1() {
        let mut chain = markov::Chain::new();
        let seq = [
            3, 8, 1, 1, 5, 9, 9, 1, 4, 2, 2, 2, 2, 2, 5, 2, 7, 4, 3, 7, 7, 3, 8, 7, 4, 1,
        ];

        chain.train(&seq);
        assert_eq!(chain.most_likely_after(0), None);
        assert!(
            chain.most_likely_after(1) == Some(4)
                || chain.most_likely_after(1) == Some(1)
                || chain.most_likely_after(1) == Some(5)
        );
        assert_eq!(chain.most_likely_after(2), Some(2));
        assert_eq!(chain.most_likely_after(3), Some(8));
        assert!(
            chain.most_likely_after(4) == Some(1)
                || chain.most_likely_after(4) == Some(3)
                || chain.most_likely_after(4) == Some(2)
        );
        assert!(chain.most_likely_after(5) == Some(2) || chain.most_likely_after(5) == Some(9));
        assert_eq!(chain.most_likely_after(7), Some(4));
        assert!(chain.most_likely_after(8) == Some(1) || chain.most_likely_after(8) == Some(7));
        assert!(chain.most_likely_after(9) == Some(1) || chain.most_likely_after(9) == Some(9));

        println!("generated seq: {:?}", chain.generate(1, 200));
    }

    #[test]
    fn test_markov_2() {
        let mut chain = markov::Chain::new();
        let seq =
            "the world is not a very nice place and the nice place which is not the world is not very cool".split(" ").collect::<Vec<&str>>();

        chain.train(&seq);
        assert_eq!(chain.most_likely_after(""), None);
        assert!(
            chain.most_likely_after("place") == Some("and")
                || chain.most_likely_after("place") == Some("which")
        );
        assert_eq!(chain.most_likely_after("the"), Some("world"));
        assert_eq!(chain.most_likely_after("which"), Some("is"));
        assert!(
            chain.most_likely_after("not") == Some("a")
                || chain.most_likely_after("not") == Some("the")
                || chain.most_likely_after("not") == Some("very")
        );

        println!("generated seq: {:?}", chain.generate("world", 50));
    }
}
