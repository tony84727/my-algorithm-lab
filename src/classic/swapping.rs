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
    use test::Bencher;

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

    #[bench]
    fn bench_swap_variables_naively_large(b: &mut Bencher) {
        let mut x = i32::MAX;
        let mut y = i32::MAX - 1;
        b.iter(move || {
            for _ in 0..10000000 {
                swap_variables_naively(&mut x, &mut y);
            }
        })
    }

    #[bench]
    fn bench_swap_variables_xor_large(b: &mut Bencher) {
        let mut x = i32::MAX;
        let mut y = i32::MAX - 1;
        b.iter(move || {
            for _ in 0..10000000 {
                swap_variables_xor(&mut x, &mut y);
            }
        })
    }

    #[bench]
    fn bench_swap_by_std_mem_swap_large(b: &mut Bencher) {
        let mut x = i32::MAX;
        let mut y = i32::MAX - 1;
        b.iter(move || {
            for _ in 0..10000000 {
                swap_by_std_mem_swap(&mut x, &mut y);
            }
        })
    }

    #[bench]
    fn bench_swap_variables_naively_small(b: &mut Bencher) {
        let mut x = 1;
        let mut y = 0;
        b.iter(move || {
            for _ in 0..10000000 {
                swap_variables_xor(&mut x, &mut y);
            }
        })
    }

    #[bench]
    fn bench_swap_variables_xor_small(b: &mut Bencher) {
        let mut x = 1;
        let mut y = 0;
        b.iter(move || {
            for _ in 0..10000000 {
                swap_variables_xor(&mut x, &mut y);
            }
        })
    }

    #[bench]
    fn bench_swap_by_std_mem_swap_small(b: &mut Bencher) {
        let mut x = 1;
        let mut y = 0;
        b.iter(move || {
            for _ in 0..10000000 {
                swap_by_std_mem_swap(&mut x, &mut y);
            }
        })
    }
}
