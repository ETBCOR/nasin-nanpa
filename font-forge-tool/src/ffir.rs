use itertools::Itertools;

#[derive(Clone)]
pub enum EncPos {
    Pos(usize),
    None,
}

impl EncPos {
    fn new(pos: Option<usize>) -> Self {
        match pos {
            Some(p) => Self::Pos(p),
            None => Self::None,
        }
    }

    fn inc(&mut self) {
        *self = match self {
            EncPos::Pos(p) => EncPos::Pos(*p + 1),
            EncPos::None => EncPos::None,
        };
    }

    fn gen(&self) -> String {
        match self {
            EncPos::Pos(p) => p.to_string(),
            EncPos::None => "-1".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct Encoding {
    pub ff_pos: usize,
    pub enc_pos: EncPos,
}

impl Encoding {
    pub fn new(ff_pos: usize, enc_pos: EncPos) -> Self {
        Self { ff_pos, enc_pos }
    }

    pub fn gen(&self) -> String {
        format!(
            "Encoding: {ff_pos} {enc_pos} {ff_pos}",
            ff_pos = self.ff_pos,
            enc_pos = self.enc_pos.gen(),
        )
    }

    pub fn gen_ref(&self, position: String) -> String {
        let Encoding { ff_pos, enc_pos } = self;
        format!(
            "Refer: {ff_pos} {enc_pos} {position}",
            enc_pos = enc_pos.gen(),
            position = position,
        )
    }
}

// A glyph reference (with positional data)
#[derive(Clone)]
pub struct Ref {
    ref_glyph: Encoding,
    position: String,
}

impl Ref {
    pub fn new(ref_glyph: Encoding, position: impl Into<String>) -> Self {
        Self {
            ref_glyph,
            position: position.into(),
        }
    }

    pub fn gen(&self) -> String {
        self.ref_glyph.gen_ref(self.position.clone())
    }
}

// A glyph representation
#[derive(Default, Clone)]
pub struct Rep {
    spline_set: String,
    references: Vec<Ref>,
}

impl Rep {
    pub fn new(spline_set: impl Into<String>, references: Vec<Ref>) -> Self {
        Self {
            spline_set: spline_set.into(),
            references,
        }
    }

    pub fn gen(&self) -> String {
        let f = if !self.spline_set.is_empty() || !self.references.is_empty() {
            "Fore\n"
        } else {
            ""
        };

        let r = self
            .references
            .clone()
            .into_iter()
            .map(|r| r.gen())
            .join("\n");

        let nl = if !self.references.is_empty() {
            "\n"
        } else {
            ""
        };

        let s = if !self.spline_set.is_empty() {
            format!("SplineSet{s}\nEndSplineSet\n", s = self.spline_set)
        } else {
            "".to_string()
        };

        format!("{f}{r}{nl}{s}")
    }
}

#[derive(Clone)]
pub struct GlyphBasic {
    pub name: String,
    pub width: usize,
    pub rep: Rep,
}

impl GlyphBasic {
    pub fn new(name: impl Into<String>, width: usize, rep: Rep) -> Self {
        Self {
            name: name.into(),
            width,
            rep,
        }
    }
}

pub struct GlyphEnc {
    glyph: GlyphBasic,
    enc: EncPos,
}

impl GlyphEnc {
    pub fn new_from_basic(glyph: GlyphBasic, enc: EncPos) -> Self {
        Self { glyph, enc }
    }

    pub fn new_from_parts(enc: EncPos, name: impl Into<String>, width: usize, rep: Rep) -> Self {
        Self {
            glyph: GlyphBasic::new(name, width, rep),
            enc,
        }
    }
}

pub enum LookupsMode {
    WordLigFromLetters,
    WordLigManual(Vec<String>),
    StartLongGlyph,
    StartLongGlyphRev,
    Alt,
    ComboFirst,
    ComboSecond,
    None,
}

#[derive(Clone)]
pub enum Lookups {
    WordLigFromLetters,
    WordLigManual(String),
    StartLongGlyph,
    StartLongGlyphRev,
    Alt,
    ComboFirst,
    ComboLast,
    None,
}

impl Lookups {
    fn new_from_mode(mode: &LookupsMode, idx: usize) -> Self {
        match mode {
            LookupsMode::WordLigFromLetters => Lookups::WordLigFromLetters,
            LookupsMode::WordLigManual(vec) => {
                let s = &vec[idx];
                if s.len() > 0 {
                    Lookups::WordLigManual(vec[idx].clone())
                } else {
                    Lookups::None
                }
            }
            LookupsMode::StartLongGlyph => Lookups::StartLongGlyph,
            LookupsMode::StartLongGlyphRev => Lookups::StartLongGlyphRev,
            LookupsMode::Alt => Lookups::Alt,
            LookupsMode::ComboFirst => Lookups::ComboFirst,
            LookupsMode::ComboSecond => Lookups::ComboLast,
            LookupsMode::None => Lookups::None,
        }
    }

    fn gen(&self, name: String, full_name: String) -> String {
        match &self {
            Lookups::WordLigFromLetters => {
                let lig = name.chars().join(" ");
                let rand = if full_name.eq("jakiTok") {
                    "AlternateSubs2: \"'rand' RAND VARIATIONS\" jakiTok_VAR01 jakiTok_VAR02 jakiTok_VAR03 jakiTok_VAR04 jakiTok_VAR05 jakiTok_VAR06 jakiTok_VAR07 jakiTok_VAR08\n"
                } else if full_name.eq("koTok") {
                    "AlternateSubs2: \"'rand' RAND VARIATIONS\" koTok_VAR01 koTok_VAR02 koTok_VAR03 koTok_VAR04 koTok_VAR05 koTok_VAR06 koTok_VAR07 koTok_VAR08\n"
                } else {
                    ""
                };
                format!("{rand}Ligature2: \"'liga' WORD PLUS SPACE\" {lig} space\nLigature2: \"'liga' WORD\" {lig}\n")
            }
            Lookups::WordLigManual(word) => {
                if word.eq("space space") {
                    format!("Ligature2: \"'liga' SPACE\" {word}\nLigature2: \"'liga' SPACE\" z z\n")
                } else {
                    format!("Ligature2: \"'liga' WORD PLUS SPACE\" {word} space\nLigature2: \"'liga' WORD\" {word}\n")
                }
            }
            Lookups::StartLongGlyph => {
                let parts: Vec<&str> = full_name.split("_").collect();
                let glyph = parts[0];
                let joiner = parts[1];
                format!("Ligature2: \"'liga' START LONG GLYPHS\" {glyph} {joiner}\n")
            }
            Lookups::StartLongGlyphRev => {
                let parts: Vec<&str> = full_name.split("_").collect();
                let glyph = parts[0];
                format!("Ligature2: \"'liga' START LONG GLYPHS\" endRevLongGlyphTok {glyph}\n")
            }
            Lookups::Alt => {
                let parts: Vec<&str> = full_name.split("_").collect();
                let glyph = parts[0];
                match (parts.len(), full_name.contains("niTok")) {
                    (2, false) => {
                        // normal variant
                        let sel = parts[1];
                        format!("Ligature2: \"'liga' VARIATIONS AND SPECIALS\" {glyph} {sel}\n")
                    }
                    (2, true) => {
                        // cardinal ni
                        let dir = parts[1];
                        let short = match dir {
                            "asciicircum" => "N",
                            "v" => "S",
                            "less" => "W",
                            "greater" => "E",
                            _ => panic!(),
                        };
                        format!("Ligature2: \"'liga' VARIATIONS AND SPECIALS\" {glyph} ZWJ arrow{short}\nLigature2: \"'liga' WORD PLUS SPACE\" {glyph} {dir} space\nLigature2: \"'liga' WORD\" {glyph} {dir}\n")
                    }
                    (3, true) => {
                        // diagonal ni
                        let dir1 = parts[1];
                        let dir2 = parts[2];
                        let short = match (dir1, dir2) {
                            ("asciicircum", "less") => "NW",
                            ("asciicircum", "greater") => "NE",
                            ("v", "less") => "SW",
                            ("v", "greater") => "SE",
                            (_, _) => panic!(),
                        };
                        format!("Ligature2: \"'liga' VARIATIONS AND SPECIALS\" {glyph} ZWJ arrow{short}\nLigature2: \"'liga' WORD PLUS SPACE\" {glyph} {dir1} {dir2} space\nLigature2: \"'liga' WORD\" {glyph} {dir2} {dir1}\n")
                    }
                    _ => panic!(),
                }
            }
            Lookups::ComboFirst => {
                let parts: Vec<&str> = full_name.split("_").collect();
                let glyph = parts[0];
                let joiner = parts[1];
                format!("Ligature2: \"'liga' GLYPH THEN JOINER\" {glyph} {joiner}\nMultipleSubs2: \"'ccmp' RESPAWN JOINER\" {full_name} {joiner}\n")
            }
            Lookups::ComboLast => {
                let parts: Vec<&str> = full_name.split("_").collect();
                let joiner = parts[0];
                let glyph = parts[1];
                format!("Ligature2: \"'liga' JOINER THEN GLYPH\" {joiner} {glyph}\n")
            }
            Lookups::None => "".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct GlyphFull {
    pub glyph: GlyphBasic,
    pub encoding: Encoding,
    pub lookups: Lookups,
    pub cc_subs: bool,
}

impl GlyphFull {
    pub fn new_from_basic(
        glyph: GlyphBasic,
        encoding: Encoding,
        lookups: Lookups,
        cc_subs: bool,
    ) -> Self {
        Self {
            glyph,
            encoding,
            lookups,
            cc_subs,
        }
    }

    pub fn new_from_enc(glyph: GlyphEnc, ff_pos: usize, lookups: Lookups, cc_subs: bool) -> Self {
        Self {
            glyph: glyph.glyph,
            encoding: Encoding::new(ff_pos, glyph.enc),
            lookups,
            cc_subs,
        }
    }

    pub fn new_from_parts(
        name: impl Into<String>,
        width: usize,
        rep: Rep,
        encoding: Encoding,
        lookups: Lookups,
        cc_subs: bool,
    ) -> Self {
        Self {
            glyph: GlyphBasic::new(name, width, rep),
            encoding,
            lookups,
            cc_subs,
        }
    }

    pub fn gen(&self, prefix: String, suffix: String, color: String) -> String {
        let name = &self.glyph.name;
        let full_name = format!("{}{}{}", prefix, name, suffix);
        let encoding = self.encoding.gen();
        let width = self.glyph.width;
        let representation = self.glyph.rep.gen();
        let lookups = self.lookups.gen(name.to_string(), full_name.clone());
        let cc_subs = if self.cc_subs {
            let halfwidth = if width == 500 { "Half" } else { "" };
            format!("MultipleSubs2: \"'cc01' CART\" {full_name} combCartExt{halfwidth}Tok\nMultipleSubs2: \"'cc02' CONT\" {full_name} combLongGlyphExt{halfwidth}Tok\n")
        } else {
            "".to_string()
        };

        let color = format!("Colour: {color}");

        let flags = if !name.contains("empty") {
            "Flags: W\n"
        } else {
            ""
        };
        format!("\nStartChar: {full_name}\n{encoding}\n{flags}Width: {width}\nLayerCount: 2\n{representation}{lookups}{cc_subs}{color}\nEndChar\n")
    }
}

pub struct GlyphDescriptor {
    pub name: &'static str,
    pub spline_set: &'static str,
    pub width: Option<usize>,
}

impl GlyphDescriptor {
    pub const fn new(name: &'static str, spline_set: &'static str) -> Self {
        Self {
            name,
            spline_set,
            width: None,
        }
    }

    pub const fn new_with_width(
        name: &'static str,
        width: usize,
        spline_set: &'static str,
    ) -> Self {
        Self {
            name,
            spline_set,
            width: Some(width),
        }
    }
}

pub struct GlyphBlock {
    pub glyphs: Vec<GlyphFull>,
    pub prefix: String,
    pub suffix: String,
    pub color: String,
}

impl GlyphBlock {
    pub fn new_from_enc_glyphs(
        ff_pos: &mut usize,
        glyphs: Vec<GlyphEnc>,
        lookups: LookupsMode,
        cc_subs: bool,
        prefix: impl Into<String>,
        suffix: impl Into<String>,
        color: impl Into<String>,
    ) -> Self {
        let mut glyphs: Vec<GlyphFull> = glyphs
            .into_iter()
            .enumerate()
            .map(|(idx, glyph)| {
                let g = GlyphFull::new_from_enc(
                    glyph,
                    *ff_pos,
                    Lookups::new_from_mode(&lookups, idx),
                    cc_subs,
                );
                *ff_pos += 1;
                g
            })
            .collect();

        let mut padding = Self::new_empty(ff_pos, 15 - ((glyphs.len() + 15) % 16), 0).glyphs;
        glyphs.append(&mut padding);

        Self {
            glyphs,
            prefix: prefix.into(),
            suffix: suffix.into(),
            color: color.into(),
        }
    }

    pub fn new_from_basic_glyphs(
        ff_pos: &mut usize,
        glyphs: Vec<GlyphBasic>,
        lookups: LookupsMode,
        cc_subs: bool,
        prefix: impl Into<String>,
        suffix: impl Into<String>,
        color: impl Into<String>,
        mut enc_pos: EncPos,
    ) -> Self {
        let mut glyphs: Vec<GlyphFull> = glyphs
            .into_iter()
            .enumerate()
            .map(|(idx, glyph)| {
                let g = GlyphFull::new_from_basic(
                    glyph,
                    Encoding::new(*ff_pos, enc_pos.clone()),
                    Lookups::new_from_mode(&lookups, idx),
                    cc_subs,
                );
                *ff_pos += 1;
                enc_pos.inc();
                g
            })
            .collect();

        let mut padding = Self::new_empty(ff_pos, 15 - ((glyphs.len() + 15) % 16), 0).glyphs;
        glyphs.append(&mut padding);

        Self {
            glyphs,
            prefix: prefix.into(),
            suffix: suffix.into(),
            color: color.into(),
        }
    }

    pub fn new_from_constants(
        ff_pos: &mut usize,
        glyphs: &'static [GlyphDescriptor],
        lookups: LookupsMode,
        cc_subs: bool,
        prefix: impl Into<String>,
        suffix: impl Into<String>,
        color: impl Into<String>,
        enc_pos: EncPos,
        fallback_width: usize,
    ) -> Self {
        let glyphs: Vec<GlyphBasic> = glyphs
            .into_iter()
            .map(
                |GlyphDescriptor {
                     name,
                     spline_set,
                     width,
                 }| {
                    GlyphBasic::new(
                        name.to_string(),
                        width.unwrap_or(fallback_width),
                        Rep::new(spline_set.to_string(), vec![]),
                    )
                },
            )
            .collect();

        Self::new_from_basic_glyphs(
            ff_pos, glyphs, lookups, cc_subs, prefix, suffix, color, enc_pos,
        )
    }

    pub fn new_from_refs(
        &self,
        ff_pos: &mut usize,
        rel_pos: String,
        static_glyph_ref: Option<Ref>,
        lookups: LookupsMode,
        cc_subs: bool,
        use_full_names: bool,
        prefix: impl Into<String>,
        suffix: impl Into<String>,
        color: impl Into<String>,
    ) -> Self {
        let glyphs: Vec<GlyphBasic> = self
            .glyphs
            .clone()
            .into_iter()
            .map(
                |GlyphFull {
                     glyph, encoding, ..
                 }| {
                    let refs = vec![
                        Some(Ref::new(encoding.clone(), rel_pos.clone())),
                        static_glyph_ref.clone(),
                    ]
                    .into_iter()
                    .flatten()
                    .collect();
                    let name = if use_full_names {
                        format!(
                            "{pre}{name}{post}",
                            pre = self.prefix,
                            name = glyph.name,
                            post = self.suffix
                        )
                    } else {
                        glyph.name
                    };
                    let g = GlyphBasic::new(name, glyph.width, Rep::new(String::default(), refs));
                    g
                },
            )
            .collect();

        Self::new_from_basic_glyphs(
            ff_pos,
            glyphs,
            lookups,
            cc_subs,
            prefix,
            suffix,
            color,
            EncPos::None,
        )
    }

    pub fn new_empty(ff_pos: &mut usize, count: usize, width: usize) -> Self {
        let end = *ff_pos + count;
        let mut glyphs = vec![];

        while *ff_pos < end {
            glyphs.push(GlyphFull::new_from_parts(
                format!("empty{i:04}", i = *ff_pos),
                width,
                Rep::default(),
                Encoding::new(*ff_pos, EncPos::None),
                Lookups::None,
                false,
            ));
            *ff_pos += 1;
        }

        Self {
            glyphs,
            prefix: String::default(),
            suffix: String::default(),
            color: "dddddd".to_string(),
        }
    }

    pub fn gen(&self) -> String {
        let mut s = String::new();
        for g in &self.glyphs {
            s += &g.gen(self.prefix.clone(), self.suffix.clone(), self.color.clone())
        }
        s
    }
}
