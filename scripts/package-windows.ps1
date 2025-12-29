# Windows Packaging Script
# Creates a portable ZIP archive for Windows distribution

param(
    [string]$Version = "",
    [string]$Target = "x86_64-pc-windows-msvc"
)

$ErrorActionPreference = "Stop"

Write-Host "Packaging SimpleImageConverter for Windows..." -ForegroundColor Green

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
            $Version = "0.2.0"
            Write-Host "Using default version: $Version" -ForegroundColor Yellow
        }
    }
}

# Set paths
$ReleaseDir = "release\windows-x64-v$Version"
$ZipName = "simpleimageconverter-$Version-windows-x64.zip"
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
$ImgConvertPath = $null
$MeshConvertPath = $null

if (Test-Path "$BinDir\img-convert.exe") {
    $ImgConvertPath = "$BinDir\img-convert.exe"
    $MeshConvertPath = "$BinDir\mesh-convert.exe"
    Write-Host "Using cross-compiled binaries from: $BinDir" -ForegroundColor Yellow
} elseif (Test-Path "$NativeBinDir\img-convert.exe") {
    $ImgConvertPath = "$NativeBinDir\img-convert.exe"
    $MeshConvertPath = "$NativeBinDir\mesh-convert.exe"
    Write-Host "Using native binaries from: $NativeBinDir" -ForegroundColor Yellow
} else {
    Write-Error "Binaries not found. Expected locations:`n  - $BinDir\img-convert.exe`n  - $NativeBinDir\img-convert.exe`n`nRun 'cargo build --release' or 'cargo build --release --target $Target' first."
    exit 1
}

# Copy binaries
Write-Host "Copying binaries..." -ForegroundColor Yellow
Copy-Item $ImgConvertPath $ReleaseDir
Copy-Item $MeshConvertPath $ReleaseDir

# Copy documentation
Write-Host "Copying documentation..." -ForegroundColor Yellow
if (Test-Path "README.md") {
    Copy-Item "README.md" $ReleaseDir
}
if (Test-Path "LICENSE") {
    Copy-Item "LICENSE" $ReleaseDir
}

# Create README for Windows users
$WindowsReadme = @"
# SimpleImageConverter for Windows

## Installation

1. Extract this ZIP file to a location of your choice (e.g., `C:\Tools\SimpleImageConverter`)
2. (Optional) Add the directory to your PATH environment variable:
   - Open System Properties > Environment Variables
   - Edit PATH and add the directory containing img-convert.exe and mesh-convert.exe
3. Open Command Prompt or PowerShell and run:
   `img-convert --help`
   `mesh-convert --help`

## Usage

\`\`\`powershell
# Convert image
img-convert input.png jpg

# Convert mesh
mesh-convert model.stl obj
\`\`\`

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
Write-Host "  1. Test the package by extracting and running the binaries" -ForegroundColor White
Write-Host "  2. Upload to GitHub Releases" -ForegroundColor White
Write-Host "  3. Update winget manifest (if applicable)" -ForegroundColor White

