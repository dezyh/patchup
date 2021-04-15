<div align="center">
    <img src="./assets/octopus.svg" width="150" />
    <h1>PatchUp CLI</h1>
    <p>A cross-platform command line tool to generate and apply binary patches across files and directories.</p>
</div>

## Usage
Run using `patchup`. For more information, see `patchup --help`.

#### Create Patch
To create a new patch file given two separate directories:
```
patchup diff <source_directory> <target_directory> <patch_file>
```

#### Apply Patch
To apply a given patch to a directory:
```
patchup patch <source_directory> <patch_file>
```

## Installation

#### Manual
```
# If required, update Rust on the stable channel
rust update stable

# Clone this repo from master
git clone https://github.com/dezyh/patchup

# Install using Cargo
cd patchup
cargo install --path cli
```

#### AUR
Normal package found here, binary package found here
```
# To compile from source:
yay -S patchup

# To download a pre-built binary:
yay -S patchup-bin
```
