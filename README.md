# Auto USB backup

Automatically backup a path onto your USB when plugged in.

## Usage

Build the executable and copy it onto your USB.

### Windows

Create a file called `autorun.inf` and put it on the root of your USB.
Add this text into it, adapting it to your executable path:

```inf
[autorun]
open=auto_usb_backup.exe
```
 