# BetterRemove - `rm` made better
BetterRemove aims to be a replacement for the `rm` command, with a more user-friendly approach to
removing files and directories.

First of all, the app doesn't bother differentiating if the provided path is a file or an empty directory.
It will remove it without any prompt, because... who really cares about the difference?

When the directory is empty, instead of having to re-type the entire command with a right recursive flag,
the user is prompted to confirm the removal of the directory and its contents.

## But... why does this even exist?
This could be of course be mostly solved by a simple shell script/function, but where is the fun in that?
I also seemed it as an opportunity to pick up on Rust, so here it is: an app that probably nobody asked for
nor needed besides me.

## How to use it
It's pretty simple and analogous to the `rm` command.
Just type `br [OPTION]... [FILE or DIRECTORY]...`.

The application by default removes files and empty directories with no prompt. 
If trying to remove a directory that contains files without providing the `-r` flag beforehand,
the user will be prompted, asking if they want to continue.

A series of flags can be used:

* `-d`, `--dir` – unused. Kept for compatibility with `rm`.
* `-f`, `--force` – ignores nonexistent files or arguments.
* `-r`, `-R` `--recursive` – removes directories and their contents recursively without prompting.
* `-s`, `--shred` – overwrite the file (or file contents of a directory) before removing it to make it
  harder to recover. Ignored in case of empty directories.
* `-v`, `--verbose` – print what is being done.


* `-i` – prompt before every removal.
* `-I` – prompt once before removing more than three files, or when removing recursively.
  Less intrusive than `-i`.
* `--interactive[=WHEN]` – prompt according to WHEN: never, once (-I), or always (-i).
  Without WHEN, it is equivalent to `-i`.


* `--one-file-system` – when removing a hierarchy, skip any directory that is on a file system different
  from that of the corresponding command line argument.
* `-N`, `--no-preserve-root` – does not treat the root directory `/` nor any system-important directory
  specially (check the table of protected directories). If not used, user will be prompted to explicitly
  confirm the removal.
* `-P`, `--preserve-root[=all]` – do not remove '/' nor any system-important directory (default);
  with 'all', reject any command line argument on a separate device from its parent.


* `-h`, `--help` – display this help and exit.
* `-V`, `--version` – output version information and exit.


To remove a file whose name starts with a `-`, for example
`-foo`, use one of these commands:

    br -- -foo
    br ./-foo

