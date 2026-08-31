use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, BorderType, Borders, HighlightSpacing, List, ListItem, ListState, Padding,
        Paragraph, StatefulWidget, Widget,
    },
};

use crate::models::Service;

#[derive(Debug, Default)]
pub struct Home {
    pub filter: String,
    pub services: ServiceList,
    set_filter: bool,
}

#[derive(Debug, Default)]
pub struct ServiceList {
    pub list: Vec<Service>,
    pub state: ListState,
}

#[derive(Debug)]
pub enum HomeAction {
    Edit(Service),
    View(Service),
    Delete(Service),
    Copy(String),
    Help,
    Quit,
    None,
}

impl Home {
    pub fn new(list: Vec<Service>) -> Self {
        let mut services = ServiceList {
            list,
            state: ListState::default(),
        };
        services.state.select_first();
        Self {
            filter: String::new(),
            services,
            set_filter: false,
        }
    }

    pub fn handle_inputs(&mut self, event: KeyEvent) -> HomeAction {
        if self.set_filter {
            return self.handle_filter_inputs(event);
        }

        match event.code {
            KeyCode::Esc => {
                if self.filter.is_empty() {
                    HomeAction::Quit
                } else {
                    self.filter.clear();
                    HomeAction::None
                }
            }
            KeyCode::Char('q') => HomeAction::Quit,
            KeyCode::Char('h' | '?') => HomeAction::Help,
            KeyCode::Char('j') | KeyCode::Down => {
                self.services.state.select_next();
                HomeAction::None
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.services.state.select_previous();
                HomeAction::None
            }
            KeyCode::Char('e') => {
                if self.services.list.is_empty() {
                    HomeAction::None
                } else {
                    let service = self.get_selected_service();
                    HomeAction::Edit(service.clone())
                }
            }
            KeyCode::Char('n') => HomeAction::Edit(Service::default()),
            // KeyCode::Char('\\') => self.mode = Mode::Cuts,
            KeyCode::Char('/') => {
                if !self.services.list.is_empty() {
                    self.set_filter = true;
                }
                HomeAction::None
            }
            KeyCode::Enter | KeyCode::Char('l') => {
                if self.services.list.is_empty() {
                    HomeAction::None
                } else {
                    let service = self.get_selected_service();
                    HomeAction::View(service.clone())
                }
            }
            KeyCode::Char('y') => {
                if self.services.list.is_empty() {
                    HomeAction::None
                } else {
                    let service = self.get_selected_service();
                    HomeAction::Copy(service.name.clone())
                }
            }
            KeyCode::Delete => {
                let service = self.get_selected_service();
                HomeAction::Delete(service.clone())
            }
            _ => HomeAction::None,
        }
    }

    fn handle_filter_inputs(&mut self, event: KeyEvent) -> HomeAction {
        match event.code {
            KeyCode::Esc => {
                self.filter = String::new();
                self.set_filter = false;
                HomeAction::None
            }
            KeyCode::Char(ch) => {
                self.filter.push(ch);
                HomeAction::None
            }
            KeyCode::Backspace => {
                self.filter.pop();
                HomeAction::None
            }
            KeyCode::Enter => {
                self.set_filter = false;
                HomeAction::None
            }
            _ => HomeAction::None,
        }
    }

    fn construct_service_list(&self) -> List<'static> {
        let list_items: Vec<ListItem> = self
            .services
            .list
            .iter()
            .filter(|service| {
                service
                    .name
                    .to_lowercase()
                    .contains(&self.filter.to_lowercase())
            })
            .map(|service| ListItem::new(Line::from(service.name.clone())))
            .collect();

        List::new(list_items)
            .highlight_symbol(" > ")
            .highlight_style(
                Style::new()
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::REVERSED),
            )
            .highlight_spacing(HighlightSpacing::Always)
    }

    fn render_filter(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .border_type(BorderType::LightQuadrupleDashed)
            .borders(Borders::BOTTOM);

        let cursor_style = if self.set_filter {
            Style::reversed(Style::default())
        } else {
            Style::default()
        };

        let line = Line::from(vec![
            Span::raw("/: "),
            Span::raw(self.filter.clone()),
            Span::styled(" ", cursor_style),
        ]);

        let text = Paragraph::new(line).block(block);

        Widget::render(text, area, buf);
    }

    fn get_selected_service(&self) -> &Service {
        let selected = self
            .services
            .state
            .selected()
            .expect("No service is selected.");
        let filtered_list = self
            .services
            .list
            .iter()
            .filter(|s| s.name.to_lowercase().contains(&self.filter.to_lowercase()))
            .collect::<Vec<_>>();

        filtered_list
            .get(selected)
            .expect("Index not found in list")
    }
}

fn construct_empty_services_alert() -> List<'static> {
    let lines = vec![
        Line::from("No services found in the database"),
        Line::from("Press 'n' to add a new one"),
    ];

    List::new(lines)
}

impl Widget for &mut Home {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let title = Line::from(" Home Mode ");
        let block = Block::bordered()
            .title(title)
            .padding(Padding::uniform(1))
            .border_type(BorderType::Rounded);

        let filter_height = match (self.set_filter, self.filter.is_empty()) {
            (true, _) | (_, false) => 2,
            (_, _) => 0,
        };

        let layout = Layout::default()
            .constraints(vec![Constraint::Length(filter_height), Constraint::Fill(1)]);

        let [header, body] = Layout::areas(&layout, Block::inner(&block, area));

        match (self.set_filter, self.filter.is_empty()) {
            (true, _) | (_, false) => self.render_filter(header, buf),
            _ => {}
        }

        if self.services.list.is_empty() {
            Widget::render(construct_empty_services_alert(), body, buf);
        } else {
            let list = self.construct_service_list();
            StatefulWidget::render(list, body, buf, &mut self.services.state);
        }

        block.render(area, buf);
    }
}
