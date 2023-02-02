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
            let word: String = rng.sample_iter(&Alphanumeric).filter(|u| u >= &65u8).map(|u| u as char).take(5).collect::<String>();
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

mod parallel_word_count {
    use std::{
        collections::HashMap,
        sync::{Arc, Mutex, RwLock},
        thread,
        time::Duration,
    };

    fn worker_thread(keys: &Vec<String>, data: Arc<RwLock<HashMap<String, Mutex<i64>>>>) {
        for key in keys
        {
            // Assume that the element already exists
            let map = data.read().expect("RwLock poisoned");
            if let Some(element) = map.get(key) {
                let mut element = element.lock().expect("Mutex poisoned");

                *element += 1;
                thread::sleep(Duration::from_secs(1));
                continue;
            }

            drop(map);
            let mut map = data.write().expect("RwLock poisoned");

            // We use HashMap::entry to handle the case where another thread
            // inserted the same key while where were unlocked.
            thread::sleep(Duration::from_millis(50));
            map.entry(key.clone()).or_insert_with(|| Mutex::new(1));
            // println!("Inserted key: {}", key);
            // Let the loop start us over to try again
        }
    }
    #[allow(dead_code)]
    fn word_count(article: &str) -> HashMap<String, i64> {
        // let article = article.to_lowercase();
        let article = article.split_whitespace().collect::<Vec<&str>>();
        let str_1 = article[0..article.len() / 4]
            .iter().map(|x| x.to_string()).collect::<Vec<String>>();
        let str_2 = article[article.len() / 4..(article.len() / 4 * 2)]
            .iter().map(|x| x.to_string()).collect::<Vec<String>>();
        let str_3 = article[(article.len() / 4 * 2)..(article.len() / 4 * 3)]
            .iter().map(|x| x.to_string()).collect::<Vec<String>>();
        let str_4 = article[article.len() / 4 * 3..article.len()]
            .iter().map(|x| x.to_string()).collect::<Vec<String>>();
        let data = Arc::new(RwLock::new(HashMap::new()));
        let mut handles = vec![];

        let data1= Arc::clone(&data);
        let handle = thread::spawn(move || {
                worker_thread(&str_1, data1);
        });
        handles.push(Some(handle));
        let data2= Arc::clone(&data);
        let handle = thread::spawn(move || {
                worker_thread(&str_2, data2);
        });
        handles.push(Some(handle));
        let data3= Arc::clone(&data);
        let handle = thread::spawn(move || {
                worker_thread(&str_3, data3);
        });
        handles.push(Some(handle));
        let data4= Arc::clone(&data);
        let handle = thread::spawn(move || {
                worker_thread(&str_4, data4);
        });
        handles.push(Some(handle));
        handles.iter_mut().for_each(|handle| {
            if let Some(handle) = handle.take() {
                handle.join().unwrap();
            }
        });
        let mut map: HashMap<String, i64> = HashMap::new();
        data.read().unwrap().iter().for_each(|(k, v)| {map.insert(k.clone(), *v.lock().unwrap());});
        map
    }

    #[cfg(test)]
    #[test]
    fn test_parallel() {
        // let map = word_count("hello  1   2 3 4 world ham sam");

        use rand::Rng;
        use rand::distributions::Alphanumeric;

        //generate a random string with 1 million random words
        let mut origin_map = HashMap::<String, i64>::new();
        let mut article = String::new();
        for _ in 0..1000000 {
            let rng = rand::thread_rng();
            let word: String = rng.sample_iter(&Alphanumeric).filter(|u| u >= &65u8).map(|u| u as char).take(5).collect::<String>();
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
