//! Materialize a product-owned checkpoint inside a fresh executor workspace.
use anyhow::{bail, ensure, Result};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, io, os::unix::fs::PermissionsExt, path::Path};
use tokio::{
    fs,
    io::{AsyncRead, AsyncReadExt, AsyncWriteExt},
};
use tokio_util::io::StreamReader;

const MAX_MANIFEST_BYTES: usize = 32 * 1024 * 1024;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Manifest {
    files: BTreeMap<String, Entry>,
}

#[derive(Deserialize)]
#[serde(untagged, deny_unknown_fields)]
enum Entry {
    File {
        sha256: String,
        size: u64,
        mode: u32,
    },
    Link {
        link: String,
    },
}

pub async fn restore(response: reqwest::Response, workspace: &Path) -> Result<()> {
    use futures_util::TryStreamExt;
    let mut source = StreamReader::new(response.bytes_stream().map_err(io::Error::other));
    materialize(&mut source, workspace).await
}

async fn materialize(source: &mut (impl AsyncRead + Unpin), workspace: &Path) -> Result<()> {
    ensure!(
        fs::symlink_metadata(workspace).await?.is_dir()
            && fs::read_dir(workspace).await?.next_entry().await?.is_none(),
        "product restore requires a fresh empty workspace"
    );
    let length = source.read_u32().await? as usize;
    ensure!(
        length <= MAX_MANIFEST_BYTES,
        "workspace manifest is too large"
    );
    let mut header = vec![0; length];
    source.read_exact(&mut header).await?;
    let manifest: Manifest = serde_json::from_slice(&header)?;
    validate(&manifest.files)?;
    for (name, entry) in &manifest.files {
        let path = workspace.join(name);
        fs::create_dir_all(path.parent().unwrap_or(workspace)).await?;
        if let Entry::File { sha256, size, mode } = entry {
            restore_file(source, &path, sha256, *size, *mode).await?;
        }
    }
    ensure!(
        source.read(&mut [0]).await? == 0,
        "workspace has trailing bytes"
    );
    for (name, entry) in manifest.files {
        if let Entry::Link { link } = entry {
            fs::symlink(link, workspace.join(name)).await?;
        }
    }
    Ok(())
}

fn validate(files: &BTreeMap<String, Entry>) -> Result<()> {
    for (name, entry) in files {
        let path = Path::new(name);
        ensure!(
            !name.contains('\\')
                && !name.chars().any(|c| c < ' ')
                && name.split('/').all(|p| !matches!(p, "" | "." | "..")),
            "invalid workspace path"
        );
        ensure!(
            !path
                .ancestors()
                .skip(1)
                .any(|p| p.to_str().is_some_and(|s| files.contains_key(s))),
            "workspace file and directory paths conflict"
        );
        if let Entry::Link { link } = entry {
            validate_link(name, link, files)?;
        }
    }
    Ok(())
}

fn validate_link(name: &str, target: &str, files: &BTreeMap<String, Entry>) -> Result<()> {
    ensure!(
        !target.is_empty() && !target.starts_with('/') && !target.chars().any(|c| c < ' '),
        "invalid workspace link"
    );
    let mut path: Vec<_> = name.split('/').collect();
    path.pop();
    let mut components = target.split('/').peekable();
    while let Some(part) = components.next() {
        match part {
            "" | "." => {}
            ".." if path.pop().is_some() => {}
            ".." => bail!("workspace link escapes"),
            _ => {
                path.push(part);
                if components.peek().is_some()
                    && matches!(files.get(&path.join("/")), Some(Entry::Link { .. }))
                {
                    bail!("workspace link traverses another link");
                }
            }
        }
    }
    Ok(())
}

async fn restore_file(
    source: &mut (impl AsyncRead + Unpin),
    path: &Path,
    expected: &str,
    mut size: u64,
    mode: u32,
) -> Result<()> {
    ensure!(mode <= 0o777, "invalid workspace file mode");
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .await?;
    let mut digest = Sha256::new();
    let mut buffer = vec![0; 64 * 1024];
    while size > 0 {
        let length = size.min(buffer.len() as u64) as usize;
        source.read_exact(&mut buffer[..length]).await?;
        digest.update(&buffer[..length]);
        file.write_all(&buffer[..length]).await?;
        size -= length as u64;
    }
    ensure!(
        format!("{:x}", digest.finalize()) == expected,
        "workspace checksum mismatch"
    );
    file.flush().await?;
    file.set_permissions(std::fs::Permissions::from_mode(mode))
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn stream(manifest: serde_json::Value, data: &[u8]) -> Vec<u8> {
        let header = manifest.to_string();
        [
            (header.len() as u32).to_be_bytes().as_slice(),
            header.as_bytes(),
            data,
        ]
        .concat()
    }

    #[tokio::test]
    async fn restores_the_registered_files_modes_and_links() {
        let bytes = b"content";
        let manifest = json!({"files": {
            "nested/tool": {"sha256": format!("{:x}", Sha256::digest(bytes)), "size": 7, "mode": 0o751},
            "link": {"link": "nested/tool"},
        }});
        let root = tempfile::tempdir().unwrap();
        materialize(&mut stream(manifest, bytes).as_slice(), root.path())
            .await
            .unwrap();
        assert_eq!(std::fs::read(root.path().join("link")).unwrap(), bytes);
        assert_eq!(
            std::fs::metadata(root.path().join("nested/tool"))
                .unwrap()
                .permissions()
                .mode()
                & 0o777,
            0o751
        );
    }

    #[tokio::test]
    async fn rejects_escape_and_truncated_file() {
        let root = tempfile::tempdir().unwrap();
        let escaped = json!({"files": {"link": {"link": "../outside"}}});
        assert!(
            materialize(&mut stream(escaped, b"").as_slice(), root.path())
                .await
                .is_err()
        );
        let truncated =
            json!({"files": {"file": {"sha256": "a".repeat(64), "size": 8, "mode": 0o644}}});
        assert!(
            materialize(&mut stream(truncated, b"short").as_slice(), root.path())
                .await
                .is_err()
        );
    }
}
