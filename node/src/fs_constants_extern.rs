// generated by ts2rs 0.1.4
// https://ts2rs.ctaggart.com/

#[wasm_bindgen(module = "fs")]
extern "C" {
    #[doc = "Constant for fs.access(). File is visible to the calling process."]
    pub static F_OK: String;
    #[doc = "Constant for fs.access(). File can be read by the calling process."]
    pub static R_OK: String;
    #[doc = "Constant for fs.access(). File can be written by the calling process."]
    pub static W_OK: String;
    #[doc = "Constant for fs.access(). File can be executed by the calling process."]
    pub static X_OK: String;
    #[doc = "Constant for fs.copyFile. Flag indicating the destination file should not be overwritten if it already exists."]
    pub static COPYFILE_EXCL: String;
    #[doc = "Constant for fs.copyFile. copy operation will attempt to create a copy-on-write reflink."]
    #[doc = "If the underlying platform does not support copy-on-write, then a fallback copy mechanism is used."]
    pub static COPYFILE_FICLONE: String;
    #[doc = "Constant for fs.copyFile. Copy operation will attempt to create a copy-on-write reflink."]
    #[doc = "If the underlying platform does not support copy-on-write, then the operation will fail with an error."]
    pub static COPYFILE_FICLONE_FORCE: String;
    #[doc = "Constant for fs.open(). Flag indicating to open a file for read-only access."]
    pub static O_RDONLY: String;
    #[doc = "Constant for fs.open(). Flag indicating to open a file for write-only access."]
    pub static O_WRONLY: String;
    #[doc = "Constant for fs.open(). Flag indicating to open a file for read-write access."]
    pub static O_RDWR: String;
    #[doc = "Constant for fs.open(). Flag indicating to create the file if it does not already exist."]
    pub static O_CREAT: String;
    #[doc = "Constant for fs.open(). Flag indicating that opening a file should fail if the O_CREAT flag is set and the file already exists."]
    pub static O_EXCL: String;
    #[doc = "Constant for fs.open(). Flag indicating that if path identifies a terminal device,"]
    #[doc = "opening the path shall not cause that terminal to become the controlling terminal for the process"]
    #[doc = "(if the process does not already have one)."]
    pub static O_NOCTTY: String;
    #[doc = "Constant for fs.open(). Flag indicating that if the file exists and is a regular file, and the file is opened successfully for write access, its length shall be truncated to zero."]
    pub static O_TRUNC: String;
    #[doc = "Constant for fs.open(). Flag indicating that data will be appended to the end of the file."]
    pub static O_APPEND: String;
    #[doc = "Constant for fs.open(). Flag indicating that the open should fail if the path is not a directory."]
    pub static O_DIRECTORY: String;
    #[doc = "constant for fs.open()."]
    #[doc = "Flag indicating reading accesses to the file system will no longer result in"]
    #[doc = "an update to the atime information associated with the file."]
    #[doc = "This flag is available on Linux operating systems only."]
    pub static O_NOATIME: String;
    #[doc = "Constant for fs.open(). Flag indicating that the open should fail if the path is a symbolic link."]
    pub static O_NOFOLLOW: String;
    #[doc = "Constant for fs.open(). Flag indicating that the file is opened for synchronous I/O."]
    pub static O_SYNC: String;
    #[doc = "Constant for fs.open(). Flag indicating that the file is opened for synchronous I/O with write operations waiting for data integrity."]
    pub static O_DSYNC: String;
    #[doc = "Constant for fs.open(). Flag indicating to open the symbolic link itself rather than the resource it is pointing to."]
    pub static O_SYMLINK: String;
    #[doc = "Constant for fs.open(). When set, an attempt will be made to minimize caching effects of file I/O."]
    pub static O_DIRECT: String;
    #[doc = "Constant for fs.open(). Flag indicating to open the file in nonblocking mode when possible."]
    pub static O_NONBLOCK: String;
    #[doc = "Constant for fs.Stats mode property for determining a file's type. Bit mask used to extract the file type code."]
    pub static S_IFMT: String;
    #[doc = "Constant for fs.Stats mode property for determining a file's type. File type constant for a regular file."]
    pub static S_IFREG: String;
    #[doc = "Constant for fs.Stats mode property for determining a file's type. File type constant for a directory."]
    pub static S_IFDIR: String;
    #[doc = "Constant for fs.Stats mode property for determining a file's type. File type constant for a character-oriented device file."]
    pub static S_IFCHR: String;
    #[doc = "Constant for fs.Stats mode property for determining a file's type. File type constant for a block-oriented device file."]
    pub static S_IFBLK: String;
    #[doc = "Constant for fs.Stats mode property for determining a file's type. File type constant for a FIFO/pipe."]
    pub static S_IFIFO: String;
    #[doc = "Constant for fs.Stats mode property for determining a file's type. File type constant for a symbolic link."]
    pub static S_IFLNK: String;
    #[doc = "Constant for fs.Stats mode property for determining a file's type. File type constant for a socket."]
    pub static S_IFSOCK: String;
    #[doc = "Constant for fs.Stats mode property for determining access permissions for a file. File mode indicating readable, writable and executable by owner."]
    pub static S_IRWXU: String;
    #[doc = "Constant for fs.Stats mode property for determining access permissions for a file. File mode indicating readable by owner."]
    pub static S_IRUSR: String;
    #[doc = "Constant for fs.Stats mode property for determining access permissions for a file. File mode indicating writable by owner."]
    pub static S_IWUSR: String;
    #[doc = "Constant for fs.Stats mode property for determining access permissions for a file. File mode indicating executable by owner."]
    pub static S_IXUSR: String;
    #[doc = "Constant for fs.Stats mode property for determining access permissions for a file. File mode indicating readable, writable and executable by group."]
    pub static S_IRWXG: String;
    #[doc = "Constant for fs.Stats mode property for determining access permissions for a file. File mode indicating readable by group."]
    pub static S_IRGRP: String;
    #[doc = "Constant for fs.Stats mode property for determining access permissions for a file. File mode indicating writable by group."]
    pub static S_IWGRP: String;
    #[doc = "Constant for fs.Stats mode property for determining access permissions for a file. File mode indicating executable by group."]
    pub static S_IXGRP: String;
    #[doc = "Constant for fs.Stats mode property for determining access permissions for a file. File mode indicating readable, writable and executable by others."]
    pub static S_IRWXO: String;
    #[doc = "Constant for fs.Stats mode property for determining access permissions for a file. File mode indicating readable by others."]
    pub static S_IROTH: String;
    #[doc = "Constant for fs.Stats mode property for determining access permissions for a file. File mode indicating writable by others."]
    pub static S_IWOTH: String;
    #[doc = "Constant for fs.Stats mode property for determining access permissions for a file. File mode indicating executable by others."]
    pub static S_IXOTH: String;
    #[doc = "When set, a memory file mapping is used to access the file. This flag"]
    #[doc = "is available on Windows operating systems only. On other operating systems,"]
    #[doc = "this flag is ignored."]
    pub static UV_FS_O_FILEMAP: String;
}