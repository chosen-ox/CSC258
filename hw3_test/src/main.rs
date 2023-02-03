use hash_map_markov_chain::Chain;
fn main() {
    let n = 1000000;
    let a: Vec<i32> = (1..2).cycle().take(n).collect();
    let b: Vec<i32> = (1i32..=100).cycle().take(n).collect();
    let c: Vec<i32> = (1i32..=10000).cycle().take(n).collect();
    let mut chain: Chain<i32> = Chain::new();
    chain.train(&a[..]);
    println!("chain: {:?}", chain.generate_from_seed(&1, 100));
    let mut chain: Chain<i32> = Chain::new();
    chain.train(&b[..]);
    println!("chain: {:?}", chain.generate_from_seed(&1, 101));
    let mut chain: Chain<i32> = Chain::new();
    chain.train(&c[..]);
    println!("chain: {:?}", chain.generate_from_seed(&1, 101));

}