use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn calculate_md5(path: &Path) -> Result<String, String> {
    let mut file = File::open(path).map_err(|e| e.to_string())?;
    let mut context = md5::Context::new();

    let mut buffer = [0; 4096];
    loop {
        let n = file.read(&mut buffer).map_err(|e| e.to_string())?;
        if n == 0 {
            break;
        }
        context.consume(&buffer[..n]);
    }

    Ok(format!("{:x}", context.compute()))
}