mod events;
#[allow(unused_imports)] 
// the import is used? and when I remove this line it errors
// so I have no idea why there is a warning
use events::dot_remover::find_if_dot;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_dot_normal() {
        assert_eq!(find_if_dot("text"), None);
    }

    #[test]
    fn ignore_codeblocks() {
        assert_eq!(find_if_dot("```\ntext\n```"), None);
    }

    #[test]
    fn ignore_three_dots() {
        assert_eq!(find_if_dot("text..."), None);
    }

    #[test]
    fn ignore_multi_sentences() {
        assert_eq!(find_if_dot("text. more text."), None);
    }

    #[test]
    fn dot_normal() {
        assert_eq!(find_if_dot("text."), Some(String::from("text")));
    }

    #[test]
    fn two_dots_normal() {
        // This could be used to trick the algorithm in removing one dot
        // but leaving the second one in
        assert_eq!(find_if_dot("text.."), Some(String::from("text")))
    }

    #[test]
    fn dot_single_line_code() {
        assert_eq!(find_if_dot("`text.`"), Some(String::from("`text`")));
    }

    #[test]
    fn dot_surrounded() {
        assert_eq!(find_if_dot("text`.`"), Some(String::from("text")));
    }

    #[test]
    fn unicode_dot() {
        // there are probably too many to get all cases but there was
        // an attempt
        assert_eq!(find_if_dot("text⸱"), Some(String::from("text")));
    }

    #[test]
    fn special_unicode_dot() {
        // A multiple byte unicode dot can be used
        // to cause an error in the algorithm
        // if string indexing is used
        assert_eq!(find_if_dot("text․"), Some(String::from("text")))
    }

    #[test]
    fn invisible_char_after_dot() {
        // there are probably too many to get all cases but there was
        // an attempt
        assert_eq!(find_if_dot(&format!("text.{}", '\u{2000}')), Some(String::from("text")));
    }

    #[test]
    fn markdown_invisible_after_dot() {
        // Discord renders some markdown invisible, like _ _
        assert_eq!(find_if_dot("text._ _"), Some(String::from("text")));
    }
}