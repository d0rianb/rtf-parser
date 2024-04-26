pub trait StrUtils {
    fn split_first_whitespace(&self) -> (&str, &str);

    fn is_only_whitespace(&self) -> bool;
}

impl StrUtils for str {
    // Split the string at the first whitespace
    // ex : split_first_whitespace("\b I'm a bold string") -> ("\b", "I'm a bold string")
    fn split_first_whitespace(&self) -> (&str, &str) {
        let mut first_whitespace_index = 0;

        let len = self.len();
        let bytes = self.as_bytes();
        let mut i = 0;
        // Faster than an iterator
        while i < len {
            let c = bytes[i] as char;
            i += 1;

            if c.is_whitespace() {
                break;
            } else {
                first_whitespace_index += 1;
            }
        }
        if first_whitespace_index > 0 && first_whitespace_index != self.len() {
            return (&self[0..first_whitespace_index], &self[first_whitespace_index + 1..]);
        } else {
            return (self, "");
        }
    }

    fn is_only_whitespace(&self) -> bool {
        // TODO
        false
    }
}

// Macros
// Specify the path to the test files
#[macro_export]
macro_rules! include_test_file {
    ($filename:expr) => {
        include_str!(concat!("../resources/tests/", $filename))
    };
}

// Recursive call to the tokenize method of the lexer
#[macro_export]
macro_rules! recursive_tokenize {
    ($tail:expr) => {
        Lexer::tokenize($tail)
    };
    ($tail:expr, $ret:expr) => {
        if $tail.len() > 0 {
            if let Ok(tail_tokens) = Lexer::tokenize($tail) {
                // Push all the tokens in the result vector
                for token in tail_tokens {
                    $ret.push(token);
                }
            }
        }
    };
}

#[macro_export]
macro_rules! recursive_tokenize_with_init {
    ($init:expr, $tail:expr) => {{
        let mut ret = vec![$init];
        recursive_tokenize!($tail, ret);
        return Ok(ret);
    }};
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
