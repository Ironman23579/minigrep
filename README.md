# minigrep
A primitive version of the grep command line tool,
made in rust as a practice project. I followed the rust tutorial
at https://doc.rust-lang.org/book/ch12-00-an-io-project.html
as a starting point, but have also added a few unique features.

BASIC USAGE:

minigrep [OPTIONS] <query> <file path>

By default, minigrep is configured to work through piping, eg:

echo "Hello World!" | minigrep o

but it can also read from a file.

Options:

-c: configures the search to be case sensitive.

-f: configures minigrep to search from a file.
