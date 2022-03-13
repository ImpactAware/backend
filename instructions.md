diesel migration run --database-url=postgres://localhost/impactaware

You may need to do postgres://username:password@localhost/impactaware

If you are using a URL other than the example one I provided on the first line of this file, please edit the string in src/lib.rs in the function "establish\_connection"

This will automatically create the table. If this isn't working, run the sql script in migrations/1\_nodes/up.sql

Then, use "cargo run --bin main" to start the web server. This will only serve the database, but won't collect information frm the hardware

In a different terminal window, use "cargo run --bin mesh\_bridge". Run with sudo if it can't access the com port. Edit the source file src/bin/mesh\_bridge.rs and look for the string "tty" to change the COM port or whatever identifier. The rust library I'm using should be cross-platform

