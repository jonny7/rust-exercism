pub fn nprime(n: usize) -> Result<i64, &'static str>{

    if n < 1 {return Result::Err("Value must be larger than 2")}
    let mut primes = 0;
    let mut i = 1;

    while primes < n {
        i+= 1;
        if is_prime(i) {primes += 1};
    }
    Result::Ok(i)
}

pub fn is_prime(n: i64) -> bool {

    if n == 1 {return false};
    if n == 2 {return true};

    for i in 2..(n as f64).sqrt() as i64+1 {
        if n % i == 0 {return false};
    }

    true
}