use std::ffi::c_char;

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
pub extern "C" fn get_fib_len(v: &mut Vec<u64>) -> usize {
    v.len()
}

#[no_mangle]
pub extern "C" fn get_fib_item(v: &mut Vec<u64>, offset: usize) -> u64 {
    v[offset]
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib_test() {
        let expects = [0, 1, 1, 2, 3, 5];
        for i in 0..5 {
            println!("{}", fib(i));
            assert_eq!(fib(i), expects[i as usize]);
        }

        let fib_array = fib_array(10);
        for i in 0..get_fib_len(&fib_array) - 1 {
            println!("{}", get_fib_item(&fib_array, i));
            assert_eq!(get_fib_item(&fib_array, i), fib(i as u64))
        }
    }
}
