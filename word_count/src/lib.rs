extern crate core;

use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};


pub mod sequential_word_count {
    use std::collections::HashMap;

    #[allow(dead_code)]
    pub fn word_count(article: &str) -> HashMap<String, i64> {
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
            let word: String = rng.sample_iter(&Alphanumeric).filter(|u| u >= &65u8)
                .map(|u| u as char)
                .take(5).collect::<String>();
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


pub mod parallel_word_count {
    use std::{
        collections::HashMap,
        sync::{Arc, Mutex, RwLock},
        thread,
    };

    pub fn handle_word(keys: &Vec<String>, data: Arc<RwLock<HashMap<String, Mutex<i64>>>>) {
        for key in keys
        {
            // Assume that the element already exists
            // read lock
            let map = data.read().expect("RwLock poisoned");
            if let Some(element) = map.get(key) {
                let mut element = element.lock().expect("Mutex poisoned");
                *element += 1;
                continue;
            }
            drop(map);

            // write lock
            let mut map = data.write().expect("RwLock poisoned");
            if let Some(element) = map.get(key) {
                let mut element = element.lock().expect("Mutex poisoned");
                *element += 1;
            } else {
                map.entry(key.clone()).or_insert_with(|| Mutex::new(1));
            }
        }
    }

    #[allow(dead_code)]
    pub fn word_count(article: &str, thread_num: usize) -> HashMap<String, i64> {
        // let article = article.to_lowercase();
        let article = article.split_whitespace().collect::<Vec<&str>>();
        let data = Arc::new(RwLock::new(HashMap::new()));
        let mut handles = vec![];
        let len = article.len() / thread_num;

        for i in 0..thread_num - 1 {
            let word = article[i * len..(i + 1) * len]
                .iter().map(|x| x.to_string()).collect::<Vec<String>>();
            let data_clone = Arc::clone(&data);
            let handle = thread::spawn(move || {
                handle_word(&word, data_clone);
            });
            handles.push(Some(handle));
        }
        let word = article[(thread_num - 1) * len..article.len()]
            .iter().map(|x| x.to_string()).collect::<Vec<String>>();
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            handle_word(&word, data_clone);
        });
        handles.push(Some(handle));


        handles.iter_mut().for_each(|handle| {
            if let Some(handle) = handle.take() {
                handle.join().unwrap();
            }
        });

        let mut map: HashMap<String, i64> = HashMap::new();
        data.read().unwrap().iter().for_each(|(k, v)| { map.insert(k.clone(), *v.lock().unwrap()); });
        map
    }

    #[cfg(test)]
    #[test]
    fn test_parallel() {
        use rand::Rng;
        use rand::distributions::Alphanumeric;

        //generate a random string with 1 million random words
        let mut origin_map = HashMap::<String, i64>::new();
        let mut article = String::new();
        for _ in 0..1000000 {
            let rng = rand::thread_rng();
            let word: String = rng.sample_iter(&Alphanumeric).filter(|u| u >= &65u8)
                .map(|u| u as char)
                .take(5).collect::<String>();
            origin_map.entry(word.clone())
                .and_modify(|e| *e += 1)
                .or_insert(1);
            article.push_str(&word);
            article.push(' ');
        }
        let map = word_count(&article, 4);
        // println!("{:?}", map);
        // println!("{:?}", origin_map);
        assert!(map.eq(&origin_map));
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize,
           receiver: Arc<Mutex<mpsc::Receiver<Job>>>,
    ) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");

                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

mod thread_pool_word_count {
    use crate::ThreadPool;
    use crate::parallel_word_count::handle_word;
    use std::sync::{Arc, RwLock};
    use std::collections::HashMap;


    #[allow(dead_code)]
    fn word_count(article: &str) -> HashMap<String, i64> {
        let data = Arc::new(RwLock::new(HashMap::new()));
        {
            let pool = ThreadPool::new(4);
            let article = article.split_whitespace().collect::<Vec<&str>>();
            let len = article.len() / 40;
            for i in 0..39 {
                let word = article[i * len..(i + 1) * len]
                    .iter().map(|x| x.to_string()).collect::<Vec<String>>();
                let data_clone = Arc::clone(&data);
                pool.execute(move || { handle_word(&word, data_clone); });
            }
            let word = article[39 * len..article.len()]
                .iter().map(|x| x.to_string()).collect::<Vec<String>>();
            let data_clone = Arc::clone(&data);
            pool.execute(move || { handle_word(&word, data_clone); });
        }

        let mut map: HashMap<String, i64> = HashMap::new();
        data.read().unwrap().iter().for_each(|(k, v)| { map.insert(k.clone(), *v.lock().unwrap()); });
        map
    }

    #[cfg(test)]
    #[test]
    fn test_thread_pool() {
        use rand::Rng;
        use rand::distributions::Alphanumeric;

        //generate a random string with 1 million random words
        let mut origin_map = HashMap::<String, i64>::new();
        let mut article = String::new();
        for _ in 0..1000000 {
            let rng = rand::thread_rng();
            let word: String = rng.sample_iter(&Alphanumeric).filter(|u| u >= &65u8)
                .map(|u| u as char)
                .take(5).collect::<String>();
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

use std::collections::HashMap;
#[allow(dead_code)]
fn timed<F : Fn(&str) ->HashMap<String, i64>, S : AsRef<str>>(name: S, f : F, input: &str) {
    // let mut count = 0;
    let mut time = 0;
    for _ in 0..100 {
        let start = std::time::Instant::now();
        f(std::hint::black_box(&input));
        let end = std::time::Instant::now();
        time += end.duration_since(start).as_nanos();
        // count += input.len();
    }
    // eprintln!("{} speed: {} tokens/sec", name.as_ref(), (count as f64) * 1.0e9 / (time as f64))
    eprintln!("{} time {} second", name.as_ref(), (time as f64) / 1.0e10)
}
#[test]
fn benchmark() {
    use crate::parallel_word_count::word_count as parallel_word_count;
    use crate::sequential_word_count::word_count as sequential_word_count;


    let s = std::fs::read_to_string("small").unwrap();
    let m = std::fs::read_to_string("medium").unwrap();
    let l = std::fs::read_to_string("large").unwrap();

    timed("s-small", |x| {sequential_word_count(x)}, &s );
    timed("p-small", |x| {parallel_word_count(x, 1)}, &s );
    timed("p-small", |x| {parallel_word_count(x, 2)}, &s );
    timed("p-small", |x| {parallel_word_count(x, 4)}, &s );
    timed("p-small", |x| {parallel_word_count(x, 8)}, &s );

    timed("s-medium", |x| {sequential_word_count(x)}, &m );
    timed("p-medium", |x| {parallel_word_count(x, 1)}, &m );
    timed("p-medium", |x| {parallel_word_count(x, 2)}, &m );
    timed("p-medium", |x| {parallel_word_count(x, 4)}, &m );
    timed("p-medium", |x| {parallel_word_count(x, 8)}, &m );

    timed("s-large", |x| {sequential_word_count(x)}, &l );
    timed("p-large", |x| {parallel_word_count(x, 1)}, &l );
    timed("p-large", |x| {parallel_word_count(x, 2)}, &l );
    timed("p-large", |x| {parallel_word_count(x, 4)}, &l );
    timed("p-large", |x| {parallel_word_count(x, 8)}, &l );

}