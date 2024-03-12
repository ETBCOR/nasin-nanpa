use ffir::*;
use glyph_blocks::*;
use itertools::Itertools;
use std::{fs::File, io::Write};

mod ffir;
mod glyph_blocks;
/*
fn gen_template() -> std::io::Result<()> {
    let filename = format!("nasin-nanpa-import-template.sfd");
    let mut file = File::create(filename)?;
    let mut ff_pos: usize = 0;

    let glyphs = TOK
        .into_iter()
        .map(|GlyphDescriptor { name, .. }| {
            NCGlyph::new(
                name.to_string(),
                1000,
                GlyphRep::new(String::default(), vec![]),
            )
        })
        .collect();

    let tok_block = GlyphBlock::new_from_glyph_vec(
        &mut ff_pos,
        None,
        glyphs,
        LigMode::None,
        SubMode::None,
        String::default(),
        String::default(),
        "4444ff".to_string(),
    );

    let time = std::time::UNIX_EPOCH.elapsed().unwrap().as_secs();
    let glyphs_string = tok_block.to_string();
    writeln!(
        &mut file,
        "{HEADER}Version: {VERSION}\n{DETAILS1}ModificationTime: {time}{DETAILS2}{OTHER}BeginChars: {ff_pos} {ff_pos}\n{glyphs_string}EndChars\nEndSplineFont",
    )?;
    Ok(())
}
*/

