#[allow(dead_code)]

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
            let mut v = vec![];
            $(v.push(vecvec!($element));)*
            v
        }
    };
    ($element: expr) => {
        $element
    };
    [ $($element:tt),* ] => {
        {
            let mut v = vec![];
            $(v.push(vecvec!($element));)*
            v
        }
    };
}

// https://stackoverflow.com/questions/26731243/how-do-i-use-a-macro-across-module-files
pub(crate) use vecvec;
