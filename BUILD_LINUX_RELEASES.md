# Building Linux Binaries for All Releases

## Prerequisites

Before building Linux binaries, ensure you have the following installed:

1. **Rust and Cargo** - Already installed ✓
2. **Build Tools** - Need to install:
   ```bash
   sudo apt-get update
   sudo apt-get install -y build-essential
   ```

## Building All Releases

Once dependencies are installed, run:

```bash
cd /home/gerry/develop/SimpleImageConverter
source "$HOME/.cargo/env"
./scripts/build-all-linux-releases.sh
```

## What the Script Does

The script will:
1. Save your current git state
2. For each release version (v0.1.0, v0.1.1, v0.2.0, v0.2.1, v0.2.2):
   - Checkout the release commit
   - Clean previous build artifacts
   - Build release binaries (img-convert, mesh-convert, converter-gui if available)
   - Package CLI binaries using `scripts/package-linux.sh`
   - Package GUI binary using `scripts/package-gui-linux.sh` (if available)
3. Return to your original branch/commit
4. List all created packages

## Output

Packages will be created in the `release/` directory:
- `simpleimageconverter-{version}-linux-x64.tar.gz` - CLI binaries
- `simpleimageconverter-gui-{version}-linux-x64.tar.gz` - GUI binary (for versions that include it)

## Release Versions

The script builds binaries for:
- **v0.1.0** - Initial release (CLI only)
- **v0.1.1** - Advanced mesh manipulation features (CLI only)
- **v0.2.0** - STEP format support (CLI only)
- **v0.2.1** - GUI Application (CLI + GUI)
- **v0.2.2** - GUI Enhancements (CLI + GUI)

## Notes

- Older versions (v0.1.0, v0.1.1, v0.2.0) may not have the GUI binary
- The script will skip packaging if binaries or packaging scripts don't exist for a particular version
- Build time will vary depending on your system (expect 10-30 minutes total)

