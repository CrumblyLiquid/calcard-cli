# calcard-cli

Easy to use tool for converting between iCalendar - JSCalendar and vCard - JSContacts.

I've cobbled this together very quickly, not sure if anything works :D

## Usage

```
Usage: calface [OPTIONS]

Options:
  -i, --input <INPUT>
          Input filepath
          
          If none is specified, it'll read the data from stdin

  -o, --output <OUTPUT>
          Output filepath
          
          If none is specified, it'll send the data into stdout

  -s, --source-type <SOURCE_TYPE>
          Source filetype
          
          TODO: Autodetection
          
          [possible values: v-card, i-calendar, js-calendar, js-contact]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
