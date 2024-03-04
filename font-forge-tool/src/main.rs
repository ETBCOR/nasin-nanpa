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
        ],
        LookupsMode::WordLigManual(vec![
            "".to_string(),
            "".to_string(),
            "one".to_string(),
            "two".to_string(),
            "three".to_string(),
            "four".to_string(),
            "five".to_string(),
            "six".to_string(),
            "seven".to_string(),
            "eight".to_string(),
        ]),
        false,
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
            "".to_string(),
            "".to_string(),
        ]),
        false,
        "",
        "Tok",
        "aaafff",
        EncPos::Pos(0xF1990),
        500,
    );

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
        "bf80ff",
        EncPos::Pos(0xF19A0),
        1000,
    );

    //
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
    let tok_ext_outer_block = GlyphBlock::new_from_constants(
        &mut ff_pos,
        TOK_EXT_OUTER.as_slice(),
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
    let tok_ext_inner_block = GlyphBlock::new_from_constants(
        &mut ff_pos,
        TOK_EXT_INNER.as_slice(),
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
    let tok_ext_lower_block = GlyphBlock::new_from_constants(
        &mut ff_pos,
        TOK_EXT_LOWER.as_slice(),
        LookupsMode::ComboFirst,
        true,
        "",
        "Tok_joinStackTok",
        "ff00",
        EncPos::None,
        1000,
    );

    let tok_upper_block = tok_lower_block.new_from_refs(
        &mut ff_pos,
        "S 1 0 0 1 0 500 2".to_string(),
        None,
        LookupsMode::ComboSecond,
        true,
        false,
        "joinStackTok_",
        "Tok",
        "80ff80",
    );
    let tok_ext_upper_block = tok_ext_lower_block.new_from_refs(
        &mut ff_pos,
        "S 1 0 0 1 0 500 2".to_string(),
        None,
        LookupsMode::ComboSecond,
        true,
        false,
        "joinStackTok_",
        "Tok",
        "80ff80",
    );

    let mut main_block = vec![
        latn_block,
        tok_no_comb_block,
        tok_block,
        tok_ext_block,
        tok_outer_block,
        tok_ext_outer_block,
        tok_inner_block,
        tok_ext_inner_block,
        tok_lower_block,
        tok_ext_lower_block,
        tok_upper_block,
        tok_ext_upper_block,
    ];

    let classes = main_block
        .iter()
        .map(|block| {
            block
                .glyphs
                .iter()
                .map(|glyph| format!("{}{}{}", block.prefix, glyph.glyph.name, block.suffix))
                .join(" ")
        })
        .join(" ");
    let classes = format!("  Class: 15884 {classes}\n  Class: 27 startCartTok combCartExtTok\n  Class: 72 startLongPiTok combLongGlyphExtTok startRevLongGlyphTok\n BClass: 15884 s\n  BClass: 15884 {classes}\n  BClass: 27 startCartTok combCartExtTok\n  BClass: 72 startLongPiTok combLongGlyphExtTok startRevLongGlyphTok\n BClass: 15884 s\n  FClass: 15884 {classes}\n  FClass: 27 startCartTok combCartExtTok\n  FClass: 73 startLongPiTok combLongGlyphExtTok startRevLongGlyphTok\n BClass: 15884 s\n\n");

    let mut meta_block = vec![ctrl_block, tok_ctrl_block];
    meta_block.append(&mut main_block);

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
