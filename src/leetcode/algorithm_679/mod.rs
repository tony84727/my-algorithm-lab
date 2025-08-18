pub struct Solution;

#[derive(Clone, PartialEq, Eq, Debug)]
enum Operator {
    Add = 0,
    Sub = 1,
    Div = 2,
    Mul = 3,
}

#[derive(Debug)]
struct Invalid;

impl TryFrom<i32> for Operator {
    type Error = Invalid;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Add),
            1 => Ok(Self::Sub),
            2 => Ok(Self::Div),
            3 => Ok(Self::Mul),
            _ => Err(Invalid),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
enum Element {
    Operator(Operator),
    Number(f32),
}

impl Solution {
    pub fn judge_point24(cards: Vec<i32>) -> bool {
        let mut elements = vec![];
        for c in cards.into_iter() {
            elements.push(Element::Number(c as f32));
        }
        for index in 0..64 {
            let mut combination = elements.clone();
            for o in Self::to_operators(index) {
                combination.push(Element::Operator(o));
            }
            for p in Self::premutation(combination.len(), &mut combination) {
                if Self::evaluate(&p) {
                    return true;
                }
            }
        }
        false
    }

    fn premutation<T>(k: usize, elements: &mut [T]) -> Vec<Vec<T>>
    where
        T: Clone,
    {
        let mut premutations = vec![];
        if k == 1 {
            return vec![elements.to_vec()];
        }
        for i in 0..k {
            let mut p = Self::premutation(k - 1, elements);
            premutations.append(&mut p);
            if k % 2 == 0 {
                elements.swap(i, k - 1);
            } else {
                elements.swap(0, k - 1);
            }
        }
        premutations
    }

    fn to_operators(mut i: i32) -> Vec<Operator> {
        let mut operator = vec![];
        for _ in 0..3 {
            operator.push(Operator::try_from(i % 4).unwrap());
            i /= 4;
        }
        operator
    }

    fn evaluate(elements: &[Element]) -> bool {
        let mut stack = vec![];
        for e in elements.iter() {
            match e {
                a @ Element::Number(_x) => stack.push(a.clone()),
                Element::Operator(Operator::Add) => {
                    let Some(Element::Number(a)) = stack.pop() else {
                        return false;
                    };
                    let Some(Element::Number(b)) = stack.pop() else {
                        return false;
                    };
                    stack.push(Element::Number(a + b));
                }
                Element::Operator(Operator::Sub) => {
                    let Some(Element::Number(a)) = stack.pop() else {
                        return false;
                    };
                    let Some(Element::Number(b)) = stack.pop() else {
                        return false;
                    };
                    stack.push(Element::Number(b - a));
                }
                Element::Operator(Operator::Mul) => {
                    let Some(Element::Number(a)) = stack.pop() else {
                        return false;
                    };
                    let Some(Element::Number(b)) = stack.pop() else {
                        return false;
                    };
                    stack.push(Element::Number(a * b));
                }
                Element::Operator(Operator::Div) => {
                    let Some(Element::Number(a)) = stack.pop() else {
                        return false;
                    };
                    let Some(Element::Number(b)) = stack.pop() else {
                        return false;
                    };
                    if a == 0.0 {
                        return false;
                    }
                    stack.push(Element::Number(b / a));
                }
            }
        }
        stack
            .first()
            .map(|n| match n {
                Element::Number(n) => (n * 100.0).round() / 100.0 == 24.0,
                _ => false,
            })
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![4,1,8,7] => true; "example 1")]
    #[test_case(vec![1,2,1,2] => false; "example 2")]
    #[test_case(vec![8,1,6,6] => true; "case 1")]
    #[test_case(vec![3,3,8,8] => true; "case 2")]
    #[test_case(vec![1,1,7,7] => false; "case 3")]
    fn test_solution(cards: Vec<i32>) -> bool {
        Solution::judge_point24(cards)
    }

    #[test]
    fn test_evaluate_1() {
        assert!(Solution::evaluate(&[
            Element::Number(6.0),
            Element::Number(1.0),
            Element::Number(6.0),
            Element::Number(8.0),
            Element::Operator(Operator::Div),
            Element::Operator(Operator::Sub),
            Element::Operator(Operator::Div),
        ]))
    }

    #[test]
    fn test_evaluate_2() {
        assert!(Solution::evaluate(&[
            Element::Number(8.0),
            Element::Number(3.0),
            Element::Number(8.0),
            Element::Number(3.0),
            Element::Operator(Operator::Div),
            Element::Operator(Operator::Sub),
            Element::Operator(Operator::Div),
        ]))
    }
}
