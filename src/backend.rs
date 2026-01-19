use::dioxus::prelude::*;

#[post("/api/save_dog")]
pub async fn save_dog(image: String) -> Result<()> {
    use std::io::Write;

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("dogs.txt")
        .unwrap();

    file.write_fmt(format_args!("{image}\n"));

    Ok(())
}