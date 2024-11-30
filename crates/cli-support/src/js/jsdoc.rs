//! BEWARE: JSDoc does not have a formal specification, so this parser is based
//! on common conventions generally tries to mimic the behavior of the
//! TypeScript's JSDoc parser when it comes to edge cases.
//!
//! Well formatted JSDoc comments will be handled correctly, but edge cases
//! (e.g. weird tags, missing/too many spaces) may be handled differently
//! compared to other parsers. See the below test cases for examples.

use super::ident::is_ident_start;

#[derive(Debug, Clone)]
pub struct JsDoc {
    /// Optional description at the start of a comment.
    pub description: String,
    pub tags: Vec<JsDocTag>,
}

#[derive(Debug, Clone)]
pub enum JsDocTag {
    Param(ParamTag),
    Returns(ReturnsTag),
    Unknown(UnknownTag),
}

#[derive(Debug, Clone)]
pub struct ParamTag {
    pub ty: Option<String>,
    pub name: String,
    pub optional: Optionality,
    /// Description of the parameter. Might be empty.
    pub description: String,
}

#[derive(Debug, Clone)]
pub enum Optionality {
    /// E.g. `@param {number} foo`
    Required,
    /// E.g. `@param {number} [foo]`
    Optional,
    /// E.g. `@param {number} [foo=123]`. In this case, the `String` value is `123`.
    OptionalWithDefault(String),
}

#[derive(Debug, Clone)]
pub struct ReturnsTag {
    pub ty: Option<String>,
    /// Description of the return value. Might be empty.
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct UnknownTag {
    pub tag: String,
    /// The text right after the tag name.
    ///
    /// E.g. for `@foo bar`, the text is `" bar"` (note that the space is included).
    pub text: String,
}

impl JsDoc {
    pub fn parse(comment: &str) -> Self {
        let comment = remove_leading_space(comment);
        let comment = trim_right(comment.as_str());

        let mut tags = Vec::new();

        let Some(mut block) = find_next_tag(comment) else {
            // there are no tags, the entire comment is the description
            return Self {
                description: comment.to_string(),
                tags,
            };
        };

        let mut description = String::new();
        let description_text = &comment[..block.0];
        if !description_text.trim().is_empty() {
            description = trim_right(description_text).to_string();

            // preserve final new line
            if description_text.ends_with("\n\n") {
                description.push('\n');
            }
        }

        loop {
            let mut rest = &comment[block.0 + block.1.len()..];

            // find the next tag, so we know where this block ends
            let next_line_index = get_line_length(rest.as_bytes());
            let next_block = find_next_tag(&rest[next_line_index..]);

            if let Some(next_block) = next_block {
                rest = trim_right(&rest[..(next_block.0 + next_line_index)]);
            }

            tags.push(Self::parse_tag(block.1, rest));

            if let Some(mut next_block) = next_block {
                // change the index of the next block to be relative to the entire comment
                next_block.0 += block.0 + block.1.len() + next_line_index;
                block = next_block;
            } else {
                // no more tags
                break;
            }
        }

        Self { description, tags }
    }

    fn parse_tag(tag_name: &str, rest: &str) -> JsDocTag {
        match tag_name {
            "@param" | "@arg" | "@argument" => {
                if let Some(tag) = ParamTag::parse(rest) {
                    return JsDocTag::Param(tag);
                }
            }
            "@returns" | "@return" => {
                if let Some(tag) = ReturnsTag::parse(rest) {
                    return JsDocTag::Returns(tag);
                }
            }
            _ => {}
        }

        JsDocTag::Unknown(UnknownTag {
            tag: tag_name.to_string(),
            text: rest.to_string(),
        })
    }

    pub fn enhance(&mut self, tags: Vec<JsDocTag>) {
        for tag in tags {
            match tag {
                JsDocTag::Param(tag) => {
                    if let Some(param_tag) = self.get_or_add_param(&tag.name) {
                        if param_tag.ty.is_none() {
                            param_tag.ty = tag.ty;
                        }
                        if matches!(param_tag.optional, Optionality::Required) {
                            param_tag.optional = tag.optional;
                        }
                    }
                }
                JsDocTag::Returns(tag) => {
                    if let Some(returns_tag) = self.get_or_add_returns() {
                        if returns_tag.ty.is_none() {
                            returns_tag.ty = tag.ty;
                        }
                    }
                }
                _ => {}
            }
        }
    }

