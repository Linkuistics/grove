Add a Bash script `stamp.sh` in the current directory. When it runs, it writes
the current UTC time in ISO 8601 form, such as `2026-11-03T04:05:06Z`, to the
file `STAMP` in its working directory. It takes no arguments.

Make `stamp.sh` executable. Run it once and check that `STAMP` holds one line
in that form. Then remove `STAMP`, so that only the script remains.