fn gen_nasin_nanpa() -> std::io::Result<()> {
    let filename = format!("nasin-nanpa-{VERSION}.sfd");
    let mut file = File::create(filename)?;
    let mut ff_pos: usize = 0;

    //
    let ctrl_block = GlyphBlock::new_from_enc_glyphs(
        &mut ff_pos,
        vec![
            GlyphEnc::new_from_parts(EncPos::Pos(0x0000), "NUL", 0, Rep::default()),
            GlyphEnc::new_from_parts(EncPos::Pos(0x200D), "ZWJ", 0, Rep::default()),
            GlyphEnc::new_from_parts(EncPos::Pos(0xFE00), "VAR01", 0, Rep::default()),
            GlyphEnc::new_from_parts(EncPos::Pos(0xFE01), "VAR02", 0, Rep::default()),
            GlyphEnc::new_from_parts(EncPos::Pos(0xFE02), "VAR03", 0, Rep::default()),
            GlyphEnc::new_from_parts(EncPos::Pos(0xFE03), "VAR04", 0, Rep::default()),
            GlyphEnc::new_from_parts(EncPos::Pos(0xFE04), "VAR05", 0, Rep::default()),
            GlyphEnc::new_from_parts(EncPos::Pos(0xFE05), "VAR06", 0, Rep::default()),
            GlyphEnc::new_from_parts(EncPos::Pos(0xFE06), "VAR07", 0, Rep::default()),
            GlyphEnc::new_from_parts(EncPos::Pos(0xFE07), "VAR08", 0, Rep::default()),
            GlyphEnc::new_from_parts(
                EncPos::None,
                "combCartExtHalfTok",
                0,
                Rep::new(
                    r#"
-550 -150 m 4
 -550 -122 -528 -100 -500 -100 c 6
 0 -100 l 2
 28 -100 50 -122 50 -150 c 0
 50 -178 28 -200 0 -200 c 2
 -500 -200 l 6
 -528 -200 -550 -178 -550 -150 c 4
-550 950 m 4
 -550 978 -528 1000 -500 1000 c 6
 0 1000 l 2
 28 1000 50 978 50 950 c 0
 50 922 28 900 0 900 c 2
 -500 900 l 6
 -528 900 -550 922 -550 950 c 4"#,
                    vec![],
                ),
            ),
            GlyphEnc::new_from_parts(
                EncPos::None,
                "combLongGlyphExtHalfTok",
                0,
                Rep::new(
                    r#"
-550 -150 m 4
 -550 -122 -528 -100 -500 -100 c 6
 0 -100 l 2
 28 -100 50 -122 50 -150 c 0
 50 -178 28 -200 0 -200 c 2
 -500 -200 l 6
 -528 -200 -550 -178 -550 -150 c 4"#,
                    vec![],
                ),
            ),
            GlyphEnc::new_from_parts(
                EncPos::None,
                "combCartExt1TickTok",
                0,
                Rep::new(
                    r#"
-500 -100 m 0
 -472 -100 -450 -122 -450 -150 c 2
 -450 -250 l 2
 -450 -278 -472 -300 -500 -300 c 0
 -528 -300 -550 -278 -550 -250 c 2
 -550 -150 l 2
 -550 -122 -528 -100 -500 -100 c 0"#,
                    vec![],
                ),
            ),
            GlyphEnc::new_from_parts(
                EncPos::None,
                "combCartExt2TickTok",
                0,
                Rep::new(
                    r#"
-400 -100 m 0
 -372 -100 -350 -122 -350 -150 c 2
 -350 -250 l 2
 -350 -278 -372 -300 -400 -300 c 0
 -428 -300 -450 -278 -450 -250 c 2
 -450 -150 l 2
 -450 -122 -428 -100 -400 -100 c 0
-600 -100 m 0
 -572 -100 -550 -122 -550 -150 c 2
 -550 -250 l 2
 -550 -278 -572 -300 -600 -300 c 0
 -628 -300 -650 -278 -650 -250 c 2
 -650 -150 l 2
 -650 -122 -628 -100 -600 -100 c 0"#,
                    vec![],
                ),
            ),
            GlyphEnc::new_from_parts(
                EncPos::None,
                "combCartExt3TickTok",
                0,
                Rep::new(
                    r#"
-300 -100 m 0
 -272 -100 -250 -122 -250 -150 c 2
 -250 -250 l 2
 -250 -278 -272 -300 -300 -300 c 0
 -328 -300 -350 -278 -350 -250 c 2
 -350 -150 l 2
 -350 -122 -328 -100 -300 -100 c 0
-500 -100 m 0
 -472 -100 -450 -122 -450 -150 c 2
 -450 -250 l 2
 -450 -278 -472 -300 -500 -300 c 0
 -528 -300 -550 -278 -550 -250 c 2
 -550 -150 l 2
 -550 -122 -528 -100 -500 -100 c 0
-700 -100 m 0
 -672 -100 -650 -122 -650 -150 c 2
 -650 -250 l 2
 -650 -278 -672 -300 -700 -300 c 0
 -728 -300 -750 -278 -750 -250 c 2
 -750 -150 l 2
 -750 -122 -728 -100 -700 -100 c 0"#,
                    vec![],
                ),
            ),
            GlyphEnc::new_from_parts(
                EncPos::None,
                "combCartExt4TickTok",
                0,
                Rep::new(
                    r#"
-400 -100 m 0
 -372 -100 -350 -122 -350 -150 c 2
 -350 -250 l 2
 -350 -278 -372 -300 -400 -300 c 0
 -428 -300 -450 -278 -450 -250 c 2
 -450 -150 l 2
 -450 -122 -428 -100 -400 -100 c 0
-200 -100 m 0
 -172 -100 -150 -122 -150 -150 c 2
 -150 -250 l 2
 -150 -278 -172 -300 -200 -300 c 0
 -228 -300 -250 -278 -250 -250 c 2
 -250 -150 l 2
 -250 -122 -228 -100 -200 -100 c 0
-600 -100 m 0
 -572 -100 -550 -122 -550 -150 c 2
 -550 -250 l 2
 -550 -278 -572 -300 -600 -300 c 0
 -628 -300 -650 -278 -650 -250 c 2
 -650 -150 l 2
 -650 -122 -628 -100 -600 -100 c 0
-800 -100 m 0
 -772 -100 -750 -122 -750 -150 c 2
 -750 -250 l 2
 -750 -278 -772 -300 -800 -300 c 0
 -828 -300 -850 -278 -850 -250 c 2
 -850 -150 l 2
 -850 -122 -828 -100 -800 -100 c 0"#,
                    vec![],
                ),
            ),
        ],
        LookupsMode::WordLigManual(vec![
            "".to_string(),
            "ampersand".to_string(),
            "one".to_string(),
            "two".to_string(),
            "three".to_string(),
            "four".to_string(),
            "five".to_string(),
            "six".to_string(),
            "seven".to_string(),
            "eight".to_string(),
            "".to_string(),
            "".to_string(),
            "comma".to_string(),
            "comma comma".to_string(),
            "comma comma comma".to_string(),
            "comma comma comma comma".to_string(),
        ]),
        true,
        "",
        "",
        "fa6791",
    );

    let tok_ctrl_block = GlyphBlock::new_from_constants(
        &mut ff_pos,
        TOK_CTRL.as_slice(),
        LookupsMode::WordLigManual(vec![
            "bracketleft".to_string(),
            "bracketright".to_string(),
            "equal".to_string(),
            "".to_string(),
            "".to_string(),
            "hyphen".to_string(),
            "plus".to_string(),
            "parenleft".to_string(),
            "parenright".to_string(),
            "underscore".to_string(),
            "braceleft".to_string(),
            "braceright".to_string(),
        ]),
        false,
        "",
        "Tok",
        "aaafff",
        EncPos::Pos(0xF1990),
        0,
    );

    let mut start_long_glyph_block = GlyphBlock::new_from_constants(
        &mut ff_pos,
        START_LONG_GLYPH.as_slice(),
        LookupsMode::StartLongGlyph,
        false,
        "",
        "Tok_startLongGlyphTok",
        "aaafff",
        EncPos::None,
        1000,
    );
    start_long_glyph_block.glyphs[7].lookups = Lookups::StartLongGlyphRev;

    // start main block
    let latn_block = GlyphBlock::new_from_constants(
        &mut ff_pos,
        glyph_blocks::LATN.as_slice(),
        LookupsMode::None,
        true,
        "",
        "",
        "fffaaa",
        EncPos::Pos(0x0020),
        500,
    );

    let mut tok_no_comb_block = GlyphBlock::new_from_constants(
        &mut ff_pos,
        TOK_NO_COMB.as_slice(),
        LookupsMode::WordLigManual(vec![
            "period".to_string(),
            "colon".to_string(),
            "period period".to_string(),
            "period period period".to_string(),
            "i t a n".to_string(),
            "l i p a m a n k a".to_string(),
            "l e p e k a ".to_string(),
        ]),
        true,
        "",
        "Tok",
        "cccfff",
        EncPos::None,
        1000,
    );
    tok_no_comb_block.glyphs[0].encoding.enc_pos = EncPos::Pos(0xF199C);
    tok_no_comb_block.glyphs[1].encoding.enc_pos = EncPos::Pos(0xF199D);

    //
    let tok_block = GlyphBlock::new_from_constants(
        &mut ff_pos,
        TOK.as_slice(),
        LookupsMode::WordLigFromLetters,
        true,
        "",
        "Tok",
        "bf80ff",
        EncPos::Pos(0xF1900),
        1000,
    );

    let tok_ext_block = GlyphBlock::new_from_constants(
        &mut ff_pos,
        TOK_EXT.as_slice(),
        LookupsMode::WordLigFromLetters,
        true,
        "",
        "Tok",
        "df80ff",
        EncPos::Pos(0xF19A0),
        1000,
    );

    let tok_alt_block = GlyphBlock::new_from_constants(
        &mut ff_pos,
        TOK_ALT.as_slice(),
        LookupsMode::WordLigFromLetters,
        true,
        "",
        "Tok",
        "ff80e6",
        EncPos::None,
        1000,
    );

    let tok_outer_block = GlyphBlock::new_from_constants(
        &mut ff_pos,
        TOK_OUTER.as_slice(),
        LookupsMode::ComboFirst,
        true,
        "",
        "Tok_joinScaleTok",
        "ffff",
        EncPos::None,
        1000,
    );

    let tok_inner_block = GlyphBlock::new_from_constants(
        &mut ff_pos,
        TOK_INNER.as_slice(),
        LookupsMode::ComboSecond,
        true,
        "joinScaleTok_",
        "Tok",
        "80ffff",
        EncPos::None,
        1000,
    );

    let tok_lower_block = GlyphBlock::new_from_constants(
        &mut ff_pos,
        TOK_LOWER.as_slice(),
        LookupsMode::ComboFirst,
        true,
        "",
        "Tok_joinStackTok",
        "ff00",
        EncPos::None,
        1000,
    );

    let tok_upper_block = GlyphBlock::new_from_constants(
        &mut ff_pos,
        TOK_UPPER.as_slice(),
        LookupsMode::ComboSecond,
        true,
        "joinStackTok_",
        "Tok",
        "80ff80",
        EncPos::None,
        1000,
    );

    // let tok_upper_block = tok_lower_block.new_from_refs(
    //     &mut ff_pos,
    //     "S 1 0 0 1 0 500 2".to_string(),
    //     None,
    //     LookupsMode::ComboSecond,
    //     true,
    //     false,
    //     "joinStackTok_",
    //     "Tok",
    //     "80ff80",
    // );

    let mut main_blocks = vec![
        latn_block,
        tok_no_comb_block,
        tok_block,
        tok_ext_block,
        tok_alt_block,
        tok_outer_block,
        tok_inner_block,
        tok_lower_block,
        tok_upper_block,
    ];

    let ctrl_names = ctrl_block
        .glyphs
        .iter()
        .filter_map(|glyph| {
            if glyph.glyph.name.contains("Half") {
                None
            } else {
                Some(format!(
                    "{}{}{}",
                    ctrl_block.prefix, glyph.glyph.name, ctrl_block.suffix
                ))
            }
        })
        .join(" ");

    let main_names = main_blocks
        .iter()
        .map(|block| {
            block
                .glyphs
                .iter()
                .map(|glyph| format!("{}{}{}", block.prefix, glyph.glyph.name, block.suffix))
                .join(" ")
        })
        .join(" ");

    let base = format!(
        "Class: 17906 {} joinStackTok joinScaleTok {}",
        ctrl_names, main_names
    );
    let cart = "Class: 46 combCartExtHalfTok startCartTok combCartExtTok";
    let cont = "Class: 599 combLongGlyphExtHalfTok startLongPiTok combLongPiExtTok startLongGlyphTok combLongGlyphExtTok startRevLongGlyphTok aTok_startLongGlyphTok alasaTok_startLongGlyphTok anuTok_startLongGlyphTok awenTok_startLongGlyphTok kamaTok_startLongGlyphTok kenTok_startLongGlyphTok kepekenTok_startLongGlyphTok laTok_startLongGlyphTok lonTok_startLongGlyphTok nanpaTok_startLongGlyphTok openTok_startLongGlyphTok piTok_startLongGlyphTok piniTok_startLongGlyphTok sonaTok_startLongGlyphTok tawaTok_startLongGlyphTok wileTok_startLongGlyphTok wile1Tok_startLongGlyphTok nTok_startLongGlyphTok waTok_startLongGlyphTok";

    let classes = format!("  {base}\n  {cart}\n  {cont}\n  B{base}\n  B{cart}\n  B{cont}\n  F{base}\n  F{cart}\n  F{cont}\n");

    let mut meta_block = vec![ctrl_block, tok_ctrl_block, start_long_glyph_block];
    meta_block.append(&mut main_blocks);

    let time = std::time::UNIX_EPOCH.elapsed().unwrap().as_secs();
    let glyphs_string = meta_block.iter().map(|block| block.gen()).join("");
    writeln!(
        &mut file,
        "{HEADER}Version: {VERSION}\n{DETAILS1}ModificationTime: {time}{DETAILS2}{LOOKUPS}{classes}{OTHER}BeginChars: {ff_pos} {ff_pos}\n{glyphs_string}EndChars\nEndSplineFont",
    )
}

fn main() -> std::io::Result<()> {
    gen_nasin_nanpa()?;
    Ok(())
}