    /// If there is a single `@returns` tag, return it. Otherwise, add a new
    /// `@returns` tag and return it.
    ///
    /// If there are multiple `@returns` tags, return `None`.
    pub fn get_or_add_param<'a>(&'a mut self, name: &str) -> Option<&'a mut ParamTag> {
        // check that there is exactly one returns tag
        let returns_count = self
            .tags
            .iter()
            .filter(|tag| match tag {
                JsDocTag::Param(tag) => {
                    if tag.name == name {
                        return true;
                    }
                    if tag.name.starts_with(name) {
                        // account for paths
                        let after = tag.name[name.len()..].chars().next();
                        return after == Some('.') || after == Some('[');
                    }
                    false
                }
                _ => false,
            })
            .count();

        if returns_count > 1 {
            // multiple return tags, we don't know which one to update
            return None;
        }
        if returns_count == 0 {
            // add a new returns tag
            // try to insert it before a returns tag
            let pos = self
                .tags
                .iter()
                .position(|tag| matches!(tag, JsDocTag::Returns(_)))
                .unwrap_or(self.tags.len());

            self.tags.insert(
                pos,
                JsDocTag::Param(ParamTag {
                    ty: None,
                    name: name.to_string(),
                    optional: Optionality::Required,
                    description: String::new(),
                }),
            );
        }

        for tag in &mut self.tags {
            if let JsDocTag::Param(tag) = tag {
                if tag.name == name {
                    // return the existing tag
                    return Some(tag);
                }
            }
        }

        None
    }

    /// If there is a single `@returns` tag, return it. Otherwise, add a new
    /// `@returns` tag and return it.
    ///
    /// If there are multiple `@returns` tags, return `None`.
    pub fn get_or_add_returns(&mut self) -> Option<&mut ReturnsTag> {
        // check that there is exactly one returns tag
        let count = self
            .tags
            .iter()
            .filter(|tag| matches!(tag, JsDocTag::Returns(_)))
            .count();

        if count > 1 {
            // multiple return tags, we don't know which one to update
            return None;
        }
        if count == 0 {
            // add a new returns tag
            self.tags.push(JsDocTag::Returns(ReturnsTag {
                ty: None,
                description: String::new(),
            }));
        }

        for tag in &mut self.tags {
            if let JsDocTag::Returns(tag) = tag {
                // return the existing tag
                return Some(tag);
            }
        }

        unreachable!()
    }

    /// Same as `to_string`, but indents the output with 1 space.
    pub fn to_string_indented(&self) -> String {
        let mut out = String::new();
        for (index, line) in self.to_string().lines().enumerate() {
            if index > 0 {
                out.push('\n');
            }
            if !line.is_empty() {
                out.push(' ');
            }
            out.push_str(line);
        }
        out
    }
}

impl std::fmt::Display for JsDoc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if !self.description.trim().is_empty() {
            writeln!(f, "{}", self.description)?;
        }

        for tag in &self.tags {
            writeln!(f, "{}", tag)?;
        }

        Ok(())
    }
}

impl std::fmt::Display for JsDocTag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            JsDocTag::Param(tag) => {
                write!(f, "@param")?;
                if let Some(ty) = &tag.ty {
                    write!(f, " {{{}}}", ty)?
                }
                match &tag.optional {
                    Optionality::Required => write!(f, " {}", tag.name)?,
                    Optionality::Optional => write!(f, " [{}]", tag.name)?,
                    Optionality::OptionalWithDefault(default) => {
                        write!(f, " [{}={}]", tag.name, default)?
                    }
                }
                if tag.description.starts_with(['\r', '\n']) {
                    write!(f, "{}", tag.description)?;
                } else if !tag.description.is_empty() {
                    write!(f, " {}", tag.description)?;
                }
            }
            JsDocTag::Returns(tag) => {
                write!(f, "@returns")?;
                if let Some(ty) = &tag.ty {
                    write!(f, " {{{}}}", ty)?
                }
                if tag.description.starts_with(['\r', '\n']) {
                    write!(f, "{}", tag.description)?;
                } else if !tag.description.is_empty() {
                    write!(f, " {}", tag.description)?;
                }
            }
            JsDocTag::Unknown(tag) => write!(f, "{}{}", tag.tag, tag.text)?,
        }

        Ok(())
    }
}

