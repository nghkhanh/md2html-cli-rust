fn parse_inline_elements(line: &str) -> String {
    line.to_string()
}

pub fn parse(markdown_input: &str) -> String {

    let mut html_output = String::new();

    let lines = markdown_input.lines();

    for line in lines {
        if line.starts_with("## "){
            let raw_content = line.strip_prefix("## ").unwrap();
            let processed_content = parse_inline_elements(raw_content);
            let h2_tag = format!("<h2>{}</h2>\n", content);

            html_output.push_str(&h2_tag);
        }
        else if line.starts_with("# "){
            let raw_content = line.strip_prefix("# ").unwrap();
            let processed_content = parse_inline_elements(raw_content);
            let h1_tag = format!("<h1>{}</h1>\n", content);

            html_output.push_str(&h1_tag);
        }
        else {
            if !line.is_empty() {
                let processed_content = parse_inline_elements(line);
                let p_tag = format!("<p>{}</p>\n", line);
                html_output.push_str(&p_tag);
            }
        }
    }

    html_output
}
