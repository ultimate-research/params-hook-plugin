# Param hook plugin

A plugin allowing for centralized hooking of parameter files, allowing multiple plugins to all hook param file loads.


## Usage
```rust
fn params_main(params_info: &smash::params::ParamsInfo<'_>) {
    let _ = params_info
        .get::<StaticCommonParams>()
        .map(|mut param_obj: StaticCommonParams| {
            param_obj.shield_damage_mul = 0.0;
            param_obj.precede = 3;
            param_obj.cliff_max_count = 0;
            param_obj.invalid_capture_frame = 900;
        });
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