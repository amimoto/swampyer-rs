#!/usr/bin/env python3

import swampyer.messages
from jinja2 import Environment, FileSystemLoader

env = Environment(loader=FileSystemLoader("."))
tmpl = env.get_template("entry-template.rs")

all_mfields = {}
message_entries = []
for mtype, mfields in swampyer.messages.MESSAGE_TYPES.items():
    #print(f"{mtype.capitalize()}")
    name = mtype.capitalize()

    field_count = len(mfields)
    code = mfields[0]

    for i, col in enumerate(mfields):
        class_name = col.__class__.__name__
        col.col_type = "Value"
        col.index = i
        col.col_type_name = class_name
        if class_name == 'URI':
            escaped_str = col.default.replace('"',r'\\"')
            col.col_default = r'\"' + escaped_str + r'\"'
            col.col_type = '&str'
            col.fetch_type = '&str'
        elif class_name == 'CODE':
            col.col_default = f"{col.default}"
            col.col_type = 'u64'
            col.fetch_type = 'int'
        elif class_name == 'ID':
            col.col_default = f"{col.default or 0}"
            col.col_type = 'u64'
            col.fetch_type = 'int'
        elif class_name == 'DICT':
            col.col_default = 'json!({})'
            col.col_type = 'dict'
            col.fetch_type = 'val'
        elif class_name == 'LIST':
            col.col_default = 'json!([])'
            col.col_type = 'list'
            col.fetch_type = 'val'
        elif class_name == 'STRING':
            col.col_default = ''
            col.col_type = '&str'
            col.fetch_type = '&str'
        else:
            col.col_type = class_name

        all_mfields[col.name] = col

    message_entries.append(dict(
        code=code,
        mtype=mtype,
        name=name,
        mfields=mfields,
        field_count=field_count,
    ))


buf = tmpl.render(
    message_entries = message_entries,
    all_mfields = all_mfields,
)

print(buf)

with open('../../src/messages.rs', 'w') as f:
    f.write(buf)


