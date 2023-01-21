#[cfg(test)]
mod test_vec {

    #[test]
    fn test_any() {
        let v = vec![1, 2, 3];

        assert!(v.iter()
            .map(|x| x + 3)
            .any(|x| x > 5));
        assert!(v.iter()
            .flat_map(|x| [x + 5, x + 6])
            .any(|x| x == 9));
        assert!(v.iter()
            .filter(|x| x > &&2)
            .any(|x| x == &3));
    }

    #[test]
    fn test_try_fold() {
        let v = vec![1, 2, 3];
        let res = v.iter()
            .map(|x| x + 3)
            .try_fold(0, |acc, x| {
                if x > 5 {
                    Err(x)
                } else {
                    Ok(acc + x)
                }
            });
        assert_eq!(res, Err(6));
        let res2 = v.iter()
            .flat_map(|x| [x + 5, x + 6])
            .try_fold(0, |acc, x| {
                if x == 10 {
                    Err(x)
                } else {
                    Ok(acc + x + 1)
                }
            });
        assert_eq!(res2, Ok(51));

        let res3 = v.iter()
            .filter(|x| x > &&2)
            .try_fold(0, |acc, x| {
                if x == &3 {
                    Err(x)
                } else {
                    Ok(acc + x + 1)
                }
            });
        assert_eq!(res3, Err(&3));
    }

    #[test]
    fn test_chain() {
        let v = vec![1, 2, 3];
        let res = v.iter()
            .map(|x| x + 3)
            .chain(v.iter().map(|x| x + 5))
            .collect::<Vec<_>>();
        assert_eq!(res, vec![4, 5, 6, 6, 7, 8]);

        let res2 = v.iter()
            .flat_map(|x| [x + 5, x + 6])
            .chain(v.iter().flat_map(|x| [x + 7, x + 8]))
            .collect::<Vec<_>>();
        assert_eq!(res2, vec![6, 7, 7, 8, 8, 9, 8, 9, 9, 10, 10, 11]);

        let res3 = v.iter()
            .filter(|x| x > &&2)
            .chain(v.iter().filter(|x| x > &&1))
            .collect::<Vec<_>>();
        assert_eq!(res3, vec![&3, &2, &3]);
    }

    #[test]
    fn test_partition() {
        let v = vec![1, 2, 3];
        let (res, res2): (Vec<_>, Vec<_>) = v.iter()
            .filter_map(|x| {
                if x < &3
                { Some(x + 3) } else { None }
            })
            .partition(|x| x > &4);
        assert_eq!(res, vec![5]);
        assert_eq!(res2, vec![4]);

        let (res3, res4): (Vec<_>, Vec<_>) = v.iter()
            .flat_map(|x| [x + 5, x + 6])
            .partition(|x| x > &7);
        assert_eq!(res3.iter().fold(0, |acc, x| acc + x), 25);
        assert_eq!(res4.iter().fold(0, |acc, x| acc + x), 20);

        let (res5, res6): (Vec<_>, Vec<_>) = v.iter()
            .map(|x| x + 3)
            .partition(|x| x > &4);
        assert_eq!(res5.iter().find(|x| x == &&5), Some(&5));
        assert_eq!(res6.iter().find(|x| x == &&4), Some(&4));
    }

    #[test]
    fn test_for_each() {
        let v = vec![1, 2, 3];
        let mut res = vec![];
        v.iter()
            .filter_map(|x| {
                if x < &3
                { Some(x + 3) } else { None }
            })
            .for_each(|x| res.push(x));
        assert_eq!(res.iter().find(|x| x == && 6), None);

        let mut res2 = vec![];
        v.iter()
            .flat_map(|x| [x + 5, x + 6])
            .for_each(|x| res2.push(x));
        assert_eq!(res2.iter().fold(0 ,|acc, x| acc + x), 45);

        let mut res3 = vec![];
        v.iter()
            .filter_map(|x| {
                if x < &2
                { Some(x * 2) } else { None }
            })
            .for_each(|x| res3.push(x));
        assert_eq!(res3, vec![2]);
    }
}

#[cfg(test)]
mod test_range {}