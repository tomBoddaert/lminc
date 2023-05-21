# Extended mode (unstable)

This mode adds a few features, some of which are still being worked on.

This is unstable, so there is no guarantee that any of this will stay the same!

## Enabling extended mode
The first number in memory must be 010. 010 anywhere else will exit!

1 `DAT 010`

Or use

1 `EXT`

## Features

### ASCII input and output:
- INA (911) => take 8-bit ASCII character input (ignores all characters after the first)
- OTA (912) => output 8-bit ASCII character

## Examples
There is an output example in [examples/extended_output.txt](examples/extended_output.txt) and an input example in [examples/extended_input.txt](examples/extended_input.txt).

## Things that need to be done
- Support escaped characters in csv test lines.
