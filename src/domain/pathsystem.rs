use regex::Regex;

pub fn is_dir_semantic_check(path: &str) -> Result<bool, &'static str> {
    if path == "" {
        return Ok(true);
    }
    let re = Regex::new(r"^[a-zA-Z0-9]+(\.md)?$").unwrap();
    let parts: Vec<&str> = path.split('/').collect();

    for (i, part) in parts.iter().enumerate() {
        if !re.is_match(part) {
            return Err("malformed path");
        }
        if part.ends_with(".md") {
            if i != parts.len() - 1 {
                return Err("malformed path");
            } else {
                return Ok(false);
            }
        }
    }
    Ok(true)
}
