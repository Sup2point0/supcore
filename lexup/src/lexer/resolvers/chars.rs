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
