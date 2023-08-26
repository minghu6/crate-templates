//!

macro_rules! arr_to_vec_2 {
    ([
        $(
            [$($e:expr),*]
        ),*
    ]) => {
        vec![$(vec![$($e),*]),*]
    };
}

macro_rules! arrstr_to_vecstring {
    ([$($e:expr),*]) => {
        vec![$($e.to_string()),*]
    };
}

macro_rules! set {
    ($e:expr) => {
        std::collections::BTreeSet::<Vec<i32>>::from_iter($e)
    };
}

macro_rules! min {
    ($($val:expr),+) => {
        {
            [$($val),+].into_iter().min().unwrap()
        }
    }
}

macro_rules! max {
    ($($val:expr),+) => {
        {
            [$($val),+].into_iter().max().unwrap()
        }
    }
}

macro_rules s {
    ($lit:literal) => {
        $lit.to_string()
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn test() {

    }
}
