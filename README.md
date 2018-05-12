# ruttle

Use Rust to update shuttle configuration file, mainly used for accessing AWS.


## Prerequisite

1. `shuttle` - <https://github.com/fitztrev/shuttle>
> `brew cask install shuttle`

2. `rust` -- <https://github.com/rust-lang/rust>, `version >= 1.26`

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