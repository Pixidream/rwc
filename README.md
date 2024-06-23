# RWC (WIP)

> A Rust implementation of [GNU's wc cli](https://www.gnu.org/software/coreutils/manual/html_node/wc-invocation.html#wc-invocation)

---

rwc - Rust implementation of wc. print newline, word, and byte counts for each file

Usage: rwc [OPTIONS] <FILE>

Arguments:
<FILE>

Options:
-c, --bytes Print the bytes counts
-m, --chars Print the character counts
-h, --help Print help
-V, --version Print version

---

## Dependencies

- clap 4.5.7 with derive: https://docs.rs/clap/latest/clap/index.html

---

This is a WIP. Only char and byte counts have been implemented.  
For now both are faster and consume less that the original wc (tested with cyrus of artamene).  
Only read from files for now.

License MIT.
