pub trait StrUtils {
    fn split_first_whitespace(&self) -> (&str, &str);

    fn is_only_whitespace(&self) -> bool;
}

impl StrUtils for str {
    // Split the string at the first whitespace
    // ex : split_first_whitespace("\b I'm a bold string") -> ("\b", "I'm a bold string")
    fn split_first_whitespace(&self) -> (&str, &str) {
        let mut first_whitespace_index = 0;
        let mut it = self.chars();
        while let Some(c) = it.next() {
            if c.is_whitespace() {
                break;
            } else {
                first_whitespace_index += 1;
            }
        }
        if first_whitespace_index > 0 && first_whitespace_index != self.len() {
            (&self[0..first_whitespace_index], &self[first_whitespace_index + 1..])
        } else {
            (self, "")
        }
    }

    fn is_only_whitespace(&self) -> bool {
        // TODO
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_split_first_whitespace() {
        let text = r"\b I'm a bold string";
        let split = text.split_first_whitespace();
        assert_eq!(split, (r"\b", r"I'm a bold string"));
    }
}
