#!/usr/bin/env python3

import swampyer.messages

names = {
}

message_entries = []
for mtype, mfields in swampyer.messages.MESSAGE_TYPES.items():
    name = mtype.capitalize()

    field_count = len(mfields)
    code = mfields[0]

    for index, field in enumerate(mfields):
        mname = field.name
        optional = ''
        if not field.required:
            optional = ' *'
        hide_from_debug = ''
        if field.hide_from_debug:
            hide_from_debug = ' !'
        names.setdefault(mname, {})[name] = f"{index}{optional}{hide_from_debug}"


for mname, whodata in names.items():
    print(f"{mname}")
    for name, index in whodata.items():
        print(f" - {name}: {index}")

    print()

