use std::cmp::max;

#[allow(dead_code)]
enum TextAlign {
    Left,
    Center,
    Right,
    Justify,
}

#[allow(dead_code)]
fn text_align(text: &str, line_length: usize, align: TextAlign) -> String {
    match align {
        TextAlign::Left => {
            let l = max(text.len(), line_length);
            let mut t = text.trim_start().to_string();
            let spaces = String::from(' ').repeat(l - t.len());
            t.push_str(&spaces);
            t
        }
        TextAlign::Center => format!("{:^line_length$}", text.trim()).to_string(),
        TextAlign::Right => {
            let l = max(text.len(), line_length);
            let mut t = text.trim_end().to_string();
            let spaces = String::from(' ').repeat(l - t.len());
            t.insert_str(0, &spaces);
            t
        }
        TextAlign::Justify => {
            let num_characters = text.chars().filter(|c| !c.is_whitespace()).count();
            let missing_whitespaces = max(line_length, text.len()) - num_characters;
            let words_cnt = text.split_whitespace().count();
            if words_cnt < 2 {
                // no need to process neither empty string nor one word string
                return text.to_string();
            }
            let num_gap = words_cnt - 1;
            let num_whites_in_gap = missing_whitespaces / num_gap;
            let remaining_whites = missing_whitespaces % num_gap;
            let mut t = String::new();
            for (i, w) in text.split_whitespace().enumerate() {
                t.push_str(w);
                if i < num_gap {
                    t.push_str(&String::from(' ').repeat(num_whites_in_gap));
                    if i + remaining_whites >= num_gap {
                        t.push(' ');
                    }
                }
            }
            t
        }
    }
}

#[allow(dead_code)]
pub fn main() {
    println!("This is main function for string manipulations");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_text_align_left() {
        let tcs: [(&str, &str); 5] = [
            ("  this is   text   ", "this is   text     "),
            ("", ""),
            ("1", "1"),
            ("2 ", "2 "),
            (" 3 ", "3  "),
        ];
        for tc in tcs {
            let res = text_align(tc.0, tc.0.len(), TextAlign::Left);
            assert_eq!(tc.1, res);
        }
    }

    #[test]
    fn test_text_align_center() {
        let tcs: [(&str, &str, usize); 5] = [
            ("", "", 0),
            ("1", "1", 1),
            (
                "this text should be centered",
                "      this text should be centered      ",
                40,
            ),
            ("2  ", " 2 ", 3),
            (" 3", "3 ", 2),
        ];
        for tc in tcs {
            let result = text_align(tc.0, tc.2, TextAlign::Center);
            assert_eq!(result, tc.1);
        }
    }

    #[test]
    fn test_text_align_right() {
        let tcs: [(&str, &str, usize); 5] = [
            ("", "", 0),
            ("1", "1", 1),
            (
                "this text should be right aligned",
                "       this text should be right aligned",
                40,
            ),
            ("2  ", "  2", 3),
            (" 3", " 3", 2),
        ];
        for tc in tcs {
            let result = text_align(tc.0, tc.2, TextAlign::Right);
            assert_eq!(result, tc.1);
        }
    }

    #[test]
    fn test_text_justify() {
        let tcs: [(&str, &str, usize); 5] = [
            ("", "", 0),
            ("1", "1", 1),
            (
                "this text should be justified",
                "this   text    should    be    justified",
                40,
            ),
            ("2  ", "2  ", 3),
            (" 3 4 5    ", "3   4    5", 2),
        ];
        for tc in tcs {
            let result = text_align(tc.0, tc.2, TextAlign::Justify);
            assert_eq!(result, tc.1);
        }
    }
}
