use glyph_blocks::*;
use itertools::Itertools;
use std::{fs::File, io::Write};

mod glyph_blocks;

const HEADER: &str = r#"SplineFontDB: 3.2
FontName: nn
FullName: nasin nanpa
FamilyName: nasin-nanpa
Weight: Regular
Copyright: jan Itan li mama. jan mute a li pona e pali ona.
"#;

const VERSION: &str = "4.0.0";

const DETAILS1: &str = r#"ItalicAngle: 0
UnderlinePosition: 0
UnderlineWidth: 0
Ascent: 1000
Descent: 0
InvalidEm: 0
sfntRevision: 0x00010000
LayerCount: 2
Layer: 0 0 "Back" 1
Layer: 1 0 "Fore" 0
XUID: [1021 700 1229584016 12833]
StyleMap: 0x0040
FSType: 0
OS2Version: 3
OS2_WeightWidthSlopeOnly: 0
OS2_UseTypoMetrics: 0
CreationTime: 1640950552
"#;

const DETAILS2: &str = r#"
PfmFamily: 81
TTFWeight: 400
TTFWidth: 5
LineGap: 0
VLineGap: 0
Panose: 0 0 8 9 0 0 0 6 0 0
OS2TypoAscent: 1000
OS2TypoAOffset: 0
OS2TypoDescent: 0
OS2TypoDOffset: 0
OS2TypoLinegap: 0
OS2WinAscent: 1000
OS2WinAOffset: 0
OS2WinDescent: 386
OS2WinDOffset: 0
HheadAscent: 1000
HheadAOffset: 0
HheadDescent: -386
HheadDOffset: 0
OS2SubXSize: 650
OS2SubYSize: 699
OS2SubXOff: 0
OS2SubYOff: 140
OS2SupXSize: 650
OS2SupYSize: 699
OS2SupXOff: 0
OS2SupYOff: 479
OS2StrikeYSize: 49
OS2StrikeYPos: 258
OS2CapHeight: 1000
OS2XHeight: 500
OS2Vendor: 'XXXX'
OS2CodePages: 00000001.00000000
OS2UnicodeRanges: 0000000c.00000000.00000000.00000000
"#;

const LOOKUPS: &str = r#"Lookup: 4 0 0 "'liga' SPACE" { "'liga' SPACE"  } ['liga' ('DFLT' <'dflt' 'latn' > 'latn' <'dflt' > ) ]
Lookup: 4 0 0 "'liga' WORDS" { "'liga' WORD PLUS SPACE"  "'liga' WORD"  } ['liga' ('DFLT' <'dflt' 'latn' > 'latn' <'dflt' > ) ]
Lookup: 4 0 0 "'liga' VARIATIONS AND SPECIALS" { "'liga' VARIATIONS AND SPECIALS"  } ['liga' ('DFLT' <'dflt' 'latn' > 'latn' <'dflt' > ) ]
Lookup: 4 0 0 "'liga' START CONTAINER" { "'liga' START CONTAINER"  } ['liga' ('DFLT' <'dflt' 'latn' > 'latn' <'dflt' > ) ]
Lookup: 4 0 0 "'liga' JOIN STACK" { "'liga' JOIN STACK"  } ['liga' ('DFLT' <'dflt' 'latn' > 'latn' <'dflt' > ) ]
Lookup: 4 0 0 "'liga' GLYPH THEN JOINER" { "'liga' GLYPH THEN JOINER"  } ['liga' ('DFLT' <'dflt' 'latn' > 'latn' <'dflt' > ) ]
Lookup: 2 0 0 "'ccmp' RESPAWN JOINER" { "'ccmp' RESPAWN JOINER"  } ['ccmp' ('DFLT' <'dflt' > 'latn' <'dflt' > ) ]
Lookup: 4 0 0 "'liga' JOINER THEN GLYPH" { "'liga' JOINER THEN GLYPH"  } ['liga' ('DFLT' <'dflt' 'latn' > 'latn' <'dflt' > ) ]
Lookup: 6 0 0 "'calt' CART AND CONT" { "'calt' CART AND CONT"  } ['calt' ('DFLT' <'dflt' 'latn' > 'latn' <'dflt' > ) ]
Lookup: 1 0 0 "'ss01' CART" { "'ss01' CART SUB" (".cart") } ['ss01' ('DFLT' <'dflt' 'latn' > 'latn' <'dflt' > ) ]
Lookup: 1 0 0 "'ss02' CONT" { "'ss02' CONT SUB" (".cont") } ['ss02' ('DFLT' <'dflt' 'latn' > 'latn' <'dflt' > ) ]
Lookup: 258 0 0 "'kern' COMBOS KERN" { "'kern' COMBOS KERN" [0,0,0] } ['kern' ('DFLT' <'dflt' 'latn' > 'latn' <'dflt' > ) ]
"#;
const OTHER: &str = r#"DEI: 91125
LangName: 1033 "" "" "" "" "" "3.1.1" "" "+ACIA-jan Itan 2023+ACIA" "+ACIAIgAA" "+ACIA-jan Itan+ACIA" "+ACIAIgAA" "+ACIAIgAA" "+ACIA-https://etbcor.com/+ACIA" "+ACIA-MIT License+ACIA" "+ACIA-https://opensource.org/licenses/MIT+ACIA" "" "nasin-nanpa" "Regular"
Encoding: Custom
UnicodeInterp: none
NameList: AGL For New Fonts
DisplaySize: -72
AntiAlias: 1
FitToEm: 1
WinInfo: 32 16 8
"#;

