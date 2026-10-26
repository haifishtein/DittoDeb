pub fn sanitize_html(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut in_tag = false;
    for c in input.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(c),
            _ => {}
        }
    }
    out.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .chars()
        .take(2000)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn strips_tags() {
        assert_eq!(sanitize_html("<b>hi</b> world"), "hi world");
    }
    #[test]
    fn truncates_long() {
        let long = "a ".repeat(3000);
        assert!(sanitize_html(&long).chars().count() <= 2000);
    }
}
