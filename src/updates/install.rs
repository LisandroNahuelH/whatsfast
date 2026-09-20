use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use anyhow::{Context, Result, bail, ensure};

use crate::i18n::{self, Key};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const LIMIT: u64 = 2 * 1024 * 1024 * 1024;
#[cfg(not(target_os = "macos"))]
const MARKER: &str = "whatsfast-portable-v1";
const PENDING_DIR: &str = ".whatsfast-pending";
const PREPARED_FILE: &str = "prepared.json";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Kind {
    Portable,
    WindowsInstaller,
    #[cfg(target_os = "macos")]
    MacBundle,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Installation {
    pub executable: PathBuf,
    pub kind: Kind,
}

impl Installation {
    fn root(&self) -> Result<&Path> {
        #[cfg(target_os = "macos")]
        if self.kind == Kind::MacBundle {
            return super::macos::bundle_root(&self.executable);
        }
        Ok(&self.executable)
    }
}

pub fn detect() -> Result<Installation> {
    detect_at(&std::env::current_exe()?.canonicalize()?)
}

pub fn detect_at(executable: &Path) -> Result<Installation> {
    let path = executable.to_string_lossy().replace('\\', "/");
    let lower = path.to_lowercase();
    if std::env::var_os("FLATPAK_ID").is_some() || path.starts_with("/app/") {
        bail!(i18n::t(Key::UpdFlatpak));
    }
    if std::env::var_os("SNAP").is_some() || path.starts_with("/snap/") {
        bail!(i18n::t(Key::UpdSnap));
    }
    if lower.contains("/.cargo/") {
        bail!(i18n::t(Key::UpdCargo));
    }
    if path.starts_with("/nix/") || lower.contains("/cellar/") || lower.contains("/caskroom/") {
        bail!(i18n::t(Key::UpdNix));
    }
    #[cfg(target_os = "linux")]
    {
        for (program, arguments, instruction) in [
            ("dpkg-query", vec!["-S"], "apt"),
            ("rpm", vec!["-qf"], "dnf"),
            ("pacman", vec!["-Qo"], "pacman"),
        ] {
            if Command::new(program)
                .args(arguments)
                .arg(executable)
                .output()
                .is_ok_and(|output| output.status.success())
            {
                bail!(i18n::f(Key::UpdThrough, &[("instruction", instruction)]));
            }
        }
        if path.starts_with("/usr/") || path.starts_with("/bin/") || path.starts_with("/sbin/") {
            bail!(i18n::t(Key::UpdSystemDir));
        }
    }
    #[cfg(not(target_os = "macos"))]
    let directory = executable.parent().context(i18n::t(Key::UpdNoInstallDir))?;
    #[cfg(windows)]
    {
        let installed = std::env::var_os("LOCALAPPDATA")
            .map(PathBuf::from)
            .map(|base| base.join("Programs/WhatsFast/whatsfast.exe"));
        if fs::read_to_string(directory.join("whatsfast-installer.txt"))
            .is_ok_and(|value| value.trim() == "whatsfast-installer-v1")
            || (installed
                .and_then(|path| path.canonicalize().ok())
                .as_deref()
                == Some(executable)
                && directory.join("unins000.exe").is_file())
        {
            return Ok(Installation {
                executable: executable.to_owned(),
                kind: Kind::WindowsInstaller,
            });
        }
    }
    #[cfg(target_os = "macos")]
    {
        super::macos::detect(executable)?;
        Ok(Installation {
            executable: executable.to_owned(),
            kind: Kind::MacBundle,
        })
    }
    #[cfg(not(target_os = "macos"))]
    {
        ensure!(
            fs::read_to_string(directory.join("whatsfast-portable.txt"))
                .is_ok_and(|value| value.trim() == MARKER),
            i18n::t(Key::UpdNotPortable)
        );
        Ok(Installation {
            executable: executable.to_owned(),
            kind: Kind::Portable,
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Prepared {
    pub installation: Installation,
    pub directory: PathBuf,
    pub payload: PathBuf,
    pub sha256: String,
    pub version: String,
}

#[derive(Serialize, Deserialize)]
struct Handoff {
    prepared: Prepared,
    parent: u32,
    arguments: Vec<String>,
}

pub fn hash(path: &Path) -> Result<String> {
    let mut file = File::open(path)?;
    let mut hash = Sha256::new();
    let mut buffer = [0; 64 * 1024];
    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hash.update(&buffer[..count]);
    }
    Ok(super::hex(&hash.finalize()))
}

fn pending_directory(installation: &Installation) -> Result<PathBuf> {
    Ok(installation
        .root()?
        .parent()
        .context(i18n::t(Key::UpdMissingInstallDir))?
        .join(PENDING_DIR))
}

/// Staging folder beside the install. One pending update at a time.
pub fn staging(installation: &Installation) -> Result<PathBuf> {
    let directory = pending_directory(installation)?;
    if directory.exists() {
        fs::remove_dir_all(&directory).context(i18n::t(Key::UpdCannotReplaceStaged))?;
    }
    fs::create_dir(&directory).context(i18n::t(Key::UpdCannotWriteInstallDir))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&directory, fs::Permissions::from_mode(0o700))?;
    }
    Ok(directory)
}

pub fn save_prepared(prepared: &Prepared) -> Result<()> {
    let path = prepared.directory.join(PREPARED_FILE);
    let mut file = File::create(&path)?;
    serde_json::to_writer(&mut file, prepared)?;
    file.flush()?;
    file.sync_all()?;
    Ok(())
}

/// A verified payload that is still newer than this build, or `None`.
pub fn load_pending(installation: &Installation) -> Result<Option<Prepared>> {
    let directory = pending_directory(installation)?;
    let path = directory.join(PREPARED_FILE);
    if !path.is_file() {
        return Ok(None);
    }
    let mut prepared: Prepared = serde_json::from_reader(File::open(&path)?)?;
    let usable = prepared.payload.is_file()
        && hash(&prepared.payload)? == prepared.sha256
        && super::is_newer(&prepared.version, env!("CARGO_PKG_VERSION"));
    if !usable {
        let _ = clear_pending(installation);
        return Ok(None);
    }
    prepared.installation = installation.clone();
    Ok(Some(prepared))
}

pub fn clear_pending(installation: &Installation) -> Result<()> {
    let directory = pending_directory(installation)?;
    if directory.exists() {
        fs::remove_dir_all(directory)?;
    }
    Ok(())
}

pub fn extract(archive: &Path, entry: &str, destination: &Path) -> Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(destination)?;
    let mut command = Command::new("tar");
    command
        .arg("-xOf")
        .arg(archive)
        .arg(entry)
        .stdout(Stdio::piped())
        .stderr(Stdio::null());
    hidden(&mut command);
    let mut child = command.spawn().context(i18n::t(Key::UpdCannotRunTar))?;
    let result = (|| -> Result<()> {
        let mut stdout = child
            .stdout
            .take()
            .context(i18n::t(Key::UpdMissingArchiveStream))?
            .take(LIMIT + 1);
        let mut file = file;
        let count = std::io::copy(&mut stdout, &mut file)?;
        ensure!(count > 0 && count <= LIMIT, i18n::t(Key::UpdExeInvalidSize));
        ensure!(child.wait()?.success(), i18n::t(Key::UpdCannotUnpackExe));
        file.sync_all()?;
        Ok(())
    })();
    if result.is_err() {
        let _ = child.kill();
        let _ = child.wait();
    }
    result?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(destination, fs::Permissions::from_mode(0o755))?;
    }
    Ok(())
}

