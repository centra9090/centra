# Centra

Centra adalah mesin eksekusi minimal dan deterministik yang dibangun di Rust. Sistem ini dirancang untuk pemrosesan teks sederhana dengan fokus pada performa, kompresi, dan modularitas tanpa dependensi eksternal.

## Fitur Utama

- **Pipeline Eksekusi**: Input string → Lexer → Parser → IR → Runtime → CSM Encode → Output
- **Kompresi Adaptif**: Menggunakan dictionary-based encoding dengan bit-packing adaptif (1-8 bit per simbol)
- **Mode Eksekusi**: Fast, Secure, Intelligent untuk kontrol fitur
- **Adapter Layer**: Mendukung output ke Memory, File, atau transport kustom
- **Decoder**: Dekompresi data yang dikompresi kembali ke string asli
- **Zero-Copy**: Optimasi untuk mengurangi alokasi memori
- **Deterministik**: Output sama untuk input sama

## Arsitektur

Proyek ini terstruktur dalam modul-modul berikut:

- `src/lib.rs`: API utama dan orkestrasi pipeline
- `src/compiler/`: Lexer, parser, dan IR
- `src/runtime/`: Eksekusi simbol
- `src/data/`: CSM, bit-packing, dan decoder
- `src/adapters/`: Transport layer (Memory, File)
- `src/system/`: Mode eksekusi

## Instalasi

Pastikan Rust terinstal (minimal versi 1.70). Clone repositori dan build:

```bash
git clone https://github.com/centra9090/centra.git
cd centra
cargo build --release
```

## Penggunaan

### API Dasar

```rust
use centra::{run_source, run, Mode, run_with_transport, MemoryTransport};

fn main() {
    // Eksekusi sederhana
    let output = run_source("hello world");
    println!("Compressed: {:?}", output);

    // Dengan mode
    let output = run("hello world", Mode::Fast);

    // Dengan transport
    let mut transport = MemoryTransport::new();
    run_with_transport("hello world", &mut transport);
    println!("Stored: {:?}", transport.buffer);

    // Dekode
    let decoded = decode_input("hello world");
    println!("Decoded: {:?}", decoded);
}
```

### Contoh Output

- Input: `"hello world"`
- Compressed: Byte array dengan header dan data dikompresi
- Decoded: `["hello", "world"]`

## Pengembangan

Proyek ini dikembangkan melalui fase iteratif:

- **Phase 1**: Pipeline dasar
- **Phase 2**: CSM layer
- **Phase 3**: Adapter layer
- **Phase 4**: Mode system
- **Phase 5**: Zero-copy optimization
- **Phase 6**: Dictionary compression
- **Phase 7**: Bit-level packing
- **Phase 8**: Adaptive compression + decoder

## Kontribusi

Kontribusi diterima! Pastikan kode tetap minimal, deterministik, dan tanpa dependensi eksternal.

## Lisensi

MIT License