//! TODO: Document (2019-02-01)

use std::fs::File;

use mobula::scene::Scene;

fn main() -> Result<(), Box<::std::error::Error>> {
    let file = File::open("scene1.json")?;
    let scene: Scene = serde_json::from_reader(file)?;
    let img = scene.render();

    img.save("out.png")?;
    Ok(())
}
