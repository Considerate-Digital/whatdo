# whatdo

*whatdo* is a simple todo management tool. All projects and categories are use a typical file structure and all TODOs are stored in markdown files.

### Install
Installation is simple however the project is under active development so documentation may be lacking and some features may be unstable.
The instructions provided here have only been tested on linux systems.


#### Requirements
- Rust
- Some command line experience


#### Process
1. Clone the repository.
2. Build the binary.
```
cargo build --release
```
3. Copy the binary to your $PATH
```
cp ./target/release/whatda /put_your_path_here/
```
4. Make your first project.
```
whatdo new my_first_project
```
5. Learn the command structure.
```
whatdo --help
whatdo list --help
```

## Why open source?
We chose the **GNU Affero General Public License v3 (AGPLv3)** because we believe that measuring website carbon emissions is a utility that should be available to every website owner and operator. By opting for the AGPLv3, we have made sure that any future development and distribution of CarbonClicks will be available to everyone. The internet uses a huge amount of energy each year and CarbonClicks is a tool that can be used to aid developers and website owners to lower their digital emissions.

We have provided [an overview](/AGPLv3_INFO.md) of the AGPLv3 license to roughly outline how it works. For more detail please read [the full license](/LICENSE).

## Contributing
We welcome contributions from everyone, whether that's fixing some of the core code or a spelling mistake. If you're interested in making contributions to CarbonClicks then please read our "Code of Conduct" before you make any suggestions or send a pull request.

To get started working on the codebase, follow our [development setup guide](/DEV_SETUP.md).

### Reporting bugs
Before submitting an issue, please take a few seconds to do a quick search to check that your issue has not already been raised or fixed. 
You can report bugs by submitting a new issue. The more detail you can provide in your issue, the better our team will be able to help you. Note that we expect anyone submitting a bug to adhere to our "Code of Conduct".

### Future Development
Here, in no particular order, are a list of future developments we'd like to undertake:
- Create the ability to add, edit and remove TODOs from specific lists.
- Add unit tests.

## History
whatdo was developed internally at Considerate Digital to help the team manage projects. We wanted to build a system that used git for version control and that had a simple command line interface.

## License
whatdo is distributed under [AGPLv3](https://www.gnu.org/licenses/agpl-3.0.en.html). 
