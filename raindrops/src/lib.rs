pub fn raindrops(n: usize) -> String {

    // make a new string to concatenate to
    let mut r = String::new();

    if n % 3 == 0 { r += "Pling" }
    if n % 5 == 0 { r += "Plang" }
    if n % 7 == 0 { r += "Plong" }

    // convert n to string
    if r.is_empty() {
        r += &n.to_string()
    }
    // r is returned
    r
}
