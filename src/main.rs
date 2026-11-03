use clap::Parser;
use input::input;

#[derive(Parser)]
struct Cli;

fn main() -> anyhow::Result<()> {
    let config = input::Config::default();
    let mut component = input::new(config);
    component.initialize()?;
    println!("SigmaOS-input ready");
    Ok(())
}
