use ratatui::{
    Frame, 
    layout::{
        Alignment, Constraint, Layout, Rect
    }, 
    widgets::{Block, List, Paragraph}
};

use crate::app::{
    App,
    AppState
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

            render_left(frame, app, left);
            render_center(frame, app, center);
            render_right(frame, app, right);
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
fn render_left(frame: &mut Frame, app: &App, area: Rect) {

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
    let title = Paragraph::new("Null Space").centered();
    frame.render_widget(title, area);
}
fn render_select(frame: &mut Frame, app: &App, area: Rect) {
    let list = List::new([
        "start",
        "exit"
    ]);
    frame.render_widget(list, area);
}
fn render_right(frame: &mut Frame, app: &App, area: Rect) {

}
fn render_header(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::bordered()
        .title("title")
        .title_alignment(Alignment::Center);
    frame.render_widget(block, area);
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