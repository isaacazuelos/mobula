use std::fs::File;

use mobula::scene::Scene;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = clap::App::new("mobula")
        .version(clap::crate_version!())
        .about("A ray tracer")
        .author(clap::crate_authors!())
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .args(&[
            clap::Arg::with_name("scene")
                .help("A scene file")
                .long_help(
                    "The scene file used by the ray tracer must be in
JSON format, and contains information about the
objects, camera, and rendering settings.",
                )
                .value_name("FILE")
                .index(1)
                .takes_value(true)
                .required(true),
            clap::Arg::with_name("width")
                .help("the width of the rendered image")
                .long("width")
                .short('w')
                .takes_value(true),
            clap::Arg::with_name("height")
                .help("the height of the rendered image")
                .long("height")
                .short('h')
                .takes_value(true),
            clap::Arg::with_name("depth")
                .help("Maximum depth of reflections")
                .long("depth")
                .short('d')
                .takes_value(true),
            clap::Arg::with_name("samples")
                .help("the number of samples per pixel")
                .long("samples")
                .short('s')
                .takes_value(true),
            clap::Arg::with_name("out")
                .help("write output to FILE")
                .long("out")
                .short('o')
                .value_name("FILE")
                .default_value("a.png")
                .takes_value(true),
        ])
        .get_matches();

    // `unwrap` is safe as it's a required arg.
    let file = File::open(matches.value_of("scene").unwrap())?;
    let mut scene: Scene = serde_json::from_reader(file)?;

    // we need to overwrite the scene config based on command line args.
    if matches.is_present("width") {
        scene.config.width = clap::value_t!(matches, "width", u32).unwrap_or_else(|e| e.exit());
    }
    if matches.is_present("height") {
        scene.config.height = clap::value_t!(matches, "height", u32).unwrap_or_else(|e| e.exit());
    }
    if matches.is_present("depth") {
        scene.config.depth = clap::value_t!(matches, "depth", u32).unwrap_or_else(|e| e.exit());
    }
    if matches.is_present("samples") {
        scene.config.samples = clap::value_t!(matches, "samples", u32).unwrap_or_else(|e| e.exit());
    }

    let img = scene.render();

    // `unwrap` is safe as it has a default
    let out_path = matches.value_of("out").unwrap();
    println!("writing file to {}", &out_path);
    img.save(&out_path)?;
    Ok(())
}