#[derive(Clone)]
struct Encoding {
    ff_pos: usize,
    // None indicates that this glyph doesn't have an encoding (to_string will use -1)
    enc_pos: Option<usize>,
}

impl Encoding {
    fn new(ff_pos: usize, codepoint: Option<usize>) -> Self {
        Self {
            ff_pos,
            enc_pos: codepoint,
        }
    }
    fn to_string(&self) -> String {
        format!(
            "Encoding: {ff_pos} {enc_pos} {ff_pos}",
            ff_pos = self.ff_pos,
            enc_pos = match self.enc_pos {
                Some(c) => c.to_string(),
                None => "-1".to_string(),
            },
        )
    }
}

// A glyph reference (with positional data)
#[derive(Clone)]
struct GlyphRef {
    ref_glyph: Encoding,
    position: String,
}

impl GlyphRef {
    fn new(ref_glyph: Encoding, position: String) -> Self {
        Self {
            ref_glyph,
            position,
        }
    }

    fn to_string(&self) -> String {
        let Encoding { ff_pos, enc_pos } = self.ref_glyph;
        format!(
            "Refer: {ff_pos} {enc_pos} {position}",
            enc_pos = match enc_pos {
                Some(c) => c.to_string(),
                None => "-1".to_string(),
            },
            position = self.position,
        )
    }
}

// A glyph representation
#[derive(Default, Clone)]
struct GlyphRep {
    spline_set: String,
    references: Vec<GlyphRef>,
}

impl GlyphRep {
    fn new(spline_set: String, references: Vec<GlyphRef>) -> Self {
        Self {
            spline_set,
            references,
        }
    }

    fn to_string(&self) -> String {
        format!(
            "{f}{r}{nl}{s}",
            f = if !self.spline_set.is_empty() || !self.references.is_empty() {
                "Fore\n"
            } else {
                ""
            },
            r = self
                .references
                .clone()
                .into_iter()
                .map(|r| r.to_string())
                .join("\n"),
            nl = if !self.references.is_empty() {
                "\n"
            } else {
                ""
            },
            s = if !self.spline_set.is_empty() {
                format!("SplineSet{s}\nEndSplineSet\n", s = self.spline_set)
            } else {
                "".to_string()
            }
        )
    }
}

// The `NoCodeGlyph` (a glyph without any encoding info)
struct NCGlyph {
    name: String,
    width: usize,
    glyph: GlyphRep,
}

impl NCGlyph {
    fn new(name: String, width: usize, glyph: GlyphRep) -> Self {
        Self { name, width, glyph }
    }
}

// The `HalfCodedGlyph` (a glyph with a codepoint but no FF pos)
struct HCGlyph {
    no_code_glyph: NCGlyph,
    enc: Option<usize>,
}

