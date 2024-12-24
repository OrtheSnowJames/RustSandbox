# Check if winget is installed
$winget_installed = Get-Command winget -ErrorAction SilentlyContinue

if ($winget_installed) {
    Write-Host "winget is installed. Proceeding with installations..." -ForegroundColor Green
} else {
    Write-Host "winget is not installed. Please install the latest version of Windows 10 or later, which includes winget." -ForegroundColor Red
    exit
}

Write-Host "Installing GLFW..." -ForegroundColor Yellow
winget install Microsoft.NuGet
nuget install glfw

# Install CMake
Write-Host "Installing CMake..." -ForegroundColor Yellow
winget install --id Kitware.CMake -e --source winget

# Install curl
Write-Host "Installing curl..." -ForegroundColor Yellow
winget install --id curl.curl -e --source winget

# Install MinGW
Write-Host "Installing MinGW..." -ForegroundColor Yellow
winget install --id mingw-21.3 -e --source winget

# Install Doxygen
Write-Host "Installing Doxygen..." -ForegroundColor Yellow
winget install --id Doxygen.Doxygen -e --source winget

# Install Git
Write-Host "Installing Git..." -ForegroundColor Yellow
winget install --id Git.Git -e --source winget

# Install Cargo
Write-Host "Installing Cargo..." -ForegroundColor Yellow
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup install stable

# Install Raylib
Write-Host "Installing Raylib..." -ForegroundColor Yellow
cargo install raylib

Write-Host "All installations are complete!" -ForegroundColor Green
