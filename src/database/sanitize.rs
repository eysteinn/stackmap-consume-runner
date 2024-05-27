use regex::Regex;

fn sanitize_schema_name(name: &str) -> Result<String, String> {
    // Replace spaces with underscores
    let newname = name.replace(" ", "_");

    // Remove any characters that are not alphanumeric or underscores
    let valid_name = Regex::new(r#"[^a-zA-Z0-9_]"#).unwrap();
    let newname = valid_name.replace_all(&newname, "").to_string();

    // Truncate the name to a reasonable length if needed
    let newname = if newname.len() > 63 {
        newname[..63].to_string()
    } else {
        newname
    };

    if newname != name {
        return Err("Project name is not allowed, use only alphanumerical characters".to_string());
    }

    Ok(newname)
}
