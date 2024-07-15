use anyhow::Result;
use walrus::Module;

pub fn list(wasm: &Module, ignored: bool) -> Result<()> {
    for export in wasm.exports.iter() {
        if !export.name.starts_with("__wbgt_") {
            continue;
        }

        let parts: Vec<&str> = export.name.split('$').collect();

        if ignored {
            if parts.len() == 3 {
                let test = parts[1];
                let test = &test[test.find("::").unwrap_or(0) + 2..];
                println!("{}: test", test);
            }
        } else {
            let test = parts[1];
            let test = &test[test.find("::").unwrap_or(0) + 2..];
            println!("{}: test", test);
        }
    }

    return Ok(());
}
