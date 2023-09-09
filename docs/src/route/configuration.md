# Configuration
The `config` property in `project.yaml` is an array of configuration objects.
You can load configuration from another file using the `use` property (See [File Structure](./file-structure.md)).
Or make your own configuration.

## Properties
Most of the time, you can use config presets provided by Celer or a 3rd party.
In case you want to make your own, here are the available properties:

|Property|Description|
|-|-|
|`icons`|Add icon definition. See [Icons](./config/icons.md) for detail|
|`tags`|Add tag definition for use in Rich Text. See [Tags](./config/tags.md) for detail|
|`presets`|Add preset definition. See [Presets](./config/presets.md) for detail|
|`map`|Define map properties. See [Map](./config/map.md) for detail|

Configurations are meant to be composed and reused with other configurations.
So most properties in all configurations are combined. An exception to this is `map`.
The compiler will give an error if multiple configurations define the map.

## Example
```yaml
# project.yaml
config:
- use: Pistonite/celer/presets/botw-map.yaml
- use: Pistonite/celer/presets/botw-presets.yaml
- icons:
    example-icon: use: hello/world/example.png
  tags:
    colorful:
      color: blue
```

## Configuration Files
The configuration files that are loaded by `use` should
be a YAML file that defines a mapping on the root level. For example:
```yaml
# something.yaml
icons:
  foo: use: bar/biz/boo.png
tags:
  colorful:
    color: blue
```

Be careful that top-level `use` is not permitted. The following config file
is invalid for others to include with `use`:
```yaml
# something.yaml
use: another/file/something.yaml
```
:::tip
You can still use `use` in the config properties themselves, like
the example above in the `icons` property
:::