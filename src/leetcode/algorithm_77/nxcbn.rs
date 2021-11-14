pub struct Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
		fn sub_20(n: i32, m: i32, ic: &mut Vec<i32>, j1: i32) {
			if m % 2 == 1 {
				// goto 90;
				return;
			}
			if ic[n as usize] == 1 {
				// goto 30
				unimplemented!();
				return;
			}
			let k1 = n-j1;
			let k2 = k1 + 1;
			//goto 130
			return;
		}

		fn sub_30(n: i32, m:i32, ic: &mut Vec<i32>) {
			if j1 % 2 == 1 {
				// goto 40
				return;
			}
			// goto 120
			return;
		}

		fn sub_40(n: i32, m: i32, ic: &mut Vec<i32>, j1: i32) {
			let jp = n - j1 - 1;
			for i in 1..=jp {
				let i1 = jp + 2 - i;
				if ic[i1 as usize] == 0 {
					// goto 50
					return;
				}
			}
		}
		
		fn sub_90(n: i32, m: i32, ic: &mut Vec<i32>, j1: i32)
		{
			if ic[n as usize] == 1 {
				// goto 110
				return;
			}
			let k2 = n - j1 - 1;
			if k2 == 0 {
				// goto 60
				return;
			}
			if ic[(k2+1) as usize] == 1 and ic[k2 as usize] == 1 {
				//goto 100
				return;
			}
			let k1 = k2 + 1;
			// goto 130
			return;
		}

		fn sub_110(n: i32, m: i32, ic: &mut Vec<i32>, j1: i32) {
			if j1 % 2 == 1 {
				//goto 120
				return;
			}
			// goto 40;
			return;
		}
		
        fn nxcbn(n: i32, m: i32, ic: &mut Vec<i32>) {
            if m >= n {
                return;
            }
            let n1 = n - 1;
            for j in 1..=n1 {
                let nj = n - j;
            }
        }
    }
}
