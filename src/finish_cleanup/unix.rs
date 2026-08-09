use super::{CleanupStep, FileIdentity};
use anyhow::{bail, Context, Result};
use std::ffi::{CStr, CString, OsStr, OsString};
use std::fs::{File, OpenOptions};
use std::io;
use std::mem::MaybeUninit;
use std::os::fd::{AsRawFd, FromRawFd};
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

pub(super) fn open_directory(path: &Path) -> io::Result<File> {
    OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC)
        .open(path)
}

pub(super) fn open_directory_at(parent: &File, name: &OsStr) -> io::Result<File> {
    let name = c_string(name)?;
    // SAFETY: `name` is NUL-terminated, `parent` is an open directory, and a
    // successful descriptor is immediately owned by `File`.
    let descriptor = unsafe {
        libc::openat(
            parent.as_raw_fd(),
            name.as_ptr(),
            libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC,
        )
    };
    if descriptor < 0 {
        Err(io::Error::last_os_error())
    } else {
        // SAFETY: `openat` returned a new owned descriptor.
        Ok(unsafe { File::from_raw_fd(descriptor) })
    }
}

pub(super) fn open_file_at(parent: &File, name: &OsStr) -> io::Result<File> {
    let name = c_string(name)?;
    // SAFETY: `name` is NUL-terminated, `parent` is an open directory, and a
    // successful descriptor is immediately owned by `File`.
    let descriptor = unsafe {
        libc::openat(
            parent.as_raw_fd(),
            name.as_ptr(),
            libc::O_RDONLY | libc::O_NOFOLLOW | libc::O_CLOEXEC,
        )
    };
    if descriptor < 0 {
        Err(io::Error::last_os_error())
    } else {
        // SAFETY: `openat` returned a new owned descriptor.
        Ok(unsafe { File::from_raw_fd(descriptor) })
    }
}

pub(super) fn create_new_file_at(parent: &File, name: &OsStr) -> io::Result<File> {
    let name = c_string(name)?;
    // SAFETY: `name` is NUL-terminated, `parent` is an open directory, and a
    // successful descriptor is immediately owned by `File`.
    let descriptor = unsafe {
        libc::openat(
            parent.as_raw_fd(),
            name.as_ptr(),
            libc::O_WRONLY | libc::O_CREAT | libc::O_EXCL | libc::O_NOFOLLOW | libc::O_CLOEXEC,
            0o600 as libc::c_uint,
        )
    };
    if descriptor < 0 {
        Err(io::Error::last_os_error())
    } else {
        // SAFETY: `openat` returned a new owned descriptor.
        Ok(unsafe { File::from_raw_fd(descriptor) })
    }
}

pub(super) fn remove_directory_contents(
    directory: &File,
    checkpoint: &mut impl FnMut(CleanupStep) -> io::Result<()>,
) -> Result<()> {
    for name in directory_names(directory)? {
        checkpoint(CleanupStep::BeforeEntry(name.clone()))
            .with_context(|| format!("cleanup checkpoint before removing {:?}", name))?;
        let metadata = metadata_at(directory, &name)?;
        if metadata.st_mode & libc::S_IFMT == libc::S_IFDIR {
            let child = open_directory_at(directory, &name)
                .with_context(|| format!("opening cleanup directory entry {:?}", name))?;
            let child_identity = FileIdentity::from_file(&child)?;
            if child_identity != FileIdentity::from_stat(&metadata) {
                bail!(
                    "cleanup directory entry identity changed before traversal: {:?}",
                    name
                );
            }
            remove_directory_contents(&child, checkpoint)?;
            validate_entry_identity(directory, &name, child_identity)?;
            unlink_at(directory, &name, libc::AT_REMOVEDIR)
                .with_context(|| format!("removing cleanup directory entry {:?}", name))?;
        } else {
            let expected_identity = FileIdentity::from_stat(&metadata);
            checkpoint(CleanupStep::BeforeNonDirectoryUnlink(name.clone()))
                .with_context(|| format!("cleanup checkpoint before unlinking entry {:?}", name))?;
            validate_entry_identity(directory, &name, expected_identity)?;
            unlink_at(directory, &name, 0)
                .with_context(|| format!("unlinking cleanup entry {:?}", name))?;
        }
    }
    Ok(())
}

