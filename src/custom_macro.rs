/// Allowing define nested vector
/// # Examples
///
/// ```rust
/// use my_algorithm_lab::vecvec;
/// let mut expected = vec![];
/// expected.push(vec![1,2,3]);
/// expected.push(vec![4,5,6]);
/// assert_eq!(expected, vecvec![[1,2,3], [4,5,6]]);
/// ```
#[macro_export]
macro_rules! vecvec {
    ([$($element: expr),*]) => {
        {
            vec![$(vecvec!($element)),*]
        }
    };
    ($element: expr) => {
        $element
    };
    [ $($element:tt),* ] => {
        {
            vec![$(vecvec!($element)),*]
        }
    };
}
