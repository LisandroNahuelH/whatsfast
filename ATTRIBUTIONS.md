# Attributions

WhatsFast is an independent project. This file lists the people and projects
this app builds on. Thank you to everyone below.

## WhatsFast

**Maintainer:** [LisandroNahuelH](https://github.com/LisandroNahuelH)  
**Repository:** [LisandroNahuelH/whatsfast](https://github.com/LisandroNahuelH/whatsfast)

WhatsFast is a Windows-focused continuation of ideas and code that began in
the open-source ZapFast lineage. It is not affiliated with Meta or WhatsApp Inc.

## ZapFast (upstream lineage)

WhatsFast derives substantial application code from
[crmne/zapfast](https://github.com/crmne/zapfast), created and maintained by
**Carmine Paolino** and contributors, under the
[MIT License](LICENSE).

Thank you to Carmine and the ZapFast contributors for the client architecture,
UI patterns, and years of work on a native WhatsApp linked-device experience.

## whatsapp-rust (protocol)

WhatsApp wire protocol and client library:

- [oxidezap/whatsapp-rust](https://github.com/oxidezap/whatsapp-rust)

Thank you to the whatsapp-rust maintainers and contributors.

## UI and assets

| Component | Source | License |
|-----------|--------|---------|
| Immediate-mode UI | [egui](https://github.com/emilk/egui) / [emilk](https://github.com/emilk) | MIT OR Apache-2.0 |
| UI icons (Lucide-style) | [Lucide](https://lucide.dev/) | ISC ([assets/icons/LICENSE.txt](assets/icons/LICENSE.txt)) |
| Montserrat variable font | [Montserrat](https://github.com/JulietaUla/Montserrat) | SIL Open Font License ([assets/fonts/Montserrat-LICENSE.txt](assets/fonts/Montserrat-LICENSE.txt)) |
| Color emoji (where used) | Noto Color Emoji | See [assets/fonts/NotoColorEmoji-LICENSE.txt](assets/fonts/NotoColorEmoji-LICENSE.txt) |

Thank you to the authors of these libraries and asset packs.

## Rust ecosystem

Hundreds of crates from [crates.io](https://crates.io/) power builds, crypto,
media, and tooling. Each crate carries its own license in `Cargo.lock` and
upstream repositories. No separate listing is required for every dependency;
the MIT license of this repository applies to **this project's own source
files**, not to relicensing third-party crates.

## WhatsApp and Meta

**WhatsApp** is a trademark of **Meta Platforms, Inc.**  
WhatsFast is an unofficial linked-device client. It is **not** endorsed,
sponsored, or affiliated with WhatsApp or Meta.

## License file

**Copyright (c) 2026 Lisandro Nahuel** — this repository and WhatsFast releases
maintained here.

Portions of the codebase derive from
[crmne/zapfast](https://github.com/crmne/zapfast); **Copyright (c) 2026 Carmine
Paolino** and contributors, also under MIT. See the ZapFast section above.

The full MIT license text is in [LICENSE](LICENSE). When you redistribute
binaries or source, keep that file and this attributions document with them.
