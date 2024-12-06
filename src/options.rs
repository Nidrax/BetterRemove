use std::io;
use getopts::Options;

pub struct ArgOptions
{
    pub recursive: bool,
    pub shred: bool,
    pub verbose: bool,
    pub preserve_root: bool,
    pub help: bool,
    pub version: bool,
    pub files: Vec<String>,
}

impl ArgOptions
{
    pub fn new() -> ArgOptions
    {
        ArgOptions
        {
            recursive: false,
            shred: false,
            verbose: false,
            preserve_root: true,
            help: false,
            version: false,
            files: Vec::new(),
        }
    }

    pub fn parse(&mut self, args: Vec<String>) -> io::Result<()>
    {
        let mut opts = Options::new();
        opts.optflag("r", "recursive", "remove directories and their contents recursively without prompting");
        opts.optflag("s", "shred", "overwrite the removed files with random data");
        opts.optflag("v", "verbose", "explain what is being done");
        opts.optflag("N", "no-preserve-root", "do not treat the root directory `/` nor any system-important directory specially");
        opts.optflag("h", "help", "display this help and exit");
        opts.optflag("V", "version", "output version information and exit");

        let matches = match opts.parse(&args[1..])
        {
            Ok(m) => m,
            Err(f) => return Err(io::Error::new(io::ErrorKind::InvalidInput, f.to_string())),
        };

        self.recursive = matches.opt_present("r");
        self.shred = matches.opt_present("s");
        self.verbose = matches.opt_present("v");
        self.help = matches.opt_present("h");
        self.version = matches.opt_present("V");

        if matches.opt_present("N")
        {
            self.preserve_root = false;
        }

        self.files = matches.free;

        Ok(())
    }
}