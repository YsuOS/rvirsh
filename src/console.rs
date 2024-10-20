use anyhow::Result;
use config::Config;
use libc;
use std::{
    ffi::c_void,
    io::{self, Read, Write},
    os::fd::AsRawFd,
};
use termios::{Termios, ECHO, ICANON, ISIG, TCSANOW};
use virt::{
    console::Console,
    event::{event_register_default_impl, event_run_default_impl, Event},
    stream::Stream,
    sys::{
        virStreamEventType, VIR_DOMAIN_CONSOLE_FORCE, VIR_EVENT_HANDLE_READABLE,
        VIR_STREAM_EVENT_READABLE, VIR_STREAM_NONBLOCK,
    },
};

use crate::{get_conn, get_domain};

fn read_callback(stream: &Stream, event_type: virStreamEventType) -> Result<()> {
    if event_type == VIR_STREAM_EVENT_READABLE {
        let mut buf = vec![0; 1024];
        let stdout = io::stdout();
        let mut stdout = stdout.lock();
        match stream.recv(buf.as_mut_slice()) {
            Ok(_) => {
                stdout.write_all(&buf)?;
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
    Ok(())
}

fn stdin_callback(
    _watch: libc::c_int,
    _fd: libc::c_int,
    events: libc::c_int,
    console_ptr: *mut c_void,
) -> Result<()> {
    if events == VIR_EVENT_HANDLE_READABLE as libc::c_int {
        let stdin = io::stdin();
        let mut stdin = stdin.lock();
        let con = unsafe { &mut *(console_ptr as *mut Console) };

        let mut buf = [0; 1];
        if let Ok(_) = stdin.read(&mut buf) {
            // 29 == Ctrl-]
            if buf[0] == 29 {
                con.cond.store(false, std::sync::atomic::Ordering::SeqCst);
            }
            con.st.send(&buf)?;
        }
    }
    Ok(())
}

fn set_raw_mode() -> Result<Termios> {
    let stdin = io::stdin();
    let fd = stdin.as_raw_fd();
    let mut termios = Termios::from_fd(fd)?;
    let orig_termios = termios.clone();

    termios.c_lflag &= !(ICANON | ECHO | ISIG);
    termios::tcsetattr(fd, TCSANOW, &termios)?;

    Ok(orig_termios)
}

fn reset_mode(orig_termios: Termios) -> Result<()> {
    let stdin = io::stdin();
    let fd = stdin.as_raw_fd();
    termios::tcsetattr(fd, TCSANOW, &orig_termios)?;
    println!("");
    Ok(())
}

pub fn main(settings: &Config, cmd: &str) -> Result<()> {
    event_register_default_impl()?;

    let conn = get_conn(settings)?;
    let dom = get_domain(&conn, cmd)?;
    let st = Stream::new(&conn, VIR_STREAM_NONBLOCK)?;

    println!("Try to connect via console");
    println!("Escape character is ^] (Ctrl + ])");

    dom.open_console(None, &st, VIR_DOMAIN_CONSOLE_FORCE)?;

    let mut console = Console::new(st);

    console
        .st
        .event_add_callback(VIR_STREAM_EVENT_READABLE, |st, event_type| {
            let _ = read_callback(st, event_type);
        })?;

    console.event_add_handle::<_, Console>(
        0,
        VIR_EVENT_HANDLE_READABLE,
        |watch, fd, events, opaque| {
            let _ = stdin_callback(watch, fd, events as libc::c_int, opaque);
        },
    )?;

    let orig_termios = set_raw_mode()?;

    while console.cond.load(std::sync::atomic::Ordering::SeqCst) {
        event_run_default_impl()?;
    }

    reset_mode(orig_termios)?;
    Ok(())
}
