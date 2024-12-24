#!/bin/bash

# Function to detect package manager
get_package_manager() {
    if command -v apt &> /dev/null; then
        echo "apt"
    elif command -v dnf &> /dev/null; then
        echo "dnf"
    elif command -v yum &> /dev/null; then
        echo "yum"
    elif command -v apk &> /dev/null; then
        echo "apk"
    elif command -v zypper &> /dev/null; then
        echo "zypper"
    elif command -v pacman &> /dev/null; then
        echo "pacman"
    elif command -v brew &> /dev/null; then
        echo "brew"
    else
        echo "unknown"
    fi
}

# Function to install packages based on package manager
install_packages() {
    local PKG_MANAGER=$(get_package_manager)
    
    case $PKG_MANAGER in
        "apt")
            sudo apt update
            sudo apt install -y cmake pkg-config libglfw3 libglfw3-dev ninja-build
            git clone https://github.com/raysan5/raylib.git
            cd raylib
            ninja -C src
            sudo ninja -C src install
            ;;
        "dnf"|"yum")
            sudo $PKG_MANAGER update -y
            sudo $PKG_MANAGER install -y cmake pkgconfig glfw-devel
            sudo $PKG_MANAGER groupinstall -y "Development Tools"
            sudo $PKG_MANAGER install raylib-devel
            ;;
        "apk")
            sudo apk update
            sudo apk add cmake pkgconf glfw-dev
            #clone and install raylib
            git clone https://github.com/raysan5/raylib.git
            cd raylib
            make
            sudo make install
            cd ..
            ;;
        "zypper")
            sudo zypper refresh
            sudo zypper install -y cmake pkg-config glfw-devel libglfw3 raylib-devel
            ;;
        "pacman")
            sudo pacman -Sy
            sudo pacman -S --noconfirm cmake pkgconf glfw-x11 raylib
            ;;
        "brew")
            brew install cmake glfw raylib
            ;;
        *)
            echo "Unsupported package manager"
            exit 1
            ;;
    esac
}

# Main installation process
if [[ "$OSTYPE" == "darwin"* ]]; then
    if ! command -v brew &> /dev/null; then
        echo "Installing Homebrew..."
        /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
    fi
fi

echo "Installing required packages..."
install_packages

echo "Installation complete!"