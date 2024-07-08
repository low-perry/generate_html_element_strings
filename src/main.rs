fn main() {
    let html_element: Vec<&str> = vec!["h1", "h2", "h3", "h4", "h5", "h6", "header", "hgroup", "i", "iframe", "img", "ins", "kbd", "label", "main", "map", "mark", "meter", "nav", "object", "p", "picture", "pre", "progress", "q", "ruby", "s", "samp", "search", "section", "select", "small", "span", "strong", "sub", "sup", "table", "textarea", "time", "u", "var", "video"];
    arr_to_html_element_string(&html_element);
}

fn arr_to_html_element_string(vec: &Vec<&str>) {
    for element in vec {
        println!("- {{{{HtmlElement(\"{}\")}}}}", element);
    }
}
