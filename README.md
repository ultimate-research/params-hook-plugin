# Param hook plugin

A plugin allowing for centralized hooking of parameter files, allowing multiple plugins to all hook param file loads.


## Usage
```rust
fn params_main(params_info: &smash::params::ParamsInfo<'_>) {
    if let Ok(common) = params_info.get::<StaticCommonParams>() {
        common.shield_damage_mul = 0.0;
        common.precede = 3;
        common.cliff_max_count = 0;
        common.invalid_capture_frame = 900;
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