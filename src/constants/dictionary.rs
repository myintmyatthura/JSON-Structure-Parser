use std::collections::HashMap;

pub fn dictionary() -> HashMap<&'static str, &'static str> {
    let mut abbreviations = HashMap::new();

    abbreviations.insert("AA_AAA", "Acronym Analysis for Advanced Algorithms");
    abbreviations.insert("MI_SUB", "Machine Intelligence and Subsystems");
    abbreviations.insert("MB_KB", "Megabytes and Kilobytes");
    abbreviations.insert("GB_FOR", "Gigabytes for Operations Research");
    abbreviations.insert(
        "OP_MNO",
        "Operational Parameters for Mobile Networks Optimization",
    );
    abbreviations.insert(
        "MNO_AA",
        "Mobile Network Operator and Authentication Algorithms",
    );
    abbreviations.insert(
        "SUB_MI_MB",
        "Subsystems in Machine Intelligence and Megabytes",
    );
    abbreviations.insert("KB_GB", "Kilobytes and Gigabytes");
    abbreviations.insert("FOR_OP", "Fortran Operations");
    abbreviations.insert(
        "AAA_MNO",
        "Advanced Authentication Algorithms for Mobile Network Operators",
    );

    return abbreviations;
}
