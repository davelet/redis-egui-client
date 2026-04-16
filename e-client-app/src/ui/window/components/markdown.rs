//! Markdown renderer for egui using pulldown-cmark Event stream directly.
//! Avoids the fragile HTML-string intermediate step.

use pulldown_cmark::{Alignment, Event, HeadingLevel, Options, Parser, Tag, TagEnd};

// ── Table state ──────────────────────────────────────────────────────────────

#[derive(Default)]
struct TableState {
    /// Column alignments from the `Tag::Table` open event.
    alignments: Vec<Alignment>,
    /// All rows collected so far (each row is a Vec of cell strings).
    rows: Vec<Vec<String>>,
    /// The row currently being built.
    current_row: Vec<String>,
    /// The cell text currently being accumulated.
    current_cell: String,
    /// Whether we are inside a `<thead>` block.
    in_head: bool,
}

// ── Inline span ──────────────────────────────────────────────────────────────

/// A run of text with formatting flags, used to build a `egui::text::LayoutJob`
/// so that mixed bold/normal/code text can appear on the same line.
#[derive(Clone)]
struct Span {
    text: String,
    strong: bool,
    em: bool,
    code: bool,
}

impl Span {
    fn new(text: impl Into<String>, strong: bool, em: bool, code: bool) -> Self {
        Self {
            text: text.into(),
            strong,
            em,
            code,
        }
    }
}

// ── Helpers ──────────────────────────────────────────────────────────────────

/// Flush a list of `Span`s as a single `egui::Label` using a `LayoutJob`.
/// Falls back to a plain label when there is only one span.
fn flush_spans(spans: &mut Vec<Span>, ui: &mut egui::Ui) {
    if spans.is_empty() {
        return;
    }

    // Fast path: single plain span
    if spans.len() == 1 {
        let s = &spans[0];
        if !s.text.is_empty() {
            let mut rt = egui::RichText::new(&s.text);
            if s.strong {
                rt = rt.strong();
            }
            if s.em {
                rt = rt.italics();
            }
            if s.code {
                rt = rt.code();
            }
            ui.add(egui::Label::new(rt).wrap());
        }
        spans.clear();
        return;
    }

    // Multi-span: build a LayoutJob so everything lands on the same line.
    let mut job = egui::text::LayoutJob::default();
    let style = ui.style();
    let visuals = &style.visuals;

    for s in spans.iter() {
        if s.text.is_empty() {
            continue;
        }
        let font_id = if s.code {
            egui::FontId::monospace(style.text_styles[&egui::TextStyle::Body].size)
        } else {
            egui::FontId::proportional(style.text_styles[&egui::TextStyle::Body].size)
        };
        let color = if s.code {
            visuals.code_bg_color // will be overridden by text color below
        } else {
            egui::Color32::TRANSPARENT // placeholder; use default
        };
        let _ = color;

        let mut fmt = egui::text::TextFormat {
            font_id,
            color: visuals.text_color(),
            ..Default::default()
        };
        if s.strong {
            fmt.color = visuals.strong_text_color();
        }
        if s.em {
            fmt.italics = true;
        }
        if s.code {
            fmt.background = visuals.code_bg_color;
            fmt.font_id = egui::FontId::monospace(style.text_styles[&egui::TextStyle::Body].size);
        }
        job.append(&s.text, 0.0, fmt);
    }

    ui.add(egui::Label::new(job).wrap());
    spans.clear();
}

