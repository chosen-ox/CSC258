extern crate core;

mod sequential_word_count {
    use std::collections::HashMap;

    #[allow(dead_code)]
    fn word_count(article: &str) -> HashMap<String, i64> {

        let mut map: HashMap<String, i64> = HashMap::new();
        // let article = article.to_lowercase();
        article.split_whitespace()
            .for_each(|word|
                *map.entry(word.to_string())
                    .or_insert(0) += 1
            );
        map
    }

    #[cfg(test)]
    #[test]
    fn test_sequential() {
        use rand::Rng;
        use rand::distributions::Alphanumeric;

        //generate a random string with 1 million random words
        let mut origin_map = HashMap::<String, i64>::new();
        let mut article = String::new();
        for _ in 0..1000000 {
            let rng = rand::thread_rng();
            let word :String = rng.sample_iter(&Alphanumeric).map(|x| x as char).take(5).collect::<String>();
            origin_map.entry(word.clone())
                .and_modify(|e| *e += 1)
                .or_insert(1);
            article.push_str(&word);
            article.push(' ');
        }
        let map = word_count(&article);
        assert!(map.eq(&origin_map));
    }
}
