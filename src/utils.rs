macro_rules! new_string {
    ($str:literal) => {
        String::from($str)
    };
}

pub(crate) use new_string;
