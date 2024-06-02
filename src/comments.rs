use unicode_segmentation::UnicodeSegmentation;

pub fn find_comment_index(line: &str) -> Option<usize> {
    // no percent means no comment
    if !line.contains("%") {
        return None;
    }

    let n = line.len();

    // empty line has no comment
    if n == 0 {
        return None;
    }

    // check the first character
    let mut prev_c: &str = line.graphemes(true).next().unwrap();
    if prev_c == "%" {
        return Some(0);
    }

    // single-character line
    if n == 1 {
        return None;
    }

    // multi-character line
    for i in 1..n {
        let c = line.graphemes(true).nth(i).unwrap();
        if c == "%" && (prev_c == " " || prev_c != "\\") {
            return Some(i);
        }
        prev_c = c;
    }
    None
}

pub fn remove_comment(line: &str, comment: Option<usize>) -> &str {
    match comment {
        Some(c) => &line.graphemes(true)[0..c],
        None => line,
    }
}

pub fn get_comment(line: &str, comment: Option<usize>) -> &str {
    match comment {
        Some(c) => &line[c..line.len()],
        None => "",
    }
}
