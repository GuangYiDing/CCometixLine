use crate::config::SegmentId;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

#[derive(Debug, Clone)]
pub struct OptionItem {
    pub key: String,
    pub value: String,
    pub value_type: ValueType,
}

#[derive(Debug, Clone)]
pub enum ValueType {
    String,
    Path,
    Number(u64),
    Enum(Vec<String>),
}

#[derive(Default)]
pub struct OptionsEditorComponent {
    pub is_open: bool,
    pub options: Vec<OptionItem>,
    pub selected_index: usize,
    pub current_input: String,
    pub editing_value: bool,
    pub title: String,
    pub segment_id: Option<SegmentId>,
}

impl OptionsEditorComponent {
    pub fn new() -> Self {
        Self {
            is_open: false,
            options: Vec::new(),
            selected_index: 0,
            current_input: String::new(),
            editing_value: false,
            title: "Options Editor".to_string(),
            segment_id: None,
        }
    }

    pub fn open(&mut self, segment_id: SegmentId, options: &[(String, serde_json::Value)]) {
        self.is_open = true;
        self.segment_id = Some(segment_id);
        self.selected_index = 0;
        self.editing_value = false;
        self.current_input.clear();

        self.options = options
            .iter()
            .map(|(key, value)| {
                let value_str = match value {
                    serde_json::Value::String(s) => s.clone(),
                    serde_json::Value::Number(n) => n.to_string(),
                    serde_json::Value::Bool(b) => b.to_string(),
                    serde_json::Value::Null => "".to_string(),
                    _ => value.to_string(),
                };

                let value_type = match key.as_str() {
                    "output_format" => ValueType::Enum(vec![
                        "text".to_string(),
                        "json".to_string(),
                        "key_value".to_string(),
                    ]),
                    "cache_duration" | "timeout" => {
                        if let Some(n) = value.as_u64() {
                            ValueType::Number(n)
                        } else {
                            ValueType::Number(0)
                        }
                    }
                    "custom_script_path" => ValueType::Path,
                    _ => ValueType::String,
                };

                OptionItem {
                    key: key.clone(),
                    value: value_str,
                    value_type,
                }
            })
            .collect();

        self.title = format!(
            "Edit Options: {}",
            match segment_id {
                SegmentId::SubscriptionQuota => "Subscription Quota",
                _ => "Segment",
            }
        );
    }

    pub fn close(&mut self) {
        self.is_open = false;
        self.options.clear();
        self.selected_index = 0;
        self.current_input.clear();
        self.editing_value = false;
    }

    pub fn move_selection(&mut self, delta: isize) {
        if self.options.is_empty() {
            return;
        }
        let new_index = (self.selected_index as isize + delta)
            .clamp(0, (self.options.len() - 1) as isize) as usize;
        self.selected_index = new_index;
    }

    pub fn start_editing(&mut self) {
        if !self.options.is_empty() {
            self.editing_value = true;
            self.current_input = self.options[self.selected_index].value.clone();
        }
    }

    pub fn stop_editing(&mut self, save: bool) -> Option<Vec<(String, String)>> {
        if !self.editing_value {
            self.close();
            return None;
        }

        self.editing_value = false;

        if save && !self.current_input.trim().is_empty() {
            self.options[self.selected_index].value = self.current_input.clone();
        }

        self.current_input.clear();

        if !save {
            self.close();
            return None;
        }

        // Return modified options
        let modified = self
            .options
            .iter()
            .map(|opt| (opt.key.clone(), opt.value.clone()))
            .collect();

        self.close();
        Some(modified)
    }

    pub fn input_char(&mut self, c: char) {
        if !self.editing_value {
            return;
        }

        // Allow all printable characters for script paths
        if c.is_ascii_graphic() || c == ' ' || c == '/' || c == '\\' || c == ':' || c == '.' || c == '_' || c == '-' {
            self.current_input.push(c);
        }
    }

    pub fn backspace(&mut self) {
        if self.editing_value {
            self.current_input.pop();
        }
    }

    pub fn render(&self, f: &mut Frame, area: Rect) {
        if !self.is_open {
            return;
        }

        let popup_width = 70_u16.min(area.width.saturating_sub(4));
        let popup_height = 20_u16.min(area.height.saturating_sub(4));

        let popup_area = Rect {
            x: (area.width - popup_width) / 2,
            y: (area.height - popup_height) / 2,
            width: popup_width,
            height: popup_height,
        };

        f.render_widget(Clear, popup_area);

        let popup_block = Block::default()
            .borders(Borders::ALL)
            .title(self.title.as_str());
        let inner = popup_block.inner(popup_area);
        f.render_widget(popup_block, popup_area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Header
                Constraint::Length(1), // Selected option
                Constraint::Min(10),   // Options list
                Constraint::Length(3), // Instructions
            ])
            .split(inner);

        // Header
        f.render_widget(
            Paragraph::new("Use ↑/↓ to select, Enter to edit, Esc to save/cancel"),
            chunks[0],
        );

        // Selected option details
        if !self.options.is_empty() {
            let selected = &self.options[self.selected_index];
            let detail = if self.editing_value {
                format!("> {}: {} <", selected.key, self.current_input)
            } else {
                format!("  {}: {}", selected.key, selected.value)
            };

            f.render_widget(
                Paragraph::new(detail).style(if self.editing_value {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default()
                }),
                chunks[1],
            );
        }

        // Options list
        if !self.options.is_empty() {
            let items: Vec<Line> = self
                .options
                .iter()
                .enumerate()
                .map(|(i, opt)| {
                    let is_selected = i == self.selected_index;
                    let prefix = if is_selected { "▶ " } else { "  " };
                    let value_preview = if opt.value.len() > 30 {
                        format!("{}...", &opt.value[..30])
                    } else {
                        opt.value.clone()
                    };

                    let content = format!("{}{}: {}", prefix, opt.key, value_preview);
                    if is_selected {
                        Line::from(Span::styled(content, Style::default().fg(Color::Cyan)))
                    } else {
                        Line::from(content)
                    }
                })
                .collect();

            f.render_widget(
                Paragraph::new(items).block(Block::default().borders(Borders::ALL).title("Options")),
                chunks[2],
            );
        }

        // Instructions
        let instructions = if self.editing_value {
            "[Enter] Save  [Esc] Cancel  [Backspace] Delete"
        } else {
            "[↑/↓] Navigate  [Enter] Edit  [Esc] Close"
        };
        f.render_widget(
            Paragraph::new(instructions).block(Block::default().borders(Borders::ALL)),
            chunks[3],
        );
    }
}
