use std::ops::{Add};

use null_space::GameCalender;
use ratatui::{
    Frame,
    layout::{
        Alignment,
        Constraint,
        Layout,
        Rect
    },
    style::{
        Color,
        Modifier, Style
    },
    text::{
        Line,
        Span
    },
    widgets::{
        Block, Gauge, List, ListState, Paragraph
    }
};

use crate::app::{
    App,
    AppState
};
use crate::game::{
    GameState
};

pub fn render(frame: &mut Frame, app: &App) {
    render_container(frame, app, frame.area());
}
fn render_container(frame: &mut Frame, app: &App, area: Rect) {    
    match app.state {
        AppState::MainScreen => {
            let horizontal = Layout::horizontal([
                Constraint::Percentage(20),
                Constraint::Percentage(60),
                Constraint::Percentage(20)
            ]);
            let [left, center, right] = area.layout(&horizontal);

            _render_left(frame, app, left);
            render_center(frame, app, center);
            _render_right(frame, app, right);
        },
        AppState::Playing => {
            let vertical = Layout::vertical([
                Constraint::Max(5),
                Constraint::Fill(1),
                Constraint::Percentage(20)
            ]);
            let [header, main, footer] = area.layout(&vertical);
            render_header(frame, app, header);
            render_main(frame, app, main);
            render_footer(frame, app, footer);
        }
    }
}
fn _render_left(_frame: &mut Frame, _app: &App, _area: Rect) {

}
fn render_center(frame: &mut Frame, app: &App, area: Rect) {
    let vertical = Layout::vertical([
        Constraint::Percentage(30),
        Constraint::Percentage(70)
    ]);
    let [head, select] = area.layout(&vertical);

    render_head(frame, app, head);
    render_select(frame, app, select);
}
fn render_head(frame: &mut Frame, app: &App, area: Rect) {
    let title = Paragraph::new(app.title).centered();
    frame.render_widget(title, area);
}
fn render_select(frame: &mut Frame, app: &App, area: Rect) {
    let list = List::new(app.select_items.clone())
        .style(Color::White)
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol("> ");
    let mut list_state = ListState::default().with_selected(Some(app.select_item));
    frame.render_stateful_widget(list, area, &mut list_state);
}
fn _render_right(_frame: &mut Frame, _app: &App, _area: Rect) {

}
fn render_header(frame: &mut Frame, app: &App, area: Rect) {
    let horizontal = Layout::horizontal([
        Constraint::Percentage(30),
        Constraint::Percentage(40),
        Constraint::Percentage(30),
    ]);
    let [ left, center, right ] = area.layout(&horizontal);
    render_header_left(frame, app, left);
    render_header_center(frame, app, center);
    render_header_right(frame, app, right);
}
fn render_header_left(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::bordered();
    if let Some(game) = &app.game {
        let paragraph = Paragraph::new(Line::from(vec![
            Span::from(fmt_year(&game.time, 12)),
            Span::from(String::from(".")),
            Span::from(fmt_num(game.time.month() + 1)),
            Span::from(String::from(".")),
            Span::from(fmt_num(game.time.day() + 1)),
            Span::from(String::from(".")),
            Span::from(fmt_num(game.time.hour())),
            Span::from(String::from(":")),
            Span::from(fmt_num(game.time.minute())),
            Span::from(String::from(":")),
            Span::from(fmt_num(game.time.second()))
        ])).block(block);
        frame.render_widget(paragraph, area);
    } else {
        frame.render_widget(block, area);
    }
}
fn render_header_center(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::bordered().title(
    if let Some(game) = &app.game {
        match game.state {
            GameState::Infomation => {
                "Infomation"
            }
            GameState::Production => {
                "Production"
            }
            GameState::PowerSavingMode => {
                "Power Saving Mode"
            }
        }
    } else {"title"}).title_alignment(Alignment::Center);
    frame.render_widget(block, area);
}
fn render_header_right(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::bordered();
    let gauge = Gauge::default()
        .block(block)
        .gauge_style(Style::new().blue().on_black())
        .percent(20);
    frame.render_widget(gauge, area);
}
fn fmt_num(num: u64) -> String {
    if num / 10 > 0 {
        num.to_string()
    } else {
        String::from("0").add(&num.to_string())
    }
}
fn render_main(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::bordered();
    frame.render_widget(block, area);
}
fn render_footer(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::bordered()
        .title_bottom("footer")
        .title_alignment(Alignment::Center);
    let paragraph = Paragraph::new(app.num.to_string())
        .block(block);
    frame.render_widget(paragraph, area);
}
pub fn fmt_year(cal: &GameCalender, size: usize) -> String {
    if cal.year().len() <= size {
        year_to_str(cal)
    } else {
        year_to_e_str(cal, 3)
    }
}
pub fn year_to_str(cal: &GameCalender) -> String {
    let year_clone = cal.year().clone();
    match year_clone
            .chunks(3)
            .rev()
            .map(|n| match n.iter()
                .rev()
                .map(|&u| u.to_string())
                .reduce(|s1, s2| s1.add(&s2)) {
                    Some(result) => {
                        result
                    },
                    None => {
                        String::from("")
                    }
                })
            .reduce(|s1, s2| s1.add(",").add(&s2)) {
        Some(result) => {
            result
        },
        None => {
            String::from("0")
        }
    }
}
fn year_to_e_str(cal: &GameCalender, significand_size: usize) -> String {
    let first = cal.year()[cal.year().len() - 1].to_string();
    let mut result = first.add(".");
    for i in 1..cal.year().len() {
        if i > significand_size {
            break;
        }
        result = result.add(&cal.year()[cal.year().len() - 1 -i].to_string());
    }
    result = result.add("e");
    result = result.add(&(cal.year().len() - 1).to_string());
    result
}