pub(super) fn directory_names(directory: &File) -> io::Result<Vec<OsString>> {
    // SAFETY: `fcntl` duplicates a valid descriptor and transfers ownership of
    // that duplicate to `fdopendir` below.
    let duplicate = unsafe { libc::fcntl(directory.as_raw_fd(), libc::F_DUPFD_CLOEXEC, 0) };
    if duplicate < 0 {
        return Err(io::Error::last_os_error());
    }
    // SAFETY: `duplicate` is an owned directory descriptor.
    let stream = unsafe { libc::fdopendir(duplicate) };
    if stream.is_null() {
        // SAFETY: `fdopendir` did not take ownership on failure.
        unsafe { libc::close(duplicate) };
        return Err(io::Error::last_os_error());
    }
    // SAFETY: `stream` is a live directory stream. `dup` shares the original
    // open-file description and therefore its cursor, so each listing must
    // explicitly start from the beginning.
    unsafe { libc::rewinddir(stream) };

    let result = (|| {
        let mut names = Vec::new();
        loop {
            set_errno(0);
            // SAFETY: `stream` remains live until `closedir` below.
            let entry = unsafe { libc::readdir(stream) };
            if entry.is_null() {
                let error = current_errno();
                if error == 0 {
                    break;
                }
                return Err(io::Error::from_raw_os_error(error));
            }
            // SAFETY: POSIX guarantees a NUL-terminated `d_name` for a returned
            // directory entry.
            let name = unsafe { CStr::from_ptr((*entry).d_name.as_ptr()) }.to_bytes();
            if name != b"." && name != b".." {
                names.push(OsString::from_vec(name.to_vec()));
            }
        }
        names.sort_by(|left, right| left.as_bytes().cmp(right.as_bytes()));
        Ok(names)
    })();
    // SAFETY: `stream` is live and owns `duplicate`.
    unsafe { libc::closedir(stream) };
    result
}

fn metadata_at(parent: &File, name: &OsStr) -> io::Result<libc::stat> {
    let name = c_string(name)?;
    let mut metadata = MaybeUninit::<libc::stat>::uninit();
    // SAFETY: all pointers are valid for the call and the output is initialized
    // only when `fstatat` succeeds.
    let status = unsafe {
        libc::fstatat(
            parent.as_raw_fd(),
            name.as_ptr(),
            metadata.as_mut_ptr(),
            libc::AT_SYMLINK_NOFOLLOW,
        )
    };
    if status < 0 {
        Err(io::Error::last_os_error())
    } else {
        // SAFETY: successful `fstatat` initialized the value.
        Ok(unsafe { metadata.assume_init() })
    }
}

pub(super) fn entry_exists(parent: &File, name: &OsStr) -> io::Result<bool> {
    match metadata_at(parent, name) {
        Ok(_) => Ok(true),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(error),
    }
}

pub(super) fn validate_file_name(parent: &File, name: &OsStr) -> io::Result<()> {
    set_errno(0);
    // SAFETY: `parent` is an open directory descriptor and `_PC_NAME_MAX` is a
    // read-only query for that filesystem.
    let name_max = unsafe { libc::fpathconf(parent.as_raw_fd(), libc::_PC_NAME_MAX) };
    if name_max < 0 {
        let error = current_errno();
        return if error == 0 {
            Ok(())
        } else {
            Err(io::Error::from_raw_os_error(error))
        };
    }
    let name_length = name.as_bytes().len();
    if name_length > name_max as usize {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "file name is {name_length} bytes but this filesystem permits at most {name_max}"
            ),
        ))
    } else {
        Ok(())
    }
}

pub(super) fn validate_entry_identity(
    parent: &File,
    name: &OsStr,
    expected: FileIdentity,
) -> Result<()> {
    let observed = FileIdentity::from_stat(&metadata_at(parent, name)?);
    if observed != expected {
        bail!(
            "cleanup entry identity changed for {:?}: expected device/inode {}/{}, observed {}/{}",
            name,
            expected.device,
            expected.inode,
            observed.device,
            observed.inode
        );
    }
    Ok(())
}

