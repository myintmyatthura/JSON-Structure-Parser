use std::collections::HashMap;

pub fn dictionary() -> HashMap<&'static str, &'static str> {
    let mut abbreviations = HashMap::new();

    abbreviations.insert("AAA", "Advanced Authentication Algorithms");
    abbreviations.insert("MNO", "Mobile Network Operators");
    abbreviations.insert("SUB", "Subsystems");
    abbreviations.insert("MI", "Machine Intelligence");
    abbreviations.insert("MB", "Megabytes");
    abbreviations.insert("KB", "Kilobytes");
    abbreviations.insert("GB", "Gigabytes");
    abbreviations.insert("FOR", "Fortran");
    abbreviations.insert("OP", "Operations");
    abbreviations.insert("AA", "Acronym Analysis");

    return abbreviations;
}
