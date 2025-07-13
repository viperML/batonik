use batonik::batonik;
use batonik::colored::Colorize;
use batonik::modules::*;

fn main() {
    let b = batonik![
        Directory::new(),
        async {
            let direnv = std::env::var("DIRENV_DIR").is_ok();
            if direnv {
                return "󰴉 ".green().to_string();
            } else {
                return "󰴉 ".red().to_string();
            }
        },
        Env::new("EDITOR"),
    ];

    b.run();
}
