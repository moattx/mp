use ratatui::prelude::Modifier;
use ratatui::widgets::Gauge;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::App;
use crate::audio::get_location;

pub fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            //Constraint::Length(3),
            //Constraint::Min(1),
            //Constraint::Length(5),
            Constraint::Percentage(10),
            Constraint::Percentage(80),
            Constraint::Percentage(10),
        ])
        .split(f.size());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Create New Json",
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    f.render_widget(title, chunks[0]);

    let playlist_items = app.playlist_items.clone();
    let playlist_list = List::new(playlist_items)
        .block(Block::default().title("List").borders(Borders::ALL))
        .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true);

    // OR: if mode == songs?
    if app.divide_list {
        let sub_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[1]);

        f.render_stateful_widget(playlist_list, sub_chunks[0], &mut app.playlist);

        let app_items = app.song_items.clone();
        let app_list = List::new(app_items)
            .block(Block::default().title("List").borders(Borders::ALL))
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true);

        f.render_stateful_widget(app_list, sub_chunks[1], &mut app.song);
    } else {
        f.render_stateful_widget(playlist_list, chunks[1], &mut app.playlist);
    }

    let mixer = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Progress"))
        .gauge_style(
            Style::default()
                .fg(Color::White)
                .bg(Color::Black)
                .add_modifier(Modifier::ITALIC),
        )
        .percent(get_location());
    f.render_widget(mixer, chunks[2]);
}
