use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::Line,
    widgets::{
        Block, BorderType, HighlightSpacing, List, ListItem, ListState, Padding, StatefulWidget,
        Widget,
    },
};

use crate::models::Service;

#[derive(Debug, Default)]
pub struct Home {
    pub filter: String,
    pub services: ServiceList,
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
    // Delete(Service),
    Copy(String),
    Help,
    Quit,
    None,
}

impl Home {
    pub fn handle_inputs(&mut self, event: KeyEvent) -> HomeAction {
        match event.code {
            KeyCode::Esc | KeyCode::Char('q') => HomeAction::Quit,
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
                    let selected = self
                        .services
                        .state
                        .selected()
                        .expect("No service is selected.");
                    let service = self
                        .services
                        .list
                        .get(selected)
                        .expect("Index not found in list.");
                    HomeAction::Edit(service.clone())
                }
            }
            KeyCode::Char('n') => {
                let service = Service::default();
                HomeAction::Edit(service)
            }
            // KeyCode::Char('\\') => self.mode = Mode::Cuts,
            KeyCode::Enter => {
                if self.services.list.is_empty() {
                    HomeAction::None
                } else {
                    let selected = self
                        .services
                        .state
                        .selected()
                        .expect("No service is selected.");
                    let service = self
                        .services
                        .list
                        .get(selected)
                        .expect("Index not found in list");
                    HomeAction::View(service.clone())
                }
            }
            KeyCode::Char('y') => {
                if self.services.list.is_empty() {
                    HomeAction::None
                } else {
                    let selected = self
                        .services
                        .state
                        .selected()
                        .expect("No service is selected.");
                    let service = self
                        .services
                        .list
                        .get(selected)
                        .expect("Index not found in list.");
                    HomeAction::Copy(service.name.clone())
                }
            }
            _ => HomeAction::None,
        }
    }

    pub fn new(list: Vec<Service>) -> Self {
        let mut services = ServiceList {
            list,
            state: ListState::default(),
        };
        services.state.select_first();
        Self {
            filter: String::new(),
            services,
        }
    }

    fn construct_service_list(&self) -> List<'static> {
        let list_items: Vec<ListItem> = self
            .services
            .list
            .iter()
            .filter(|service| service.name.contains(&self.filter))
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

        if self.services.list.is_empty() {
            Widget::render(
                construct_empty_services_alert(),
                Block::inner(&block, area),
                buf,
            );
        } else {
            let list = self.construct_service_list();
            StatefulWidget::render(
                list,
                Block::inner(&block, area),
                buf,
                &mut self.services.state,
            );
        }

        block.render(area, buf);
    }
}
