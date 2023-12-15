pub fn multiply_string(s: &str, n: u32) -> String {
    if n == 0 {
        return String::new();
    }
    
    let mut owned_str = s.to_owned();
    let mut i = 1;

    while i < n {
        owned_str += s;
        i += 1;
    }

    owned_str
}