# nasin sitelen tan anpa nanpa

## o lukin e [lipu Releases](https://github.com/ETBCOR/nasin-nanpa/releases) a!

![nasin nanpa 3.1.0](/renders/nasin-nanpa-3.1.0.png)

This font currently follows the the standard described in [this document](https://www.kreativekorp.com/ucsur/charts/sitelen.html).<br>
nasin sitelen tan anpa nanpa li kepeken nasin pi [lipu ni](https://www.kreativekorp.com/ucsur/charts/sitelen.html).

## Ligatures Guide
Ligatures are a font feature that allow nasin nanpa (and many other sitelen pona fonts) to display strings of existing Unicode characters as sitelen pona glyphs. However, not every text rendering context (web browser, text editing program, etc.) supports this font feature by default, and some may not at all (so see the **AHK Script Guide Section** at the end of this README)!

This table describes both the ligatures in nasin nanpa and the AutoHotKey scripts:
| Codepoint | Latin Text | Resulting Codepoint / Glyph |
| --------- | ---------- | --------------------------- |
| **U+F1900** -<br>**U+F1988** | `a`, `akesi` ... `wile` \| `namako` ... `ku` | _A_, _AKESI_ ... _WILE_ \| _NAMAKO_ ... _KU_ |
| **U+3000** | `  ` / `zz`| _IDEOGRAPHIC SPACE_ |
| **U+F1990** | `[` | _START OF CARTOUCHE_ |
| **U+F1991** | `]` | _END OF CARTOUCHE_ |
| **U+F1992** | `__` | _COMBINING CARTOUCHE EXTENSION_ |
| **U+F1993** | (none) | _START OF LONG PI_ |
| **U+F1994** | (none) | _COMBINING LONG PI EXTENSION_ |
| **U+F1995** | `-` | _STACKING JOINER_ |
| **U+F1996** | `+` | _SCALING JOINER_ |
| **U+F1997** | `(` | _START OF LONG GLYPH_ |
| **U+F1998** | `)` | _END OF LONG GLYPH_ |
| **U+F1999** | `_` | _COMBINING LONG GLYPH EXTENSION_ |
| **U+F199A** | (none) | _START OF REVERSE LONG GLYPH_ |
| **U+F199B** | (none) | _END OF REVERSE LONG GLYPH_ |
| **U+F199C** | `.` | _MIDDLE DOT_ |
| **U+F199D** | `:` | _COLON_ |
| **U+FE00** -<br>**U+FE03** | `1` - `4` | _VARIATION SELECTOR 1_ - _VARIATION SELECTOR 4_ |
| (none) | `itan` | jan Itan's personal glyph |
| (none) | `lepeka` | jan Lepeka's personal glyph |
| (none) | `lipamanka` | lipamanka's personal glyph |

## AHK Scripts Guide
[The AutoHotKey scripts](/ahk-script/) maintained in this repository can be used to input [the appropriate UCSUR character or control character](https://www.kreativekorp.com/ucsur/charts/sitelen.html) by replacing specific strings of (Latin) characters you type. There are 3 versions, all of which work in a very similar way:
| Version | Format of file name | Source text example | Note |
| ------- | ------------------- | ------------------- | ---- |
| Main version | `sitelen-pona-X.Y.ahk` | ``akesi`​`` | ``​`​`` ≈ confirm.
| Shorthand version | `stl-pon-X.Y.ahk` | ``aks`​`` | ``​`​`` ≈ confirm. All the words have 3 letter codes (you can view them by right-clicking the script and selecting 'Edit in Notepad')
| Toggle version | `sitelen-pona-toggle-X.Y.ahk` | `akesi ` | ` ` ≈ confirm. Script can be toggled on/off with `Alt+Space`

Note: the script version numbering is independent of nasin nanpa's main version numbering.
