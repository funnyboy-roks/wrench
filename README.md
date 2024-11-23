# Wrench

A command-line tool for mass-renaming of files from one pattern to another using
regular expressions.

## Example

```sh
$ ls
'Episode 1 - Hello.mkv'  'Episode 2 - World.mkv'  'Episode 3 - How.mkv'  'Episode 4 - Are.mkv'  'Episode 5 - You.mkv'
'Episode 1 - Hello.srt'  'Episode 2 - World.srt'  'Episode 3 - How.srt'  'Episode 4 - Are.srt'  'Episode 5 - You.srt'

$ wrench '^Episode (\d+).*\.' 'S01E0$1.' *
'Episode 1 - Hello.mkv' -> 'S01E01.mkv'
'Episode 1 - Hello.srt' -> 'S01E01.srt'
'Episode 2 - World.mkv' -> 'S01E02.mkv'
'Episode 2 - World.srt' -> 'S01E02.srt'
'Episode 3 - How.mkv' -> 'S01E03.mkv'
'Episode 3 - How.srt' -> 'S01E03.srt'
'Episode 4 - Are.mkv' -> 'S01E04.mkv'
'Episode 4 - Are.srt' -> 'S01E04.srt'
'Episode 5 - You.mkv' -> 'S01E05.mkv'
'Episode 5 - You.srt' -> 'S01E05.srt'

$ ls
S01E01.mkv  S01E01.srt  S01E02.mkv  S01E02.srt  S01E03.mkv  S01E03.srt  S01E04.mkv  S01E04.srt  S01E05.mkv  S01E05.srt
```
