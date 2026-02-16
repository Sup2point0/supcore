/// Join 2 `char`s into a single `String`.
pub fn join(left: char, right: char) -> String
{
    format!("{left}{right}")
}

/// Append `c` onto `acc`.
pub fn joins(mut acc: String, c: char) -> String
{
    acc.push(c);
    acc
}

/// Prepend `c` onto `cs`.
pub fn cons(c: char, mut str: Vec<char>) -> String
{
    str.insert(0, c);
    str.into_iter().collect()
}
