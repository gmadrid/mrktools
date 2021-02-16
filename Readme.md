# mrktools

A command line tool for performing some simple tasks with the Remarkable
device that the desktop app doesn't do for me.

## Overview

The Remarkable device is a linux box, and it's running an ssh server. This
means that we can use 'ssh' and 'sshfs' to manipulate the device state and
the files which are installed on it.

`mrktools` assumes that:

* `ssh` is available in the `PATH`. Ideally, you have set up a public key
on the device so that a password isn't necessary, but a password prompt
will work if necessary.

* `sshfs` is available in the `PATH`. See the comments under `ssh`.

## Usage

Documentation gets stale. Please check `mrktools --help` for the most
up-to-date usage information.

Since everything is done over a `ssh` tunnel, the `host` param is almost
always required.

* `--host` - the IP address or hostname of the Remarkable device.

* `--user` - the username of the root user on the device. Defaults to `root`
and there is probably no reason to change it

* `--mount_point` - a non-existing directory to use as a mount point for
`sshfs`. Defaults to `./rem`.



### Subcommands

* `restart` - will restart the `xochitl` process on the device. Changes to the
files in the data directory will not be noticed, so a restart is often
desired.

* `ls` - list all of the files in the `xochitl` data directory. This is a
recursive listing. I only really found it useful for debugging, but maybe
it has other uses.

* `copy` - recursively copy a directory to the data directory on the device
with an optional restart.

* `ipdf` - convert images into pdf files. Optionally, convert them to something
that looks better on the device. Also optionally, copy them to the device
and/or restart the `xochitl` process.

## Examples

```
mrktools --host remarkable.mynet ipdf -b -p 'Parent' -c -r -a 25 *.jpg
```

Copies all jpegs in the current directory to the Remarkable under the
'Parent' folder. Those files will be converted to dithered black-and-white
with a pre-multiplied alpha value of 25%.

```
mrktools --host remarkable.mynet restart
```

Restart the `xochitl` server on the device. Note that this does _not_
restart the actual device.


## Warnings

`sshfs` runs a fairly fragile service on a low-level of the kernel stack.
When it fails, you may need to restart your entire device. At the very least,
you will get non-killable zombies that will slow everything down.

I suggest opening the device and ensuring that it is active before running
`mrktools` to avoid any problems. `mrktools` tries to be very clean about
starting and stopping the `sshfs` service when it runs, but if it fails
to shutdown cleanly, bad stuff can happen. Not ideal, but it was the most
direct route to the functionality I desired.

## To Do

At this point, this meets my needs, but it's a fun little project (when it's
not locking up my computer), so I may make some tweaks. Specific things on
my radar:

* Only mounting `sshfs` when required. Currently, it mounts immediately
which can cause delays and unnecessary fragility - especially for operations
like `restart` which don't require any access to the filesystem.

* Adding a environment variable for the host, allowing running `mrktools`
without putting the `--host` param in there every time. The only reason that
I haven't done this yet is a) I have hard-coded the value for my home network,
and b) the value will change as you move networks, so it's of limited utility on a laptop that you carry around.

## Resources

These operations were cribbed largely from these sources:

* [Software for the reMarkable tablet](https://www.ucl.ac.uk/~ucecesf/remarkable/) - Prof. Eric Fraga's notes on a
similar project. In his case, he is manipulating PDFs whereas I wanted to
install images on the device that don't look like crap. (And that have
thumbnails. Just copying a PDF to the device will often not get a thumb.)

* [Filesystem Layout](https://remarkablewiki.com/tech/filesystem) - from the
reMarkableWiki. This is an invaluable resource for anyone doing anything
with the reMarkable.
