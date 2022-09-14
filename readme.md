# Pacman Util (WIP)

This is a utility to help you to manage packages in your arch linux.

# Dependency

+ `pacman`
+ `cargo` (rust)

# Features
+ [x] List packages from all repo `pacman -Sl`
+ [x] List installed package `pacman -Qi`
+ [x] Join together
+ [x] Have warning
+ [x] Compile error

# Planned
```shell
# Reinstalling will keep as deps

# Replace testing package from all testing repo 
# with available repository if possible
pacutil removeall '{"repo":"testing"}'
```