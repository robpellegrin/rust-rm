///
/// =====================================================================
/// Project Name: rust rm
/// Description: An enhanced version of the common rm utility.
/// Author: Robert Pellegrin
/// Date: 2025-05-17
/// Version: 0.0.1
/// License: MIT
/// Repository:
/// =====================================================================
///
use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
};

use color_eyre::Result;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Margin, Rect},
    style::{self, Color, Modifier, Style, Stylize},
    text::Text,
    widgets::{
        Block, BorderType, Cell, HighlightSpacing, Paragraph, Row, Scrollbar, ScrollbarOrientation,
        ScrollbarState, Table, TableState,
    },
};

use style::palette::tailwind;
use unicode_width::UnicodeWidthStr;

const PALETTES: [tailwind::Palette; 4] = [
    tailwind::BLUE,
    tailwind::EMERALD,
    tailwind::INDIGO,
    tailwind::RED,
];
const INFO_TEXT: [&str; 2] = [
    "(Esc) quit | (↑) move up | (↓) move down | (←) move left | (→) move right",
    "(d) delete | (r) restore | (e) empty trash",
];

const ITEM_HEIGHT: usize = 4;

struct TableColors {
    buffer_bg: Color,
    header_bg: Color,
    header_fg: Color,
    row_fg: Color,
    selected_row_style_fg: Color,
    selected_column_style_fg: Color,
    selected_cell_style_fg: Color,
    normal_row_color: Color,
    alt_row_color: Color,
    footer_border_color: Color,
}

impl TableColors {
    const fn new(color: &tailwind::Palette) -> Self {
        Self {
            buffer_bg: tailwind::SLATE.c950,
            header_bg: color.c900,
            header_fg: tailwind::SLATE.c200,
            row_fg: tailwind::SLATE.c200,
            selected_row_style_fg: color.c400,
            selected_column_style_fg: color.c400,
            selected_cell_style_fg: color.c600,
            normal_row_color: tailwind::SLATE.c950,
            alt_row_color: tailwind::SLATE.c900,
            footer_border_color: color.c400,
        }
    }
}

struct TrashEntry {
    file: String,
    path: String,
    date: String,
}

impl TrashEntry {
    const fn ref_array(&self) -> [&String; 3] {
        [&self.file, &self.path, &self.date]
    }

    fn file(&self) -> &str {
        &self.file
    }

    fn path(&self) -> &str {
        &self.path
    }

    fn date(&self) -> &str {
        &self.date
    }
}

struct App {
    state: TableState,
    items: Vec<TrashEntry>,
    longest_item_lens: (u16, u16, u16), // order is (file, path, date)
    scroll_state: ScrollbarState,
    colors: TableColors,
    color_index: usize,
}

impl App {
    fn new() -> Self {
        let data_vec = get_trash_info();
        Self {
            state: TableState::default().with_selected(0),
            longest_item_lens: constraint_len_calculator(&data_vec),
            scroll_state: ScrollbarState::new((data_vec.len() - 1) * ITEM_HEIGHT),
            colors: TableColors::new(&PALETTES[0]),
            color_index: 0,
            items: data_vec,
        }
    }

    pub fn next_row(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn previous_row(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn next_column(&mut self) {
        self.state.select_next_column();
    }

    pub fn previous_column(&mut self) {
        self.state.select_previous_column();
    }

    pub fn next_color(&mut self) {
        self.color_index = (self.color_index + 1) % PALETTES.len();
    }

    pub fn previous_color(&mut self) {
        let count = PALETTES.len();
        self.color_index = (self.color_index + count - 1) % count;
    }

    pub fn set_colors(&mut self) {
        self.colors = TableColors::new(&PALETTES[self.color_index]);
    }

    // TODO
    pub fn delete_selection(&mut self) {
        self.items.pop();
    }

    // TOOD
    pub fn restore_selection(&mut self) {
        self.items.pop();
    }

    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    let shift_pressed = key
                        .modifiers
                        .contains(ratatui::crossterm::event::KeyModifiers::SHIFT);
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Char('j') | KeyCode::Down => self.next_row(),
                        KeyCode::Char('k') | KeyCode::Up => self.previous_row(),
                        KeyCode::Char('l') | KeyCode::Right if shift_pressed => self.next_color(),
                        KeyCode::Char('h') | KeyCode::Left if shift_pressed => {
                            self.previous_color();
                        }
                        KeyCode::Char('d') => self.delete_selection(),
                        KeyCode::Char('r') => self.restore_selection(),
                        KeyCode::Char('l') | KeyCode::Right => self.next_column(),
                        KeyCode::Char('h') | KeyCode::Left => self.previous_column(),
                        _ => {}
                    }
                }
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let vertical = &Layout::vertical([Constraint::Min(5), Constraint::Length(4)]);
        let rects = vertical.split(frame.area());

        self.set_colors();

