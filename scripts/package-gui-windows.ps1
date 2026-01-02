# Windows GUI Packaging Script
# Creates a portable ZIP archive for Windows GUI distribution

param(
    [string]$Version = "",
    [string]$Target = "x86_64-pc-windows-msvc"
)

$ErrorActionPreference = "Stop"

Write-Host "Packaging SimpleImageConverter GUI for Windows..." -ForegroundColor Green

# Extract version from Git tag or Cargo.toml if not provided
if ([string]::IsNullOrEmpty($Version)) {
    # Try to get version from Git tag
    $gitTag = git describe --tags --exact-match 2>$null
    if ($gitTag) {
        $Version = $gitTag -replace '^v', ''
        Write-Host "Using version from Git tag: $Version" -ForegroundColor Yellow
    } else {
        # Fall back to Cargo.toml
        $cargoVersion = Select-String -Path "Cargo.toml" -Pattern '^version = "([^"]+)"' | ForEach-Object { $_.Matches.Groups[1].Value }
        if ($cargoVersion) {
            $Version = $cargoVersion
            Write-Host "Using version from Cargo.toml: $Version" -ForegroundColor Yellow
        } else {
            $Version = "0.2.2"
            Write-Host "Using default version: $Version" -ForegroundColor Yellow
        }
    }
}

# SECURITY: Validate version format to prevent injection attacks
# Allow semantic versioning: X.Y.Z or X.Y.Z-pre (e.g., 0.2.2, 0.2.2-alpha1)
if ($Version -notmatch '^\d+\.\d+\.\d+(-[a-zA-Z0-9.-]+)?$') {
    Write-Error "Invalid version format: '$Version'. Expected format: X.Y.Z or X.Y.Z-pre"
    exit 1
}

# SECURITY: Sanitize version for path usage (remove any remaining special characters)
$VersionSanitized = $Version -replace '[^0-9A-Za-z.-]', ''
if ($VersionSanitized -ne $Version) {
    Write-Warning "Version sanitized from '$Version' to '$VersionSanitized' for path safety"
    $Version = $VersionSanitized
}

# SECURITY: Validate target format to prevent path traversal
if ($Target -notmatch '^[a-zA-Z0-9_-]+$') {
    Write-Error "Invalid target format: '$Target'. Only alphanumeric, underscore, and hyphen allowed."
    exit 1
}

# Set paths (using sanitized version)
$ReleaseDir = "release\windows-x64-gui-v$Version"
$ZipName = "simpleimageconverter-gui-$Version-windows-x64.zip"
$BinDir = "target\$Target\release"
$NativeBinDir = "target\release"

# Clean previous release
if (Test-Path $ReleaseDir) {
    Remove-Item -Recurse -Force $ReleaseDir
}
if (Test-Path $ZipName) {
    Remove-Item -Force $ZipName
}

# Create release directory
New-Item -ItemType Directory -Force -Path $ReleaseDir | Out-Null

# Determine binary location (check cross-compiled first, then native)
$GuiBinaryPath = $null

if (Test-Path "$BinDir\converter-gui.exe") {
    $GuiBinaryPath = "$BinDir\converter-gui.exe"
    Write-Host "Using cross-compiled binary from: $BinDir" -ForegroundColor Yellow
} elseif (Test-Path "$NativeBinDir\converter-gui.exe") {
    $GuiBinaryPath = "$NativeBinDir\converter-gui.exe"
    Write-Host "Using native binary from: $NativeBinDir" -ForegroundColor Yellow
} else {
    Write-Error "GUI binary not found. Expected locations:`n  - $BinDir\converter-gui.exe`n  - $NativeBinDir\converter-gui.exe`n`nRun 'cargo build --release --bin converter-gui' or 'cargo build --release --target $Target --bin converter-gui' first."
    exit 1
}

# Copy binary
Write-Host "Copying GUI binary..." -ForegroundColor Yellow
Copy-Item $GuiBinaryPath $ReleaseDir

# Copy documentation
Write-Host "Copying documentation..." -ForegroundColor Yellow
if (Test-Path "README.md") {
    Copy-Item "README.md" $ReleaseDir
}
if (Test-Path "LICENSE-APACHE") {
    Copy-Item "LICENSE-APACHE" $ReleaseDir
}
if (Test-Path "LICENSE-MIT") {
    Copy-Item "LICENSE-MIT" $ReleaseDir
}

# Create README for Windows GUI users
$WindowsReadme = @"
# SimpleImageConverter GUI for Windows

## Installation

1. Extract this ZIP file to a location of your choice (e.g., `C:\Tools\SimpleImageConverter`)
2. Double-click `converter-gui.exe` to launch the application
3. (Optional) Create a desktop shortcut for easy access

## Usage

1. Launch `converter-gui.exe`
2. Drag and drop a file into the drop zone, or click "Browse Files..." to select a file
3. Select the output format from the radio buttons
4. Adjust options (optional):
   - Change output filename
   - Select output location
   - Adjust quality slider (for JPEG/WebP images)
5. Click "Convert" to start the conversion
6. View results in the status bar and messages area

## Features

- Drag-and-drop file support
- Visual format selection
- Quality settings for lossy image formats
- User-friendly error messages
- Progress indicators for long operations
- Thread-safe conversion processing
- Batch processing (v0.2.2)
- Preview functionality (v0.2.2)
- Settings persistence (v0.2.2)
- Conversion history (v0.2.2)

## Supported Formats

**Images:** PNG, JPEG, BMP, GIF, TIFF, WebP (SVG read-only)
**Meshes:** STL, OBJ, PLY, OFF, glTF, DXF (STEP read-only, feature-gated)

For more information, see the main README.md file.
"@
$WindowsReadme | Out-File -FilePath "$ReleaseDir\INSTALL.txt" -Encoding UTF8

# Create ZIP archive
Write-Host "Creating ZIP archive..." -ForegroundColor Yellow
Compress-Archive -Path "$ReleaseDir\*" -DestinationPath $ZipName -Force

# Display results
$ZipSize = (Get-Item $ZipName).Length / 1MB
Write-Host "`nPackage created successfully!" -ForegroundColor Green
Write-Host "  File: $ZipName" -ForegroundColor Cyan
Write-Host "  Size: $([math]::Round($ZipSize, 2)) MB" -ForegroundColor Cyan
Write-Host "  Location: $(Resolve-Path $ZipName)" -ForegroundColor Cyan

Write-Host "`nNext steps:" -ForegroundColor Yellow
Write-Host "  1. Test the package by extracting and running converter-gui.exe" -ForegroundColor White
Write-Host "  2. Upload to GitHub Releases" -ForegroundColor White
Write-Host "  3. Update winget manifest (if applicable)" -ForegroundColor White

