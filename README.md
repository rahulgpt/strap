# Strap

### Simple React Component Boilerplate Generater

A command-line utility for bootstraping react component template.

## Quickstart

Install via Cargo:

```bash
cargo install strap
```

`cd` to project's directory, and run `strap Button` to generate the boilerplate code.

Your project will now have a new directory at `src/components/Button`. This directory has two files:

```jsx
// `Button/index.js`

export { default } from "./Button";
```

```jsx
// `Button/Button.js`

import React from "react";

export default function Button() {
  return <></>;
}
```

## Customizing Templates

Custom Templates with a template type-name can be provide. The name of the file or directory should be **functional** or **class** for functional or class component respectively.

Custom Templates can be provided in two ways:

#### Files

Templates can be customized by providing custom templates. You will need to add the following two files with `_component` placeholder that will be replaced by the component name.

```jsx
// functional.js

import React from "react";

function _component() {
  return <></>;
}

export default _component;
```

```jsx
// class.js

import React from "react";

class _component {
  render() {
    return <></>;
  }
}

export default _component;
```

Strap will look for the custom templates in `src/components/.templates` directory. Template path can be customized by **templateDirectory** in **strap-config.json**.

#### Directory

If there are multiple files, a directory with the component type name can be used instead. To replace the filename with component name, use `_component.js` as the filename.

```jsx
// index.js

export { default } from "_component.js";
```

```jsx
// _component.js
import React from "react";

export default function _component() {
  return <></>;
}
```

Strap will create a dir with the same name as the component name and put every file in it. Every instance of `_component` will be replaced with component name in every file.

# Config

`strap-config.json` can be used to change the defaults. To generate the config file use `strap --init` command.

| Property      | Default                    | Summary                                                              | Type                        |
| ------------- | -------------------------- | -------------------------------------------------------------------- | --------------------------- | 
| basePath      | `src/components`           | Base dir in which components will be generated                       | string                      |
| templatesDir  | `src/component/.templates` | Path to look for templates dir                                       | string                      |
| componentType | func                       | Component type that will be generated if the type flag not specified | `func / functional / class` |
| verboseOutput | false                      | Whether to show verbose output or not                                | bool                        |
| force         | false                      | Whether to overwrite existing component                              | bool                        |

# Examples

#### Nested Path

`strap Hello/World/Button`

#### Class Component

`strap Button -c` or `strap Button --class`
