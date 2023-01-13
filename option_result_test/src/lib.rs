extern crate core;

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn test_as_ref() {
        let x: Option<&str> = Some("Hello, world!");
        let y: Option<&str> = None;
        // First, cast `Option<String>` to `Option<&String>` with `as_ref`,
        // then unwrap it to get a `&str`.
        // if it fails to unwrap, it will return &"error"
        // if it succeeds, it should return &"Hello, world!"
        // use * to dereference the &str to a str
        // use match to check if it is "Hello, world!"
        match *x.as_ref().unwrap_or(&"error") {
            "Hello, world!" => println!("Some('Hello, world!') [as_ref] passed"),
            _ => panic!("Some('Hello, world!') [as_ref] failed"),
        }
        // First, cast `None` with `as_ref`
        // use match to check if it is None
        match y.as_ref() {
            None => println!("None [as_ref] passed"),
            _ => panic!("None [as_ref] failed"),
        }
        println!("test [as_ref] passed");
    }

    #[test]
    fn test_as_mut() {
        let mut x: Option<&str> = Some("Hello, world!");
        let mut y: Option<&str> = None;
        // First, cast `Option<str>` to `Option<&mut str>` with `as_mut`,
        // then unwrap it to get a `&mut str`.
        // if it fails to unwrap, it will return &mut "error"
        // if it succeeds, it should return &mut "Hello, world!"
        // use * to dereference the &mut str to a mut str
        // use match to check if it is "Hello, world!"
        match *x.as_mut().unwrap_or(&mut "error") {
            "Hello, world!" => println!("Some('Hello, world!') [as_mut] passed"),
            _ => panic!("Some('Hello, world!') [as_mut] failed"),
        }
        // First, cast `None` with `as_mut`
        // use match to check if it is None
        match y.as_mut() {
            None => println!("None [as_mut] passed"),
            _ => panic!("None [as_mut] failed"),
        }
        println!("test [as_mut] passed");
    }
    #[test]
    #[should_panic(expected = "None [expect] passed")]
    fn test_expect() {
        let x: Option<&str> = Some("Hello, world!");
        assert_eq!(x.expect("Some('Hello world!') [expect] failed"), "Hello, world!");
        let y: Option<&str> = None;
        y.expect("None [expect] passed");
        println!("test [expect] passed");
    }
    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn test_unwrap() {
        let x: Option<&str> = Some("Hello, world!");
        // use assert_eq! to check if it is "Hello, world!"
        assert_eq!(x.unwrap(), "Hello, world!");
        let y: Option<&str> = None;
        // use assert_eq! to check if it is None
        assert_eq!(y.unwrap(), "Hello, world!");
        println!("test [unwrap] passed");
    }
    #[test]
    fn test_unwrap_or_else() {
        let x: Option<&str> = Some("Hello, world!");
        // use assert_eq! to check if it is "Hello, world!"
        assert_eq!(x.unwrap_or_else(||"error"), "Hello, world!");
        let y: Option<&str> = None;
        // use assert_eq! to check if it is None
        assert_eq!(y.unwrap_or_else(||"error"), "error");
        println!("test [unwrap_or_else] passed");
    }
    #[test]
    fn test_ok_or() {
        let x: Option<&str> = Some("Hello, world!");
        // use assert_eq! to check if it is "Hello, world!"
        assert_eq!(x.ok_or("error"), Ok("Hello, world!"));
        let y: Option<&str> = None;
        // use assert_eq! to check if it is None
        assert_eq!(y.ok_or("error"), Err("error"));
        println!("test [ok_or] passed");
    }
    #[test]
    fn test_ok_or_else() {
        let x: Option<&str> = Some("Hello, world!");
        // use assert_eq! to check if it is "Hello, world!"
        assert_eq!(x.ok_or_else(||"error"), Ok("Hello, world!"));
        let y: Option<&str> = None;
        // use assert_eq! to check if it is None
        assert_eq!(y.ok_or_else(||"error"), Err("error"));
        println!("test [ok_or_else] passed");
    }
    #[test]
    fn test_transpose() {
        /*
        pub fn transpose(self) -> Result<Option<T>, E>
        Transposes an Option of a Result into a Result of an Option.
        None will be mapped to Ok(None). Some(Ok(_)) and Some(Err(_)) will be mapped to Ok(Some(_)) and Err(_).
        */
        let x: Option<Result<&str, &str>> = Some(Ok("Hello, world!"));
        // use assert_eq! to check if it is Ok(Some("Hello, world!"))
        assert_eq!(x.transpose(), Ok(Some("Hello, world!")));
        let y: Option<Result<&str, &str>> = Some(Err("error"));
        // use assert_eq! to check if it is Err("error")
        assert_eq!(y.transpose(), Err("error"));
        let z: Option<Result<&str, &str>> = None;
        // use assert_eq! to check if it is Ok(None)
        assert_eq!(z.transpose(), Ok(None));
        println!("test [transpose] passed");
    }
    #[test]
    fn test_filter() {
        let x: Option<&str> = Some("Hello, world!");
        // use assert_eq! to check if it is Some("Hello, world!")
        assert_eq!(x.filter(|&x| x == "Hello, world!"), Some("Hello, world!"));
        assert_eq!(x.filter(|&x| x == "error"), None);
        let y: Option<&str> = Some("Hello, world!");
        // use assert_eq! to check if it is None
        assert_eq!(y.filter(|&x| x == "error"), None);
        println!("test [filter] passed");
    }
    #[test]
    fn test_flatten() {
        let x: Option<Option<&str>> = Some(Some("Hello, world!"));
        // use assert_eq! to check if it is Some("Hello, world!")
        assert_eq!(x.flatten(), Some("Hello, world!"));
        let y: Option<Option<&str>> = Some(None);
        // use assert_eq! to check if it is None
        assert_eq!(y.flatten(), None);
        let z: Option<Option<&str>> = None;
        // use assert_eq! to check if it is None
        assert_eq!(z.flatten(), None);
        println!("test [flatten] passed");
    }
    #[test]
    fn test_map() {
        let x: Option<&str> = Some("Hello, world!");
        // use assert_eq! to check if it is Some("Hello, world!")
        assert_eq!(x.map(|x| x.len()), Some(13));
        let y: Option<&str> = None;
        // use assert_eq! to check if it is None
        assert_eq!(y.map(|x| x), None);
        println!("test [map] passed");
    }
    #[test]
    fn test_map_or() {
        let x: Option<&str> = Some("Hello, world!");
        // use assert_eq! to check if it is 13
        assert_eq!(x.map_or(0, |x| x.len()), 13);
        let y: Option<&str> = None;
        // use assert_eq! to check if it is 0
        assert_eq!(y.map_or(0, |x| x.len()), 0);
        println!("test [map_or] passed");
    }
    #[test]
    fn test_map_or_else() {
        let x: Option<&str> = Some("Hello, world!");
        // use assert_eq! to check if it is 13
        assert_eq!(x.map_or_else(||0, |x| x.len()), 13);
        let y: Option<&str> = None;
        // use assert_eq! to check if it is 0
        assert_eq!(y.map_or_else(||0, |x| x.len()), 0);
        println!("test [map_or_else] passed");
    }
    #[test]
    fn test_zip() {
        let x: Option<&str> = Some("Hello, world!");
        let y: Option<i32> = Some(1);
        let z: Option<&str> = None;
        // check if it is Some(("Hello, world!", 1))
        assert_eq!(x.zip(y), Some(("Hello, world!", 1)));
        // check if it is None
        assert_eq!(x.zip(z), None);
        // check if it is None
        assert_eq!(z.zip(z), None);
        println!("test [zip] passed");
    }
    #[test]
    fn test_xor() {
        let x: Option<&str> = Some("Hello, world!");
        let y: Option<&str> = None;
        // check if it is Some("Hello, world!")
        assert_eq!(x.xor(y), Some("Hello, world!"));
        // check if it is Some("Hello, world!")
        assert_eq!(y.xor(x), Some("Hello, world!"));
        // check if it is None
        assert_eq!(x.xor(x), None);
        // check if it is None
        assert_eq!(y.xor(y), None);
        println!("test [xor] passed");
    }
    #[test]
    fn test_and_then() {
        let x: Option<&str> = Some("Hello, world!");
        let y: Option<&str> = None;
        // check if it is Some("Hello, world!")
        assert_eq!(x.and_then(|x| Some(x)), Some("Hello, world!"));
        // check if it is None
        assert_eq!(y.and_then(|x|Some(x)), None);
        println!("test [and_then] passed");
    }
    #[test]
    fn test_or_else() {
        let x: Option<&str> = Some("Hello, world!");
        let y: Option<&str> = None;
        // check if it is Some("Hello, world!")
        assert_eq!(x.or_else(|| Some("error")), Some("Hello, world!"));
        // check if it is Some("error")
        assert_eq!(y.or_else(|| Some("error")), Some("error"));
        println!("test [or_else] passed");
    }
    #[test]
    fn test_insert() {
        let mut x: Option<&str> = Some("Hello, world!");
        let mut y: Option<&str> = None;
        // check if it is Some("Hello, world!")
        assert_eq!(x.insert("error"),  &"error");
        // check if it is None
        assert_eq!(y.insert("error"), &"error");
        // check if it is Some("error")
        assert_eq!(x, Some("error"));
        // check if it is Some("error")
        assert_eq!(y, Some("error"));
        println!("test [insert] passed");
    }
    #[test]
    fn test_get_or_insert() {
        let mut x: Option<&str> = Some("Hello, world!");
        let mut y: Option<&str> = None;
        // check if it is Some("Hello, world!")
        assert_eq!(x.get_or_insert("error"),  &"Hello, world!");
        // check if it is Some("error")
        assert_eq!(y.get_or_insert("error"), &"error");
        // check if it is Some("Hello, world!")
        assert_eq!(x, Some("Hello, world!"));
        // check if it is Some("error")
        assert_eq!(y, Some("error"));
        println!("test [get_or_insert] passed");
    }
    #[test]
    fn test_get_or_insert_with() {
        let mut x: Option<&str> = Some("Hello, world!");
        let mut y: Option<&str> = None;
        // check if it is Some("Hello, world!")
        assert_eq!(x.get_or_insert_with(|| "error"),  &"Hello, world!");
        // check if it is Some("error")
        assert_eq!(y.get_or_insert_with(|| "error"), &"error");
        // check if it is Some("Hello, world!")
        assert_eq!(x, Some("Hello, world!"));
        // check if it is Some("error")
        assert_eq!(y, Some("error"));
        println!("test [get_or_insert_with] passed");
    }
    #[test]
    fn test_take() {
        let mut x: Option<&str> = Some("Hello, world!");
        let mut y: Option<&str> = None;
        // check if it is Some("Hello, world!")
        assert_eq!(x.take(), Some("Hello, world!"));
        // check if it is None
        assert_eq!(y.take(), None);
        // check if it is None
        assert_eq!(x, None);
        // check if it is None
        assert_eq!(y, None);
        println!("test [take] passed");
    }
    #[test]
    fn test_replace() {
        let mut x: Option<&str> = Some("Hello, world!");
        let mut y: Option<&str> = None;
        // check if it is Some("Hello, world!")
        assert_eq!(x.replace("error"), Some("Hello, world!"));
        // check if it is Some("error")
        assert_eq!(y.replace("error"), None);
        // check if it is Some("error")
        assert_eq!(x, Some("error"));
        // check if it is Some("error")
        assert_eq!(y, Some("error"));
        println!("test [replace] passed");
    }
    #[test]
    fn test_into_iter() {
        let x: Option<&str> = Some("Hello, world!");
        let y: Option<&str> = None;
        let v: Vec<&str> = x.into_iter().collect();
        let w: Vec<&str> = y.into_iter().collect();
        // check if it is ["Hello, world!"]
        assert_eq!(v, ["Hello, world!"]);
        // check if it is empty
        assert!(w.is_empty());
        println!("test [into_iter] passed");
    }
    #[test]
    fn test_copied() {
        let s = "Hello, world!";
        let x: Option<&&str> = Some(&s);
        // check if it is "Hello, world!"
        assert_eq!(x.copied(), Some(s));
        // check if it is "Hello, world!"
        println!("test [copied] passed");
    }
    #[test]
    fn test_cloned() {
        let s = "Hello, world!".to_string();
        let x: Option<&String> = Some(&s);
        // check if it is "Hello, world!"
        assert_eq!(x.cloned(), Some(s));
        // check if it is "Hello, world!"
        println!("test [cloned] passed");
    }

}
