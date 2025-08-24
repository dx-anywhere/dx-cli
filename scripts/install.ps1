# dx-cli installer (Windows PowerShell)
# Usage (PowerShell):
#   iwr https://raw.githubusercontent.com/OWNER/REPO/main/scripts/install.ps1 -UseBasicParsing | iex
#   # Optional: $env:DXANY_VERSION = "v0.1.0"; iwr ... | iex

$ErrorActionPreference = 'Stop'

$repoOwner = if ($env:DXANY_REPO_OWNER) { $env:DXANY_REPO_OWNER } else { 'dx-anywhere' }
$repoName  = if ($env:DXANY_REPO_NAME)  { $env:DXANY_REPO_NAME }  else { 'dx-cli' }
$version   = if ($env:DXANY_VERSION -and $env:DXANY_VERSION -ne '') { $env:DXANY_VERSION } else { 'latest' }

# Determine architecture and candidate variants
$procArch = $env:PROCESSOR_ARCHITECTURE
$archVariants = @()
switch -Regex ($procArch) {
  'AMD64'   { $archVariants = @('x86_64','amd64'); break }
  'ARM64'   { $archVariants = @('aarch64','arm64'); break }
  default   { throw "Unsupported architecture: $procArch" }
}

# Candidate assets to try (prefer .exe, then .zip)
$assets = @()
foreach ($a in $archVariants) {
  $assets += "dxany-windows-$a.exe"
  $assets += "dxany-windows-$a.zip"
}

# Resolve URL per asset and attempt download/extraction
$cwd = Get-Location
$outExe = Join-Path $cwd 'dxany.exe'
$tempDir = Join-Path $env:TEMP ("dxany_install_" + [System.Guid]::NewGuid().ToString('N'))
New-Item -ItemType Directory -Path $tempDir | Out-Null

$tried = @()
$success = $false

function Get-DownloadUrl([string]$asset) {
  if ($version -eq 'latest') {
    return "https://github.com/$repoOwner/$repoName/releases/latest/download/$asset"
  } else {
    return "https://github.com/$repoOwner/$repoName/releases/download/$version/$asset"
  }
}

function Download-File([string]$url, [string]$dest) {
  if (Get-Command Invoke-WebRequest -ErrorAction SilentlyContinue) {
    Invoke-WebRequest -Uri $url -OutFile $dest -UseBasicParsing
  }
  else {
    $wc = New-Object System.Net.WebClient
    $wc.DownloadFile($url, $dest)
  }
}

try {
  foreach ($asset in $assets) {
    $url = Get-DownloadUrl $asset
    $tried += $url
    Write-Host "Trying $asset from $url ..."

    if ($asset.EndsWith('.exe')) {
      try {
        Download-File -url $url -dest $outExe
        if ((Test-Path $outExe) -and ((Get-Item $outExe).Length -gt 0)) { $success = $true; break }
      } catch { continue }
    }
    elseif ($asset.EndsWith('.zip')) {
      $zipPath = Join-Path $tempDir $asset
      try {
        Download-File -url $url -dest $zipPath
        if ((Test-Path $zipPath) -and ((Get-Item $zipPath).Length -gt 0)) {
          $extractDir = Join-Path $tempDir 'extracted'
          New-Item -ItemType Directory -Path $extractDir -ErrorAction SilentlyContinue | Out-Null
          Expand-Archive -Path $zipPath -DestinationPath $extractDir -Force
          $found = Get-ChildItem -Path $extractDir -Recurse -Filter 'dxany.exe' | Select-Object -First 1
          if ($null -ne $found) {
            Copy-Item -Path $found.FullName -Destination $outExe -Force
            $success = $true; break
          }
        }
      } catch { continue }
    }
  }
}
finally {
  # Cleanup temp
  if (Test-Path $tempDir) { Remove-Item -Recurse -Force $tempDir }
}

if (-not $success) {
  Write-Error "Failed to download dx-cli Windows asset. Tried URLs:`n  - $(($tried -join "`n  - "))`nHint: Set DXANY_VERSION (e.g., v0.1.0) if you need a specific tag, and ensure the asset naming matches your platform."
  throw
}

if (-Not (Test-Path $outExe)) {
  throw "Download completed but dxany.exe not found at $outExe"
}

$size = (Get-Item $outExe).Length
if ($size -le 0) {
  throw "Downloaded dxany.exe has zero size; the download might have failed."
}

Write-Host "Installed .\dxany.exe"
Write-Host "Run: .\dxany.exe --help"
