# Define the target names with .xdll extension
TARGET_X86 = output/x86/web_socket.xdll
TARGET_X64 = output/x64/web_socket.xdll

# Default target
all: create_output_dirs $(TARGET_X86) $(TARGET_X64)

# Create output directories
create_output_dirs:
	mkdir -p output/x86
	mkdir -p output/x64

# Build x86 DLL
$(TARGET_X86):
	cargo build --release --target=i686-pc-windows-msvc
	mv target/i686-pc-windows-msvc/release/web_socket.dll $(TARGET_X86)

# Build x64 DLL
$(TARGET_X64):
	cargo build --release --target=x86_64-pc-windows-msvc
	mv target/x86_64-pc-windows-msvc/release/web_socket.dll $(TARGET_X64)

# Clean up the build files
clean:
	cargo clean
	rm -rf output
