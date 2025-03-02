# Flight OS

Self contained, bootable binary that approximates a flight simulator

## Build instructions

```shell
# have to use nightly since it uses unstable features
 rustup update nightly --force

 # build the app
 cargo build

 #install bootimage for the nightly verison (ideally only needs to be run once)
 cargo install bootimage --force
 
 # i forget probably just pulls down some assembly compiles it and gets it ready to paste
 cargo bootimage
 
 # run in qemu
 cargo run
```