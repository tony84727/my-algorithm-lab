#[allow(dead_code)]
#[allow(unused_macros)]
/// Allowing define nested vector
/// # Examples
///
/// ```rust
/// let mut expected = vec![];
/// expected.push(vec![1,2,3]);
/// expected.push(vec![4,5,6]);
/// assert_eq!(expected, vecvec![[1,2,3], [4,5,6]]);
/// ```
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

#[allow(unused_imports)]
// https://stackoverflow.com/questions/26731243/how-do-i-use-a-macro-across-module-files
pub(crate) use vecvec;
