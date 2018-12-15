use ini::ini::Properties;
use ini::Ini;
use ini::ini::Error::{ Io, Parse };
use shellexpand::tilde;

pub fn get_assume_settings<'a>(properties: &'a Properties) -> Option<(&'a String, &'a String)> {
    let role_arn = properties.get("role_arn");
    let source_profile = properties.get("source_profile");
    if let (Some(arn), Some(profile)) = (role_arn, source_profile) {
        return Some((arn, profile))
    }

    None
}

pub fn get_value_of_tuple<'a>((_, properties): (&Option<String>, &'a Properties)) -> &'a Properties {
    properties
}

pub fn load_ini(path: &String) -> Result<Ini, String> {
    let expanded_path = tilde(path).to_string();

    match Ini::load_from_file(expanded_path) {
        Ok(f) => Ok(f),
        Err(Io(_)) => Err(format!("failed to load file {}", path)),
        Err(Parse(_)) => Err(format!("invalid file {}", path)),
    }
}

pub fn compose<A, B, C>(fn1: impl Fn(A) -> B, fn2: impl Fn(B) -> C) -> impl Fn(A) -> C {
    move |b| fn2(fn1(b))
}
