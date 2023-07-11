
#[macro_export]
macro_rules! arr_to_vec_2 {
    ([
        $(
            [$($e:expr),*]
        ),*
    ]) => {
        vec![$(vec![$($e),*]),*]
    };
}


#[macro_export]
macro_rules! arrstr_to_vecstring {
    ([$($e:expr),*]) => {
        vec![$($e.to_string()),*]
    };
}


#[cfg(test)]
mod tests {

}

