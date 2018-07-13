# ruttle

Use Rust to update shuttle configuration file, mainly used for accessing AWS EC2.


## Prerequisite

1. `shuttle` - <https://github.com/fitztrev/shuttle>
> `brew cask install shuttle`

2. `rust` -- <https://github.com/rust-lang/rust>
> `version >= 1.27` (support `impl Trait`)

## Configuration

`shuttle` configuration is in following format and use `type: "AWS"` to mark the corresponding item is for AWS EC2:

```json
{
    "terminal": "iTerm.app",
    "iTerm_version": "nightly",
    "default_theme": "Homebrew",
    "open_in": "new",
    "hosts": [{
        "[GROUP_NAME]": {
            "id": "",
            "name": "",
            "title": "",
            "type": "AWS",
            "instanceID": "[Instance ID]",
            "cmd": "ssh -i <YOUR_PEM_FILE> <USER>@<PUBLIC_IP>",
            "inTerminal": "tab",
            "theme": "basic",
        }
    }]
}
```

1. `GROUP_NAME` is used to assemble several items together.
2. `instanceID` is id of `AWS EC2` instance, required.

`ruttle` will update each instance's `PUBLIC_IP` based on its `instanceID`.

## TODO List

- [x] Read `shuttle` configuration file
- [x] Read `AWS` configuration
- [ ] Parse configuration to get each AWS instance id
- [ ] Fetch public ip of AWS instance id
- [ ] Overwrite public ip with the newly one to Configuration
- [ ] Dump configuration to file
- [ ] Refactor project structure

## Note

1. `shuttle` configuration is located at `~/.shuttle.json`