impl ParamTag {
    fn parse(rest: &str) -> Option<Self> {
        let mut text = trim_left(rest);

        let mut optional_by_type = false;

        let mut ty = None;
        if text.starts_with('{') {
            let mut type_len = 0;
            ty = consume_type_script_expression(&text[1..]).map(|mut t| {
                type_len = t.len() + 2;
                t = t.trim_matches(' ');
                if t.ends_with('=') {
                    optional_by_type = true;
                    t = t[..t.len() - 1].trim_matches(' ');
                }
                t.to_string()
            });

            if ty.is_some() {
                text = trim_left(&text[type_len..]);
            } else {
                // the type expression is not terminated, so the tag is not well-formed
                return None;
            }
        }
        ty = post_process_typescript_expression(ty);

        let (optional, name) = if text.starts_with('[') {
            // skip the `[`
            text = trim_left_space(&text[1..]);

            let Some(name) = consume_parameter_name_path(text) else {
                // the name is not well-formed
                return None;
            };
            text = trim_left_space(&text[name.len()..]);

            let mut default = None;
            if text.starts_with('=') {
                text = trim_left_space(&text[1..]);
                // the default value doesn't have to be a valid JS expression,
                // so we just search for ']', '\n', or end of string
                let end = text.find([']', '\n']).unwrap_or(text.len());
                let default_text = text[..end].trim();
                if !default_text.is_empty() {
                    default = Some(default_text.to_string());
                }

                text = &text[end..];
                if !text.is_empty() {
                    text = trim_left_space(&text[1..]);
                }
            } else if text.starts_with(']') {
                text = trim_left_space(&text[1..]);
            }

            (
                default
                    .map(Optionality::OptionalWithDefault)
                    .unwrap_or(Optionality::Optional),
                name.to_string(),
            )
        } else {
            let Some(name) = consume_parameter_name_path(text) else {
                // the name is not well-formed
                return None;
            };
            text = trim_left_space(&text[name.len()..]);
            (
                if optional_by_type {
                    Optionality::Optional
                } else {
                    Optionality::Required
                },
                name.to_string(),
            )
        };

        Some(Self {
            ty,
            optional,
            name,
            description: text.to_string(),
        })
    }
}

impl ReturnsTag {
    fn parse(rest: &str) -> Option<Self> {
        // A bit careful now, because we want to keep the initial new lines of
        // the description.
        let mut text = {
            let trimmed = trim_left(rest);
            if trimmed.starts_with('{') {
                trimmed
            } else {
                trim_left_space(rest)
            }
        };

        let mut ty = None;
        if text.starts_with('{') {
            ty = consume_type_script_expression(&text[1..]).map(|t| t.to_string());

            if let Some(ty) = &ty {
                text = trim_left_space(&text[(ty.len() + 2)..]);
            } else {
                // the type expression is not terminated, so the tag is not well-formed
                return None;
            }
        }
        ty = post_process_typescript_expression(ty);

        Some(Self {
            ty,
            description: text.to_string(),
        })
    }
}

fn post_process_typescript_expression(expr: Option<String>) -> Option<String> {
    expr.and_then(|e| if e.trim().is_empty() { None } else { Some(e) })
}

/// A version trim_start that ignores text direction.
fn trim_left(s: &str) -> &str {
    let mut first_non_space = None;
    for (index, c) in s.char_indices() {
        if !c.is_whitespace() {
            first_non_space = Some(index);
            break;
        }
    }
    if let Some(first_non_space) = first_non_space {
        &s[first_non_space..]
    } else {
        ""
    }
}

/// Trims all space and tab characters from the left side of the string.
fn trim_left_space(s: &str) -> &str {
    let mut first_non_space = None;
    for (index, c) in s.char_indices() {
        if c != ' ' && c != '\t' {
            first_non_space = Some(index);
            break;
        }
    }
    if let Some(first_non_space) = first_non_space {
        &s[first_non_space..]
    } else {
        ""
    }
}

