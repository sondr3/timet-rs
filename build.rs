use std::{
    env,
    fs::File,
    io::Error,
    path::{Path, PathBuf},
};

use clap::{CommandFactory, ValueEnum};
use clap_complete::generate_to;
use clap_mangen::Man;

include!("src/cli.rs");

fn build_shell_completion(out_dir: &Path) -> Result<(), Error> {
    let mut app = Cli::command();
    let shells = Shell::value_variants();

    for shell in shells {
        generate_to(*shell, &mut app, "timet", out_dir)?;
    }

    Ok(())
}

fn build_manpages(outdir: &Path) -> Result<(), Error> {
    let app = Cli::command();

    let file = outdir.join("timet.1");
    let mut file = File::create(file)?;

    Man::new(app).render(&mut file)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    let out_path = PathBuf::from(out_dir);
    let mut path = out_path.ancestors().nth(4).unwrap().to_owned();
    path.push("assets");
    std::fs::create_dir_all(&path).unwrap();

    build_shell_completion(&path)?;
    build_manpages(&path)?;

    Ok(())
}
