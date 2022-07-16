use crossterm::{
    event::{self,  DisableMouseCapture, EnableMouseCapture, Event, KeyCode }, 
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Layout, Direction, Constraint},
    //style::{Color, Modifier, Style},
    widgets::{Block,  Borders, Paragraph},
    Frame, Terminal
}; 

mod ui;
use crate::ui::{
        Menu,GraphBox
};

use std::{
    error::Error,
    io,
};
mod app;
use crate::app::App;

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let menu_items = vec![vec![
        String::from("Plot"),
        String::from("logx") ,
        String::from("logy") 
    ], 
    vec![
        String::from("load"),
        String::from("save"),
        String::from("delete")
        ]
    ];
    let main_menu = Menu::new(
           &menu_items, 
        );
    let app = App::new(
        String::from("Terminal_graph"),
        main_menu,
        );
    let res = run_app(&mut terminal,app);
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Char('h') => {app.menu.move_left(); continue},
                KeyCode::Char('j') => {app.menu.move_down(); continue},
                KeyCode::Char('k') => {app.menu.move_up(); continue},
                KeyCode::Char('l') => {app.menu.move_right(); continue},
                _ => {}
            }
        }
    }
}

fn ui<B: Backend> (f: &mut Frame<B>, app: &mut app::App) { 
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(5)
        .constraints(
            [
                Constraint::Percentage(75),
                Constraint::Percentage(25),
            ]
            )
            .split(size);

    let graph_block = Block::default().borders(Borders::ALL);
    let graph = Paragraph::new("Graph_placeholder").block(graph_block).alignment(Alignment::Center);
    f.render_widget(graph,chunks[0]);

    app.menu.ui(f,chunks[1]);

    
    //f.render_widget(app_block,size);
}