/// Render a simple grid table from collected rows.
fn render_table(state: &TableState, ui: &mut egui::Ui) {
    if state.rows.is_empty() {
        return;
    }

    let col_count = state.rows.iter().map(|r| r.len()).max().unwrap_or(0);
    if col_count == 0 {
        return;
    }

    // Compute column widths (character-based heuristic, capped).
    let body_font_size = ui.style().text_styles[&egui::TextStyle::Body].size;
    let char_w = body_font_size * 0.55; // rough monospace char width
    let min_col_w = 60.0_f32;
    let max_col_w = 300.0_f32;

    let col_widths: Vec<f32> = (0..col_count)
        .map(|c| {
            let max_chars = state
                .rows
                .iter()
                .map(|r| r.get(c).map(|s| s.len()).unwrap_or(0))
                .max()
                .unwrap_or(4) as f32;
            (max_chars * char_w + 16.0).clamp(min_col_w, max_col_w)
        })
        .collect();

    use egui_extras::{Column, TableBuilder};

    let mut builder = TableBuilder::new(ui)
        .striped(true)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center));
    for &w in &col_widths {
        builder = builder.column(Column::initial(w).resizable(true));
    }

    // Split header row (first row) from body rows.
    let (header_rows, body_rows) = if state.rows.len() > 1 {
        (&state.rows[0..1], &state.rows[1..])
    } else {
        (&state.rows[0..1], &state.rows[0..0])
    };

    builder
        .header(body_font_size + 8.0, |mut header| {
            for cell in header_rows[0].iter() {
                header.col(|ui| {
                    ui.label(egui::RichText::new(cell).strong());
                });
            }
            // Pad missing columns
            for _ in header_rows[0].len()..col_count {
                header.col(|_ui| {});
            }
        })
        .body(|mut body| {
            for row in body_rows {
                body.row(body_font_size + 6.0, |mut r| {
                    for cell in row.iter() {
                        r.col(|ui| {
                            ui.label(cell);
                        });
                    }
                    for _ in row.len()..col_count {
                        r.col(|_ui| {});
                    }
                });
            }
        });
}

// ── Public API ───────────────────────────────────────────────────────────────

