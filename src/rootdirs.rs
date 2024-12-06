use phf::phf_map;

pub static ROOT_DIRS: phf::Map<&'static str, &'static str> = phf_map!
{
    "/" => "do not remove `/`",
    "/bin" => "do not remove `/bin`",
    "/boot" => "do not remove `/boot`",
    "/dev" => "do not remove `/dev`",
    "/etc" => "do not remove `/etc`",
    "/home" => "do not remove `/home`",
    "/lib" => "do not remove `/lib`",
    "/lib64" => "do not remove `/lib64`",
    "/media" => "do not remove `/media`",
    "/mnt" => "do not remove `/mnt`",
    "/opt" => "do not remove `/opt`",
    "/proc" => "do not remove `/proc`",
    "/root" => "do not remove `/root`",
    "/run" => "do not remove `/run`",
    "/sbin" => "do not remove `/sbin`",
    "/srv" => "do not remove `/srv`",
    "/sys" => "do not remove `/sys`",
    "/tmp" => "do not remove `/tmp`",
    "/usr" => "do not remove `/usr`",
    "/var" => "do not remove `/var`",
};

pub static WIN_DIRS: phf::Map<&'static str, &'static str> = phf_map!
{
    "Program Files" => "do not remove `Program Files`",
    "Program Files (x86)" => "do not remove `Program Files (x86)`",
    "ProgramData" => "do not remove `ProgramData`",
    "Users" => "do not remove `Users`",
    "Windows" => "do not remove `Windows`",
};