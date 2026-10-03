# EmuConf

Universal emulator configuration conversion infrastructure.

EmuConf converts emulator configuration files through a canonical intermediate model instead of maintaining pairwise converters.

## M0 scope

M0 establishes the architecture for Amiga emulators: WinUAE, FS-UAE, Amiberry, Fellow, FellowNG, and Copperline.

The canonical representation is the **EmuConf Intermediate Model (ECIM)**. M0 provides the Rust workspace, ECIM foundations, emulator identifiers, diagnostics/loss model, format detection API, CLI surface, fixtures, and tests. M1 implements the first practical WinUAE, FS-UAE, and Amiberry conversion layer.

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

Conversion targets currently include WinUAE, FS-UAE, Amiberry, Fellow, FellowNG, and Copperline.

## Roadmap

- **M0** Foundation and ECIM
- **M1** WinUAE, FS-UAE, and Amiberry import/export — implemented
- **M2** Fellow and FellowNG — implemented
- **M3** Copperline — implemented
- **M4** fidelity reporting and round-trip corpus
- **M5** Amilea/AmiVM integration
- **M6** stable API/CLI and v1.0.0

## License

MIT

## M1 supported model

The UAE-family adapters currently normalize model, CPU/FPU/MMU/JIT, chipset and video standard, Chip/Slow/Fast/Z3 RAM, Kickstart ROM, RTG, audio, joystick ports, four floppy slots, hardfile mounts, and directory mounts. Unknown key/value options are preserved when possible.

The mappings are intentionally conservative. Emulator-specific semantics will continue to be refined against real configuration corpora.

## M2–M3 adapters

Fellow, FellowNG, and Copperline now participate in the same ECIM import/export pipeline and CLI conversion surface. Their initial mappings are conservative: common Amiga machine/storage/input/audio state is normalized, while emulator-specific key/value options remain preserved for later fidelity refinement.
