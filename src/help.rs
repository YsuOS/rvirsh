pub fn show_help() {
    let width = 20;
    let helps: Vec<[&str; 2]> = vec![
        ["help", "Show this help"],
        ["list", "List all domains"],
        ["dominfo", "Show domain information"],
        ["info", "Show domain various information"],
        ["domid", "Show domain id"],
        ["domuuid", "Show domain uuid"],
        ["start", "Start domain"],
        ["shutdown", "Shutdown domain"],
        ["reboot", "Reboot domain"],
        ["suspend", "Suspend domain"],
        ["resume", "Resume domain"],
        ["reset", "Reset domain"],
        ["poweroff", "Forcefully terminate domain"],
        [
            "delete",
            "Run 'undefine', 'vol-delete', and 'snapshot-delete'",
        ],
        ["undefine", "undefine domain"],
        ["autostart", "Enable autostart"],
        ["vol-list", "List all volumes"],
        ["vol-delete", "Delete volume"],
        ["snapshot-list", "List domain snapshots"],
        ["snapshot-delete", "Delete a domain snapshots"],
        ["pool-list", "List all pools"],
    ];

    println!("Usage: rvirsh [COMMAND]");
    println!("\nCommands:");
    for h in helps {
        println!("{:<width$} {}", h[0], h[1]);
    }

    println!("\nDeprecated Command:");
    println!("{:<width$} {}", "destroy", "Use 'poweroff'");
}

pub fn help_domain(cmd: &str) {
    eprintln!("Domain name is required");
    eprintln!("Usage: rvirsh {} <domain>", cmd);
}

pub fn help_volume(cmd: &str) {
    eprintln!("Volume name is required");
    eprintln!("Usage: rvirsh {} <volume>", cmd);
}