/// Render a markdown string into egui widgets by walking the Event stream.
pub fn render_markdown(text: &str, ui: &mut egui::Ui) {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);

    // ── Inline formatting state ──────────────────────────────────────────
    let mut spans: Vec<Span> = Vec::new();
    let mut current_text = String::new();
    let mut is_strong = false;
    let mut is_em = false;
    let mut heading_level: Option<HeadingLevel> = None;

    // ── Block state ──────────────────────────────────────────────────────
    let mut in_code_block = false;
    let mut code_block_buf = String::new();
    let mut _in_list_item = false;
    let mut list_indent: u32 = 0;

    // ── Table state ──────────────────────────────────────────────────────
    let mut table: Option<TableState> = None;
    let mut in_table_cell = false;

    // Push accumulated `current_text` as a span (does NOT flush to UI).
    macro_rules! push_span {
        () => {
            if !current_text.is_empty() {
                spans.push(Span::new(
                    std::mem::take(&mut current_text),
                    is_strong,
                    is_em,
                    false,
                ));
            }
        };
    }

    for event in Parser::new_ext(text, options) {
        // ── Table cell text accumulation (bypass normal rendering) ───────
        if in_table_cell {
            match &event {
                Event::Text(t) | Event::Code(t) => {
                    if let Some(tbl) = table.as_mut() {
                        tbl.current_cell.push_str(t);
                    }
                    continue;
                }
                Event::End(TagEnd::TableCell) => {
                    if let Some(tbl) = table.as_mut() {
                        let cell = std::mem::take(&mut tbl.current_cell);
                        tbl.current_row.push(cell);
                    }
                    in_table_cell = false;
                    continue;
                }
                // Ignore formatting tags inside cells for simplicity.
                Event::Start(Tag::Strong)
                | Event::End(TagEnd::Strong)
                | Event::Start(Tag::Emphasis)
                | Event::End(TagEnd::Emphasis)
                | Event::Start(Tag::Link { .. })
                | Event::End(TagEnd::Link) => {
                    continue;
                }
                _ => {}
            }
        }

        match event {
            // ── Table ────────────────────────────────────────────────────
            Event::Start(Tag::Table(aligns)) => {
                push_span!();
                flush_spans(&mut spans, ui);
                table = Some(TableState {
                    alignments: aligns,
                    ..Default::default()
                });
            }
            Event::End(TagEnd::Table) => {
                if let Some(tbl) = table.take() {
                    render_table(&tbl, ui);
                }
                ui.add_space(4.0);
            }
            Event::Start(Tag::TableHead) => {
                if let Some(tbl) = table.as_mut() {
                    tbl.in_head = true;
                }
            }
            Event::End(TagEnd::TableHead) => {
                if let Some(tbl) = table.as_mut() {
                    let row = std::mem::take(&mut tbl.current_row);
                    tbl.rows.push(row);
                    tbl.in_head = false;
                }
            }
            Event::Start(Tag::TableRow) => {}
            Event::End(TagEnd::TableRow) => {
                if let Some(tbl) = table.as_mut() {
                    let row = std::mem::take(&mut tbl.current_row);
                    if !row.is_empty() {
                        tbl.rows.push(row);
                    }
                }
            }
            Event::Start(Tag::TableCell) => {
                in_table_cell = true;
            }
            // TableCell End is handled in the bypass block above.

            // ── Headings ─────────────────────────────────────────────────
            Event::Start(Tag::Heading { level, .. }) => {
                push_span!();
                flush_spans(&mut spans, ui);
                heading_level = Some(level);
            }
            Event::End(TagEnd::Heading(_)) => {
                push_span!();
                let text_content: String = spans.iter().map(|s| s.text.as_str()).collect();
                spans.clear();
                if !text_content.is_empty() {
                    let size = match heading_level {
                        Some(HeadingLevel::H1) => 22.0,
                        Some(HeadingLevel::H2) => 18.0,
                        Some(HeadingLevel::H3) => 15.0,
                        _ => 14.0,
                    };
                    ui.add(
                        egui::Label::new(egui::RichText::new(&text_content).size(size).strong())
                            .wrap(),
                    );
                }
                heading_level = None;
                ui.add_space(2.0);
            }

            // ── Code block ───────────────────────────────────────────────
            Event::Start(Tag::CodeBlock(_)) => {
                push_span!();
                flush_spans(&mut spans, ui);
                in_code_block = true;
                code_block_buf.clear();
            }
            Event::End(TagEnd::CodeBlock) => {
                let buf = std::mem::take(&mut code_block_buf);
                egui::Frame::group(ui.style())
                    .fill(ui.visuals().code_bg_color)
                    .show(ui, |ui| {
                        ui.add(egui::Label::new(egui::RichText::new(&buf).code()).wrap());
                    });
                in_code_block = false;
                ui.add_space(4.0);
            }

            // ── Lists ────────────────────────────────────────────────────
            Event::Start(Tag::List(_)) => {
                list_indent += 1;
            }
            Event::End(TagEnd::List(_)) => {
                if list_indent > 0 {
                    list_indent -= 1;
                }
                ui.add_space(4.0);
            }
            Event::Start(Tag::Item) => {
                push_span!();
                flush_spans(&mut spans, ui);
                _in_list_item = true;
            }
            Event::End(TagEnd::Item) => {
                push_span!();
                // Render the list item: bullet + collected spans on the same line
                let indent = (list_indent.saturating_sub(1) as f32) * 16.0 + 8.0;
                ui.horizontal_wrapped(|ui| {
                    ui.add_space(indent);
                    ui.label("•");
                    flush_spans(&mut spans, ui);
                });
                _in_list_item = false;
            }

            // ── Inline formatting ────────────────────────────────────────
            Event::Start(Tag::Strong) => {
                push_span!();
                is_strong = true;
            }
            Event::End(TagEnd::Strong) => {
                push_span!();
                is_strong = false;
            }
            Event::Start(Tag::Emphasis) => {
                push_span!();
                is_em = true;
            }
            Event::End(TagEnd::Emphasis) => {
                push_span!();
                is_em = false;
            }

            // ── Paragraph ────────────────────────────────────────────────
            Event::Start(Tag::Paragraph) => {
                push_span!();
                flush_spans(&mut spans, ui);
            }
            Event::End(TagEnd::Paragraph) => {
                push_span!();
                flush_spans(&mut spans, ui);
                ui.add_space(4.0);
            }

            // ── Leaf events ──────────────────────────────────────────────
            Event::Text(t) => {
                if in_code_block {
                    code_block_buf.push_str(&t);
                } else {
                    current_text.push_str(&t);
                }
            }
            Event::Code(t) => {
                // Inline code: flush pending text first, then push a code span.
                push_span!();
                spans.push(Span::new(t.as_ref(), false, false, true));
            }
            Event::SoftBreak => {
                // Treat soft break as a newline to preserve original line breaks
                // in plain text content.
                push_span!();
                flush_spans(&mut spans, ui);
            }
            Event::HardBreak => {
                push_span!();
                flush_spans(&mut spans, ui);
            }
            Event::Rule => {
                push_span!();
                flush_spans(&mut spans, ui);
                ui.separator();
            }

            // ── Catch-all ────────────────────────────────────────────────
            Event::Start(_) => {}
            Event::End(_) => {
                push_span!();
                flush_spans(&mut spans, ui);
            }
            _ => {}
        }
    }

    // Flush any remaining content.
    push_span!();
    flush_spans(&mut spans, ui);
}
