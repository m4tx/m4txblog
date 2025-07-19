use std::io;
use std::io::{BufWriter, Write};

use comrak::html::{
    format_document_with_formatter, format_node_default, render_sourcepos, ChildRendering, Context,
};
use comrak::nodes::{AstNode, NodeValue};
use comrak::{parse_document, Arena, Options, Plugins};

pub(super) fn markdown_to_html(md: &str, options: &Options, plugins: &Plugins) -> String {
    let arena = Arena::new();
    let root = parse_document(&arena, md, options);
    let mut bw = BufWriter::new(Vec::new());
    format_document_with_formatter(root, options, &mut bw, plugins, format_node_custom, ())
        .unwrap();

    String::from_utf8(bw.into_inner().unwrap()).unwrap()
}

fn format_node_custom<'a, T>(
    context: &mut Context<T>,
    node: &'a AstNode<'a>,
    entering: bool,
) -> io::Result<ChildRendering> {
    match node.data.borrow().value {
        NodeValue::Table(_) => render_table_custom(context, node, entering),
        _ => format_node_default(context, node, entering),
    }
}

fn render_table_custom<'a, T>(
    context: &mut Context<T>,
    node: &'a AstNode<'a>,
    entering: bool,
) -> io::Result<ChildRendering> {
    if entering {
        context.cr()?;
        // add the Bootstrap "table" class
        context.write_all(b"<table class=\"table\"")?;
        render_sourcepos(context, node)?;
        context.write_all(b">")?;
    } else {
        if !node
            .last_child()
            .unwrap()
            .same_node(node.first_child().unwrap())
        {
            context.cr()?;
            context.write_all(b"</tbody>")?;
        }
        context.cr()?;
        context.write_all(b"</table>")?;
    }

    Ok(ChildRendering::HTML)
}