pub fn hidden(command: &mut Command) {
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x08000000);
    }
    #[cfg(not(windows))]
    let _ = command;
}

pub fn verify_version(executable: &Path, expected: &str) -> Result<()> {
    let mut command = Command::new(executable);
    command
        .arg("--version")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null());
    hidden(&mut command);
    let mut child = command
        .spawn()
        .context(i18n::t(Key::UpdDownloadCannotRun))?;
    let start = Instant::now();
    loop {
        if let Some(status) = child.try_wait()? {
            ensure!(status.success(), i18n::t(Key::UpdStartupCheckFailed));
            let mut version = String::new();
            child
                .stdout
                .take()
                .context(i18n::t(Key::UpdMissingVersionOutput))?
                .take(4096)
                .read_to_string(&mut version)?;
            ensure!(
                version.trim() == format!("whatsfast {expected}"),
                i18n::t(Key::UpdWrongVersion)
            );
            return Ok(());
        }
        if start.elapsed() >= Duration::from_secs(10) {
            let _ = child.kill();
            let _ = child.wait();
            bail!(i18n::t(Key::UpdStartupCheckNoAnswer));
        }
        std::thread::sleep(Duration::from_millis(50));
    }
}

pub fn handoff(prepared: &Prepared, arguments: Vec<String>) -> Result<()> {
    ensure!(
        hash(&prepared.payload)? == prepared.sha256,
        i18n::t(Key::UpdStagedChanged)
    );
    let helper = prepared.directory.join(if cfg!(windows) {
        "helper.exe"
    } else {
        "helper"
    });
    fs::copy(std::env::current_exe()?, &helper)?;
    let job = prepared.directory.join("handoff.json");
    let mut file = File::create(&job)?;
    serde_json::to_writer(
        &mut file,
        &Handoff {
            prepared: prepared.clone(),
            parent: std::process::id(),
            arguments,
        },
    )?;
    file.flush()?;
    file.sync_all()?;
    let mut command = Command::new(helper);
    command
        .arg("--apply-update")
        .arg(&job)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    hidden(&mut command);
    let mut child = command
        .spawn()
        .context(i18n::t(Key::UpdCannotStartHelper))?;
    let ready = prepared.directory.join("ready");
    let start = Instant::now();
    while start.elapsed() < Duration::from_secs(10) {
        if ready.exists() {
            return Ok(());
        }
        ensure!(child.try_wait()?.is_none(), i18n::t(Key::UpdHelperExited));
        std::thread::sleep(Duration::from_millis(50));
    }
    let _ = child.kill();
    let _ = child.wait();
    bail!(i18n::t(Key::UpdHelperNoStart))
}

