use std::fs;
use std::io::{self, Write};
use std::process::Command;

fn main() -> io::Result<()> {
    println!(r"
  _________       ___.      ___ ___                   __
 /   _____/ __ __ \_ |__   /   |   \  __ __   ____  _/  |_
 \_____  \ |  |  \ | __ \ /    ~    \|  |  \ /    \ \   __\
 /        \|  |  / | \_\ \\    Y    /|  |  /|   |  \ |  |
/_______  /|____/  |___  / \___|_  / |____/ |___|  / |__|
        \/             \/        \/              \/
--> github.com/kodaksec :°
");

    let mut input = String::new();

    print!("Enter the domain URL (e.g., google.com) --> ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    let domain = input.trim().to_string();

    let search_dir = format!("search/{}", domain);
    fs::create_dir_all(&search_dir)?;

    let subdomains_file = format!("{}/subdomains.txt", search_dir);
    println!("Search all SubDomains :)");
    let subfinder_output = Command::new("subfinder")
        .args(["-d", &domain, "-o", &subdomains_file])
        .output()?;

    if !subfinder_output.status.success() {
        eprintln!("Not Found Subfinder");
        return Ok(());
    }
    println!("SubDomains -->  {}.", subdomains_file);

    let activesubs_file = format!("{}/activesubs.txt", search_dir);
    println!("Search all ActiveSubs ^.^");
    let httpx_output = Command::new("httpx-pd")
        .args([
            "-l", &subdomains_file,
            "-o", &activesubs_file,
            "-threads", "200",
            "-status-code",
            "-follow-redirects",
        ])
        .output()?;

    if !httpx_output.status.success() {
        eprintln!("Error running httpx.");
        return Ok(());
    }
    println!("ActiveSubs --> {}.", activesubs_file);

    let active_subs_content = fs::read_to_string(&activesubs_file)?;
    let cleaned_content = strip_ansi_codes(&active_subs_content);
    fs::write(&activesubs_file, &cleaned_content)?;

    input.clear();
    print!("Do you want to view the results? (yes/no): ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    let open_results = input.trim().to_lowercase();

    if open_results == "yes" {
        let styled_file = format!("{}/results_{}.txt", search_dir, domain);
        let mut styled_file_handle = fs::File::create(&styled_file)?;

        writeln!(styled_file_handle, r#"
  _________       ___.      ___ ___                   __
 /   _____/ __ __ \_ |__   /   |   \  __ __   ____  _/  |_
 \_____  \ |  |  \ | __ \ /    ~    \|  |  \ /    \ \   __\
 /        \|  |  / | \_\ \\    Y    /|  |  /|   |  \ |  |
/_______  /|____/  |___  / \___|_  / |____/ |___|  / |__|
        \/             \/        \/              \/
--> github.com/kodaksec :)
"#)?;

        writeln!(styled_file_handle, "Results for {}\n", domain)?;
        writeln!(styled_file_handle, "{}", cleaned_content)?;

        println!(
            "Results have been saved in the styled file: {}",
            styled_file
        );

        println!("Opening the results file...");
        open_file_with_default_app(&styled_file)?;
    }

    println!("Thank you for using SubHunt!");
    Ok(())
}

fn strip_ansi_codes(input: &str) -> String {
    let ansi_escape_sequence = regex::Regex::new(r"\x1B\[[0-?]*[ -/]*[@-~]").unwrap();
    ansi_escape_sequence.replace_all(input, "").to_string()
}

fn open_file_with_default_app(file_path: &str) -> io::Result<()> {
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(["/C", "start", file_path])
            .spawn()?;
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(file_path)
            .spawn()?;
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(file_path)
            .spawn()?;
    }

    Ok(())
}
