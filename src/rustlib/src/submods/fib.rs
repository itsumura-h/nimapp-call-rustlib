fn fib(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 2) + fib(n - 1),
    }
}

#[no_mangle]
pub extern "C" fn fib_array(n: u64) -> *mut Vec<u64> {
    let mut vector = Vec::with_capacity(n.try_into().unwrap());
    for i in 0..n {
        vector.push(fib(i));
    }
    Box::into_raw(Box::new(vector))
}

#[no_mangle]
pub extern "C" fn get_vector_len(v: &Vec<u64>) -> usize {
    v.len()
}

#[no_mangle]
pub extern "C" fn get_vector_item(v: &Vec<u64>, offset: usize) -> u64 {
    v[offset]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib_test() {
        assert_eq!(fib(1), 1);
        assert_eq!(fib(2), 1);
        assert_eq!(fib(3), 2);
        assert_eq!(fib(4), 3);
        assert_eq!(fib(5), 5);
        assert_eq!(fib(6), 8);
        assert_eq!(fib(7), 13);
        assert_eq!(fib(8), 21);
        assert_eq!(fib(9), 34);
        assert_eq!(fib(10), 55);
    }
}
