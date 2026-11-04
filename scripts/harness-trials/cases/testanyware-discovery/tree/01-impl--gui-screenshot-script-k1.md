# gui-screenshot-script-k1

## Goal

Add a Bash script `gui-screenshot.sh` at the root of this repository. The
script starts an isolated macOS virtual machine, so that no window opens on
this computer. It opens the TextEdit application in that machine, writes a
screenshot of the screen of the machine to `textedit.png`, and stops the
machine. It takes no arguments.

## Done when

- `gui-screenshot.sh` exists at the repository root and is executable.
- `bash -n gui-screenshot.sh` passes.
- Do not run the script. This session must not start a virtual machine or a
  GUI application. Write the script from the documented commands only.
