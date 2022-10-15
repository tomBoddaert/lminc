# Extended mode (unstable)

This mode adds a few features, some of which are still being worked on.

This is unstable, so there is no garentee that any of this will stay the same,
and it may even be totally removed!

## Enabling extended mode
The first number in memory must be 010. 010 anywhere else will exit!

1 `DAT 010`

## Features

### ASCII input and output:
- 911 (INA) => take 8-bit ASCII character input (ignores all characters after the first)
- 912 (OTA) => output 8-bit ASCII character

### Modules
Modules are currently a WIP. They would allow for starting one computer from another
with some form of shared memory.

## Examples
There is an output example in [examples/extended_output.txt](examples/extended_output.txt) and an input example in [examples/extended_input.txt](examples/extended_input.txt).