impl HCGlyph {
    fn new(enc: Option<usize>, name: String, width: usize, glyph: GlyphRep) -> Self {
        Self {
            no_code_glyph: NCGlyph::new(name, width, glyph),
            enc,
        }
    }
}

// The `Glyph` (a glyph with an encoding slot)
#[derive(Clone)]
struct Glyph {
    name: String,
    encoding: Encoding,
    width: usize,
    glyph: GlyphRep,
    ligs: String,
    subs: String,
}

impl Glyph {
    fn new(
        name: String,
        encoding: Encoding,
        width: usize,
        glyph: GlyphRep,
        ligs: String,
        subs: String,
    ) -> Self {
        Self {
            name,
            encoding,
            width,
            glyph,
            ligs,
            subs,
        }
    }

    fn to_string(&self, prefix: String, suffix: String, color: String) -> String {
        format!(
            "\nStartChar: {name}\n{encoding}\nWidth: {width}\nLayerCount: 2\n{representation}{ligs}{subs}{color}\nEndChar\n",
            name = format!("{}{}{}", prefix, self.name, suffix),
            encoding = self.encoding.to_string(),
            width = self.width,
            representation = self.glyph.to_string(),
            ligs = self.ligs,
            subs = self.subs,
            color = format!("Colour: {color}"),
        )
    }
}

struct GlyphDescriptor {
    name: &'static str,
    spline_set: &'static str,
    width: Option<usize>,
}

impl GlyphDescriptor {
    const fn new(name: &'static str, spline_set: &'static str) -> Self {
        Self {
            name,
            spline_set,
            width: None,
        }
    }

    const fn new_with_width(name: &'static str, width: usize, spline_set: &'static str) -> Self {
        Self {
            name,
            spline_set,
            width: Some(width),
        }
    }
}

enum LigMode {
    FromName,
    GlyphThenJoiner(String),
    JoinerThenGlyph(String, String),
    None,
}

impl LigMode {
    fn to_string(&self, name: String) -> String {
        match self {
            LigMode::FromName => {
                let lig = name.chars().join(" ");
                let lig = lig.trim();
                format!("Ligature2: \"'liga' WORD PLUS SPACE\" {lig} space\nLigature2: \"'liga' WORD\" {lig}\n")
            }
            LigMode::GlyphThenJoiner(s) => {
                let name = format!("{name}{s}");
                let lig = name.replace("_", " ");
                format!("Ligature2: \"'liga' GLYPH THEN JOINER\" {lig}\n")
            }
            LigMode::JoinerThenGlyph(pre, post) => {
                let name = format!("{pre}{name}{post}");
                let lig = name.replace("_", " ");
                format!("Ligature2: \"'liga' JOINER THEN GLYPH\" {lig}\n")
            }
            LigMode::None => "".to_string(),
        }
    }
}

enum SubMode {
    CartCont,
    CCAndRespawn,
    None,
}

impl SubMode {
    fn to_string(&self, name: String) -> String {
        match self {
            SubMode::CartCont => {
                format!("Substitution2: \"'ss01' CART SUB\" {name}.cart\nSubstitution2: \"'ss01' CONT SUB\" {name}.cont\n")
            }
            SubMode::CCAndRespawn => {
                let lig: Vec<&str> = name.split("_").collect();
                format!(
                    "\"'ccmp' RESPAWN JOINER\" {name} {joiner}\nSubstitution2: \"'ss01' CART SUB\" {name}.cart\nSubstitution2: \"'ss01' CONT SUB\" {name}.cont\n",
                    joiner = lig[lig.len() - 1],
                )
            }
            SubMode::None => "".to_string(),
        }
    }
}

// A block of FF-encoded Glyphs
struct GlyphBlock {
    glyphs: Vec<Glyph>,
    prefix: String,
    suffix: String,
    color: String,
}

