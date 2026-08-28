use std::{
    env, fs,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
    process,
};

const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

struct Sha256 {
    state: [u32; 8],
    buffer: [u8; 64],
    buffer_len: usize,
    length_bytes: u64,
}

impl Sha256 {
    fn new() -> Self {
        Self {
            state: [
                0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
                0x5be0cd19,
            ],
            buffer: [0; 64],
            buffer_len: 0,
            length_bytes: 0,
        }
    }

    fn transform(&mut self, block: &[u8; 64]) {
        let mut w = [0u32; 64];
        for (index, chunk) in block.chunks_exact(4).take(16).enumerate() {
            w[index] = u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        }
        for index in 16..64 {
            let s0 = w[index - 15].rotate_right(7)
                ^ w[index - 15].rotate_right(18)
                ^ (w[index - 15] >> 3);
            let s1 = w[index - 2].rotate_right(17)
                ^ w[index - 2].rotate_right(19)
                ^ (w[index - 2] >> 10);
            w[index] = w[index - 16]
                .wrapping_add(s0)
                .wrapping_add(w[index - 7])
                .wrapping_add(s1);
        }

        let mut a = self.state[0];
        let mut b = self.state[1];
        let mut c = self.state[2];
        let mut d = self.state[3];
        let mut e = self.state[4];
        let mut f = self.state[5];
        let mut g = self.state[6];
        let mut h = self.state[7];

        for index in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let choice = (e & f) ^ ((!e) & g);
            let temp1 = h
                .wrapping_add(s1)
                .wrapping_add(choice)
                .wrapping_add(K[index])
                .wrapping_add(w[index]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let majority = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(majority);

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        self.state[0] = self.state[0].wrapping_add(a);
        self.state[1] = self.state[1].wrapping_add(b);
        self.state[2] = self.state[2].wrapping_add(c);
        self.state[3] = self.state[3].wrapping_add(d);
        self.state[4] = self.state[4].wrapping_add(e);
        self.state[5] = self.state[5].wrapping_add(f);
        self.state[6] = self.state[6].wrapping_add(g);
        self.state[7] = self.state[7].wrapping_add(h);
    }

    fn update(&mut self, input: &[u8]) {
        self.length_bytes = self.length_bytes.wrapping_add(input.len() as u64);
        let mut cursor = 0;

        while cursor < input.len() {
            let space = 64 - self.buffer_len;
            let take = space.min(input.len() - cursor);
            self.buffer[self.buffer_len..self.buffer_len + take]
                .copy_from_slice(&input[cursor..cursor + take]);
            self.buffer_len += take;
            cursor += take;

            if self.buffer_len == 64 {
                let block = self.buffer;
                self.transform(&block);
                self.buffer_len = 0;
            }
        }
    }

    fn finalize(mut self) -> [u8; 32] {
        let bit_length = self.length_bytes.wrapping_mul(8);
        self.buffer[self.buffer_len] = 0x80;
        self.buffer_len += 1;

        if self.buffer_len > 56 {
            self.buffer[self.buffer_len..].fill(0);
            let block = self.buffer;
            self.transform(&block);
            self.buffer = [0; 64];
            self.buffer_len = 0;
        }

        self.buffer[self.buffer_len..56].fill(0);
        self.buffer[56..64].copy_from_slice(&bit_length.to_be_bytes());
        let block = self.buffer;
        self.transform(&block);

        let mut digest = [0u8; 32];
        for (index, word) in self.state.iter().enumerate() {
            digest[index * 4..index * 4 + 4].copy_from_slice(&word.to_be_bytes());
        }
        digest
    }
}

fn hex_digest(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push_str(&format!("{byte:02x}"));
    }
    output
}

fn hash_reader(mut reader: impl Read) -> Result<String, String> {
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 64 * 1024];
    loop {
        let count = reader
            .read(&mut buffer)
            .map_err(|error| format!("read failed: {error}"))?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }
    Ok(hex_digest(&hasher.finalize()))
}

fn hash_file(path: &Path) -> Result<String, String> {
    let file = File::open(path)
        .map_err(|error| format!("failed to open '{}': {error}", path.display()))?;
    hash_reader(file)
}

fn manifest_name(manifest: &Path, file: &Path) -> Result<String, String> {
    let parent = manifest.parent().unwrap_or_else(|| Path::new("."));
    let stored = file.strip_prefix(parent).unwrap_or(file);
    let value = stored
        .to_str()
        .ok_or_else(|| format!("path '{}' is not valid UTF-8", stored.display()))?;
    if value.contains(['\n', '\r']) {
        return Err(format!("path '{}' contains a line break", stored.display()));
    }
    Ok(value.to_owned())
}

