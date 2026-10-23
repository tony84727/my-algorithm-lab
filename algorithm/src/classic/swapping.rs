#[allow(clippy::manual_swap)]
pub fn swap_variables_naively<'a>(a: &'a mut i32, b: &'a mut i32) {
    let temp = *a;
    *a = *b;
    *b = temp;
}
#[allow(clippy::manual_swap)]
pub fn swap_variables_xor(a: &mut i32, b: &mut i32) {
    *a ^= *b;
    *b ^= *a;
    *a ^= *b;
}

pub fn swap_by_std_mem_swap(a: &mut i32, b: &mut i32) {
    std::mem::swap(a, b);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_variables_naively() {
        let mut a = 1;
        let mut b = 2;
        swap_variables_naively(&mut a, &mut b);
        assert_eq!(2, a);
        assert_eq!(1, b);
    }

    #[test]
    fn test_swap_variables_xor() {
        let mut a = 1;
        let mut b = 2;
        swap_variables_xor(&mut a, &mut b);
        assert_eq!(2, a);
        assert_eq!(1, b);
    }
}
