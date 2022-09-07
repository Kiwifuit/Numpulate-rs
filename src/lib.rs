#[link(name = "numpulate", kind = "static")]
extern "C" {
    fn negate(i: isize) -> isize;
    fn reverse(i: isize) -> isize;
}

#[cfg(test)]
mod tests {
    use super::*;
}
