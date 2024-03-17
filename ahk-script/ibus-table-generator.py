import os
import uuid
import datetime

now = datetime.datetime.now().strftime("%Y%m%d")

# taken and modified from here: https://github.com/neroist/sitelen-pona-ucsur-guide/blob/main/tokipona.txt
# created by jan Komi and edited by authors of sitelen pona ucsur guide and kule Piton
def gen_table(needed_input_chars, file_name):
    table_id = str(uuid.uuid4())
    file_name = file_name.replace('.ahk', '').replace('-', ' ')
    needed_input_chars = ''.join(sorted(needed_input_chars))
    return f'''### File header must not be modified
### This file must be encoded into UTF-8.
### This table under LGPL
### comments start with ### not single #
### Derive from the format of SCIM Table, so you can modify the table from
### scim-tables' table
SCIM_Generic_Table_Phrase_Library_TEXT
VERSION_1_0

### Begin Table definition.
BEGIN_DEFINITION

### License
LICENSE = LGPL

### An unique id to distinguish this table among others.
### Use uuidgen to generate this kind of id.
UUID = {table_id}

### A unique number indicates the version of this file.
### For example the last modified date of this file.
### This number must be less than 2^32.
### Just make your table version-able
SERIAL_NUMBER = {now}

### ICON can be any format as long as your pygtk can recognized
### the most widely ones are "png" and "svg", letter one is recommended
ICON = ibus-table.svg

### The symbol to be displayed in IM switchers
SYMBOL = 󱥬

### The default name of this table, this is needed
NAME = {file_name}

### The local names of this table, this is optional
### NAME.zh_CN = 形码
### NAME.zh_HK = 形碼
### NAME.zh_TW = 形碼

### Description
DESCRIPTION = sitelen pona input method for IBus, ported from nasin-nanpa's ahk script.

### Supported languages of this table
### sigle "zh_CN" just be recognized as zh_CN,
### but "zh_CN, zh_HK" or more zh_XX will be recognized as zh;
### and "en_US, zh_CN" will be just ignored.
LANGUAGES = en_US

### The author of this table
AUTHOR = jan Komi

### Prompt string to be displayed in the status area, CN will be replaced by
### the gettext tools in runtime as 中.
STATUS_PROMPT = toki

### Valid input chars.
VALID_INPUT_CHARS = {needed_input_chars}

### Layout
LAYOUT = us

### The max number of input keys for every phrase or character.
MAX_KEY_LENGTH = 20

### Use auto_commit mode as default
AUTO_COMMIT = TRUE

### Automatically selects the first phrase when typing
AUTO_SELECT = FALSE

### Use full width punctuation by default
DEF_FULL_WIDTH_PUNCT = FALSE
### Not use full width letter by default
DEF_FULL_WIDTH_LETTER = FALSE

### Whether user are allow to define phrase, default is true
### You have to define the word construction rules below.
### For input methods which do not input phrases, set this to False
USER_CAN_DEFINE_PHRASE = FALSE

### Whether support PinYin Mode, default is true.
### this feature is just for Chinese, set it to False if your IM is not
### Chinese.
PINYIN_MODE = FALSE

### If true then the phrases' frequencies will be adjusted dynamically
### according your using frequency.
DYNAMIC_ADJUST = TRUE 

### Some characters whose frequencies should be fix all the time, e.g. 
### some punctuations
### NO_CHECK_CHARS = 

### Rules for constructing user defined phrase
### "ce" stands for "ci equal", a Chinese English :), means "phrase length
### equal to", thus ce2 -> phrase length equal to 2; and "ca" means "phrase
### length equal or above", so ca4 -> phrase length equal or above 4.
### p21 -> the 1st key of 2nd character in the phrase, and so on.
### Each rule separate via ";". 
### Example below is a complete rule-set, 
### becuase [2,2] ∩ [3,3] ∩ [4,+∞] = [2,+∞], which is the range of length
### of phrase. This have to be satisfied if you need ibus-table to build up
### your own inputed phrase via your daily using.
### RULES = ce2:p11+p12+p21+p22;ce3:p11+p21+p22+p31;ca4:p11+p21+p31+p41

### The key strokes to page up the lookup table.
### PAGE_UP_KEYS = Page_Up,KP_Page_Up,minus,comma

### The key strokes to page down.
### PAGE_DOWN_KEYS = Page_Down,KP_Page_Down,equal,period

### The key strokes to select candidiate phrases.
### Usually "1,2,3,4,5,6,7,8,9" but if this conflicts with
### characters one wants to use for input one can also
### use something like “F1,F2,F3,F4,F5,F6,F7,F8,F9”
SELECT_KEYS = 1,2,3,4,5,6,7,8,9

### The default orientation of the candidate list
### TRUE means the candidate list is vertical, FALSE means it is horizontal
ORIENTATION=TRUE

END_DEFINITION

### Begin Table data.
### Format of every line whose formated in "input_keys\\tphrase\\tfreq\\n" is an
### entry.
### From left to right, the 1st column are the input key combination that you
### entered via keyboard; the 2nd column are presented character or phrase of
### the key combination you want; the 3rd column are frequency of the character
### or phrase.
BEGIN_TABLE
'''

def process(f, filename):
    lines = f.readlines()
    table = []
    needed_input_chars = set()
    new_file = filename.replace('.ahk', '.ibus-table')
    for line in lines:
        if not line.startswith('::'):
            continue
        line = line.split('::')[1:]
        if len(line) != 2:
            continue
        key = line[0].strip().replace('`', '').replace('=', '*')
        for c in key:
            needed_input_chars.add(c)
        comment_pos = line[1].find(';')
        if comment_pos != -1:
            phrase = line[1][:comment_pos].strip()
        else:
            phrase = line[1].strip()
        if phrase.startswith(':'):
            key = ':' + key
            phrase = phrase[1:]
            needed_input_chars.add(':')
        if phrase.startswith('{'):
            unicoide = int(phrase[3:-1], base=16)
            phrase = chr(unicoide)
        table.append((key, phrase))
    with open(new_file, 'w') as f:
        print(f'Writing {new_file}')
        f.write(gen_table(needed_input_chars, filename))
        for item in table:
            f.write(f'{item[0]}\t{item[1]}\t1\n')
        f.write('END_TABLE')

def main():
    current_dir = os.path.dirname(os.path.abspath(__file__))
    for filename in os.listdir(current_dir):
        if filename.endswith('.ahk'):
            if 'toggle' in filename:
                continue
            print(f'Processing {filename}')
            with open(filename, 'r') as f:
                process(f, filename)

if __name__ == '__main__':
    main()