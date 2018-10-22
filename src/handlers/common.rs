use ini::Ini;
use ini::ini::Error::{ Io, Parse };
use shellexpand::tilde;

pub fn load_ini(path: &String) -> Result<Ini, String> {
    let expanded_path = tilde(path).to_string();

    match Ini::load_from_file(expanded_path) {
        Ok(f) => Ok(f),
        Err(Io(_)) => Err(format!("failed to load file {}", path)),
        Err(Parse(_)) => Err(format!("invalid file {}", path)),
    }
}
