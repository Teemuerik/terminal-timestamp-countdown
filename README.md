# terminal-timestamp-countdown
A simple terminal program that displays a countdown to a given UNIX timestamp.

Pass in either a UNIX timestamp as the first argument, or "deltarune" to count down to when Chapter 5 is released.

## Usage

`./terminal_countdown [timestamp] [arguments]`

`timestamp`: This can be either an integer UNIX timestamp in seconds or a special string. More on them below.


Arguments:

- `--no-center`: Will not center the time.
- `--dms-desktop-command`: Puts the program in DMS Desktop Command plugin mode.
This makes it instantly render a single output of the current time.
This also implies `--no-center`.
More info on how to use this with Desktop Command is in a following section.
- `--right-align-length=N`: Right-aligns the text so that the last character stays in the position of the `N`th character.
This can be used with all alignment modes. The argument has to be at least the character length of the shown text.
One way to use this is in conjunction with `--dms-desktop-command` to make sure that the
counter is always aligned with the right edge of the screen without having to reposition the widget.
To use it this way, count how many characters the counter has at the start, and set this to that amount.

## Special timestamp values

Below is the current list of special values that can be passed instead of a numerical timestamp.
These are subject to change, especially after the timestamps have passed.

- `deltarune`: Sets the timestamp to 1782313200, the UNIX timestamp when Deltarune Chapter 5 will be released.
(Has an accuracy down to the hour).

## Usage with DMS Desktop Command

To use this with the [Desktop Command plugin](https://github.com/yayuuu/desktopCommand), first build it and put the release files in a known directory.
Then give the widget the command `cd [program directory] && ./terminal_countdown [timestamp] --dms-desktop-command`.
Replace `[program directory]` with the directory you put the built terminal countdown program in and `[timestamp]` with the timestamp you want to count down to.
You then have to set the widget to Auto Refresh, with a Refresh Interval of whatever refresh rate you want for the countdown.
You can also easily align the countdown with something on the right side of the widget by using `--right-align-length=N` as explained in the Usage section.
