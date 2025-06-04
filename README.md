# Windortent

**Windortent** is a Windows-exclusive installer module for the [AHQ Store](https://github.com/ahqstore/client), designed to handle executable-based application formats.

It supports `.exe`, `.msi`, `.zip`, `.ahqdb`, `.msix` application bundles, enabling seamless installations that may require user interaction or elevated permissions.

---

## Features

- ✅ Installs `.exe`, `.msi`, `.ahqdb`, `.zip` and `.msix` application files
- ✅ Supports custom icons and descriptions
- ✅ Integrated directly with the AHQ Store’s Windows installation flow

---

## Supported Formats

| Format   | Description                  |
| -------- | ---------------------------- |
| `.exe`   | Standard Windows executables |
| `.msix`  | Windows application packages |
| `.msi`   | Standard Windows installers  |
| `.zip`   | Compressed application files |
| `.ahqdb` | AHQ Store database files     |

> Note: The `.ahqdb` format is a custom format used by the AHQ Store to store application metadata.

---

## Installation Flow

1. AHQ Store recognizes the installer as Windortent-compatible.
2. The installer is downloaded and extracted.

---

## License

This project is licensed under the MIT License. See the [`LICENSE`](./LICENSE) file for details.

---

## Reporting Issues

Please submit issues or feature requests to the [AHQ Store GitHub repository](https://github.com/ahqstore/client/issues).

---

## Maintained By

1. **A. Chakraborty**:
   [@ahqsoftwares](https://github.com/ahqsoftwares)

2. **AHQ Softwares**:
   [@AHQ-Softwares](https://github.com/AHQ-Softwares)

3. **AHQ Store**:
   [@ahqstore](https://github.com/ahqstore)
