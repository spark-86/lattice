# `rhex-craft` Utility Manual

## What does this tool do?

This tool creates a R⬢ and saves it to the disk to be signed over **OR** can attach the finalization (`curr_hash`) to the R⬢. This tool is typically used in conjunction with the `keytool` which does the actual signing from the "enclave" (currently just a folder with keys)

## Commands

Using the `rhex-craft` tool, one has several options. Each command executes a separate action on a R⬢ data source.

- **build**: Creates a new R⬢ file, as a single R⬢ output.
- **finalize**: Does some basic checking over the R⬢ and then attaches the `current_hash` value to the R⬢.
- **genesis**: Creates the initial records to start your own lattice.
- **view**: Views one or more R⬢ in a file. Typically scopes are held as a single chain of R⬢ in a file.
- **help**: View basic command line help.

## `build` command

Example:

`./rhex-craft build --prev somedata --scope some.scope --author pk-in-base64 --usher pk-in-base64 --schema rhex://somewhere --rt key:assign --data ./rhex-data.rdata ./rhex-file.rhex`

The `rhex-craft` tool requires multiple command line arguments (I'm sure we'll get to a GUI soon). Each argument specifies a value in the R⬢, or if not required and omitted, the default value will be used. Final value in the CLI is the path to the location to store on disk.

- `prev`*: Blake3 hash of the previous R⬢, in base64.
- `scope`*: Target scope for this R⬢.
- `author`*: Author's public key, in base64.
- `usher`*: Usher's public key, in base64,
- `schema`: Optional URL to the schema definition (either https:// or rhex://).
- `rt`*: The string value of the record type (e.g. `policy:set`, `key:assign`, etc).
- `data`: Full path to the data blob on disk.
- `help`: Shows fields.

\* = required

## `finalize` command

Example:

`./rhex-craft finalize --use-curr --input ./rhex-file.rhex`

This command adds the `curr_hash` value to the R⬢, as well as storing the finalized record in a file, potentially using the hex value of the `curr_hash` as the filename.

- `input`: The file of the stored R⬢ that we are adding the hash to
- `output`: The location to store the R⬢. If we are using the option `use-curr` then this is the directory to output to, otherwise it's the fully qualified path including filename.
- `use-curr`: Uses the hex value of `curr_hash` as the filename + `.rhex`
- `append`: Instead of overwriting the singular record, it will attach it to a chain of R⬢ in a file **IF** the last R⬢'s `curr_hash` equals our `prev` that was originally stored.
