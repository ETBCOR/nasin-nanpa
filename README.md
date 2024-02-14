# nasin sitelen tan anpa nanpa

## o lukin e [lipu Releases](https://github.com/ETBCOR/nasin-nanpa/releases) a!

![nasin nanpa 3.1.0](/renders/nasin-nanpa-3.1.0.png)

nasin sitelen tan anpa nanpa li kepeken nasin pi [lipu ni](https://www.kreativekorp.com/ucsur/charts/sitelen.html).

## Ligatures Guide
Ligatures are a font feature that allow nasin nanpa (and other many sitelen pona fonts) to display strings of existing Unicode characters as sitelen pona glyphs. However, not every text rendering context supports this font feature by default, and some may not at all! 
- type the name of any glyph in latin characters (with or without a trailing space) and it will turn into the correct sitelen pona
- ideographic space can be done with `  ` (double space) or `zz`
- cartouches get opened with `[` and closed with `]`
- cartouche colon is `:`
- cartouche interpunct is `.`
- manual lines are `_` for character extensions and `__` for cartouches (unless you want to do some fancy editing or opentype autocartouching isn't available, these shouldn't be necessary)
- containers (character extensions) get opened with `(` and closed with `)`
- scaled glyph combinations are done with `+`
- stacked glyph combinations are done with `-`
- directional ni can be done by adding `>` `^` `<` `v` and combinations thereof to `ni`
- if a glyph has variants, put a number after the word to access it

## AHK Scripts Guide

[The AutoHotKey scripts](/ahk-script/) maintained in this repository can be used to input [the appropriate UCSUR character or control character](https://www.kreativekorp.com/ucsur/charts/sitelen.html) by replacing specific strings of (Latin) characters you type. There are 3 versions, all of which work in a very similar way:
| Version | Format of file name | Source text example | Note |
| ------- | ------------------- | ------------------- | ---- |
| Main version | `sitelen-pona-X.Y.ahk` | ``akesi`​`` | ``​`​`` ≈ confirm.
| Shorthand version | `stl-pon-X.Y.ahk` | ``aks`​`` | ``​`​`` ≈ confirm. All the words have 3 letter codes (you can view them by right-clicking the script and selecting 'Edit in Notepad')
| Toggle version | `sitelen-pona-toggle-X.Y.ahk` | `akesi ` | ` ` ≈ confirm. Script can be toggled on/off with `Alt+Space`

All 3 scripts also support *most* of the control characters described in the __Ligatures Guide__ above in their respective formats.
Hopefully, soon, this will say *all* instead of most, but there's some work to be done before I can claim that.

There is also an [old](/ahk-script/old/) folder that contains older versions of the scripts.
Note: the script version numbering is independent of nasin nanpa's main version numbering.
