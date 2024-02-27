## AHK Scripts Guide

[The AutoHotKey scripts](/ahk-script/) maintained in this repository can be used to input [the appropriate UCSUR character or control character](https://www.kreativekorp.com/ucsur/charts/sitelen.html) by replacing specific strings of (Latin) characters you type. There are 3 versions, all of which work in a very similar way:
| Version | Format of file name | Source text example | Note |
| ------- | ------------------- | ------------------- | ---- |
| Main version | `sitelen-pona-X.Y.ahk` | ``akesi`​`` | ``​`​`` ≈ confirm.
| Shorthand version | `stl-pon-X.Y.ahk` | ``aks`​`` | ``​`​`` ≈ confirm. All the words have 3 letter codes (you can view them by right-clicking the script and selecting 'Edit in Notepad')
| Toggle version | `sitelen-pona-toggle-X.Y.ahk` | `akesi ` | ` ` ≈ confirm. Script can be toggled on/off with `Alt+Space`

All 3 scripts also support *most* of the control characters described in the __Ligatures Guide__ in their respective formats.
Hopefully, soon, this will say *all* instead of most, but there's some work to be done before I can claim that.

Note: the script version numbering is independent of nasin nanpa's main version numbering.
