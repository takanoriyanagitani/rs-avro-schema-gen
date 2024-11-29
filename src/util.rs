#[macro_export]
macro_rules! bind {
    ($io: expr, $f: expr) => {
        move || {
            let t = $io()?;
            $f(t)()
        }
    };
}

#[macro_export]
macro_rules! lift {
    ($pure: expr) => {
        move |t| move || $pure(t)
    };
}