/// A version trim_end that ignores text direction.
fn trim_right(s: &str) -> &str {
    let mut last_space = s.len();
    for (index, c) in s.char_indices().rev() {
        if c.is_whitespace() {
            last_space = index;
        } else {
            break;
        }
    }
    &s[..last_space]
}

// returns the byte index of the `@` symbol of the next tag as well as the tag name.
fn find_next_tag(comment: &str) -> Option<(usize, &str)> {
    // This function essentially implement this regex: `/^[ ]*@/m.exec(comment)`

    let mut index = 0;
    while index < comment.len() {
        // we are at the start of a line

        // skip leading spaces
        while let Some(b' ') = comment.as_bytes().get(index) {
            index += 1;
        }

        if let Some(tag_name) = parse_tag_name(&comment[index..]) {
            return Some((index, tag_name));
        }

        // skip to the next line
        while index < comment.len() {
            if comment.as_bytes()[index] == b'\n' {
                index += 1;
                break;
            }
            index += 1;
        }
    }

    None
}

/// If the given string starts with a syntactically valid tag, it will returns
/// the string slice with the tag.
///
/// E.g. `@param {string} foo` will return `Some("@param")`.
fn parse_tag_name(comment: &str) -> Option<&str> {
    if comment.starts_with('@') && comment.len() > 1 {
        let stop = comment[1..].find(|c: char| c.is_whitespace() || c == '{');
        if let Some(stop) = stop {
            if stop == 0 {
                None
            } else {
                Some(&comment[..stop + 1])
            }
        } else {
            // the entire string is the tag
            Some(comment)
        }
    } else {
        None
    }
}

/// Returns the length in bytes of the current line including the trailing new
/// line character.
fn get_line_length(comment: &[u8]) -> usize {
    comment
        .iter()
        .position(|&c| c == b'\n')
        .map(|p| p + 1)
        .unwrap_or(comment.len())
}

/// Consumes a TypeScript expression from the beginning of the given string
/// until a `}` character is found.
///
/// The return string will be the TypeScript expression without the braces.
/// However, if `Some` is returned, the next character after the string will be
/// the closing `}` character.
fn consume_type_script_expression(comment: &str) -> Option<&str> {
    // Okay, so the main challenge here is that TypeScript expressions can
    // contain nested `{}` pairs, strings, comments, and string template
    // literals. So we have to handle those 4 things, but that's it.
    //
    // We will also assume that the expression is valid TypeScript. If it's not,
    // the results may be unexpected.

    /// Returns the number of bytes consumed by the string, including the
    /// opening and closing quotes.
    ///
    /// If the string is not terminated, `None` is returned.
    fn consume_string(bytes: &[u8]) -> Option<usize> {
        debug_assert!(bytes[0] == b'"' || bytes[0] == b'\'');

        let kind = bytes[0];

        // string can't contain certain characters (e.g. new lines), but that's
        // not a problem, because we assume valid strings.

        let mut index = 1;
        while index < bytes.len() {
            let c = bytes[index];
            if c == b'\\' {
                // skip the next character
                index += 1;
            } else if c == kind {
                return Some(index + 1);
            }
            index += 1;
        }

        // the string is not terminated
        None
    }
    /// Returns the number of bytes consumed by the single line comment,
    /// including the trailing new line.
    fn consume_single_line_comment(bytes: &[u8]) -> usize {
        debug_assert!(bytes[0] == b'/');
        debug_assert!(bytes[1] == b'/');

        get_line_length(bytes)
    }
    /// Returns the number of bytes consumed by braced (`{}`) expression,
    /// including the closing `}`.
    fn consume_string_template(bytes: &[u8]) -> Option<usize> {
        debug_assert!(bytes[0] == b'`');

        let mut index = 1;
        while index < bytes.len() {
            let c = bytes[index];
            if c == b'\\' {
                // skip the next character
                index += 1;
            } else if c == b'`' {
                return Some(index + 1);
            } else if c == b'$' {
                if let Some(b'{') = bytes.get(index + 1) {
                    // interpolated expression
                    index = consume_brace_expression(&bytes[index + 2..])?;
                }
            }
            index += 1;
        }

        // the string is not terminated
        None
    }
    /// Returns the number of bytes consumed by braced (`{}`) expression,
    /// including the closing `}`.
    fn consume_brace_expression(bytes: &[u8]) -> Option<usize> {
        let mut brace_depth = 0;

        let mut index = 0;
        while index < bytes.len() {
            let c = bytes[index];
            match c {
                b'{' => {
                    brace_depth += 1;
                    index += 1;
                }
                b'}' => {
                    if brace_depth == 0 {
                        return Some(index + 1);
                    }
                    brace_depth -= 1;
                    index += 1;
                }
                b'"' | b'\'' => {
                    index += consume_string(&bytes[index..])?;
                }
                b'`' => {
                    index += consume_string_template(&bytes[index..])?;
                }
                b'/' => {
                    // might be a comment
                    if let Some(b'/') = bytes.get(index + 1) {
                        index += consume_single_line_comment(&bytes[index..]);
                    } else {
                        index += 1;
                    }
                }
                _ => {
                    index += 1;
                }
            }
        }

        // not terminated
        None
    }

    let braced_len = consume_brace_expression(comment.as_bytes())?;
    debug_assert!(braced_len > 0);

    // There is no closing brace
    Some(&comment[..braced_len - 1])
}

