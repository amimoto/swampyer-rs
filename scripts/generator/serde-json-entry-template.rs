use serde_json::{ Value, to_vec, from_slice, json };

// To help us build Builders
use derive_builder::Builder;

// For our errors

/**************************************************/
use std::fmt;

#[derive(Debug)]
enum WampError {
    NotArray,
    IncorrectElementCount,
    IncorrectElementType,
}

#[derive(Debug, Clone)]
struct NotArray;

impl fmt::Display for NotArray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Message was not an array")
    }
}

#[derive(Debug, Clone)]
struct IncorrectElementCount;

impl fmt::Display for IncorrectElementCount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Incorrect number of elements")
    }
}


#[derive(Debug, Clone)]
struct IncorrectElementType;

impl fmt::Display for IncorrectElementType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Incorrect number of elements")
    }
}
/**************************************************/

{%for m in message_entries%}

/***********************************************
 * {{m.name}} id:{{m.code.default}}
 ***********************************************/
#[derive(Default, Builder)]
#[builder(setter(into))]
struct {{name}} {
{%- for col in m.mfields %}
    #[builder(default = "{{col.col_default}}")]
    {{ col.name }}: {{ col.col_type }},
{%- endfor %}
}
impl {{m.name}} {
    fn load( message: &Value ) -> Result<{{m.name}}, WampError>
        if !message.is_array() {
            return Err(WampError::NotArray)
        }

        // Unwrap since we've already done the is_array test
        let entries = message.as_array().unwrap();

        // Make sure we have the expected number of elements
        if entries.len() != {{field_count}} {
            return Err(WampError::IncorrectElementCount)
        };

        // Copy over the relevant attributes
    {%- for col in m.mfields %}
        {% if col.fetch_type == 'int' -%}
        let {{col.name}} = entries[{{col.index}}].as_u64().ok_or(WampError::IncorrectElementType)?;
        {%- elif col.fetch_type == 'val' -%}
        let {{col.name}} = entries[{{col.index}}].clone();
        {%- else -%}
        // {{col.fetch_type}} : {{col.col_type}}
        let {{col.name}} = entries[{{col.index}}].clone();
        {%- endif -%}
    {%- endfor %}

        return {{m.name}} {
    {%- for col in m.mfields %}
            {{col.name}},
    {%- endfor %}
        }
    }
}

{% endfor %}