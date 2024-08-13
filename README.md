# BetterRemove - `rm` made better
It always annoys me that I have to either use the `rmdir` command or `-d` flag for the `rm` command to remove a directory... or yet another `-r` flag to delete it if it's not empty. Otherwise the command will just print an error and exit.

This obnoxious behaviour is being justified by alleged "safety", since accidentally deleting one file is less painful than deleting an entire directory with all of its contents, so the command requires the additional hassle of using of an additional flag to make sure you actually _mean_ it.

But you know what also is secure and is a valid way of making sure the user wants to delete a directory? ***Asking the user!*** A simple yes/no prompt solves the issue without the need to repeat the command with a proper flag added.

## But... why does this even exist?
This could be of course be mostly solved by a simple shell script/function, but where is the fun in that? I also seemed it as an opportunity to pick up on Rust, so here it is: an app that probably nobody needed besides me and nobody asked for.

## How to use it
It's dead-ass simple. Just type `br [FILE or DIRECTORY to remove]`. The application by default removes files and empty directories with no prompt. If trying to remove a driectory that contains files, you will be prompted, asking if you want to continue.

A serie of other flags can be used:

* `-f`, `--force` – removes non-empty directories without prompt, ignores nonexistent files or aguments.
* `-R`, `--no-preserve-root` – does not treat the root directory `/` nor any system-important directory specially (check the table of protected directories). If not used, user will be prompted to explicitly confirm the removal. 
* `-v`, `--verbose` – print what is being done.
* `-V`, `--version` – output version information and exit