+++
title = 'Terminals: an introduction'
date = 2025-03-02T12:10:35+01:00
draft = false
+++

Remember all those retrofuturistic movies where the
characters interact with computers using common language?

![navi](/lambda/navi.webp)

The idea of using a computer only through voice commands, although technically possible,
ignores some of the fundamental flaws of human language.
By saying something like "open a new file"
there are a lot of variants we are assuming:
- What will the name of that file be?
- Where will it be located?
- Which editor will you open it with?

Human language is careless and full of assumptions;
Computer language, on the other hand, is very rigid.

Terminal language aims to provide a way to speak to computers
in a way humans can understand.
It is, in a way,
a more realistic implementation of that concept.

## First steps
> This tutorial is only meant for systems that use bash, zsh and the like.

Open a new terminal. Type "firefox" and press enter.
Firefox will open.
Congratulations, you just ran your first command!

You can also follow it with a url of your choice
to open it in firefox:

```bash
# open firefox:
firefox 

# open the url "searx.cat" in firefox:
firefox searx.cat  

# open multiple urls in firefox:
firefox searx.cat cheat.sh gohugo.io
```

This is how the syntax works:
- First, write the application/command name to execute it.

- Optionally, follow it with __arguments__ to pass information to the application.

```bash
firefox
# application to execute: firefox
# arguments: none

firefox searx.cat
# application to execute: firefox
# arguments: searx.cat
```

Use the "clear" command to clean up previous steps before moving to the next.
```bash
clear
```

## Terminal user interfaces
There are many cases where the applications
need nothing but text as visual aid,
without the need of any fancy graphics.
Some of these run in the terminal!

Since they run in the terminal,
they rarely accept mouse input
so you can only use the keyboard to interact.

![image of ncmpcpp](/lambda/ncmpcpp.jpg)


There are many reasons to choose a text-based applications over GUIs.
They are lightweight,
they're fast,
they have a lot less visual noise
and they provide faster workflows
(since they are keybind-based).
They will never be as intuitive,
but their simplicity and ease of use make up for it.

> Want to try one out? Type "vimtutor" in your terminal!

## Simple bash commands
The terminal interpreter (bash/zsh)
provides some basic commands to move and interact
with files and directories.

Here are some of the important ones:

```bash
### Moving around ###
cd    # Change the current directory
ls    # List contents in the current directory
pwd   # Print current directory

### Other utilities ###
mv    # Move/Rename file/directory
cp    # Copy file/directory
mkdir # Create a directory
rm    # Remove file/directory
rmdir # Remove empty directory

### Useful shortcuts ###
# CTRL+L: clear terminal
# TAB: autocomplete
```

Wondering what a command does?

Type "man" followed by that command
to access its documentation locally:

`man command_name`




> Ncmpcpp screenshot taken from [vledit's blog](https://vlevit.org/ru/blog/tech/mnuc)
