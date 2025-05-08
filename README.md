# Windortent

**Windortent** is a Windows-exclusive installer module for the [AHQ Store](https://github.com/ahqstore/client), designed to handle executable-based application formats.

It supports `.exe` and `.msix` application bundles, enabling seamless installations that may require user interaction or elevated permissions.

---

## Features

- ✅ Installs `.exe` and `.msix` application files
- ✅ Parses and executes `.exec` manifests
- ✅ Supports custom icons and descriptions
- ✅ Integrated directly with the AHQ Store’s Windows installation flow

---

## Supported Formats

| Format  | Description                  |
| ------- | ---------------------------- |
| `.exe`  | Standard Windows executables |
| `.msix` | Windows application packages |

---

## `.exec` Manifest Specification

> This is only meant to be taken as a reference

Each Windortent app includes an `.exec` file that defines how the application should be executed:

```json
{
  "exe": "path/to/app.exe",
  "args": "--start --mode=full",
  "icon": "optional/icon/path.ico",
  "desc": "Optional description of the application"
}
```

### Field Breakdown

| Field  | Type   | Description                                       |
| ------ | ------ | ------------------------------------------------- |
| `exe`  | string | Path to the executable (relative to package root) |
| `args` | string | Optional command-line arguments                   |
| `icon` | string | Optional path to an icon file                     |
| `desc` | string | Optional description of the application           |

> Note: `args` is a **string**, consistent with Windows API usage and `.desktop`-style launching logic.

---

## Installation Flow

1. AHQ Store recognizes the installer as Windortent-compatible.
2. The `.msix` or `.exe` file is unpacked or executed directly.
3. The `.exec` manifest is parsed for metadata.
4. The application is launched with the specified arguments.
5. Optionally, icons and descriptions are used for shortcuts or AHQ Store entries.

---

## License

This project is licensed under the MIT License. See the [`LICENSE`](./LICENSE) file for details.

---

## Reporting Issues

Please submit issues or feature requests to the [AHQ Store GitHub repository](https://github.com/ahqstore/client/issues).

---

## Maintained By

1. **AHQ Softwares**:
   [@AHQ-Softwares](https://github.com/AHQ-Softwares)
