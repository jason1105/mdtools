# Mdtools

Mdtools is a little tool used for editing markdown file. It is able to:

- Add tag into frontmatter.
- Make footlinks.

# Usage

## Add tags

Content before:

![](https://i.imgur.com/ZlatH2s.png)

Run command

![](https://i.imgur.com/pralDiV.png)

Have done

![](https://i.imgur.com/XhtrFvf.png)

## Make footlinks

Footlinks refers to Reference-style link.

Content before

![](https://i.imgur.com/EL05vMO.png)

Run command

![](https://i.imgur.com/sNgynUp.png)

Have done

![](https://i.imgur.com/EouiSu1.png)

# Use cases: Make footlinks in [Obsidian][3]

To combine Plugin `Shell commands` to make footlinks:

- Plugin `Shell commands`
- Cmd tools `mdtools`

### Install `Shell commands`

1. Open `Obsidian`
2. Settings -> Communiti Plugins -> Browser
3. Search `shell command` and install it

### Install `mdtools`

1. [Download `mdtools`][1]
2. Expand to some directory (e.g. C:/program/bin/), add the directory to variable of PATH of environment if it is not in.

### Configure `Shell commands`

1] Open `Obsidian`

2] Click "Shell commands" under "PLUGIN OPTIONS" in Settings.

3] Click "New command" to add a command:

`mdtools make-footlink --path "{{file_path:absolute}}"`

4] Click icon of "Alias, Confirmation" on right side of command just been added

5] Modify "Alias" to "footlinks" 

5] Switch to "Event" tab, scroll to bottom and open "Editor menu"

### Make footlinks

Open a note and click right button of mouse, you will see a menu item:

![](https://i.imgur.com/VvEdkb7.png)


---

[1]: https://github.com/jason1105/mdtools/releases/tag/v0.1.0
[2]: https://wbd.ms/share/v2/aHR0cHM6Ly93aGl0ZWJvYXJkLm1pY3Jvc29mdC5jb20vYXBpL3YxLjAvd2hpdGVib2FyZHMvcmVkZWVtL2RlNmRjMjAzYWNmNDQzZjViYjhkNzY0NmYwMjYyNmU4X0JCQTcxNzYyLTEyRTAtNDJFMS1CMzI0LTVCMTMxRjQyNEUzRF8wYWRlZWJkYy1hOGM3LTRiYzgtOTc2Mi1jYjRiODc5OTdmZTI=
[3]: https://obsidian.md/

# Issues

Welcome to write issues if you have any question.