impl GlyphBlock {
    fn new_from_enc_glyph_vec(
        ff_pos: &mut usize,
        glyphs: Vec<HCGlyph>,
        prefix: String,
        suffix: String,
        color: String,
    ) -> Self {
        let len = glyphs.len();
        let mut glyphs: Vec<Glyph> = glyphs
            .into_iter()
            .map(|HCGlyph { no_code_glyph, enc }| {
                let g = Glyph::new(
                    no_code_glyph.name,
                    Encoding::new(*ff_pos, enc),
                    no_code_glyph.width,
                    no_code_glyph.glyph,
                    String::default(),
                    String::default(),
                );
                *ff_pos += 1;
                g
            })
            .collect();

        let mut padding = Self::new_empty(ff_pos, 15 - ((len + 15) % 16), 0).glyphs;
        glyphs.append(&mut padding);

        Self {
            glyphs,
            prefix,
            suffix,
            color,
        }
    }

    fn new_from_glyph_vec(
        ff_pos: &mut usize,
        enc_rng_start: Option<usize>,
        glyphs: Vec<NCGlyph>,
        lig_mode: LigMode,
        sub_mode: SubMode,
        prefix: String,
        suffix: String,
        color: String,
    ) -> Self {
        let len = glyphs.len();

        let mut glyphs: Vec<Glyph> = match enc_rng_start {
            Some(start_pos) => glyphs
                .into_iter()
                .zip(start_pos..(start_pos + len))
                .map(|(NCGlyph { name, width, glyph }, idx)| {
                    let g = Glyph::new(
                        name.clone(),
                        Encoding::new(*ff_pos, Some(idx)),
                        width,
                        glyph,
                        lig_mode.to_string(name),
                        "".to_string(),
                    );
                    *ff_pos += 1;
                    g
                })
                .collect(),

            None => glyphs
                .into_iter()
                .map(|NCGlyph { name, width, glyph }| {
                    let g = Glyph::new(
                        name.clone(),
                        Encoding::new(*ff_pos, None),
                        width,
                        glyph,
                        lig_mode.to_string(name),
                        "".to_string(),
                    );
                    *ff_pos += 1;
                    g
                })
                .collect(),
        };

        let mut padding = Self::new_empty(ff_pos, 15 - ((len + 15) % 16), 0).glyphs;
        glyphs.append(&mut padding);

        Self {
            glyphs,
            prefix,
            suffix,
            color,
        }
    }

    fn new_from_spline_sets(
        ff_pos: &mut usize,
        enc_rng_start: Option<usize>,
        glyphs: &'static [GlyphDescriptor],
        lig_mode: LigMode,
        sub_mode: SubMode,
        fallback_width: usize,
        prefix: String,
        suffix: String,
        color: String,
    ) -> Self {
        let glyphs = glyphs
            .into_iter()
            .map(
                |GlyphDescriptor {
                     name,
                     spline_set,
                     width,
                 }| {
                    NCGlyph::new(
                        name.to_string(),
                        width.unwrap_or(fallback_width),
                        GlyphRep::new(spline_set.to_string(), vec![]),
                    )
                },
            )
            .collect();

        Self::new_from_glyph_vec(
            ff_pos,
            enc_rng_start,
            glyphs,
            lig_mode,
            sub_mode,
            prefix,
            suffix,
            color,
        )
    }

    fn new_from_refs(
        ff_pos: &mut usize,
        // used for references, not glyphs
        mut encoding: Encoding,
        glyphs: &'static [GlyphDescriptor],
        position: String,
        static_glyph_ref: Option<GlyphRef>,
        lig_mode: LigMode,
        sub_mode: SubMode,
        fallback_width: usize,
        prefix: String,
        suffix: String,
        color: String,
    ) -> Self {
        let glyphs: Vec<NCGlyph> = glyphs
            .into_iter()
            .map(
                |GlyphDescriptor {
                     name,
                     spline_set,
                     width,
                 }| {
                    let refs = vec![
                        Some(GlyphRef::new(encoding.clone(), position.clone())),
                        static_glyph_ref.clone(),
                    ]
                    .into_iter()
                    .flatten()
                    .collect();
                    let g = NCGlyph::new(
                        name.to_string(),
                        width.unwrap_or(fallback_width),
                        GlyphRep::new(String::default(), refs),
                    );
                    encoding = Encoding::new(
                        encoding.ff_pos + 1,
                        match encoding.enc_pos {
                            Some(n) => Some(n + 1),
                            None => None,
                        },
                    );
                    g
                },
            )
            .collect();

        Self::new_from_glyph_vec(
            ff_pos, None, glyphs, lig_mode, sub_mode, prefix, suffix, color,
        )
    }

