# EmuConf

Universal emulator configuration conversion infrastructure.

EmuConf converts emulator configuration files through a canonical intermediate model instead of maintaining pairwise converters.

## M0 scope

M0 establishes the architecture for Amiga emulators: WinUAE, FS-UAE, Amiberry, Fellow, FellowNG, and Copperline.

The canonical representation is the **EmuConf Intermediate Model (ECIM)**. M0 provides the Rust workspace, ECIM foundations, emulator identifiers, diagnostics/loss model, format detection API, CLI surface, fixtures, and tests. Full configuration mappings begin in M1.

## Principles

- Never silently lose configuration.
- Keep the core independent of any single emulator.
- Preserve unknown source fields where possible.
- Report mapping fidelity as exact, mapped, approximate, unsupported, or preserved.
- Keep import/export adapters separate from ECIM.

## CLI

```text
emuconf detect <config>
emuconf inspect <config>
emuconf convert <config> --to <format>
```

During M0, `convert` intentionally reports that conversion adapters are not implemented.

## Roadmap

- **M0** Foundation and ECIM
- **M1** WinUAE, FS-UAE, and Amiberry import/export
- **M2** Fellow and FellowNG
- **M3** Copperline
- **M4** fidelity reporting and round-trip corpus
- **M5** Amilea/AmiVM integration
- **M6** stable API/CLI and v1.0.0

## License

MIT
