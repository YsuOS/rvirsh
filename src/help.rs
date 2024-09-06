pub fn show_help() {
    let width = 20;
    let main_helps: Vec<[&str; 2]> = vec![
        ["help", "Show this help"],
        ["list", "List all domains"],
        ["info", "Show domain various information"],
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
        ["vol-list", "List all volumes"],
        ["vol-delete", "Delete volume"],
        ["snapshot-list", "List domain snapshots"],
        ["snapshot-delete", "Delete a domain snapshots"],
        ["pool-list", "List all pools"],
        [
            "pool-clean",
            "Run 'pool-stop', 'pool-delete', and 'pool-undefine'",
        ],
        ["hostname", "Show the hypervisor hostname"],
        ["hostinfo", "Show the host information"],
        ["uri", "Show the hypervisor URI"],
        ["version", "Show version"],
    ];
    let sub_helps: Vec<[&str; 2]> = vec![
        ["autostart", "Enable autostart"],
        ["noautostart", "Disable autostart"],
        ["undefine", "Undefine domain"],
        ["dominfo", "Show domain information"],
        ["domid", "Show domain id"],
        ["domuuid", "Show domain uuid"],
        ["vol-info", "Show volume information"],
        ["vol-key", "Show volume key"],
        ["vol-path", "Show volume path"],
        ["vol-pool", "Show pool name the volume belongs to"],
        ["vol-dumpxml", "Show volume information in XML"],
        ["vol-wipe", "Wipe volume"],
        ["pool-info", "Show pool information"],
        ["pool-uuid", "Show pool uuid"],
        ["pool-refresh", "Refresh pool"],
        ["pool-stop", "Stop pool"],
        ["pool-delete", "Delete pool"],
        ["pool-undefine", "Undefine pool"],
    ];

    println!("Usage: rvirsh [COMMAND]");
    println!("\nMain Commands:");
    for h in main_helps {
        println!("{:<width$} {}", h[0], h[1]);
    }
    println!("\nSub Commands:");
    for h in sub_helps {
        println!("{:<width$} {}", h[0], h[1]);
    }

    println!("\nDeprecated Command:");
    println!("{:<width$} {}", "destroy", "Use 'poweroff'");
    println!("{:<width$} {}", "nodeinfo", "Use 'hostinfo'");
    println!("{:<width$} {}", "pool-destroy", "Use 'pool-stop'");
}

pub fn help_domain(cmd: &str) {
    eprintln!("Domain name is required");
    eprintln!("Usage: rvirsh {} <domain>", cmd);
}

pub fn help_volume(cmd: &str) {
    eprintln!("Volume name is required");
    eprintln!("Usage: rvirsh {} <volume>", cmd);
}

// For vol-pool
pub fn help_volume_path(cmd: &str) {
    eprintln!("Volume path is required");
    eprintln!("Usage: rvirsh {} <volume path>", cmd);
}

pub fn help_pool(cmd: &str) {
    eprintln!("Pool name is required");
    eprintln!("Usage: rvirsh {} <pool>", cmd);
}
