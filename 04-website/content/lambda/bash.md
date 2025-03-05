+++
title = 'Simple Bash Commands'
date = 2025-03-05T11:52:50+01:00
draft = true
+++

## Simple bash commands
Our terminal interpreter (bash)
provides some basic commands to move and interact
with files and directories.

```bash
### Moving around ###
cd    # Change the current directory
ls    # List contents in the current directory
pwd   # Print current directory

### Other utilities ###
mv    # Move file/folder
cp    # Copy file/folder
mkdir # Create a directory
rm    # Remove file/folder
rmdir # Remove empty directory

### Useful shortcuts ###
# CTRL+L: clear terminal
# TAB: autocomplete
```

> Wondering what a command does?
> 
> Type "man" followed by that command
to access its documentation locally:
>
>`man command_name`


### Moving around
The main commands you'll need to move around the system are:
- __pwd__ (print current/working directory)
- __ls__ (list) and
- __cd__ (change directory)

__pwd__ prints the current directory.
Your bash session usually starts at the home directory,
so that should be what it prints.

```bash
pwd    # example output: /home/my_username
```

__ls__ is used to __list__ contents inside a directory,
similar to how the file explorer shows the contents of where you're in.

```bash
ls     # list contents at current directory
ls -a  # list all files (including invisible)
ls -l  # list with additional information
```

__cd__ is used to __change__ the current __directory__.
To move into a different directory, use __cd__.

```bash
cd directory_name  # basic command

# this can be used for relative paths:
cd Downloads  

# and also full paths:
cd /home/my_username/Downloads

cd ..  # move to the parent directory
cd --  # move to the home directory
cd -   # move to the previous directory
```

### Other utilities
Bash offers a variety of utilities to move around
and manage files and directories.
Most of them are very simple
and can be found in a quick internet search.
Here's a few of them anyways:

```bash
### mv (move) ###
mv file1 file2       # move or rename file/directory
mv file1 directory/  # move file/directory into another directory

### cp (copy) ###
cp file1 file2            # copy single file
cp -r folder1 folder2     # copy multiple files (recursive)
cp file1 directory_name/  # copy file into directory

### mkdir (make directory) ###
mkdir directory_name  # create a directory

### rm (remove) ###
rm file1              # remove single files
rm file1 file2 file3  # remove multiple files
rm -rf directory_name # remove directory and its files

### rmdir (remove directory) ###
rmdir empty_directory # remove empty directory

# use rm -rf if the directory is not empty
rm -rf directory
```
