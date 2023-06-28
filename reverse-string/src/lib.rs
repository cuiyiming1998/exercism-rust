use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let str = input.to_string();
    let g = str.graphemes(true);
    g.rev().collect()
}
