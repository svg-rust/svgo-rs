use swc_xml_ast::*;
use regex::Regex;

use crate::collections::get_text_elems;

#[derive(Debug, Clone)]
pub enum Eol {
    Lf,
    Crlf
}

pub struct StringifyOptions {
    doctype_start: String,
    doctype_end: String,
    proc_inst_start: String,
    proc_inst_end: String,
    tag_open_start: String,
    tag_open_end: String,
    tag_close_start: String,
    tag_close_end: String,
    tag_short_start: String,
    tag_short_end: String,
    attr_start: String,
    attr_end: String,
    comment_start: String,
    comment_end: String,
    cdata_start: String,
    cdata_end: String,
    text_start: String,
    text_end: String,
    indent: usize,
    reg_entities: Regex,
    reg_val_entities: Regex,
    encode_entity: Option<Box<dyn Fn(char) -> String>>,
    pretty: bool,
    use_short_tags: bool,
    eol: Eol,
    final_newline: bool,
}

impl Default for StringifyOptions {
    fn default() -> Self {
        Self {
            doctype_start: "<!DOCTYPE".to_string(),
            doctype_end: ">".to_string(),
            proc_inst_start: "<?".to_string(),
            proc_inst_end: "?>".to_string(),
            tag_open_start: "<".to_string(),
            tag_open_end: ">".to_string(),
            tag_close_start: "</".to_string(),
            tag_close_end: ">".to_string(),
            tag_short_start: "<".to_string(),
            tag_short_end: "/>".to_string(),
            attr_start: "=\"".to_string(),
            attr_end: "\"".to_string(),
            comment_start: "<!--".to_string(),
            comment_end: "-->".to_string(),
            cdata_start: "<![CDATA[".to_string(),
            cdata_end: "]]>".to_string(),
            text_start: "".to_string(),
            text_end: "".to_string(),
            indent: 4,
            reg_entities: Regex::new(r#"[&'"<>]"#).unwrap(),
            reg_val_entities: Regex::new(r#"[&"<>]"#).unwrap(),
            encode_entity: Some(Box::new(
                |c: char| -> String {
                    match c {
                        '&' => "&amp;".to_string(),
                        '\'' => "&apos;".to_string(),
                        '"' => "&quot;".to_string(),
                        '>' => "&gt;".to_string(),
                        '<' => "&lt;".to_string(),
                        _ => c.to_string(),
                    }
                }
            )),
            pretty: false,
            use_short_tags: true,
            eol: Eol::Lf,
            final_newline: false,
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Ctx<'a> {
    indent: String,
    indent_level: usize,
    text_context: Option<&'a Element>,
}

pub struct Stringifier<'a> {
    options: StringifyOptions,
    defaults: StringifyOptions,
    ctx: Ctx<'a>,
    text_elems: Vec<&'static str>,
}

impl Stringifier<'_> {
    pub fn new(user_options: StringifyOptions) -> Self {
        let eol = match user_options.eol {
            Eol::Crlf => "\r\n",
            Eol::Lf => "\n",
        };

        let mut options: StringifyOptions = Default::default();

        if options.pretty {
            options.doctype_end = user_options.doctype_end + eol;
            options.proc_inst_end = user_options.proc_inst_end + eol;
            options.comment_end = user_options.comment_end + eol;
            options.cdata_end = user_options.cdata_end + eol;
            options.tag_short_end = user_options.tag_short_end + eol;
            options.tag_open_end = user_options.tag_open_end + eol;
            options.tag_close_end = user_options.tag_close_end + eol;
            options.text_end = user_options.text_end + eol;
        }

        let indent = if options.indent < 0 {
            "\t".to_string()
        } else {
            " ".repeat(options.indent)
        };

        let ctx = Ctx {
            indent,
            indent_level: 0,
            text_context: None,
        };

        Self {
            options,
            defaults: Default::default(),
            ctx,
            text_elems: get_text_elems(),
        }
    }

    fn emit_document(&mut self, n: &Document) -> String {
        let mut svg = String::new();

        for n in n.children.iter() {
            svg += &self.emit_child(n);
        }

        if self.options.final_newline && svg.len() > 0 && svg[svg.len() - 1..].to_string() != "\n" {
            let eol = match self.options.eol {
                Eol::Crlf => "\r\n",
                Eol::Lf => "\n",
            };
            svg += eol;
        }

        svg
    }

    fn emit_child(&mut self, n: &Child) -> String {
        self.ctx.indent_level += 1;
        let result = match n {
            Child::DocumentType(n) => self.emit_document_doctype(n),
            Child::Element(n) => self.emit_element(n),
            Child::Text(n) => self.emit_text(n),
            Child::Comment(n) => self.emit_comment(n),
            Child::ProcessingInstruction(n) => self.emit_processing_instruction(n),
            Child::CdataSection(n) => self.emit_cdata_section(n),
        };
        self.ctx.indent_level -= 1;
        result
    }

    fn emit_document_doctype(&mut self, n: &DocumentType) -> String {
        let mut doctype = String::from(self.options.doctype_start.clone());
        if let Some(name) = &n.name {
            doctype.push(' ');
            doctype.push_str(name);
        }

        doctype.push_str(&self.options.doctype_end);

        doctype
    }

    fn emit_element(&mut self, n: &Element) -> String {
        let mut element = String::new();

        // empty element and short tag
        if n.children.len() == 0 {
            if self.options.use_short_tags {
                element.push_str(&self.create_indent());
                element.push_str(&self.options.tag_short_start);
                element.push_str(&n.tag_name);
                element.push_str(&self.emit_attributes(n));
                element.push_str(&self.options.tag_short_end);
            } else {
                element.push_str(&self.create_indent());
                element.push_str(&self.options.tag_open_start);
                element.push_str(&n.tag_name);
                element.push_str(&self.emit_attributes(n));
                element.push_str(&self.options.tag_open_end);
                element.push_str(&self.options.tag_close_start);
                element.push_str(&n.tag_name);
                element.push_str(&self.options.tag_close_end);
            }
            return element;
        }

        let mut tag_open_start = self.options.tag_open_start.clone();
        let mut tag_open_end = self.options.tag_open_end.clone();
        let mut tag_close_start = self.options.tag_close_start.clone();
        let mut tag_close_end = self.options.tag_close_end.clone();
        let mut open_indent = self.create_indent();
        let mut close_indent = self.create_indent();
        if self.ctx.text_context.is_some() {
            tag_open_start = self.defaults.tag_open_start.clone();
            tag_open_end = self.defaults.tag_open_end.clone();
            tag_close_start = self.defaults.tag_close_start.clone();
            tag_close_end = self.defaults.tag_close_end.clone();
            open_indent = "".to_string();
        } else if self.text_elems.contains(&n.tag_name.to_string().as_str()) {
            tag_open_end = self.defaults.tag_open_end.clone();
            tag_close_start = self.defaults.tag_close_start.clone();
            close_indent = "".to_string();

            unsafe {
                let r = n as *const Element;
                self.ctx.text_context = r.as_ref();
            }
        }

        let mut children = String::new();
        for child in &n.children {
            children.push_str(&self.emit_child(child))
        }

        if let Some(text_context) = self.ctx.text_context {
            if text_context == n {
                self.ctx.text_context = None;
            }
        }

        element.push_str(&open_indent);
        element.push_str(&tag_open_start);
        element.push_str(&n.tag_name);
        element.push_str(&self.emit_attributes(n));
        element.push_str(&tag_open_end);
        element.push_str(&children);
        element.push_str(&close_indent);
        element.push_str(&tag_close_start);
        element.push_str(&n.tag_name);
        element.push_str(&tag_close_end);

        element
    }

    fn emit_attributes(&mut self, n: &Element) -> String {
        let mut attrs = String::new();

        for attr in &n.attributes {
            if let Some(value) = &attr.value {
                let encoded_value = if let Some(encode_entity) = &self.options.encode_entity {
                    value.chars().map(encode_entity).collect()
                } else {
                    value.to_string()
                };
                attrs.push_str(" ");
                attrs.push_str(&attr.name.to_string());
                attrs.push_str(&self.options.attr_start);
                attrs.push_str(&encoded_value);
                attrs.push_str(&self.options.attr_end);
            } else {
                attrs.push_str(" ");
                attrs.push_str(&attr.name.to_string());
            }
        }

        attrs
    }

    fn emit_text(&mut self, n: &Text) -> String {
        let mut text = String::new();

        text.push_str(&self.create_indent());
        text.push_str(&self.options.text_start);

        let encoded_data = if let Some(encode_entity) = &self.options.encode_entity {
            n.data.chars().map(encode_entity).collect()
        } else {
            n.data.to_string()
        };
        text.push_str(&encoded_data);

        if self.ctx.text_context.is_none() {
            text.push_str(&self.options.text_end);
        }

        text
    }

    fn emit_comment(&mut self, n: &Comment) -> String {
        let mut comment = String::new();

        comment.push_str(&self.options.comment_start);
        comment.push_str(&n.data);
        comment.push_str(&self.options.comment_end);

        comment
    }

    fn emit_processing_instruction(&mut self, n: &ProcessingInstruction) -> String {
        let mut processing_instruction = String::new();

        processing_instruction.push_str(&self.options.proc_inst_start);
        processing_instruction.push_str(&n.target);
        processing_instruction.push(' ');
        processing_instruction.push_str(&n.data);
        processing_instruction.push_str(&self.options.proc_inst_end);

        processing_instruction
    }

    fn emit_cdata_section(&mut self, n: &CdataSection) -> String {
        let mut cdata_section = String::new();

        cdata_section.push_str(&self.create_indent());
        cdata_section.push_str(&self.options.cdata_start);
        cdata_section.push_str(&n.data);
        cdata_section.push_str(&self.options.cdata_end);

        cdata_section
    }

    fn create_indent(&mut self) -> String {
        let mut indent = String::new();
        if self.options.pretty && self.ctx.text_context.is_none() {
            indent = self.ctx.indent.repeat(self.ctx.indent_level - 1);
        }
        indent
    }
}

/// convert XAST to SVG string
pub fn stringify_svg(doc: &Document) -> String {
    let mut stringifier = Stringifier::new(Default::default());
    stringifier.emit_document(doc)
}
