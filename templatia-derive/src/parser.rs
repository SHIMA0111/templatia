pub(crate) enum TemplateSegments<'a> {
    Literal(&'a str),
    Placeholder(&'a str),
}

pub(crate) fn parse_template(template: &'_ str) -> Result<Vec<TemplateSegments<'_>>, String> {
    let mut segments = Vec::new();
    let mut last_end = 0;
    let mut chars = template.char_indices().peekable();

    while let Some((i, c)) = chars.next() {
        match c {
            '{' => {
                if let Some(&(_, next_char)) = chars.peek() {
                    // if the next char is a `{`, it means escaped brace, so it shouldn't be treated as a placeholder.
                    if next_char == '{' {
                        chars.next();
                        continue;
                    }
                }

                if i > last_end {
                    segments.push(TemplateSegments::Literal(&template[last_end..i]));
                }

                // Skip placeholder brace
                let start = i + 1;
                let end = template[start..]
                    .find('}')
                    .map(|e| start + e)
                    .ok_or_else(|| "Unmatched opening brace '{'".to_string())?;
                let placeholder = &template[start..end];
                if placeholder.contains('{') {
                    return Err(format!("Nested braces are not supported: {}", placeholder));
                }
                segments.push(TemplateSegments::Placeholder(placeholder.trim()));

                // Proceed last_end to after the placeholder's end brace('}')
                last_end = end + 1;
                // Proceed char's iterator to after the placeholder's end brace('}')
                while let Some((idx, _)) = chars.peek().copied() {
                    // If the template is 'key1 = {value1}, key2 = {value2}',
                    // the first execution of this branch, `{` of {value1}. This index is 7.
                    // And the end of the placeholder brace(`}`)'s index is 14.
                    // So, the first execution should be proceeded to 15 (14 is the end brace, so the iterator should be in 15 after the execution).
                    // In the next index is index 14, the chars.next() returns (14, '}').
                    // The next root while loop gets the next index, which is 15.
                    if idx <= end {
                        chars.next();
                    } else {
                        break;
                    }
                }
            },
            '}' => {
                if let Some(&(_, next_char)) = chars.peek() {
                    // if the next char is a `}`, it means escaped brace, so it shouldn't be treated as an end brace.
                    if next_char == '}' {
                        chars.next();
                        continue;
                    }
                }
                return Err("Unmatched closing brace '{'".to_string());
            },
            _  => {}
        }
    }

    if last_end < template.len() {
        segments.push(TemplateSegments::Literal(&template[last_end..]));
    }

    Ok(segments)
}