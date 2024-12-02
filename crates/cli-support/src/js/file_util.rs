use base64::{prelude::BASE64_STANDARD_NO_PAD, Engine};

pub(crate) fn create_load_inline_bytes_snippet(bytes: &[u8], variable_name: String) -> String {
    format!(
        "
        let {variable_name};
        const base64 = \"{base64}\";
        if (typeof Buffer === 'undefined') {{
            {variable_name} = Uint8Array.from(atob(base64), c => c.charCodeAt(0));
        }} else {{
            {variable_name} = Buffer.from(base64, 'base64');
        }}
        ",
        variable_name = variable_name,
        base64 = BASE64_STANDARD_NO_PAD.encode(bytes),
    )
}
