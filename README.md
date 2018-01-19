# ruttle

Use Rust to update shuttle configuration file, mainly used for accessing AWS.

## Prerequisite
1. `shuttle` - <https://github.com/fitztrev/shuttle>
> `brew cask install shuttle`

2. `rust` -- <https://github.com/rust-lang/rust> 

## Procedure
1. Read `shuttle` configuration file
2. Parse configuration to get each AWS instance id
3. Fetch public ip of AWS instance id
4. Overwrite public ip with the newly one to Configuration
5. Dump configuration to file

## Note
1. `shuttle` configuration is located at `~/.shuttle.json`