use std::collections::HashMap;
use std::hash::Hash;
use std::ops::AddAssign;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hashmap_drain() {
        let mut map = HashMap::from([(1, 1), (2, 2), (3, 3)]);
        let x = map.drain().next().expect("should have 1 element");
        assert!([(1, 1), (2, 2), (3, 3)].contains(&x));
        assert!(!map.contains_key(&x.0))
    }

    #[test]
    fn hashmap_iter_mut() {
        let mut map = HashMap::from([(1, 1), (2, 2), (3, 3)]);
        map.iter_mut().for_each(|(_i, j)| *j += 1);
        map.iter().for_each(|(i, j)| assert_eq!(*i, j - 1));
    }

    #[test]
    fn hashmap_get_key_value() {
        let map = HashMap::from([(1, 1), (2, 2), (3, 3)]);
        let pair = map.get_key_value(&1).expect("should have value");
        assert_eq!(pair.0, pair.1);
        assert!(map.get_key_value(&5).is_none());
    }

    #[test]
    fn hashmap_get_mut() {
        let mut map = HashMap::from([(1, 1), (2, 2), (3, 3)]);
        map.get_mut(&1).into_iter().for_each(|x| *x += 1);
        assert_eq!(map.get(&1).cloned(), Some(2));
    }
    #[test]
    fn hashmap_remove_entry() {
        let mut map = HashMap::from([(1, 1), (2, 2), (3, 3)]);
        assert_eq!(map.remove_entry(&1), Some((1, 1)));
    }
    #[test]
    fn hashmap_entry_and_modify() {
        let mut map = HashMap::from([(1, 1), (2, 2), (3, 3)]);
        map.entry(4).and_modify(|x| *x += 1);
        map.entry(1).and_modify(|x| *x += 1);
        let mut res = map.drain().collect::<Vec<_>>();
        res.sort();
        assert_eq!(res, [(1, 2), (2, 2), (3, 3)]);
    }
    #[test]
    fn hashmap_entry_or_insert() {
        let mut map = HashMap::from([(1, 1), (2, 2), (3, 3)]);
        map.entry(4).or_insert(4);
        map.entry(1).or_insert(0);
        let mut res = map.drain().collect::<Vec<_>>();
        res.sort();
        assert_eq!(res, [(1, 1), (2, 2), (3, 3), (4, 4)]);
    }
    #[test]
    fn hashmap_entry_or_default() {
        let mut map = HashMap::from([(1, 1), (2, 2), (3, 3)]);
        map.entry(4).or_default();
        map.entry(1).or_default();
        let mut res = map.drain().collect::<Vec<_>>();
        res.sort();
        assert_eq!(res, [(1, 1), (2, 2), (3, 3), (4, 0)]);
    }
    #[test]
    fn hashmap_entry_or_insert_with() {
        let mut map = HashMap::from([(1, 1), (2, 2), (3, 3)]);
        map.entry(4).or_insert_with(|| 4);
        map.entry(1).or_insert_with(|| 0);
        let mut res = map.drain().collect::<Vec<_>>();
        res.sort();
        assert_eq!(res, [(1, 1), (2, 2), (3, 3), (4, 4)]);
    }
    #[test]
    fn hashmap_entry_or_insert_with_key() {
        let mut map = HashMap::from([(1, 1), (2, 2), (3, 3)]);
        map.entry(4).or_insert_with_key(|x| x - 1);
        map.entry(1).or_insert_with_key(|x| x - 1);
        let mut res = map.drain().collect::<Vec<_>>();
        res.sort();
        assert_eq!(res, [(1, 1), (2, 2), (3, 3), (4, 3)]);
    }
}

#[derive(Default)]
pub struct Chain<T>
where
    T: Eq + Hash + Clone,
{
    transitions: HashMap<(T, T), usize>,
    space: HashMap<T, usize>,
}

impl<T> Chain<T>
where
    T: Eq + Hash + Clone,
{
    pub fn train(&mut self, sequence: &[T]) -> &mut Self {
        sequence.windows(2).for_each(|pair| {
            self.space.entry(pair[0].clone()).or_default().add_assign(1);
            self.transitions
                .entry((pair[0].clone(), pair[1].clone()))
                .or_default()
                .add_assign(1);
        });
        self
    }
    pub fn most_likely_after(&self, token: T) -> Option<T> {
        self.space
            .iter()
            .map(|x| ((token.clone(), x.0.clone()), *x.1))
            .filter_map(|(t, cnt)| {
                self.transitions
                    .get_key_value(&t)
                    .map(|t| (t.0.clone(), (*t.1, cnt)))
            })
            .max_by_key(|(_, key)| *key)
            .map(|((_, target), _)| target)
    }
}

#[cfg(test)]
mod chain_test {
    use super::*;
    #[test]
    fn easy_chain() {
        assert_eq!(
            Chain::default()
                .train(&[1, 2, 1, 2, 1, 2, 1, 2, 1, 2])
                .most_likely_after(2),
            Some(1)
        );
        assert_eq!(
            Chain::default()
                .train(&[1, 2, 1, 2, 1, 2, 1, 2, 1, 2])
                .most_likely_after(3),
            None
        )
    }
    #[test]
    fn variance_chain() {
        assert_eq!(
            Chain::default()
                .train(&[1, 2, 3, 4, 1, 2, 3, 4, 1, 3, 2, 4])
                .most_likely_after(1),
            Some(2)
        )
    }
}
fn timed<F : Fn(&[i32]), S : AsRef<str>>(name: S, f : F, input: Vec<i32>) {
    let mut count = 0;
    let mut time = 0;
    for _ in 0..100 {
        let start = std::time::Instant::now();
        f(std::hint::black_box(&input));
        let end = std::time::Instant::now();
        time += end.duration_since(start).as_nanos();
        count += input.len();
    }
    eprintln!("{} speed: {} tokens/sec", name.as_ref(), (count as f64) * 1.0e9 / (time as f64))
}

#[test]
fn benchmark() {
    let n = 1000000;
    {
        let a: Vec<i32> = (1i32..=1i32).cycle().take(n).collect();
        timed("cycle-1", |x| { Chain::default().train(x); }, a);
    }
    {
        let a: Vec<i32> = (1i32..=100i32).cycle().take(n).collect();
        timed("cycle-100", |x| { Chain::default().train(x); }, a);
    }
    {
        let a: Vec<i32> = (1i32..=10000).cycle().take(n).collect();
        timed("cycle-10k", |x| { Chain::default().train(x); }, a);
    }
}