use colored::Colorize;

const SECTION_WIDTH: usize = 20;

// log
pub fn log(msg: &str) {
    println!("{} {}", pp_now().yellow(), msg);
}

// log error
pub fn loge(msg: &str) {
    println!("{} {}", pp_now().red(), msg);
}

// long log
pub fn llog(section: &str, msg: &str) {
    println!(
        "{} {} {}",
        pp_now().yellow(),
        pp_section(section).cyan(),
        msg
    );
}

// long log error
pub fn lloge(section: &str, msg: &str) {
    println!("{} {} {}", pp_now().red(), pp_section(section).cyan(), msg);
}

//--------------------------------------------------------------------------------
// misc
//--------------------------------------------------------------------------------

fn pp_now() -> String {
    let now = chrono::Local::now();
    format!("[{}]", now.format("%H:%M:%S"))
}

fn pp_section(section: &str) -> String {
    if section.len() > SECTION_WIDTH {
        return format!("[{}]", section[..SECTION_WIDTH].to_string());
    }
    format!("[{:>width$}]", section, width = SECTION_WIDTH)
}
