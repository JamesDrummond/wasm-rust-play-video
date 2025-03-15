# WebAssembly Rust Project

This is a simple WebAssembly project using Rust that demonstrates basic Rust to WebAssembly compilation and JavaScript interop.

## Features
- Basic string manipulation with the `greet` function
- Simple arithmetic with the `add` function
- Console logging from Rust to JavaScript
- VS Code integration for development
- One-click build and run support

## Prerequisites

1. Install Rust and cargo (Windows PowerShell):
```powershell
# Download and run rustup-init.exe
Invoke-WebRequest https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe -OutFile rustup-init.exe
.\rustup-init.exe

# After installation, restart your PowerShell
```

2. Install wasm-pack:
```powershell
cargo install wasm-pack
```

3. Install basic-http-server:
```powershell
cargo install basic-http-server
```

## Building and Running

### Using VS Code/Cursor IDE (Recommended)
1. Open the project in VS Code or Cursor IDE
2. Start the project (choose one method):
   
   **Method 1 - One-Click Build and Run:**
   - Press `Ctrl+Shift+D` to open the Run and Debug view
   - Select "Build and Run WASM" from the dropdown at the top
   - Click the green play button or press `F5`
   - This will automatically:
     1. Build the WebAssembly module
     2. Start the server
     3. Open your browser to http://127.0.0.1:4000

   **Method 2 - Manual Steps:**
   - Build the WebAssembly module:
     - Open the integrated terminal (Press `` Ctrl+` `` or View -> Terminal)
     - Run: `wasm-pack build --target web`
   - Start the server:
     - Select "Launch WASM Server" from the Run and Debug dropdown
     - Press `F5`

### Development Workflow
1. Make changes to `src/lib.rs`
2. Click "Build and Run WASM" in the Run and Debug view (or press `F5` with it selected)
3. The browser will automatically refresh with your changes

### Manual Method
1. Build the WebAssembly module:
```powershell
wasm-pack build --target web
```

2. Start the server:
```powershell
basic-http-server .
```

3. Open http://127.0.0.1:4000 in your browser

## Project Structure
- `src/lib.rs` - Rust source code with WebAssembly exports
- `Cargo.toml` - Rust project configuration and dependencies
- `index.html` - Example usage of the WebAssembly module
- `pkg/` - Generated WebAssembly module and JavaScript bindings
- `.vscode/` - VS Code configuration files
  - `launch.json` - Debug and server launch configuration with build tasks

## IDE Features
The project includes VS Code/Cursor IDE configuration for:
- One-click build and run with `F5`
- Automatic browser opening
- Integrated terminal for running commands
- Debug console for server output
- Rust-analyzer integration for code completion and hints

## Testing
Run the Rust tests with:
```powershell
cargo test
```

## Troubleshooting

### Common Windows Issues
1. If you get a permission error, try running PowerShell as Administrator
2. If commands are not found after installation, restart PowerShell to refresh the PATH
3. Make sure you have the Microsoft Visual Studio C++ build tools installed (required for Rust on Windows)
4. If the server doesn't start, check if port 4000 is already in use:
```powershell
# Check if port 4000 is in use
Get-NetTCPConnection -LocalPort 4000 -ErrorAction SilentlyContinue

# If needed, you can kill the process using the port
$process = Get-NetTCPConnection -LocalPort 4000 -ErrorAction SilentlyContinue | Select-Object OwningProcess
if ($process) { Stop-Process -Id $process.OwningProcess -Force }
```

### IDE-specific Issues
1. If the Run/Debug button is grayed out:
   - Make sure you have the project folder open (not just a single file)
   - Try reloading the window (Ctrl+Shift+P -> "Reload Window")
   - Check that `.vscode/launch.json` exists and is properly formatted

2. If the browser doesn't open automatically:
   - Check that the server started successfully in the Debug Console
   - Try manually opening http://127.0.0.1:4000
   - Make sure no other process is using port 4000

3. If the build step fails:
   - Check the Debug Console for error messages
   - Make sure wasm-pack is installed correctly
   - Try running `wasm-pack build --target web` manually in the terminal
