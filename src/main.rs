use dsa_viz::{tui::App, Graph};


fn main() -> color_eyre::Result<()> {
    let g  = Graph::new(8, 10);
    let events = g.bfs();
    color_eyre::install()?;
    ratatui::run(|terminal| App::new(g, events).run(terminal))?;
    Ok(())
}

