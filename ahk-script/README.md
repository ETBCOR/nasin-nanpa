## AHK Scripts Guide

[The AutoHotKey scripts](/ahk-script/) maintained in this repository can be used to input [the appropriate UCSUR character or control character](https://www.kreativekorp.com/ucsur/charts/sitelen.html) by replacing specific strings of (Latin) characters you type. There are 3 versions, all of which work in a very similar way:
| Version | Format of file name | Source text example | Note |
| ------- | ------------------- | ------------------- | ---- |
| Main version | `sitelen-pona-X.Y.ahk` | ``akesi`​`` | ``​`​`` ≈ confirm.
| Shorthand version | `stl-pon-X.Y.ahk` | ``aks`​`` | ``​`​`` ≈ confirm. All the words have 3 letter codes (you can view them by right-clicking the script and selecting 'Edit in Notepad')
| Toggle version | `sitelen-pona-toggle-X.Y.ahk` | `akesi ` | ` ` ≈ confirm. Script can be toggled on/off with `Alt+Space`

All 3 scripts also support *most* of the control characters described in the __Ligatures Guide__ in their respective formats.
Hopefully, soon, this will say *all* instead of most, but there's some work to be done before I can claim that.

Main script features:
*    Use `` [` `` and `` ]` `` for cartouches. (For alternate "extender" encoding, use `` _` ``)
*    Use `` (` `` and `` )` `` after an extendable glyph for extended glyphs (like pi).
*    Use `` -` `` for default joiner, `` ^` `` for stacking joiner, and `` *` `` for scaling joiner.
*    Use `` .` `` to type a sitelen pona middle dot, and `` :` `` for sitelen pona colon, and `` [space][space]` `` for fullwidth space.
*    Use `` <` `` and `` >` `` for CJK quotes.
*    Use `` ~` `` and `` ~~` `` for alternate glyphs.

Script variants:

*    New script variant: the toggle script! ("sitelen-pona-toggle-4.0.ahk") Instead of using `` ` `` after words, use <kbd>Alt</kbd> + <kbd>Space</kbd> to toggle the script on and off. While on (it will be on when it starts), simply type the word followed by a space. In the same way, the symbols don't require the `` ` `` symbol ( e.g.: [space][space] -> [fullwidth space] / [period] -> [sitelen pona period] ).
*    Use "sitelen-pona-4.0.ahk" to type full words followed by the `` ` `` character.
*    Use "stl-pon-4.0.ahk" to instead use 3-letter word abbreviations (with `` ` ``).

Note: the script version numbering is independent of nasin nanpa's main version numbering.