fn atomic_write(path: &Path, content: &str) -> Result<(), String> {
    let file_name = path
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| "manifest path has no UTF-8 file name".to_owned())?;
    let temp_name = format!(".{file_name}.tmp.{}", process::id());
    let temp = path.with_file_name(temp_name);
    fs::write(&temp, content)
        .map_err(|error| format!("failed to write '{}': {error}", temp.display()))?;
    fs::rename(&temp, path)
        .map_err(|error| format!("failed to replace '{}': {error}", path.display()))?;
    Ok(())
}

fn create_manifest(manifest: &Path, files: &[String]) -> Result<(), String> {
    if files.is_empty() {
        return Err("at least one release file is required".to_owned());
    }

    let mut entries = Vec::new();
    for file in files {
        let path = Path::new(file);
        if !path.is_file() {
            return Err(format!("'{}' is not a regular file", path.display()));
        }
        let name = manifest_name(manifest, path)?;
        let digest = hash_file(path)?;
        entries.push((name, digest));
    }
    entries.sort_by(|left, right| left.0.cmp(&right.0));

    let mut content = String::new();
    for (name, digest) in entries {
        content.push_str(&format!("{digest}  {name}\n"));
    }
    atomic_write(manifest, &content)
}

fn parse_manifest(input: &str) -> Result<Vec<(String, String)>, String> {
    let mut entries = Vec::new();
    for (index, line) in input.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let Some((digest, path)) = line.split_once("  ") else {
            return Err(format!("invalid manifest line {}", index + 1));
        };
        if digest.len() != 64 || !digest.chars().all(|ch| ch.is_ascii_hexdigit()) {
            return Err(format!("invalid SHA-256 digest on line {}", index + 1));
        }
        if path.is_empty() {
            return Err(format!("missing path on line {}", index + 1));
        }
        entries.push((digest.to_ascii_lowercase(), path.to_owned()));
    }
    if entries.is_empty() {
        return Err("manifest contains no files".to_owned());
    }
    Ok(entries)
}

fn resolve_manifest_entry(manifest: &Path, entry: &str) -> PathBuf {
    let path = Path::new(entry);
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        manifest
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join(path)
    }
}

fn verify_manifest(manifest: &Path) -> Result<bool, String> {
    let input = fs::read_to_string(manifest)
        .map_err(|error| format!("failed to read '{}': {error}", manifest.display()))?;
    let entries = parse_manifest(&input)?;
    let mut all_ok = true;

    for (expected, entry) in entries {
        let path = resolve_manifest_entry(manifest, &entry);
        match hash_file(&path) {
            Ok(actual) if actual == expected => println!("OK  {entry}"),
            Ok(_) => {
                println!("MISMATCH  {entry}");
                all_ok = false;
            }
            Err(error) => {
                println!("ERROR  {entry}: {error}");
                all_ok = false;
            }
        }
    }
    Ok(all_ok)
}

fn help() {
    println!(
        "ReleaseSeal 0.1.0-dev\n\nUSAGE:\n  releaseseal create <MANIFEST> <FILE>...\n  releaseseal verify <MANIFEST>\n\nThe current preview creates deterministic SHA-256 manifests and verifies them. SBOM, provenance attestations, and signing are not implemented yet."
    );
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() || matches!(args[0].as_str(), "--help" | "-h") {
        help();
        return;
    }
    if matches!(args[0].as_str(), "--version" | "-V") {
        println!("releaseseal 0.1.0-dev");
        return;
    }

    match args[0].as_str() {
        "create" if args.len() >= 3 => {
            let manifest = Path::new(&args[1]);
            if let Err(error) = create_manifest(manifest, &args[2..]) {
                eprintln!("releaseseal: {error}");
                process::exit(2);
            }
            println!("WROTE  {}", manifest.display());
        }
        "verify" if args.len() == 2 => match verify_manifest(Path::new(&args[1])) {
            Ok(true) => {}
            Ok(false) => process::exit(1),
            Err(error) => {
                eprintln!("releaseseal: {error}");
                process::exit(2);
            }
        },
        _ => {
            eprintln!("releaseseal: invalid arguments; use --help");
            process::exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{hash_reader, parse_manifest};
    use std::io::Cursor;

    #[test]
    fn sha256_matches_known_abc_vector() {
        let digest = hash_reader(Cursor::new(b"abc")).expect("hash succeeds");
        assert_eq!(
            digest,
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn parses_standard_manifest_line() {
        let line = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad  app.tar.gz\n";
        let entries = parse_manifest(line).expect("valid manifest");
        assert_eq!(entries[0].1, "app.tar.gz");
    }

    #[test]
    fn rejects_short_digest() {
        assert!(parse_manifest("abc  app.tar.gz\n").is_err());
    }
}
