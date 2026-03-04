use dsa_viz::{Graph};


fn main() -> color_eyre::Result<()> {
    let g  = Graph::new(2, 2);
    let events = g.dfs();
    for event in events {
        println!("{:?}", event);
    }
    /*color_eyre::install()?;
    ratatui::run(|terminal| App::new().run(terminal))?;*/
    Ok(())
}