        self.render_table(frame, rects[0]);
        self.render_scrollbar(frame, rects[0]);
        self.render_footer(frame, rects[1]);
    }

    fn render_table(&mut self, frame: &mut Frame, area: Rect) {
        let header_style = Style::default()
            .fg(self.colors.header_fg)
            .bg(self.colors.header_bg);
        let selected_row_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(self.colors.selected_row_style_fg);
        let selected_col_style = Style::default().fg(self.colors.selected_column_style_fg);
        let selected_cell_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(self.colors.selected_cell_style_fg);

        let header = ["File Name", "Original Path", "Date"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(header_style)
            .height(1);
        let rows = self.items.iter().enumerate().map(|(i, data)| {
            let color = match i % 2 {
                0 => self.colors.normal_row_color,
                _ => self.colors.alt_row_color,
            };
            let item = data.ref_array();
            item.into_iter()
                .map(|content| Cell::from(Text::from(format!("\n{content}\n"))))
                .collect::<Row>()
                .style(Style::new().fg(self.colors.row_fg).bg(color))
                .height(2)
        });
        let bar = " █ ";
        let t = Table::new(
            rows,
            [
                // + 1 is for padding.
                Constraint::Length(self.longest_item_lens.0 + 1),
                Constraint::Min(self.longest_item_lens.1 + 1),
                Constraint::Min(self.longest_item_lens.2),
            ],
        )
        .header(header)
        .row_highlight_style(selected_row_style)
        .column_highlight_style(selected_col_style)
        .cell_highlight_style(selected_cell_style)
        .highlight_symbol(Text::from(vec![
            "".into(),
            bar.into(),
            bar.into(),
            "".into(),
        ]))
        .bg(self.colors.buffer_bg)
        .highlight_spacing(HighlightSpacing::Always);
        frame.render_stateful_widget(t, area, &mut self.state);
    }

    fn render_scrollbar(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_stateful_widget(
            Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(None)
                .end_symbol(None),
            area.inner(Margin {
                vertical: 1,
                horizontal: 1,
            }),
            &mut self.scroll_state,
        );
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        let info_footer = Paragraph::new(Text::from_iter(INFO_TEXT))
            .style(
                Style::new()
                    .fg(self.colors.row_fg)
                    .bg(self.colors.buffer_bg),
            )
            .centered()
            .block(
                Block::bordered()
                    .border_type(BorderType::Double)
                    .border_style(Style::new().fg(self.colors.footer_border_color)),
            );
        frame.render_widget(info_footer, area);
    }
}

fn get_trash_info() -> Vec<TrashEntry> {
    let home_dir = dirs_next::home_dir().expect("Failed to get home directory");
    let trash_dir = home_dir.join(".local/share/Trash/files");

    if !trash_dir.exists() {
        println!("Trash directory does not exist at {:?}", trash_dir);
        return Vec::new();
    }

    let mut entries: Vec<TrashEntry> = Vec::new();

    match fs::read_dir(&trash_dir) {
        Ok(dir_entries) => {
            for entry in dir_entries {
                if let Ok(entry) = entry {
                    let file_name = entry.file_name().to_string_lossy().to_string();
                    let original_path = get_info_from_trashinfo(&file_name, "Path=")
                        .unwrap_or_else(|| "Unknown".to_string());
                    let date_info = get_info_from_trashinfo(&file_name, "DeletionDate=")
                        .unwrap_or_else(|| "Unknown".to_string());

                    entries.push(TrashEntry {
                        file: file_name,
                        path: original_path,
                        date: date_info,
                    });
                }
            }
        }
        Err(_) => {}
    }
    return entries;
}

fn constraint_len_calculator(items: &[TrashEntry]) -> (u16, u16, u16) {
    let name_len = items
        .iter()
        .map(TrashEntry::file)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);
    let path_len = items
        .iter()
        .map(TrashEntry::path)
        .flat_map(str::lines)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);
    let date_len = items
        .iter()
        .map(TrashEntry::date)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);

    #[allow(clippy::cast_possible_truncation)]
    (name_len as u16, path_len as u16, date_len as u16)
}

pub fn list_trash_contents_table() {
    let home_dir = dirs_next::home_dir().expect("Failed to get home directory");
    let trash_dir = home_dir.join(".local/share/Trash/files");

    if !trash_dir.exists() {
        println!("Trash directory does not exist at {:?}", trash_dir);
        return;
    }

    let mut entries: Vec<TrashEntry> = Vec::new();

    match fs::read_dir(&trash_dir) {
        Ok(dir_entries) => {
            let mut has_files = false;

            for entry in dir_entries {
                if let Ok(entry) = entry {
                    let file_name = entry.file_name().to_string_lossy().to_string();
                    let original_path = get_info_from_trashinfo(&file_name, "Path=")
                        .unwrap_or_else(|| "Unknown".to_string());
                    let date_info = get_info_from_trashinfo(&file_name, "DeletionDate=")
                        .unwrap_or_else(|| "Unknown".to_string());

                    entries.push(TrashEntry {
                        file: file_name,
                        path: original_path,
                        date: date_info,
                    });
                    has_files = true;
                }
            }

            if has_files {
                match color_eyre::install() {
                    Ok(it) => it,
                    Err(_) => {}
                };

                let terminal = ratatui::init();
                let _ = App::new().run(terminal);
                ratatui::restore();
            } else {
                println!("The trash is empty.");
            }
        }

        Err(e) => {
            eprintln!("Failed to read the trash directory: {}", e);
        }
    }
}

/// Extracts info from .trashinfo file. i.e. path or datetime.
fn get_info_from_trashinfo(file_name: &str, search_term: &str) -> Option<String> {
    let home_dir = dirs_next::home_dir()?;
    let info_path = home_dir
        .join(".local/share/Trash/info")
        .join(format!("{}.trashinfo", file_name));

    if !info_path.exists() {
        return None;
    }

    let file = File::open(info_path).ok()?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            if line.starts_with(search_term) {
                return Some(line[search_term.len()..].trim().to_string());
            }
        }
    }

    None
}
