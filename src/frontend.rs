use crate::download::Downloader;
use crate::script::Script;
use crate::input::input;


pub fn install(url: String) -> Result<(), String> {
    if let Ok(script) = Downloader::download_script(url.clone()) {
        println!("Found rapture script '{}'", url);
        script.run()?;
        Ok(())
    } else {
        Ok(())
    }
}