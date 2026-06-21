param(
    [ValidateSet("x64", "arm64")]
    [string]$Arch = "x64",
    [string]$TargetTriple = "",
    [string]$WixArch = ""
)

$ErrorActionPreference = "Stop"

$version = $env:GITHUB_REF_NAME
if ([string]::IsNullOrWhiteSpace($version)) {
    $version = "0.0.0"
}
$version = $version.TrimStart("v")
if ($version -notmatch '^\d+\.\d+\.\d+(\.\d+)?$') {
    $manifest = Get-Content "crates\aktsql_app\Cargo.toml"
    $versionLine = $manifest | Where-Object { $_ -match '^version\s*=' } | Select-Object -First 1
    $version = ($versionLine -replace 'version\s*=\s*"', '') -replace '"', ''
}

$root = (Resolve-Path ".").Path
if ([string]::IsNullOrWhiteSpace($TargetTriple)) {
    $sourceDir = Join-Path $root "target\release"
} else {
    $sourceDir = Join-Path $root "target\$TargetTriple\release"
}
$dist = Join-Path $root "dist\windows-$Arch"
$exe = Join-Path $sourceDir "aktsql.exe"
$wxs = Join-Path $root "packaging\windows\AktSQL.wxs"
$wixObj = Join-Path $dist "AktSQL-$Arch.wixobj"
$stableExe = Join-Path $dist "AktSQL-windows-$Arch.exe"
$stableMsi = Join-Path $dist "AktSQL-windows-$Arch.msi"
$versionedExe = Join-Path $dist "AktSQL-$version-windows-$Arch.exe"
$versionedMsi = Join-Path $dist "AktSQL-$version-windows-$Arch.msi"

if ([string]::IsNullOrWhiteSpace($WixArch)) {
    $WixArch = $Arch
}

New-Item -ItemType Directory -Force -Path $dist | Out-Null
Copy-Item $exe $stableExe -Force
Copy-Item $exe $versionedExe -Force

$wixBin = "${env:ProgramFiles(x86)}\WiX Toolset v3.14\bin"
if (Test-Path $wixBin) {
    $env:PATH = "$env:PATH;$wixBin"
}

candle.exe -arch "$WixArch" -dSourceDir="$sourceDir" -dProductVersion="$version" -dPackagePlatform="$WixArch" -out "$wixObj" "$wxs"
light.exe -ext WixUIExtension -out "$stableMsi" "$wixObj"
Copy-Item $stableMsi $versionedMsi -Force

Remove-Item "$dist\*.wixobj" -Force
Remove-Item "$dist\*.wixpdb" -Force -ErrorAction SilentlyContinue