pub(super) fn rename_at_noreplace(
    source_parent: &File,
    source: &OsStr,
    destination_parent: &File,
    destination: &OsStr,
) -> io::Result<()> {
    let source = c_string(source)?;
    let destination = c_string(destination)?;
    #[cfg(any(target_os = "linux", target_os = "android"))]
    {
        // SAFETY: both directory descriptors and NUL-terminated names are
        // valid, and the platform flag makes replacement impossible.
        let status = unsafe {
            libc::renameat2(
                source_parent.as_raw_fd(),
                source.as_ptr(),
                destination_parent.as_raw_fd(),
                destination.as_ptr(),
                libc::RENAME_NOREPLACE as _,
            )
        };
        status_result(status)
    }

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    {
        // SAFETY: both directory descriptors and NUL-terminated names are
        // valid, and RENAME_EXCL makes replacement impossible.
        let status = unsafe {
            libc::renameatx_np(
                source_parent.as_raw_fd(),
                source.as_ptr(),
                destination_parent.as_raw_fd(),
                destination.as_ptr(),
                libc::RENAME_EXCL,
            )
        };
        status_result(status)
    }

    #[cfg(not(any(
        target_os = "linux",
        target_os = "android",
        target_os = "macos",
        target_os = "ios"
    )))]
    {
        let _ = (source_parent, source, destination_parent, destination);
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "atomic no-replace rename is unavailable on this platform",
        ))
    }
}

pub(super) fn rename_at_replace(
    source_parent: &File,
    source: &OsStr,
    destination_parent: &File,
    destination: &OsStr,
) -> io::Result<()> {
    let source = c_string(source)?;
    let destination = c_string(destination)?;
    // SAFETY: both directory descriptors and NUL-terminated names are valid.
    // POSIX rename atomically replaces a non-directory destination.
    let status = unsafe {
        libc::renameat(
            source_parent.as_raw_fd(),
            source.as_ptr(),
            destination_parent.as_raw_fd(),
            destination.as_ptr(),
        )
    };
    status_result(status)
}

fn status_result(status: libc::c_int) -> io::Result<()> {
    if status < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

pub(super) fn unlink_at(parent: &File, name: &OsStr, flags: i32) -> io::Result<()> {
    let name = c_string(name)?;
    // SAFETY: the directory descriptor and NUL-terminated name are valid.
    let status = unsafe { libc::unlinkat(parent.as_raw_fd(), name.as_ptr(), flags) };
    if status < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

fn c_string(name: &OsStr) -> io::Result<CString> {
    CString::new(name.as_bytes())
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "file name contains NUL"))
}

#[cfg(any(target_os = "linux", target_os = "android"))]
fn errno_pointer() -> *mut libc::c_int {
    // SAFETY: libc exposes the calling thread's errno pointer.
    unsafe { libc::__errno_location() }
}

#[cfg(any(target_os = "macos", target_os = "ios", target_os = "freebsd"))]
fn errno_pointer() -> *mut libc::c_int {
    // SAFETY: libc exposes the calling thread's errno pointer.
    unsafe { libc::__error() }
}

#[cfg(target_os = "openbsd")]
fn errno_pointer() -> *mut libc::c_int {
    // SAFETY: libc exposes the calling thread's errno pointer.
    unsafe { libc::__errno() }
}

fn set_errno(value: libc::c_int) {
    // SAFETY: the pointer is the calling thread's valid errno slot.
    unsafe { *errno_pointer() = value };
}

fn current_errno() -> libc::c_int {
    // SAFETY: the pointer is the calling thread's valid errno slot.
    unsafe { *errno_pointer() }
}

#[cfg(test)]
mod tests {
    #[cfg(any(target_os = "freebsd", target_os = "openbsd"))]
    use super::rename_at_noreplace;
    use super::{directory_names, open_directory};
    #[cfg(any(target_os = "freebsd", target_os = "openbsd"))]
    use std::ffi::OsStr;
    use std::ffi::OsString;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn repeated_directory_listing_starts_from_the_beginning() {
        let temporary = TempDir::new().unwrap();
        fs::write(temporary.path().join("first"), "first\n").unwrap();
        fs::write(temporary.path().join("second"), "second\n").unwrap();
        let directory = open_directory(temporary.path()).unwrap();
        let expected = vec![OsString::from("first"), OsString::from("second")];

        assert_eq!(directory_names(&directory).unwrap(), expected);
        assert_eq!(directory_names(&directory).unwrap(), expected);
    }

    #[cfg(any(target_os = "freebsd", target_os = "openbsd"))]
    #[test]
    fn unsupported_bsd_atomic_rename_returns_an_error() {
        let temporary = TempDir::new().unwrap();
        let directory = open_directory(temporary.path()).unwrap();

        let error = rename_at_noreplace(
            &directory,
            OsStr::new("source"),
            &directory,
            OsStr::new("destination"),
        )
        .unwrap_err();

        assert_eq!(error.kind(), std::io::ErrorKind::Unsupported);
    }
}
