use ffir::*;
use glyph_blocks::*;
use itertools::Itertools;
use std::{collections::HashSet, fs::File, io::Write};

mod ffir;
mod glyph_blocks;

#[derive(PartialEq, Eq, Clone, Copy)]
enum NasinNanpaVariation {
    Main,
    Ucsur,
}

fn gen_nasin_nanpa(variation: NasinNanpaVariation) -> std::io::Result<()> {
    let mut ff_pos: usize = 0;

    let mut ctrl_block = GlyphBlock::new_from_enc_glyphs(
        &mut ff_pos,
        vec![
            GlyphEnc::new_from_parts(EncPos::Pos(0xFE00), "VAR01", 0, Rep::default()),
            GlyphEnc::new_from_parts(EncPos::Pos(0xFE01), "VAR02", 0, Rep::default()),
            GlyphEnc::new_from_parts(EncPos::Pos(0xFE02), "VAR03", 0, Rep::default()),
            GlyphEnc::new_from_parts(EncPos::Pos(0xFE03), "VAR04", 0, Rep::default()),
            GlyphEnc::new_from_parts(EncPos::Pos(0xFE04), "VAR05", 0, Rep::default()),
            GlyphEnc::new_from_parts(EncPos::Pos(0xFE05), "VAR06", 0, Rep::default()),
            GlyphEnc::new_from_parts(EncPos::Pos(0xFE06), "VAR07", 0, Rep::default()),
            GlyphEnc::new_from_parts(EncPos::Pos(0xFE07), "VAR08", 0, Rep::default()),
            GlyphEnc::new_from_parts(EncPos::Pos(0x2190), "arrowW", 0, Rep::default()),
            GlyphEnc::new_from_parts(EncPos::Pos(0x2191), "arrowN", 0, Rep::default()),
            GlyphEnc::new_from_parts(EncPos::Pos(0x2192), "arrowE", 0, Rep::default()),
            GlyphEnc::new_from_parts(EncPos::Pos(0x2193), "arrowS", 0, Rep::default()),
            GlyphEnc::new_from_parts(EncPos::Pos(0x2196), "arrowNW", 0, Rep::default()),
            GlyphEnc::new_from_parts(EncPos::Pos(0x2197), "arrowNE", 0, Rep::default()),
            GlyphEnc::new_from_parts(EncPos::Pos(0x2198), "arrowSE", 0, Rep::default()),
            GlyphEnc::new_from_parts(EncPos::Pos(0x2199), "arrowSW", 0, Rep::default()),
            GlyphEnc::new_from_parts(EncPos::Pos(0x0000), "NUL", 0, Rep::default()),
            GlyphEnc::new_from_parts(EncPos::Pos(0x200C), "ZWNJ", 0, Rep::default()),
            GlyphEnc::new_from_parts(EncPos::Pos(0x200D), "ZWJ", 0, Rep::default()),
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
        if variation == NasinNanpaVariation::Main {
            LookupsMode::WordLigManual(vec![
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                "arrow".to_string(),
                "arrow".to_string(),
                "arrow".to_string(),
                "arrow".to_string(),
                "arrow".to_string(),
                "arrow".to_string(),
                "arrow".to_string(),
                "arrow".to_string(),
                String::new(),
                "bar".to_string(),
                "ampersand".to_string(),
                String::new(),
                String::new(),
                "comma".to_string(),
                "comma comma".to_string(),
                "comma comma comma".to_string(),
                "comma comma comma comma".to_string(),
            ])
        } else {
            LookupsMode::None
        },
        Cc::Participant,
        "",
        "",
        "fa6791",
    );
    ctrl_block.glyphs[0].cc_subs = Cc::None;
    ctrl_block.glyphs[10].cc_subs = Cc::None;
    ctrl_block.glyphs[11].cc_subs = Cc::None;

    let mut tok_ctrl_block = GlyphBlock::new_from_constants(
        &mut ff_pos,
        TOK_CTRL.as_slice(),
        if variation == NasinNanpaVariation::Main {
            LookupsMode::WordLigManual(vec![
                "bracketleft".to_string(),
                "bracketright".to_string(),
                "equal".to_string(),
                String::new(),
                String::new(),
                "hyphen".to_string(),
                "plus".to_string(),
                "parenleft".to_string(),
                "parenright".to_string(),
                "underscore".to_string(),
                "braceleft".to_string(),
                "braceright".to_string(),
                "startCartAlt".to_string(),
                "endCartAlt".to_string(),
            ])
        } else {
            LookupsMode::None
        },
        Cc::None,
        "",
        "Tok",
        "aaafff",
        EncPos::Pos(0xF1990),
        0,
    );
    tok_ctrl_block.glyphs[5].cc_subs = Cc::Participant;
    tok_ctrl_block.glyphs[6].cc_subs = Cc::Participant;
    tok_ctrl_block.glyphs[12].encoding.enc_pos = EncPos::None;
    tok_ctrl_block.glyphs[13].encoding.enc_pos = EncPos::None;

    let mut start_long_glyph_block = GlyphBlock::new_from_constants(
        &mut ff_pos,
        START_LONG_GLYPH.as_slice(),
        LookupsMode::StartLongGlyph,
        Cc::None,
        "",
        "_startLongGlyphTok",
        "aaafff",
        EncPos::None,
        1000,
    );
    start_long_glyph_block.glyphs[7].lookups = Lookups::EndLongGlyph;

    let latn_block = if variation == NasinNanpaVariation::Main {
        GlyphBlock::new_from_constants(
            &mut ff_pos,
            glyph_blocks::LATN.as_slice(),
            LookupsMode::None,
            Cc::Half,
            "",
            "",
            "fffaaa",
            EncPos::Pos(0x0020),
            500,
        )
    } else {
        GlyphBlock::new_empty(&mut ff_pos, 0, 0)
    };

    let mut tok_no_comb_block = GlyphBlock::new_from_constants(
        &mut ff_pos,
        TOK_NO_COMB.as_slice(),
        if variation == NasinNanpaVariation::Main {
            LookupsMode::WordLigManual(vec![
                "period".to_string(),
                "colon".to_string(),
                "middleDotTok middleDotTok".to_string(),
                "middleDotTok middleDotTok middleDotTok".to_string(),
                "space space".to_string(),
                "t e".to_string(),
                "t o".to_string(),
                "i t a n".to_string(),
                "l i p a m a n k a".to_string(),
                "l e p e k a".to_string(),
            ])
        } else {
            LookupsMode::None
        },
        Cc::Full,
        "",
        "Tok",
        "cccfff",
        EncPos::None,
        1000,
    );
    tok_no_comb_block.glyphs[0].encoding.enc_pos = EncPos::Pos(0xF199C);
    tok_no_comb_block.glyphs[1].encoding.enc_pos = EncPos::Pos(0xF199D);
    tok_no_comb_block.glyphs[4].encoding.enc_pos = EncPos::Pos(0x3000);

    let tok_block = GlyphBlock::new_from_constants(
        &mut ff_pos,
        TOK.as_slice(),
        if variation == NasinNanpaVariation::Main {
            LookupsMode::WordLigFromLetters
        } else {
            LookupsMode::None
        },
        Cc::Full,
        "",
        "Tok",
        "bf80ff",
        EncPos::Pos(0xF1900),
        1000,
    );

    let mut tok_ext_block = GlyphBlock::new_from_constants(
        &mut ff_pos,
        TOK_EXT.as_slice(),
        if variation == NasinNanpaVariation::Main {
            LookupsMode::WordLigFromLetters
        } else {
            LookupsMode::None
        },
        Cc::Full,
        "",
        "Tok",
        "df80ff",
        EncPos::Pos(0xF19A0),
        1000,
    );
    tok_ext_block.glyphs[8].encoding.enc_pos = EncPos::Pos(0xf19ae);
    tok_ext_block.glyphs[7].encoding.enc_pos = EncPos::Pos(0xf19c0);

    let tok_alt_block = GlyphBlock::new_from_constants(
        &mut ff_pos,
        TOK_ALT.as_slice(),
        LookupsMode::Alt,
        Cc::Full,
        "",
        "",
        "ff80e6",
        EncPos::None,
        1000,
    );

    let tok_outer_block = GlyphBlock::new_from_constants(
        &mut ff_pos,
        TOK_OUTER.as_slice(),
        LookupsMode::ComboFirst,
        Cc::Full,
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
        Cc::Full,
        "",
        "Tok_joinScaleTok",
        "ffff",
        EncPos::None,
        1000,
    );

    let tok_alt_outer_block = GlyphBlock::new_from_constants(
        &mut ff_pos,
        TOK_ALT_OUTER.as_slice(),
        LookupsMode::ComboFirst,
        Cc::Full,
        "",
        "_joinScaleTok",
        "ffff",
        EncPos::None,
        1000,
    );

    let tok_inner_block = GlyphBlock::new_from_constants(
        &mut ff_pos,
        TOK_INNER.as_slice(),
        LookupsMode::ComboLast,
        Cc::Full,
        "joinScaleTok_",
        "Tok",
        "80ffff",
        EncPos::None,
        0,
    );

    let tok_ext_inner_block = GlyphBlock::new_from_constants(
        &mut ff_pos,
        TOK_EXT_INNER.as_slice(),
        LookupsMode::ComboLast,
        Cc::Full,
        "joinScaleTok_",
        "Tok",
        "80ffff",
        EncPos::None,
        0,
    );

    let tok_alt_inner_block = GlyphBlock::new_from_constants(
        &mut ff_pos,
        TOK_ALT_INNER.as_slice(),
        LookupsMode::ComboLast,
        Cc::Full,
        "joinScaleTok_",
        "",
        "80ffff",
        EncPos::None,
        0,
    );

    let tok_lower_block = GlyphBlock::new_from_constants(
        &mut ff_pos,
        TOK_LOWER.as_slice(),
        LookupsMode::ComboFirst,
        Cc::Full,
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
        Cc::Full,
        "",
        "Tok_joinStackTok",
        "ff00",
        EncPos::None,
        1000,
    );

    let tok_alt_lower_block = GlyphBlock::new_from_constants(
        &mut ff_pos,
        TOK_ALT_LOWER.as_slice(),
        LookupsMode::ComboFirst,
        Cc::Full,
        "",
        "_joinStackTok",
        "ff00",
        EncPos::None,
        1000,
    );

    let tok_upper_block = tok_lower_block.new_from_refs(
        &mut ff_pos,
        "S 1 0 0 1 -1000 500 2".to_string(),
        None,
        LookupsMode::ComboLast,
        Cc::Full,
        false,
        "joinStackTok_",
        "Tok",
        "80ff80",
        Some(0),
        Some(Anchor::new_stack(AnchorType::Mark)),
    );

    let tok_ext_upper_block = tok_ext_lower_block.new_from_refs(
        &mut ff_pos,
        "S 1 0 0 1 -1000 500 2".to_string(),
        None,
        LookupsMode::ComboLast,
        Cc::Full,
        false,
        "joinStackTok_",
        "Tok",
        "80ff80",
        Some(0),
        Some(Anchor::new_stack(AnchorType::Mark)),
    );

    let tok_alt_upper_block = tok_alt_lower_block.new_from_refs(
        &mut ff_pos,
        "S 1 0 0 1 -1000 500 2".to_string(),
        None,
        LookupsMode::ComboLast,
        Cc::Full,
        false,
        "joinStackTok_",
        "",
        "80ff80",
        Some(0),
        Some(Anchor::new_stack(AnchorType::Mark)),
    );

    let context_subs = {
        let scale_names = vec![&tok_outer_block, &tok_ext_outer_block, &tok_alt_outer_block]
            .iter()
            .enumerate()
            .map(|(i, &block)| {
                block
                    .glyphs
                    .iter()
                    .filter_map(|glyph| {
                        if glyph.glyph.name.contains("empty") {
                            None
                        } else {
                            Some(format!(
                                "{}{}",
                                glyph.glyph.name,
                                if i != 2 { "Tok" } else { "" }
                            ))
                        }
                    })
                    .join(" ")
            })
            .join(" ");

        let scale_glyphs = vec![&tok_outer_block, &tok_ext_outer_block, &tok_alt_outer_block]
            .iter()
            .map(|block| {
                block
                    .glyphs
                    .iter()
                    .filter_map(|glyph| {
                        if glyph.glyph.name.contains("empty") {
                            None
                        } else {
                            Some(glyph.glyph.name.clone())
                        }
                    })
                    .collect_vec()
            })
            .flatten()
            .collect::<HashSet<_>>();

        let stack_names = vec![&tok_lower_block, &tok_ext_lower_block, &tok_alt_lower_block]
            .iter()
            .enumerate()
            .map(|(i, block)| {
                block
                    .glyphs
                    .iter()
                    .filter_map(|glyph| {
                        if glyph.glyph.name.contains("empty")
                            || glyph.glyph.name.contains("arrow")
                            || scale_glyphs.contains(&glyph.glyph.name)
                        {
                            None
                        } else {
                            Some(format!(
                                "{}{}",
                                glyph.glyph.name,
                                if i != 2 { "Tok" } else { "" }
                            ))
                        }
                    })
                    .join(" ")
            })
            .join(" ");

        let put_in_class = |orig: String| format!("Class: {} {}", orig.len(), orig);

        let zwj = put_in_class("ZWJ".to_string());
        let scale = put_in_class(scale_names);
        let stack = put_in_class(stack_names);

        let put_in_sub = |c: &str| format!("  {c}{zwj}\n  {c}{scale}\n  {c}{stack}\n");

        let subs = format!("{}{}{}", put_in_sub(""), put_in_sub("B"), put_in_sub("F"));

        format!("ContextSub2: class \"'calt' SPECIFIC COMBOS\" 4 4 4 2\n{subs}")
    };

    let mut main_blocks = vec![
        latn_block,
        tok_no_comb_block,
        tok_block,
        tok_ext_block,
        tok_alt_block,
        tok_outer_block,
        tok_ext_outer_block,
        tok_alt_outer_block,
        tok_inner_block,
        tok_ext_inner_block,
        tok_alt_inner_block,
        tok_lower_block,
        tok_ext_lower_block,
        tok_alt_lower_block,
        tok_upper_block,
        tok_ext_upper_block,
        tok_alt_upper_block,
    ];

    let chain_subs = {
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

        let put_in_class = |orig: String| format!("Class: {} {}", orig.len(), orig);

        let base = put_in_class(format!(
            "{} joinStackTok joinScaleTok {}",
            ctrl_names, main_names
        ));

        let cart = put_in_class("combCartExtHalfTok startCartTok combCartExtTok startCartAltTok".to_string());

        let cont = start_long_glyph_block
            .glyphs
            .iter()
            .filter_map(|glyph| {
                if glyph.glyph.name.eq("la") {
                    None
                } else {
                    Some(format!(
                        "{}{}{}",
                        start_long_glyph_block.prefix,
                        glyph.glyph.name,
                        start_long_glyph_block.suffix
                    ))
                }
            })
            .join(" ");

        let cont = put_in_class(format!("combLongGlyphExtHalfTok startLongPiTok combLongPiExtTok startLongGlyphTok combLongGlyphExtTok startRevLongGlyphTok {}", cont));

        let put_in_sub = |c: &str| format!("  {c}{base}\n  {c}{cart}\n  {c}{cont}\n");

        let subs = format!("{}{}{}", put_in_sub(""), put_in_sub("B"), put_in_sub("F"));

        format!("ChainSub2: class \"'calt' CART AND CONT\" 4 4 4 2\n{subs}")
    };

    let mut meta_block = vec![ctrl_block, tok_ctrl_block, start_long_glyph_block];
    meta_block.append(&mut main_blocks);
    let glyphs_string = format!(
        "{}",
        meta_block.iter().map(|block| block.gen(variation)).join("")
    );

    let time = std::time::UNIX_EPOCH.elapsed().unwrap().as_secs();

    let filename = format!(
        "nasin-nanpa-{VERSION}{}.sfd",
        if variation == NasinNanpaVariation::Ucsur {
            "-UCSUR"
        } else {
            ""
        }
    );
    let mut file = File::create(filename)?;

    writeln!(
        &mut file,
        r#"{HEADER}Version: {VERSION}
{DETAILS1}ModificationTime: {time}{DETAILS2}{LOOKUPS}DEI: 91125
{context_subs}{AFTER_CONTEXT_SUBS}{chain_subs}{AFTER_CHAIN_SUBS}{VERSION}{OTHER}BeginChars: {ff_pos} {ff_pos}
{glyphs_string}EndChars
EndSplineFont
"#
    )
}

fn main() -> std::io::Result<()> {
    gen_nasin_nanpa(NasinNanpaVariation::Main)?;
    gen_nasin_nanpa(NasinNanpaVariation::Ucsur)?;
    Ok(())
}