/// `@param` tags support not just simple Js identifiers for the parameter
/// name, but also paths (e.g. `foo.bar.baz`) and array items
/// (e.g. `foo[].bar`).
///
/// See https://jsdoc.app/tags-param
fn consume_parameter_name_path(text: &str) -> Option<&str> {
    let mut chars = text.char_indices();

    let mut valid_first = false;
    if let Some((_, c)) = chars.next() {
        if is_ident_start(c) {
            valid_first = true;
        }
    }
    if !valid_first {
        return None;
    }

    while let Some((index, c)) = chars.next() {
        if c == '[' {
            // this is only allowed if followed by a `].`
            if let Some((_, ']')) = chars.next() {
                if let Some((_, '.')) = chars.next() {
                    continue;
                }
            }
            return None;
        }

        if c == '.' {
            // the next character must be a valid identifier start
            if let Some((_, c)) = chars.next() {
                if is_ident_start(c) {
                    continue;
                }
            }
            return None;
        }

        if c.is_whitespace() || matches!(c, ']' | '=') {
            // found stop character
            return Some(&text[..index]);
        }

        if !is_ident_start(c) {
            return None;
        }
    }

    Some(text)
}

/// If all lines are empty or start with a leading space, remove the
/// leading space from all lines.
fn remove_leading_space(comment: &str) -> String {
    let leading_space = comment.lines().all(|l| l.is_empty() || l.starts_with(' '));
    if leading_space {
        let mut out = String::new();
        for (index, line) in comment.lines().enumerate() {
            if index > 0 {
                out.push('\n');
            }
            if !line.is_empty() {
                out.push_str(&line[1..]);
            }
        }
        out
    } else {
        comment.to_string()
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn weird_tag_names() {
        fn test(comment: &str, expected: Option<&str>) {
            assert_eq!(
                parse_tag_name(comment),
                expected,
                "Expected comment {:?} to be parsed as {:?}",
                comment,
                expected
            );
        }

        // doesn't start with @
        test("", None);
        test("foo", None);
        test(" @param", None);

        test("@", None);
        test("@ foo", None);
        test("@param", Some("@param"));
        test("@param {type} name", Some("@param"));
        test("@param{type}name", Some("@param"));
        test("@type{type}", Some("@type"));

        // unicode
        test("@üwü", Some("@üwü"));
        test("@üwü äwoo", Some("@üwü"));

        // unicode spaces
        // no-break space
        test("@üwü\u{A0}", Some("@üwü"));
        // line separator
        test("@üwü\u{2028}", Some("@üwü"));
    }

    #[test]
    fn parser_snapshots() {
        let mut suite = Suite::new();

        // description
        suite.test(r"This is a description.");
        suite.test_lines(&["This is", " ", "", "a multi-line", "description."]);
        suite.test_lines(&["This is a description with tag.", "", "@public"]);
        suite.test_lines(&["This is a description with tag.", "@public"]);

        // @param

        // well-formed
        suite.test_lines(&[
            "@param foo",
            "@param foo description",
            "@param foo\ndescription",
            "@param [foo]",
            "@param [foo] description",
            "@param [foo]\ndescription",
            "@param [foo=123]",
            "@param [foo=123] description",
            "@param [foo=123]\ndescription",
            "@param {number} foo",
            "@param {number} foo description",
            "@param {number} [foo]",
            "@param {number} [foo] description",
            "@param {number} [foo=123]",
            "@param {number} [foo=123] description",
            // new objects
            "@param {object} obj",
            "@param {string} obj.name",
            "@param {object[]} obj.locations",
            "@param {string} obj.locations[].address",
            "@param {string} [obj.locations[].address]",
            "@param {} foo",
        ]);
        // weird
        suite.test_lines(&[
            "@param {string} foo",
            "@param{string}foo  ",
            "@param{string}[foo]",
            "@param{string}[foo=]",
            "@param   {   string   }  [  foo  =  123  ]",
            "@param   {      }  [  foo  =  123  ]",
        ]);
        // weird types
        suite.test_lines(&[
            "@param   {",
            "string",
            "} foo",
            "@param   {string // comment",
            "| number}  foo",
            "@param { number = } foo",
            "@param {  =  } foo",
            "@param {{",
            "  name: { first: string, last: string };",
            "}} foo",
            "@param {'}' | \"}\" | `}${{'}': \"}\"}}}`} foo",
        ]);
        // alias
        suite.test_lines(&[
            "@arg foo",
            "@arg {string} foo",
            "@argument foo",
            "@argument {string} foo",
        ]);

        // @returns

        // well-formed
        suite.test_lines(&[
            "@returns",
            "@returns description",
            "@returns\ndescription",
            "@returns {string}",
            "@returns\n\n\n{number}",
            "@returns {string} description",
            "@returns {string}\ndescription",
        ]);
        // weird
        suite.test_lines(&[
            "@returns   ",
            "@returns   description",
            "@returns  {} ",
            "@returns{void}",
            "@returns{void}   ",
            "@returns{void}description",
            "@returns{void}   description",
            "@returns\n\n\n{\n\nvoid\n\n}\n",
        ]);
        // invalid
        suite.test_lines(&["@returns  {"]);
        // alias
        suite.test("@return {string} description");

        suite.assert();

        struct Suite {
            output: String,
        }
        impl Suite {
            fn new() -> Self {
                Self {
                    output: String::new(),
                }
            }
            fn test_lines(&mut self, lines: &[&str]) {
                self.test(&lines.join("\n"));
            }
            fn test(&mut self, comment: &str) {
                fn indent(s: &str) -> String {
                    s.lines()
                        .map(|l| {
                            if l.is_empty() {
                                l.to_string()
                            } else {
                                format!("    {}", l)
                            }
                        })
                        .collect::<Vec<_>>()
                        .join("\n")
                }

                let js_doc = JsDoc::parse(comment);
                let output = js_doc.to_string();

                self.output.push_str("\nInput: |\n");
                self.output.push_str(indent(comment).as_str());
                self.output.push_str("\n\nOutput: |\n");
                self.output.push_str(indent(&output).as_str());
                self.output.push_str("\n\nAst: |\n");
                self.output
                    .push_str(indent(&format!("{:#?}", js_doc)).as_str());
                self.output.push_str("\n\n---\n");
            }
            fn assert(&self) {
                let path = PathBuf::from("tests/snapshots/jsdoc.yml");

                // read the file and remember the content/error for later
                let expected = std::fs::read_to_string(&path);

                // update the snapshot file (and create parent dir if necessary)
                std::fs::create_dir_all(path.parent().unwrap()).unwrap();
                std::fs::write(&path, self.output.as_bytes()).unwrap();

                // compare the expected content with the actual content
                let mut expected = expected.expect("Failed to read the snapshot file");
                // normalize line endings
                expected = expected.replace("\r\n", "\n");
                let actual = self.output.replace("\r\n", "\n");

                if actual != expected {
                    let first_diff_line = expected
                        .lines()
                        .zip(actual.lines())
                        .position(|(e, a)| e != a)
                        .unwrap_or_else(|| expected.lines().count().min(actual.lines().count()));

                    eprintln!("Snapshots differ!");
                    eprintln!("First diff at line {}", first_diff_line + 1);
                    panic!("Snapshots differ");
                }
            }
        }
    }
}
