# Param hook plugin

A plugin allowing for centralized hooking of parameter files, allowing multiple plugins to all hook param file loads.


## Usage
```rust
fn params_main(params_info: &smash::params::ParamsInfo<'_>) {
    if params_info.is_type::<CommonParams>()) {
        let common_params = StaticCommonParams::try_from(*params_info.object_ptr).unwrap();
        common_params.precede = 3;
    }
}

// in main
smash::params::add_hook(params_main).unwrap();
```

## Install

**Note:** Requires [cargo-skyline](https://github.com/jam1garner/cargo-skyline) to be installed

To install, simply run:

```
   cargo skyline install
```

Or, if your IP isn't configured yet, use
```
   cargo skyline install --ip X.X.X.X
```
Where `X.X.X.X` is your switch's IP address