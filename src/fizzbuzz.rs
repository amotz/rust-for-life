use std::borrow::Cow;

pub fn fizzbuzz(n: i32) -> Cow<'static, str> {
    match (n % 3, n % 5) {
        (0, 0) => "fizzbuzz".into(),
        (_, 0) => "buzz".into(),
        (0, _) => "fizz".into(),
        (_, _) => n.to_string().into()
    }
}