# Cipher
## A modern file encryption tool built with Rust
> Secure file encryption using ChaCha20-Poly1305 with interactive CLI.

## How to Install Cipher
### From Source
```bash
git clone https://github.com/Adel-Ayoub/Cipher.git
cd Cipher
cargo build --release
```

## CLI Options
| Flag | Command | Description |
| ---- | ------- | ----------- |
| **-i** | `--interactive` | Launch interactive mode |
| **-e** | `--encrypt` | Encrypt a file |
| **-d** | `--decrypt` | Decrypt a file |
| **-h** | `--help` | Show help information |
| **-V** | `--version` | Show version |

## Technical Stack
- **Rust** - Systems programming language for performance and safety
- **ChaCha20-Poly1305** - Modern authenticated encryption algorithm
- **PBKDF2-SHA3** - Key derivation function with configurable rounds
- **XXHash32** - Fast integrity verification checksums

## Requirements
- Rust 1.70+ and Cargo
- No external dependencies (statically compiled)
- Compatible with Windows, macOS, and Linux

## File Format
| Component | Size | Description |
| --------- | ---- | ----------- |
| **Extension** | - | `.cph` (cipher file extension) |
| **Magic Number** | 8 bytes | `cph3rust` identifier |
| **Version** | 2 bytes | File format version (0x0001) |
| **Salt** | 8 bytes | Random salt for PBKDF2 |
| **Rounds** | 4 bytes | PBKDF2 iteration count |
| **Nonce** | 12 bytes | ChaCha20 encryption nonce |
| **Length** | 8 bytes | Size of encrypted data |
| **Encrypted Data** | Variable | ChaCha20-Poly1305 encrypted content |
| **Checksum** | 4 bytes | XXHash32 integrity verification |

## Usage
### Interactive Mode
```bash
# Launch interactive interface
./target/release/cipher -i
```

### Encrypt a File
```bash
# Choose option 1 in interactive mode
# Enter password and security rounds
# Specify input and output files
```

### Decrypt a File
```bash
# Choose option 2 in interactive mode
# Enter the same password used for encryption
# Specify encrypted file and output location
```

## Example Usage

### Complete Encryption/Decryption Workflow
```bash
# Create test file
echo "This is secret data!" > secret.txt
```

```bash
# Encrypt the file
./target/release/cipher -i
# Interactive prompts:
# your choice > 1
# enter password: MySecurePassword123
# how many rounds?: 100000
# enter a filepath: secret.txt
# where do u want to save the file?: encrypted_secret
# Result: Creates encrypted_secret.cph
```

```bash
# Verify encrypted file was created
ls -la secret*
# secret.txt (original file)
# encrypted_secret.cph (encrypted file - safe to share/store)
```

```bash
# Decrypt the file
./target/release/cipher -i
# Interactive prompts:
# your choice > 2
# what file do you want to decrypt?: encrypted_secret.cph
# enter password: MySecurePassword123
# where do u want to save the file?: recovered_secret.txt
# Result: Creates recovered_secret.txt with original content
```

```bash
# Verify content matches original
diff secret.txt recovered_secret.txt
# No output means files are identical
```

### Understanding .cph Files
- **Extension**: All encrypted files get `.cph` extension automatically
- **Format**: Binary file containing encrypted data + metadata
- **Portability**: Can be safely moved, copied, or shared
- **Security**: Cannot be read without the original password
- **Size**: Slightly larger than original (due to encryption overhead)