fn wait_for_parent(parent: u32, ready: &Path) -> Result<()> {
    #[cfg(windows)]
    {
        use windows_sys::Win32::Foundation::{CloseHandle, WAIT_OBJECT_0};
        use windows_sys::Win32::System::Threading::{
            OpenProcess, PROCESS_SYNCHRONIZE, WaitForSingleObject,
        };
        let process = unsafe { OpenProcess(PROCESS_SYNCHRONIZE, 0, parent) };
        ensure!(!process.is_null(), i18n::t(Key::UpdCannotWatchApp));
        let result = fs::write(ready, b"ready");
        if result.is_err() {
            unsafe {
                CloseHandle(process);
            }
        }
        result?;
        let outcome = unsafe { WaitForSingleObject(process, 60_000) };
        unsafe {
            CloseHandle(process);
        }
        ensure!(outcome == WAIT_OBJECT_0, i18n::t(Key::UpdAppDidNotClose));
    }
    #[cfg(target_os = "linux")]
    {
        let process = PathBuf::from(format!("/proc/{parent}/stat"));
        let original = fs::read_to_string(&process).context(i18n::t(Key::UpdCannotWatchApp))?;
        let identity = process_identity(&original).context(i18n::t(Key::UpdCannotIdentifyApp))?;
        fs::write(ready, b"ready")?;
        let start = Instant::now();
        loop {
            let current = match fs::read_to_string(&process) {
                Ok(current) => current,
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => break,
                Err(error) => return Err(error).context(i18n::t(Key::UpdCannotWatchApp)),
            };
            if process_identity(&current) != Some(identity)
                || current
                    .rsplit_once(')')
                    .is_some_and(|(_, fields)| fields.trim_start().starts_with('Z'))
            {
                break;
            }
            ensure!(
                start.elapsed() < Duration::from_secs(60),
                i18n::t(Key::UpdAppDidNotClose)
            );
            std::thread::sleep(Duration::from_millis(100));
        }
    }
    #[cfg(not(any(windows, target_os = "linux")))]
    {
        fs::write(ready, b"ready")?;
        let start = Instant::now();
        while Command::new("/bin/kill")
            .args(["-0", &parent.to_string()])
            .stderr(Stdio::null())
            .status()?
            .success()
        {
            ensure!(
                start.elapsed() < Duration::from_secs(60),
                i18n::t(Key::UpdAppDidNotClose)
            );
            std::thread::sleep(Duration::from_millis(100));
        }
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn process_identity(stat: &str) -> Option<&str> {
    stat.rsplit_once(')')?.1.split_whitespace().nth(19)
}

pub fn replace(prepared: &Prepared) -> Result<()> {
    ensure!(
        hash(&prepared.payload)? == prepared.sha256,
        i18n::t(Key::UpdStagedChecksumChanged)
    );
    let target = &prepared.installation.executable;
    let backup = prepared.directory.join("previous");
    match prepared.installation.kind {
        #[cfg(target_os = "macos")]
        Kind::MacBundle => super::macos::replace(prepared)?,
        Kind::Portable => {
            ensure!(!backup.exists(), i18n::t(Key::UpdAlreadyApplied));
            backup_current(target, &backup).context(i18n::t(Key::UpdCannotBackUp))?;
            #[cfg(windows)]
            fs::remove_file(target).context(i18n::t(Key::UpdAppStillRunning))?;
            if let Err(error) = fs::rename(&prepared.payload, target) {
                #[cfg(windows)]
                fs::copy(&backup, target).context(i18n::t(Key::UpdCannotRestore))?;
                return Err(error).context(i18n::t(Key::UpdCannotReplaceApp));
            }
        }
        Kind::WindowsInstaller => {
            backup_current(target, &backup).context(i18n::t(Key::UpdCannotBackUp))?;
            let mut command = Command::new(&prepared.payload);
            command
                .args([
                    "/VERYSILENT",
                    "/SUPPRESSMSGBOXES",
                    "/NORESTART",
                    "/CLOSEAPPLICATIONS",
                    "/NORESTARTAPPLICATIONS",
                ])
                .arg(format!(
                    "/DIR={}",
                    installer_path(
                        target
                            .parent()
                            .context(i18n::t(Key::UpdMissingInstallDir))?
                    )
                ))
                .arg(format!(
                    "/LOG={}",
                    installer_path(&prepared.directory.join("installer.log"))
                ));
            hidden(&mut command);
            ensure!(
                command.status()?.success(),
                i18n::t(Key::UpdInstallerFailed)
            );
        }
    }
    Ok(())
}

fn backup_current(target: &Path, backup: &Path) -> Result<()> {
    let mut source = File::open(target)?;
    let permissions = source.metadata()?.permissions();
    write_backup(&mut source, backup, permissions)
}

/// Rollback recognizes only `previous`. Publish that name after the complete
/// copy has been synced, so a failed copy cannot replace a working executable
/// with the partial backup it left behind.
fn write_backup(source: &mut impl Read, backup: &Path, permissions: fs::Permissions) -> Result<()> {
    ensure!(
        fs::symlink_metadata(backup)
            .is_err_and(|error| error.kind() == std::io::ErrorKind::NotFound),
        i18n::t(Key::UpdHasBackup)
    );
    let partial = backup.with_extension("partial");
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&partial)?;
    let result = (|| -> Result<()> {
        std::io::copy(source, &mut output)?;
        output.sync_all()?;
        fs::set_permissions(&partial, permissions)?;
        Ok(())
    })();
    drop(output);
    if let Err(error) = result {
        let _ = fs::remove_file(&partial);
        return Err(error);
    }
    if let Err(error) = fs::rename(&partial, backup) {
        let _ = fs::remove_file(&partial);
        return Err(error.into());
    }
    Ok(())
}

fn installer_path(path: &Path) -> String {
    let path = path.to_string_lossy();
    if let Some(unc) = path.strip_prefix(r"\\?\UNC\") {
        format!(r"\\{unc}")
    } else {
        path.strip_prefix(r"\\?\").unwrap_or(&path).to_owned()
    }
}

pub fn run_helper(job: &Path) -> Result<()> {
    let handoff: Handoff = serde_json::from_reader(File::open(job)?)?;
    let prepared = &handoff.prepared;
    ensure!(
        job.parent() == Some(prepared.directory.as_path()),
        i18n::t(Key::UpdInvalidJobDir)
    );
    ensure!(
        prepared.payload.parent() == Some(prepared.directory.as_path()),
        i18n::t(Key::UpdInvalidPayload)
    );
    ensure!(
        prepared.directory.parent() == prepared.installation.root()?.parent(),
        i18n::t(Key::UpdInvalidInstallDir)
    );
    ensure!(
        hash(&prepared.payload)? == prepared.sha256,
        i18n::t(Key::UpdStagedChecksumChanged)
    );
    wait_for_parent(handoff.parent, &prepared.directory.join("ready"))?;
    let result = replace(prepared);
    if let Err(error) = result {
        restore_and_restart(prepared, &handoff.arguments)?;
        fs::write(
            prepared.directory.join("result.txt"),
            i18n::f(Key::UpdFailed, &[("error", &format!("{error:#}"))]),
        )?;
        return Err(error);
    }
    let mut command = Command::new(&prepared.installation.executable);
    command
        .args(&handoff.arguments)
        .arg("--update-receipt")
        .arg(job);
    hidden(&mut command);
    let launch = (|| -> Result<()> {
        let mut child = command
            .spawn()
            .context(i18n::t(Key::UpdCannotLaunchUpdated))?;
        let start = Instant::now();
        loop {
            if prepared.directory.join("started").is_file() {
                return Ok(());
            }
            ensure!(
                child.try_wait()?.is_none(),
                i18n::t(Key::UpdUpdatedAppExited)
            );
            if start.elapsed() >= Duration::from_secs(60) {
                let _ = child.kill();
                let _ = child.wait();
                bail!(i18n::t(Key::UpdUpdatedAppNoWindow));
            }
            std::thread::sleep(Duration::from_millis(100));
        }
    })();
    if let Err(error) = launch {
        restore_and_restart(prepared, &handoff.arguments)?;
        fs::write(
            prepared.directory.join("result.txt"),
            i18n::f(Key::UpdFailedRestored, &[("error", &format!("{error:#}"))]),
        )?;
        return Err(error);
    }
    fs::write(
        prepared.directory.join("result.txt"),
        i18n::f(Key::UpdUpdatedTo, &[("version", &prepared.version)]),
    )?;
    Ok(())
}

fn restore_and_restart(prepared: &Prepared, arguments: &[String]) -> Result<()> {
    #[cfg(target_os = "macos")]
    if prepared.installation.kind == Kind::MacBundle {
        super::macos::restore(prepared)?;
    }
    let backup = prepared.directory.join("previous");
    if backup.is_file() {
        fs::copy(&backup, &prepared.installation.executable)
            .context(i18n::t(Key::UpdCannotRestore))?;
    }
    let mut command = Command::new(&prepared.installation.executable);
    command
        .args(arguments)
        .args(["--update-error", i18n::t(Key::UpdCouldNotStartRestored)]);
    hidden(&mut command);
    command
        .spawn()
        .context(i18n::t(Key::UpdCannotRestartPrevious))?;
    Ok(())
}

pub fn acknowledge(job: &Path) -> Result<()> {
    let handoff: Handoff = serde_json::from_reader(File::open(job)?)?;
    ensure!(
        job.parent() == Some(handoff.prepared.directory.as_path()),
        i18n::t(Key::UpdInvalidReceiptDir)
    );
    ensure!(
        std::env::current_exe()?.canonicalize()?
            == handoff.prepared.installation.executable.canonicalize()?,
        i18n::t(Key::UpdReceiptOtherInstall)
    );
    ensure!(
        env!("CARGO_PKG_VERSION") == handoff.prepared.version,
        i18n::t(Key::UpdUpdatedWrongVersion)
    );
    fs::write(
        handoff.prepared.directory.join("started"),
        env!("CARGO_PKG_VERSION"),
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "linux")]
    #[test]
    fn helper_restarts_a_verified_update_and_rolls_back_a_failed_start() {
        use std::os::unix::fs::PermissionsExt;
        for starts in [true, false] {
            let directory = tempfile::tempdir().unwrap();
            let target = directory.path().join("whatsfast");
            let original =
                b"#!/bin/sh\nprintf '%s\\n' \"$@\" > \"$(dirname \"$0\")/restart-arguments\"\n";
            fs::write(&target, original).unwrap();
            fs::set_permissions(&target, fs::Permissions::from_mode(0o755)).unwrap();
            let installation = Installation {
                executable: target.clone(),
                kind: Kind::Portable,
            };
            let stage = staging(&installation).unwrap();
            let payload = stage.join("next");
            let incoming: &[u8] = if starts {
                b"#!/bin/sh\nprintf started > \"$(dirname \"$2\")/started\"\n"
            } else {
                b"#!/bin/sh\nexit 1\n"
            };
            fs::write(&payload, incoming).unwrap();
            fs::set_permissions(&payload, fs::Permissions::from_mode(0o755)).unwrap();
            let prepared = Prepared {
                installation,
                directory: stage.clone(),
                payload: payload.clone(),
                sha256: hash(&payload).unwrap(),
                version: "99.0.0".into(),
            };
            // Simulate a parent which exits after the helper starts watching it.
            let mut parent = Command::new("/bin/sleep").arg("0.2").spawn().unwrap();
            let job = stage.join("handoff.json");
            serde_json::to_writer(
                File::create(&job).unwrap(),
                &Handoff {
                    prepared,
                    parent: parent.id(),
                    arguments: Vec::new(),
                },
            )
            .unwrap();
            let result = run_helper(&job);
            parent.wait().unwrap();
            assert!(stage.join("ready").is_file());
            assert_eq!(fs::read(stage.join("previous")).unwrap(), original);
            if starts {
                result.unwrap();
                assert_eq!(fs::read(&target).unwrap(), incoming);
                assert!(stage.join("started").is_file());
            } else {
                assert!(result.is_err());
                assert_eq!(fs::read(&target).unwrap(), original);
                let arguments = directory.path().join("restart-arguments");
                let deadline = Instant::now() + Duration::from_secs(3);
                while !fs::read_to_string(&arguments)
                    .is_ok_and(|text| text.contains("--update-error"))
                {
                    assert!(
                        Instant::now() < deadline,
                        "rollback did not restart the original app"
                    );
                    std::thread::sleep(Duration::from_millis(5));
                }
                assert!(
                    fs::read_to_string(arguments)
                        .unwrap()
                        .contains("--update-error")
                );
            }
        }
    }

    #[test]
    fn an_interrupted_backup_is_never_available_to_rollback() {
        struct InterruptedCopy(bool);
        impl Read for InterruptedCopy {
            fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
                if self.0 {
                    return Err(std::io::Error::other("injected copy failure"));
                }
                self.0 = true;
                buffer[0] = b'p';
                Ok(1)
            }
        }
        let directory =
            std::env::temp_dir().join(format!("whatsfast-backup-test-{}", rand::random::<u64>()));
        fs::create_dir(&directory).unwrap();
        let target = directory.join("whatsfast");
        fs::write(&target, b"working executable").unwrap();
        let backup = directory.join("previous");
        let permissions = fs::metadata(&target).unwrap().permissions();
        assert!(write_backup(&mut InterruptedCopy(false), &backup, permissions).is_err());
        assert!(
            !backup.exists(),
            "rollback must not see the incomplete copy"
        );
        assert!(!backup.with_extension("partial").exists());
        assert_eq!(fs::read(&target).unwrap(), b"working executable");
        backup_current(&target, &backup).unwrap();
        assert_eq!(fs::read(&backup).unwrap(), b"working executable");
        fs::write(&target, b"new executable").unwrap();
        assert!(backup_current(&target, &backup).is_err());
        assert_eq!(
            fs::read(&backup).unwrap(),
            b"working executable",
            "a retry keeps the known backup"
        );
        fs::remove_dir_all(directory).unwrap();
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn process_identity_handles_spaces_and_parentheses_in_names() {
        let fields = "S 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 98765 20";
        assert_eq!(
            process_identity(&format!("42 (app (test)) {fields}")),
            Some("98765")
        );
        assert_eq!(process_identity("42 (app) S"), None);
        let current = fs::read_to_string(format!("/proc/{}/stat", std::process::id())).unwrap();
        assert!(process_identity(&current).unwrap().parse::<u64>().is_ok());
    }

    #[test]
    fn unknown_and_package_managed_paths_are_not_portable() {
        for path in [
            "/usr/bin/whatsfast",
            "/nix/store/package/bin/whatsfast",
            "/home/test/.cargo/bin/whatsfast",
            "/unknown/whatsfast",
        ] {
            assert!(detect_at(Path::new(path)).is_err());
        }
    }

    #[test]
    fn installer_arguments_use_paths_inno_setup_accepts() {
        assert_eq!(
            installer_path(Path::new(r"\\?\C:\Users\test\WhatsFast")),
            r"C:\Users\test\WhatsFast"
        );
        assert_eq!(
            installer_path(Path::new(r"\\?\UNC\server\share\WhatsFast")),
            r"\\server\share\WhatsFast"
        );
    }

    #[test]
    fn replacement_verifies_before_touching_the_current_executable() {
        let directory =
            std::env::temp_dir().join(format!("whatsfast-updater-test-{}", rand::random::<u64>()));
        fs::create_dir(&directory).unwrap();
        let target = directory.join("whatsfast");
        fs::write(&target, b"old").unwrap();
        let installation = Installation {
            executable: target.clone(),
            kind: Kind::Portable,
        };
        let stage = staging(&installation).unwrap();
        let payload = stage.join("next");
        fs::write(&payload, b"new").unwrap();
        let mut prepared = Prepared {
            installation,
            directory: stage.clone(),
            payload: payload.clone(),
            sha256: "wrong".into(),
            version: "1.0.0".into(),
        };
        assert!(replace(&prepared).is_err());
        assert_eq!(fs::read(&target).unwrap(), b"old");
        prepared.sha256 = hash(&payload).unwrap();
        replace(&prepared).unwrap();
        assert_eq!(fs::read(&target).unwrap(), b"new");
        assert_eq!(fs::read(stage.join("previous")).unwrap(), b"old");
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn a_pending_payload_survives_reload_until_this_build_is_newer() {
        let directory =
            std::env::temp_dir().join(format!("whatsfast-pending-test-{}", rand::random::<u64>()));
        fs::create_dir(&directory).unwrap();
        let target = directory.join("whatsfast");
        fs::write(&target, b"old").unwrap();
        let installation = Installation {
            executable: target,
            kind: Kind::Portable,
        };
        let stage = staging(&installation).unwrap();
        let payload = stage.join("next");
        fs::write(&payload, b"new").unwrap();
        let sha256 = hash(&payload).unwrap();
        let prepared = Prepared {
            installation: installation.clone(),
            directory: stage,
            payload,
            sha256: sha256.clone(),
            version: "99.0.0".into(),
        };
        save_prepared(&prepared).unwrap();
        let loaded = load_pending(&installation).unwrap().expect("pending");
        assert_eq!(loaded.version, "99.0.0");
        assert_eq!(loaded.sha256, sha256);
        let mut current = prepared.clone();
        current.version = env!("CARGO_PKG_VERSION").into();
        save_prepared(&current).unwrap();
        assert!(load_pending(&installation).unwrap().is_none());
        fs::remove_dir_all(directory).unwrap();
    }
}
