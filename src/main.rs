mod container_with_most_water;
mod chat_app;

fn gcd(mut n: u64, mut m:u64) -> u64 {
    assert!(n!=0 && m!=0);
    while m!=0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}


fn basic_main() {
    let answer = gcd(10, 20);
    println!("gcd is: {}", answer);
    let list: Vec<usize> = vec![42, 16, 98, 12, 43, 12, 54];
    println!("The most water that is contained is {} for the list {:?}",
             container_with_most_water::container_with_most_water_fn(&list), list)
}

fn main() {
    chat_app::start();
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
}