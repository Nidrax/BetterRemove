mod options;
mod rootdirs;

use std::fs;
use std::io;
use std::io::Write;
use crate::options::ArgOptions;
use colored::Colorize;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn print_version(prog_name: &str)
{
    println!("{} v{}", prog_name.green(), VERSION);
}

fn print_help(prog_name: &str)
{
    println!("Usage: {} {}... {}...", prog_name.green(), "[OPTION]".yellow(), "[FILE] or [DIRECTORY]".yellow());
    println!();
    println!("Remove (unlink) the FILE(s).");
    println!("  {}  remove directories and their contents recursively without prompting",
             "-r, --recursive   ".cyan());
    println!("  {}  overwrite the removed files with random data",
             "-s, --shred       ".cyan());
    println!("  {}  explain what is being done",
             "-v, --verbose     ".cyan());
    println!("  {}  display this help and exit",
             "-h, --help        ".cyan());
    println!("  {}  output version information and exit",
             "-V, --version     ".cyan());
    println!("  {}  do not treat the root directory `/` nor any system-important directory specially",
             "--no-preserve-root".cyan());
}

fn check_root(path: &str) -> u8
{
    //check if running on windows
    if cfg!(windows) || path.starts_with("/mnt")
    {
        if rootdirs::WIN_DIRS.contains_key(path)
        {
            return 2;
        }
    }
    else
    {
        if rootdirs::ROOT_DIRS.contains_key(path)
        {
            return 1;
        }
    }

    0
}

fn is_dir(path: &str) -> bool
{
    match fs::metadata(path)
    {
        Ok(metadata) => metadata.is_dir(),
        Err(_) => false,
    }
}

fn is_empty_dir(path: &str) -> bool
{
    match fs::read_dir(path)
    {
        Ok(mut entries) => entries.next().is_none(),
        Err(_) => false,
    }
}

fn shred_file(path: &str, verbose: Option<bool>) -> io::Result<()>
{
    if verbose.unwrap_or(false)
    {
        println!("shredding '{}'", path);
    }

    let mut file = fs::OpenOptions::new().write(true).open(path)?;
    let size = file.metadata()?.len();
    let buffer = vec![rand::random::<u8>(); 1024 * 1024];

    for _ in 0..size / buffer.len() as u64
    {
        file.write_all(&buffer)?;
    }

    file.write_all(&buffer[0..(size % buffer.len() as u64) as usize])?;

    Ok(())
}

fn shred_recursive(path: &str, verbose: Option<bool>) -> io::Result<()>
{
    for entry in fs::read_dir(path)?
    {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir()
        {
            shred_recursive(&path.to_string_lossy(), verbose)?;
        }
        else
        {
            shred_file(&path.to_string_lossy(), verbose)?;
        }
    }

    Ok(())
}

fn remove(opts: &ArgOptions, path: &str, prog_name: &str) -> io::Result<()>
{
    if !is_dir(path)
    {
        if opts.shred
        {
            shred_file(path, Option::from(opts.verbose))?;
        }

        if opts.verbose
        {
            println!("removing '{}'", path);
        }
        fs::remove_file(path)?;
    }
    else
    {
        if check_root(path) == 1 && !opts.preserve_root
        {
            println!("{} '{}' {}",
                     "Directory".yellow(), path.red(), "is a special system directory!".yellow());
            println!("Deleting it can – {} – {}, however we're not stopping the users from shooting \
            their own foot if they wish to do so.",
                     "and most probably WILL".yellow(), "break your system".red());
            println!("Do you understand the consequences of your actions and want to continue with it? {}",
                     "(y/n)".cyan());
        }
        else if check_root(path) == 2
        {
            println!("It looks like you're trying to remove a system directory on a Windows file system.");
            println!("This most probably {}.", "will break your Windows installation".red());
            println!("Do you understand what you're about to do and do you want to continue? {}",
                     "(y/n)".cyan());
        }

        if is_empty_dir(path)
        {
            if opts.verbose
            {
                println!("removing '{}'", path);
            }
            fs::remove_dir(path)?;
        }
        else
        {
            if !opts.recursive
            {
                println!("Directory '{}' is not empty. Do you want to remove it? {}",
                         path.yellow(), "(y/n)".cyan());
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                if input.trim() != "y"
                {
                    return Ok(());
                }
            }

            if opts.shred
            {
                println!("You are attempting to shred a directory recursively, this may take a long time.");
                println!("Are you sure you want to continue? {}", "(y/n)".cyan());
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                if input.trim() == "y"
                {
                    shred_recursive(path, Option::from(opts.verbose))?;
                }
            }

            if opts.verbose
            {
                println!("Removing '{}'", path);
            }

            fs::remove_dir_all(path)?;
        }
    }

    Ok(())
}

fn main() -> io::Result<()>
{
    let args: Vec<String> = std::env::args().collect();
    let args_copy = args.clone();
    let prog_name: &str = args_copy[0].as_str();
    let mut opts = ArgOptions::new();
    opts.parse(args)?;

    if opts.version
    {
        print_version(prog_name);
    }
    if opts.help
    {
        print_help(prog_name);
    }
    if opts.version || opts.help
    {
        return Ok(());
    }

    if opts.files.is_empty()
    {
        println!("{}: {}", prog_name.green(), "missing operand".red());
        print_help(prog_name);
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "missing operand"));
    }

    for file in opts.files.iter()
    {
        remove(&opts, file, prog_name)?;
    }

    Ok(())
}
