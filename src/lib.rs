#[link(name = "numpulate", kind = "static")]
extern "C" {
    fn negate(i: isize) -> isize;
    fn reverse(i: isize) -> isize;
}

pub fn negate_number(i: isize) -> isize {
    unsafe { negate(i) }
}

pub fn reverse_number(i: isize) -> isize {
    unsafe { reverse(i) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_negated() {
        let pos = 32;
        let neg = -30;

        assert_eq!(negate_number(pos), -32);
        assert_eq!(negate_number(neg), 30);
    }

    #[test]
    fn num_reversed() {
        let rev = 931;

        assert_eq!(reverse_number(rev), 139);
    }
}