    fn new_empty(ff_pos: &mut usize, count: usize, width: usize) -> Self {
        let end = *ff_pos + count;
        let mut glyphs = vec![];

        while *ff_pos < end {
            glyphs.push(Glyph::new(
                format!("empty{i:04}", i = *ff_pos),
                Encoding::new(*ff_pos, None),
                width,
                GlyphRep::default(),
                String::default(),
                String::default(),
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

    fn to_string(&self) -> String {
        let mut s = String::new();
        for g in &self.glyphs {
            s += &g.to_string(self.prefix.clone(), self.suffix.clone(), self.color.clone())
        }
        s
    }
}

fn main() -> std::io::Result<()> {
    let filename = format!("nasin-nanpa-{VERSION}.sfd");
    let mut file = File::create(filename)?;
    let mut ff_pos: usize = 0;

    let ctrl_block = GlyphBlock::new_from_enc_glyph_vec(
        &mut ff_pos,
        vec![
            HCGlyph::new(Some(0x0000), "NUL".to_string(), 0, GlyphRep::default()),
            HCGlyph::new(Some(0x200D), "ZWJ".to_string(), 0, GlyphRep::default()),
            HCGlyph::new(Some(0xFE00), "VAR01".to_string(), 0, GlyphRep::default()),
            HCGlyph::new(Some(0xFE01), "VAR02".to_string(), 0, GlyphRep::default()),
            HCGlyph::new(Some(0xFE02), "VAR03".to_string(), 0, GlyphRep::default()),
            HCGlyph::new(Some(0xFE03), "VAR04".to_string(), 0, GlyphRep::default()),
            HCGlyph::new(Some(0xFE04), "VAR05".to_string(), 0, GlyphRep::default()),
            HCGlyph::new(Some(0xFE05), "VAR06".to_string(), 0, GlyphRep::default()),
            HCGlyph::new(Some(0xFE06), "VAR07".to_string(), 0, GlyphRep::default()),
            HCGlyph::new(Some(0xFE07), "VAR08".to_string(), 0, GlyphRep::default()),
        ],
        String::default(),
        String::default(),
        "fa6791".to_string(),
    );

    let tok_ctrl_block = GlyphBlock::new_from_spline_sets(
        &mut ff_pos,
        Some(0xF1990),
        TOK_CTRL.as_slice(),
        LigMode::FromName,
        SubMode::None,
        500,
        String::default(),
        "Tok".to_string(),
        "aaafff".to_string(),
    );

    // start main
    let latn_block = GlyphBlock::new_from_spline_sets(
        &mut ff_pos,
        Some(0x0020),
        glyph_blocks::LATN.as_slice(),
        LigMode::None,
        SubMode::CartCont,
        500,
        String::default(),
        String::default(),
        "fffaaa".to_string(),
    );

    let tok_no_comb_block = GlyphBlock::new_from_enc_glyph_vec(
        &mut ff_pos,
        vec![
            HCGlyph::new(
                None,
                TOK_NO_COMB[0].name.to_string(),
                1000,
                GlyphRep::new(TOK_NO_COMB[0].spline_set.to_string(), vec![]),
            ),
            HCGlyph::new(
                None,
                TOK_NO_COMB[1].name.to_string(),
                1000,
                GlyphRep::new(TOK_NO_COMB[1].spline_set.to_string(), vec![]),
            ),
            HCGlyph::new(
                None,
                TOK_NO_COMB[2].name.to_string(),
                1000,
                GlyphRep::new(TOK_NO_COMB[2].spline_set.to_string(), vec![]),
            ),
            HCGlyph::new(
                Some(0xF199C),
                TOK_NO_COMB[3].name.to_string(),
                1000,
                GlyphRep::new(TOK_NO_COMB[3].spline_set.to_string(), vec![]),
            ),
            HCGlyph::new(
                Some(0xF199D),
                TOK_NO_COMB[4].name.to_string(),
                1000,
                GlyphRep::new(TOK_NO_COMB[4].spline_set.to_string(), vec![]),
            ),
            HCGlyph::new(
                None,
                TOK_NO_COMB[5].name.to_string(),
                1000,
                GlyphRep::new(TOK_NO_COMB[5].spline_set.to_string(), vec![]),
            ),
            HCGlyph::new(
                None,
                TOK_NO_COMB[6].name.to_string(),
                1000,
                GlyphRep::new(TOK_NO_COMB[6].spline_set.to_string(), vec![]),
            ),
        ],
        String::default(),
        "Tok".to_string(),
        "cccfff".to_string(),
    );

    let tok_block = GlyphBlock::new_from_spline_sets(
        &mut ff_pos,
        Some(0xF1900),
        TOK.as_slice(),
        LigMode::FromName,
        SubMode::CartCont,
        1000,
        String::default(),
        "Tok".to_string(),
        "888fff".to_string(),
    );

    let tok_ext_block = GlyphBlock::new_from_spline_sets(
        &mut ff_pos,
        Some(0xF19A0),
        TOK_EXT.as_slice(),
        LigMode::FromName,
        SubMode::CartCont,
        1000,
        String::default(),
        "Tok".to_string(),
        "666fff".to_string(),
    );

    //      start of combos section
    let tok_outer_block = GlyphBlock::new_from_spline_sets(
        &mut ff_pos,
        None,
        TOK_OUTER.as_slice(),
        LigMode::GlyphThenJoiner("Tok_joinScaleTok".to_string()),
        SubMode::CCAndRespawn,
        1000,
        String::default(),
        "Tok_joinScaleTok".to_string(),
        "eeeeff".to_string(),
    );
    // let tok_ext_outer_block = ();

    let tok_inner_block = GlyphBlock::new_from_spline_sets(
        &mut ff_pos,
        None,
        TOK_INNER.as_slice(),
        LigMode::JoinerThenGlyph("joinScaleTok_".to_string(), "Tok".to_string()),
        SubMode::CCAndRespawn,
        1000,
        "joinScaleTok_".to_string(),
        "Tok".to_string(),
        "eeeeff".to_string(),
    );
    // let tok_ext_inner_block = ();

    let tok_lower_block = GlyphBlock::new_from_spline_sets(
        &mut ff_pos,
        None,
        TOK_LOWER.as_slice(),
        LigMode::GlyphThenJoiner("Tok_joinStackTok".to_string()),
        SubMode::CCAndRespawn,
        1000,
        String::default(),
        "Tok_joinStackTok".to_string(),
        "eeeeff".to_string(),
    );
    // let tok_ext_lower_block = ();

    let tok_upper_block = GlyphBlock::new_from_refs(
        &mut ff_pos,
        tok_lower_block.glyphs[0].encoding.clone(),
        TOK_LOWER.as_slice(),
        "S 1 0 0 1 0 500 2".to_string(),
        None,
        LigMode::JoinerThenGlyph("joinStackTok_".to_string(), "Tok".to_string()),
        SubMode::CCAndRespawn,
        1000,
        "joinStackTok_".to_string(),
        "Tok".to_string(),
        "eeeeff".to_string(),
    );
    // let tok_ext_upper_block = ();

    let mut main_block = vec![
        latn_block,
        tok_no_comb_block,
        tok_block,
        tok_ext_block,
        tok_outer_block,
        // tok_ext_outer_block,
        tok_inner_block,
        // tok_ext_inner_block,
        tok_lower_block,
        // tok_ext_lower_block,
        tok_upper_block,
        // tok_ext_upper_block,
    ];

    let cart_block = ();
    let cont_block = ();

    let mut meta_block = vec![ctrl_block, tok_ctrl_block];
    meta_block.append(&mut main_block);
    // cart_block,
    // cont_block,

    let time = std::time::UNIX_EPOCH.elapsed().unwrap().as_secs();
    let glyphs_string = meta_block.iter().map(|b| b.to_string()).join("");
    writeln!(
        &mut file,
        "{HEADER}Version: {VERSION}\n{DETAILS1}ModificationTime: {time}{DETAILS2}{LOOKUPS}{OTHER}BeginChars: {ff_pos} {ff_pos}\n{glyphs_string}EndChars\nEndSplineFont",
    )?;
    Ok(())
}
