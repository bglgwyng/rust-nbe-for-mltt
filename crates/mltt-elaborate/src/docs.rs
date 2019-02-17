use crate::TypeError;

/// Concatenate a bunch of lines of documentation into a single string, removing
/// comment prefixes if they are found.
pub fn concat(doc_lines: &[String]) -> String {
    let mut doc = String::new();
    for doc_line in doc_lines {
        // Strip the `||| ` or `|||` prefix left over from tokenization
        // We assume that each line of documentation has a trailing new line
        doc.push_str(match doc_line {
            doc_line if doc_line.starts_with("||| ") => &doc_line["||| ".len()..],
            doc_line if doc_line.starts_with("|||") => &doc_line["|||".len()..],
            doc_line => doc_line,
        });
    }
    doc
}

/// Select the documentation from either the declaration or the definition,
/// returning an error if both are present.
pub fn merge(name: &str, decl_docs: &[String], defn_docs: &[String]) -> Result<String, TypeError> {
    match (decl_docs, defn_docs) {
        ([], []) => Ok("".to_owned()),
        (docs, []) => Ok(concat(docs)),
        ([], docs) => Ok(concat(docs)),
        (_, _) => Err(TypeError::AlreadyDocumented(name.to_owned())),
    }
}
