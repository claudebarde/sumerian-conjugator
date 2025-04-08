#[derive(Debug, Clone)]
pub struct VerbAspect<'a> {
    pub perf: &'a str,
    pub imperf: &'a str,
}

// 12.4.2
#[derive(Debug, Clone)]
pub struct VerbForm<'a> {
    pub stem: &'a str,
    pub meaning: &'a str,
    pub transitive: bool,
    pub sing: VerbAspect<'a>,
    pub plur: VerbAspect<'a>,
}

pub const VERBFORMS: [VerbForm; 9] = [
    VerbForm {
        stem: "gub",
        meaning: "to stand",
        transitive: false,
        sing: VerbAspect {
            perf: "gub",
            imperf: "gub",
        },
        plur: VerbAspect {
            perf: "šug",
            imperf: "šug",
        },
    },
    VerbForm {
        stem: "dug",
        meaning: "to do",
        transitive: true,
        sing: VerbAspect {
            perf: "dug",
            imperf: "dug",
        },
        plur: VerbAspect {
            perf: "did",
            imperf: "did",
        },
    },
    VerbForm {
        stem: "re",
        meaning: "to bring",
        transitive: true,
        sing: VerbAspect {
            perf: "re",
            imperf: "re",
        },
        plur: VerbAspect {
            perf: "tum",
            imperf: "tum",
        },
    },
    VerbForm {
        stem: "ĝen",
        meaning: "to go",
        transitive: false,
        sing: VerbAspect {
            perf: "ĝen",
            imperf: "du",
        },
        plur: VerbAspect {
            perf: "er",
            imperf: "sub",
        },
    },
    VerbForm {
        stem: "tuš",
        meaning: "to sit",
        transitive: false,
        sing: VerbAspect {
            perf: "tuš",
            imperf: "dur",
        },
        plur: VerbAspect {
            perf: "durun",
            imperf: "durun",
        },
    },
    VerbForm {
        stem: "hulu",
        meaning: "to be bad",
        transitive: false,
        sing: VerbAspect {
            perf: "hulu",
            imperf: "hulhulu",
        },
        plur: VerbAspect {
            perf: "hulu",
            imperf: "hulhulu",
        },
    },
    VerbForm {
        stem: "tuku",
        meaning: "to have",
        transitive: true,
        sing: VerbAspect {
            perf: "tuku",
            imperf: "tuktuku",
        },
        plur: VerbAspect {
            perf: "tuku",
            imperf: "tuktuku",
        },
    },
    VerbForm {
        stem: "til",
        meaning: "to live",
        transitive: false,
        sing: VerbAspect {
            perf: "til",
            imperf: "til",
        },
        plur: VerbAspect {
            perf: "se",
            imperf: "se",
        },
    },
    VerbForm {
        stem: "zig",
        meaning: "to rise, to raise",
        transitive: false,
        sing: VerbAspect {
            perf: "zig",
            imperf: "zigzig",
        },
        plur: VerbAspect {
            perf: "zig",
            imperf: "zigzig",
        },
    },
    // VerbForm {
    //     stem: "",
    //     meaning: "to ",
    //     transitive: ,
    //     sing: VerbAspect {
    //         perf: "",
    //         imperf: "",
    //     },
    //     plur: VerbAspect {
    //         perf: "",
    //         imperf: "",
    //     },
    // },
];

pub fn available_verb_stems() -> Vec<String> {
    let mut stems = Vec::new();
    for verb in VERBFORMS.iter() {
        stems.push(verb.stem.to_string());
    }
    stems
}

pub fn find_verb(stem: &str) -> Result<&'static VerbForm<'static>, String> {
    for verb in VERBFORMS.iter() {
        if verb.stem == stem {
            return Ok(verb);
        }
    }
    Err(format!("Verb '{}' not found in lexicon", stem))
}
