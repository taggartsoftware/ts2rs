// generated by ts2rs 0.2.0
// https://ts2rs.ctaggart.com/

impl ChildProcess {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl ChildProcessByStdio {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl ChildProcessWithoutNullStreams {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl CommonOptions {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl ExecException {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl ExecFileOptions {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl ExecFileOptionsWithBufferEncoding {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl ExecFileOptionsWithOtherEncoding {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl ExecFileOptionsWithStringEncoding {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl ExecFileSyncOptions {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl ExecFileSyncOptionsWithBufferEncoding {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl ExecFileSyncOptionsWithStringEncoding {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl ExecOptions {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl ExecOptionsWithBufferEncoding {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl ExecOptionsWithStringEncoding {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl ExecSyncOptions {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl ExecSyncOptionsWithBufferEncoding {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl ExecSyncOptionsWithStringEncoding {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl ForkOptions {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl MessageOptions {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl ProcessEnvOptions {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl PromiseWithChild {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl SpawnOptions {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl SpawnOptionsWithStdioTuple {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl SpawnOptionsWithoutStdio {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl SpawnSyncOptions {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl SpawnSyncOptionsWithBufferEncoding {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl SpawnSyncOptionsWithStringEncoding {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl SpawnSyncReturns {
    pub fn new() -> Self {
        Object::new().unchecked_into()
    }
}
impl AsRef<node_js::EventEmitter> for ChildProcess {
    fn as_ref(&self) -> &node_js::EventEmitter {
        self.unchecked_ref()
    }
}
impl From<ChildProcess> for node_js::EventEmitter {
    fn from(child: ChildProcess) -> Self {
        child.unchecked_into()
    }
}
impl ChildProcess {
    pub fn to_node_js_event_emitter(self) -> node_js::EventEmitter {
        self.unchecked_into()
    }
    pub fn as_node_js_event_emitter(&self) -> &node_js::EventEmitter {
        self.unchecked_ref()
    }
}
impl AsRef<node_js::EventEmitter> for ChildProcessByStdio {
    fn as_ref(&self) -> &node_js::EventEmitter {
        self.unchecked_ref()
    }
}
impl From<ChildProcessByStdio> for node_js::EventEmitter {
    fn from(child: ChildProcessByStdio) -> Self {
        child.unchecked_into()
    }
}
impl ChildProcessByStdio {
    pub fn to_node_js_event_emitter(self) -> node_js::EventEmitter {
        self.unchecked_into()
    }
    pub fn as_node_js_event_emitter(&self) -> &node_js::EventEmitter {
        self.unchecked_ref()
    }
}
impl AsRef<ChildProcess> for ChildProcessByStdio {
    fn as_ref(&self) -> &ChildProcess {
        self.unchecked_ref()
    }
}
impl From<ChildProcessByStdio> for ChildProcess {
    fn from(child: ChildProcessByStdio) -> Self {
        child.unchecked_into()
    }
}
impl ChildProcessByStdio {
    pub fn to_child_process(self) -> ChildProcess {
        self.unchecked_into()
    }
    pub fn as_child_process(&self) -> &ChildProcess {
        self.unchecked_ref()
    }
}
impl AsRef<node_js::EventEmitter> for ChildProcessWithoutNullStreams {
    fn as_ref(&self) -> &node_js::EventEmitter {
        self.unchecked_ref()
    }
}
impl From<ChildProcessWithoutNullStreams> for node_js::EventEmitter {
    fn from(child: ChildProcessWithoutNullStreams) -> Self {
        child.unchecked_into()
    }
}
impl ChildProcessWithoutNullStreams {
    pub fn to_node_js_event_emitter(self) -> node_js::EventEmitter {
        self.unchecked_into()
    }
    pub fn as_node_js_event_emitter(&self) -> &node_js::EventEmitter {
        self.unchecked_ref()
    }
}
impl AsRef<ChildProcess> for ChildProcessWithoutNullStreams {
    fn as_ref(&self) -> &ChildProcess {
        self.unchecked_ref()
    }
}
impl From<ChildProcessWithoutNullStreams> for ChildProcess {
    fn from(child: ChildProcessWithoutNullStreams) -> Self {
        child.unchecked_into()
    }
}
impl ChildProcessWithoutNullStreams {
    pub fn to_child_process(self) -> ChildProcess {
        self.unchecked_into()
    }
    pub fn as_child_process(&self) -> &ChildProcess {
        self.unchecked_ref()
    }
}
impl AsRef<ProcessEnvOptions> for CommonOptions {
    fn as_ref(&self) -> &ProcessEnvOptions {
        self.unchecked_ref()
    }
}
impl From<CommonOptions> for ProcessEnvOptions {
    fn from(child: CommonOptions) -> Self {
        child.unchecked_into()
    }
}
impl CommonOptions {
    pub fn to_process_env_options(self) -> ProcessEnvOptions {
        self.unchecked_into()
    }
    pub fn as_process_env_options(&self) -> &ProcessEnvOptions {
        self.unchecked_ref()
    }
}
impl AsRef<Error> for ExecException {
    fn as_ref(&self) -> &Error {
        self.unchecked_ref()
    }
}
impl From<ExecException> for Error {
    fn from(child: ExecException) -> Self {
        child.unchecked_into()
    }
}
impl ExecException {
    pub fn to_error(self) -> Error {
        self.unchecked_into()
    }
    pub fn as_error(&self) -> &Error {
        self.unchecked_ref()
    }
}
impl AsRef<CommonOptions> for ExecFileOptions {
    fn as_ref(&self) -> &CommonOptions {
        self.unchecked_ref()
    }
}
impl From<ExecFileOptions> for CommonOptions {
    fn from(child: ExecFileOptions) -> Self {
        child.unchecked_into()
    }
}
impl ExecFileOptions {
    pub fn to_common_options(self) -> CommonOptions {
        self.unchecked_into()
    }
    pub fn as_common_options(&self) -> &CommonOptions {
        self.unchecked_ref()
    }
}
impl AsRef<ProcessEnvOptions> for ExecFileOptions {
    fn as_ref(&self) -> &ProcessEnvOptions {
        self.unchecked_ref()
    }
}
impl From<ExecFileOptions> for ProcessEnvOptions {
    fn from(child: ExecFileOptions) -> Self {
        child.unchecked_into()
    }
}
impl ExecFileOptions {
    pub fn to_process_env_options(self) -> ProcessEnvOptions {
        self.unchecked_into()
    }
    pub fn as_process_env_options(&self) -> &ProcessEnvOptions {
        self.unchecked_ref()
    }
}
impl AsRef<CommonOptions> for ExecFileOptionsWithBufferEncoding {
    fn as_ref(&self) -> &CommonOptions {
        self.unchecked_ref()
    }
}
impl From<ExecFileOptionsWithBufferEncoding> for CommonOptions {
    fn from(child: ExecFileOptionsWithBufferEncoding) -> Self {
        child.unchecked_into()
    }
}
impl ExecFileOptionsWithBufferEncoding {
    pub fn to_common_options(self) -> CommonOptions {
        self.unchecked_into()
    }
    pub fn as_common_options(&self) -> &CommonOptions {
        self.unchecked_ref()
    }
}
impl AsRef<ExecFileOptions> for ExecFileOptionsWithBufferEncoding {
    fn as_ref(&self) -> &ExecFileOptions {
        self.unchecked_ref()
    }
}
impl From<ExecFileOptionsWithBufferEncoding> for ExecFileOptions {
    fn from(child: ExecFileOptionsWithBufferEncoding) -> Self {
        child.unchecked_into()
    }
}
impl ExecFileOptionsWithBufferEncoding {
    pub fn to_exec_file_options(self) -> ExecFileOptions {
        self.unchecked_into()
    }
    pub fn as_exec_file_options(&self) -> &ExecFileOptions {
        self.unchecked_ref()
    }
}
impl AsRef<ProcessEnvOptions> for ExecFileOptionsWithBufferEncoding {
    fn as_ref(&self) -> &ProcessEnvOptions {
        self.unchecked_ref()
    }
}
impl From<ExecFileOptionsWithBufferEncoding> for ProcessEnvOptions {
    fn from(child: ExecFileOptionsWithBufferEncoding) -> Self {
        child.unchecked_into()
    }
}
impl ExecFileOptionsWithBufferEncoding {
    pub fn to_process_env_options(self) -> ProcessEnvOptions {
        self.unchecked_into()
    }
    pub fn as_process_env_options(&self) -> &ProcessEnvOptions {
        self.unchecked_ref()
    }
}
impl AsRef<CommonOptions> for ExecFileOptionsWithOtherEncoding {
    fn as_ref(&self) -> &CommonOptions {
        self.unchecked_ref()
    }
}
impl From<ExecFileOptionsWithOtherEncoding> for CommonOptions {
    fn from(child: ExecFileOptionsWithOtherEncoding) -> Self {
        child.unchecked_into()
    }
}
impl ExecFileOptionsWithOtherEncoding {
    pub fn to_common_options(self) -> CommonOptions {
        self.unchecked_into()
    }
    pub fn as_common_options(&self) -> &CommonOptions {
        self.unchecked_ref()
    }
}
impl AsRef<ExecFileOptions> for ExecFileOptionsWithOtherEncoding {
    fn as_ref(&self) -> &ExecFileOptions {
        self.unchecked_ref()
    }
}
impl From<ExecFileOptionsWithOtherEncoding> for ExecFileOptions {
    fn from(child: ExecFileOptionsWithOtherEncoding) -> Self {
        child.unchecked_into()
    }
}
impl ExecFileOptionsWithOtherEncoding {
    pub fn to_exec_file_options(self) -> ExecFileOptions {
        self.unchecked_into()
    }
    pub fn as_exec_file_options(&self) -> &ExecFileOptions {
        self.unchecked_ref()
    }
}
impl AsRef<ProcessEnvOptions> for ExecFileOptionsWithOtherEncoding {
    fn as_ref(&self) -> &ProcessEnvOptions {
        self.unchecked_ref()
    }
}
impl From<ExecFileOptionsWithOtherEncoding> for ProcessEnvOptions {
    fn from(child: ExecFileOptionsWithOtherEncoding) -> Self {
        child.unchecked_into()
    }
}
impl ExecFileOptionsWithOtherEncoding {
    pub fn to_process_env_options(self) -> ProcessEnvOptions {
        self.unchecked_into()
    }
    pub fn as_process_env_options(&self) -> &ProcessEnvOptions {
        self.unchecked_ref()
    }
}
impl AsRef<CommonOptions> for ExecFileOptionsWithStringEncoding {
    fn as_ref(&self) -> &CommonOptions {
        self.unchecked_ref()
    }
}
impl From<ExecFileOptionsWithStringEncoding> for CommonOptions {
    fn from(child: ExecFileOptionsWithStringEncoding) -> Self {
        child.unchecked_into()
    }
}
impl ExecFileOptionsWithStringEncoding {
    pub fn to_common_options(self) -> CommonOptions {
        self.unchecked_into()
    }
    pub fn as_common_options(&self) -> &CommonOptions {
        self.unchecked_ref()
    }
}
impl AsRef<ExecFileOptions> for ExecFileOptionsWithStringEncoding {
    fn as_ref(&self) -> &ExecFileOptions {
        self.unchecked_ref()
    }
}
impl From<ExecFileOptionsWithStringEncoding> for ExecFileOptions {
    fn from(child: ExecFileOptionsWithStringEncoding) -> Self {
        child.unchecked_into()
    }
}
impl ExecFileOptionsWithStringEncoding {
    pub fn to_exec_file_options(self) -> ExecFileOptions {
        self.unchecked_into()
    }
    pub fn as_exec_file_options(&self) -> &ExecFileOptions {
        self.unchecked_ref()
    }
}
impl AsRef<ProcessEnvOptions> for ExecFileOptionsWithStringEncoding {
    fn as_ref(&self) -> &ProcessEnvOptions {
        self.unchecked_ref()
    }
}
impl From<ExecFileOptionsWithStringEncoding> for ProcessEnvOptions {
    fn from(child: ExecFileOptionsWithStringEncoding) -> Self {
        child.unchecked_into()
    }
}
impl ExecFileOptionsWithStringEncoding {
    pub fn to_process_env_options(self) -> ProcessEnvOptions {
        self.unchecked_into()
    }
    pub fn as_process_env_options(&self) -> &ProcessEnvOptions {
        self.unchecked_ref()
    }
}
impl AsRef<CommonOptions> for ExecFileSyncOptions {
    fn as_ref(&self) -> &CommonOptions {
        self.unchecked_ref()
    }
}
impl From<ExecFileSyncOptions> for CommonOptions {
    fn from(child: ExecFileSyncOptions) -> Self {
        child.unchecked_into()
    }
}
impl ExecFileSyncOptions {
    pub fn to_common_options(self) -> CommonOptions {
        self.unchecked_into()
    }
    pub fn as_common_options(&self) -> &CommonOptions {
        self.unchecked_ref()
    }
}
impl AsRef<ProcessEnvOptions> for ExecFileSyncOptions {
    fn as_ref(&self) -> &ProcessEnvOptions {
        self.unchecked_ref()
    }
}
impl From<ExecFileSyncOptions> for ProcessEnvOptions {
    fn from(child: ExecFileSyncOptions) -> Self {
        child.unchecked_into()
    }
}
impl ExecFileSyncOptions {
    pub fn to_process_env_options(self) -> ProcessEnvOptions {
        self.unchecked_into()
    }
    pub fn as_process_env_options(&self) -> &ProcessEnvOptions {
        self.unchecked_ref()
    }
}
impl AsRef<CommonOptions> for ExecFileSyncOptionsWithBufferEncoding {
    fn as_ref(&self) -> &CommonOptions {
        self.unchecked_ref()
    }
}
impl From<ExecFileSyncOptionsWithBufferEncoding> for CommonOptions {
    fn from(child: ExecFileSyncOptionsWithBufferEncoding) -> Self {
        child.unchecked_into()
    }
}
impl ExecFileSyncOptionsWithBufferEncoding {
    pub fn to_common_options(self) -> CommonOptions {
        self.unchecked_into()
    }
    pub fn as_common_options(&self) -> &CommonOptions {
        self.unchecked_ref()
    }
}
impl AsRef<ExecFileSyncOptions> for ExecFileSyncOptionsWithBufferEncoding {
    fn as_ref(&self) -> &ExecFileSyncOptions {
        self.unchecked_ref()
    }
}
impl From<ExecFileSyncOptionsWithBufferEncoding> for ExecFileSyncOptions {
    fn from(child: ExecFileSyncOptionsWithBufferEncoding) -> Self {
        child.unchecked_into()
    }
}
impl ExecFileSyncOptionsWithBufferEncoding {
    pub fn to_exec_file_sync_options(self) -> ExecFileSyncOptions {
        self.unchecked_into()
    }
    pub fn as_exec_file_sync_options(&self) -> &ExecFileSyncOptions {
        self.unchecked_ref()
    }
}
impl AsRef<ProcessEnvOptions> for ExecFileSyncOptionsWithBufferEncoding {
    fn as_ref(&self) -> &ProcessEnvOptions {
        self.unchecked_ref()
    }
}
impl From<ExecFileSyncOptionsWithBufferEncoding> for ProcessEnvOptions {
    fn from(child: ExecFileSyncOptionsWithBufferEncoding) -> Self {
        child.unchecked_into()
    }
}
impl ExecFileSyncOptionsWithBufferEncoding {
    pub fn to_process_env_options(self) -> ProcessEnvOptions {
        self.unchecked_into()
    }
    pub fn as_process_env_options(&self) -> &ProcessEnvOptions {
        self.unchecked_ref()
    }
}
impl AsRef<CommonOptions> for ExecFileSyncOptionsWithStringEncoding {
    fn as_ref(&self) -> &CommonOptions {
        self.unchecked_ref()
    }
}
impl From<ExecFileSyncOptionsWithStringEncoding> for CommonOptions {
    fn from(child: ExecFileSyncOptionsWithStringEncoding) -> Self {
        child.unchecked_into()
    }
}
impl ExecFileSyncOptionsWithStringEncoding {
    pub fn to_common_options(self) -> CommonOptions {
        self.unchecked_into()
    }
    pub fn as_common_options(&self) -> &CommonOptions {
        self.unchecked_ref()
    }
}
impl AsRef<ExecFileSyncOptions> for ExecFileSyncOptionsWithStringEncoding {
    fn as_ref(&self) -> &ExecFileSyncOptions {
        self.unchecked_ref()
    }
}
impl From<ExecFileSyncOptionsWithStringEncoding> for ExecFileSyncOptions {
    fn from(child: ExecFileSyncOptionsWithStringEncoding) -> Self {
        child.unchecked_into()
    }
}
impl ExecFileSyncOptionsWithStringEncoding {
    pub fn to_exec_file_sync_options(self) -> ExecFileSyncOptions {
        self.unchecked_into()
    }
    pub fn as_exec_file_sync_options(&self) -> &ExecFileSyncOptions {
        self.unchecked_ref()
    }
}
impl AsRef<ProcessEnvOptions> for ExecFileSyncOptionsWithStringEncoding {
    fn as_ref(&self) -> &ProcessEnvOptions {
        self.unchecked_ref()
    }
}
impl From<ExecFileSyncOptionsWithStringEncoding> for ProcessEnvOptions {
    fn from(child: ExecFileSyncOptionsWithStringEncoding) -> Self {
        child.unchecked_into()
    }
}
impl ExecFileSyncOptionsWithStringEncoding {
    pub fn to_process_env_options(self) -> ProcessEnvOptions {
        self.unchecked_into()
    }
    pub fn as_process_env_options(&self) -> &ProcessEnvOptions {
        self.unchecked_ref()
    }
}
impl AsRef<CommonOptions> for ExecOptions {
    fn as_ref(&self) -> &CommonOptions {
        self.unchecked_ref()
    }
}
impl From<ExecOptions> for CommonOptions {
    fn from(child: ExecOptions) -> Self {
        child.unchecked_into()
    }
}
impl ExecOptions {
    pub fn to_common_options(self) -> CommonOptions {
        self.unchecked_into()
    }
    pub fn as_common_options(&self) -> &CommonOptions {
        self.unchecked_ref()
    }
}
impl AsRef<ProcessEnvOptions> for ExecOptions {
    fn as_ref(&self) -> &ProcessEnvOptions {
        self.unchecked_ref()
    }
}
impl From<ExecOptions> for ProcessEnvOptions {
    fn from(child: ExecOptions) -> Self {
        child.unchecked_into()
    }
}
impl ExecOptions {
    pub fn to_process_env_options(self) -> ProcessEnvOptions {
        self.unchecked_into()
    }
    pub fn as_process_env_options(&self) -> &ProcessEnvOptions {
        self.unchecked_ref()
    }
}
impl AsRef<CommonOptions> for ExecOptionsWithBufferEncoding {
    fn as_ref(&self) -> &CommonOptions {
        self.unchecked_ref()
    }
}
impl From<ExecOptionsWithBufferEncoding> for CommonOptions {
    fn from(child: ExecOptionsWithBufferEncoding) -> Self {
        child.unchecked_into()
    }
}
impl ExecOptionsWithBufferEncoding {
    pub fn to_common_options(self) -> CommonOptions {
        self.unchecked_into()
    }
    pub fn as_common_options(&self) -> &CommonOptions {
        self.unchecked_ref()
    }
}
impl AsRef<ExecOptions> for ExecOptionsWithBufferEncoding {
    fn as_ref(&self) -> &ExecOptions {
        self.unchecked_ref()
    }
}
impl From<ExecOptionsWithBufferEncoding> for ExecOptions {
    fn from(child: ExecOptionsWithBufferEncoding) -> Self {
        child.unchecked_into()
    }
}
impl ExecOptionsWithBufferEncoding {
    pub fn to_exec_options(self) -> ExecOptions {
        self.unchecked_into()
    }
    pub fn as_exec_options(&self) -> &ExecOptions {
        self.unchecked_ref()
    }
}
impl AsRef<ProcessEnvOptions> for ExecOptionsWithBufferEncoding {
    fn as_ref(&self) -> &ProcessEnvOptions {
        self.unchecked_ref()
    }
}
impl From<ExecOptionsWithBufferEncoding> for ProcessEnvOptions {
    fn from(child: ExecOptionsWithBufferEncoding) -> Self {
        child.unchecked_into()
    }
}
impl ExecOptionsWithBufferEncoding {
    pub fn to_process_env_options(self) -> ProcessEnvOptions {
        self.unchecked_into()
    }
    pub fn as_process_env_options(&self) -> &ProcessEnvOptions {
        self.unchecked_ref()
    }
}
impl AsRef<CommonOptions> for ExecOptionsWithStringEncoding {
    fn as_ref(&self) -> &CommonOptions {
        self.unchecked_ref()
    }
}
impl From<ExecOptionsWithStringEncoding> for CommonOptions {
    fn from(child: ExecOptionsWithStringEncoding) -> Self {
        child.unchecked_into()
    }
}
impl ExecOptionsWithStringEncoding {
    pub fn to_common_options(self) -> CommonOptions {
        self.unchecked_into()
    }
    pub fn as_common_options(&self) -> &CommonOptions {
        self.unchecked_ref()
    }
}
impl AsRef<ExecOptions> for ExecOptionsWithStringEncoding {
    fn as_ref(&self) -> &ExecOptions {
        self.unchecked_ref()
    }
}
impl From<ExecOptionsWithStringEncoding> for ExecOptions {
    fn from(child: ExecOptionsWithStringEncoding) -> Self {
        child.unchecked_into()
    }
}
impl ExecOptionsWithStringEncoding {
    pub fn to_exec_options(self) -> ExecOptions {
        self.unchecked_into()
    }
    pub fn as_exec_options(&self) -> &ExecOptions {
        self.unchecked_ref()
    }
}
impl AsRef<ProcessEnvOptions> for ExecOptionsWithStringEncoding {
    fn as_ref(&self) -> &ProcessEnvOptions {
        self.unchecked_ref()
    }
}
impl From<ExecOptionsWithStringEncoding> for ProcessEnvOptions {
    fn from(child: ExecOptionsWithStringEncoding) -> Self {
        child.unchecked_into()
    }
}
impl ExecOptionsWithStringEncoding {
    pub fn to_process_env_options(self) -> ProcessEnvOptions {
        self.unchecked_into()
    }
    pub fn as_process_env_options(&self) -> &ProcessEnvOptions {
        self.unchecked_ref()
    }
}
impl AsRef<CommonOptions> for ExecSyncOptions {
    fn as_ref(&self) -> &CommonOptions {
        self.unchecked_ref()
    }
}
impl From<ExecSyncOptions> for CommonOptions {
    fn from(child: ExecSyncOptions) -> Self {
        child.unchecked_into()
    }
}
impl ExecSyncOptions {
    pub fn to_common_options(self) -> CommonOptions {
        self.unchecked_into()
    }
    pub fn as_common_options(&self) -> &CommonOptions {
        self.unchecked_ref()
    }
}
impl AsRef<ProcessEnvOptions> for ExecSyncOptions {
    fn as_ref(&self) -> &ProcessEnvOptions {
        self.unchecked_ref()
    }
}
impl From<ExecSyncOptions> for ProcessEnvOptions {
    fn from(child: ExecSyncOptions) -> Self {
        child.unchecked_into()
    }
}
impl ExecSyncOptions {
    pub fn to_process_env_options(self) -> ProcessEnvOptions {
        self.unchecked_into()
    }
    pub fn as_process_env_options(&self) -> &ProcessEnvOptions {
        self.unchecked_ref()
    }
}
impl AsRef<CommonOptions> for ExecSyncOptionsWithBufferEncoding {
    fn as_ref(&self) -> &CommonOptions {
        self.unchecked_ref()
    }
}
impl From<ExecSyncOptionsWithBufferEncoding> for CommonOptions {
    fn from(child: ExecSyncOptionsWithBufferEncoding) -> Self {
        child.unchecked_into()
    }
}
impl ExecSyncOptionsWithBufferEncoding {
    pub fn to_common_options(self) -> CommonOptions {
        self.unchecked_into()
    }
    pub fn as_common_options(&self) -> &CommonOptions {
        self.unchecked_ref()
    }
}
impl AsRef<ExecSyncOptions> for ExecSyncOptionsWithBufferEncoding {
    fn as_ref(&self) -> &ExecSyncOptions {
        self.unchecked_ref()
    }
}
impl From<ExecSyncOptionsWithBufferEncoding> for ExecSyncOptions {
    fn from(child: ExecSyncOptionsWithBufferEncoding) -> Self {
        child.unchecked_into()
    }
}
impl ExecSyncOptionsWithBufferEncoding {
    pub fn to_exec_sync_options(self) -> ExecSyncOptions {
        self.unchecked_into()
    }
    pub fn as_exec_sync_options(&self) -> &ExecSyncOptions {
        self.unchecked_ref()
    }
}
impl AsRef<ProcessEnvOptions> for ExecSyncOptionsWithBufferEncoding {
    fn as_ref(&self) -> &ProcessEnvOptions {
        self.unchecked_ref()
    }
}
impl From<ExecSyncOptionsWithBufferEncoding> for ProcessEnvOptions {
    fn from(child: ExecSyncOptionsWithBufferEncoding) -> Self {
        child.unchecked_into()
    }
}
impl ExecSyncOptionsWithBufferEncoding {
    pub fn to_process_env_options(self) -> ProcessEnvOptions {
        self.unchecked_into()
    }
    pub fn as_process_env_options(&self) -> &ProcessEnvOptions {
        self.unchecked_ref()
    }
}
impl AsRef<CommonOptions> for ExecSyncOptionsWithStringEncoding {
    fn as_ref(&self) -> &CommonOptions {
        self.unchecked_ref()
    }
}
impl From<ExecSyncOptionsWithStringEncoding> for CommonOptions {
    fn from(child: ExecSyncOptionsWithStringEncoding) -> Self {
        child.unchecked_into()
    }
}
impl ExecSyncOptionsWithStringEncoding {
    pub fn to_common_options(self) -> CommonOptions {
        self.unchecked_into()
    }
    pub fn as_common_options(&self) -> &CommonOptions {
        self.unchecked_ref()
    }
}
impl AsRef<ExecSyncOptions> for ExecSyncOptionsWithStringEncoding {
    fn as_ref(&self) -> &ExecSyncOptions {
        self.unchecked_ref()
    }
}
impl From<ExecSyncOptionsWithStringEncoding> for ExecSyncOptions {
    fn from(child: ExecSyncOptionsWithStringEncoding) -> Self {
        child.unchecked_into()
    }
}
impl ExecSyncOptionsWithStringEncoding {
    pub fn to_exec_sync_options(self) -> ExecSyncOptions {
        self.unchecked_into()
    }
    pub fn as_exec_sync_options(&self) -> &ExecSyncOptions {
        self.unchecked_ref()
    }
}
impl AsRef<ProcessEnvOptions> for ExecSyncOptionsWithStringEncoding {
    fn as_ref(&self) -> &ProcessEnvOptions {
        self.unchecked_ref()
    }
}
impl From<ExecSyncOptionsWithStringEncoding> for ProcessEnvOptions {
    fn from(child: ExecSyncOptionsWithStringEncoding) -> Self {
        child.unchecked_into()
    }
}
impl ExecSyncOptionsWithStringEncoding {
    pub fn to_process_env_options(self) -> ProcessEnvOptions {
        self.unchecked_into()
    }
    pub fn as_process_env_options(&self) -> &ProcessEnvOptions {
        self.unchecked_ref()
    }
}
impl AsRef<ProcessEnvOptions> for ForkOptions {
    fn as_ref(&self) -> &ProcessEnvOptions {
        self.unchecked_ref()
    }
}
impl From<ForkOptions> for ProcessEnvOptions {
    fn from(child: ForkOptions) -> Self {
        child.unchecked_into()
    }
}
impl ForkOptions {
    pub fn to_process_env_options(self) -> ProcessEnvOptions {
        self.unchecked_into()
    }
    pub fn as_process_env_options(&self) -> &ProcessEnvOptions {
        self.unchecked_ref()
    }
}
impl AsRef<Promise> for PromiseWithChild {
    fn as_ref(&self) -> &Promise {
        self.unchecked_ref()
    }
}
impl From<PromiseWithChild> for Promise {
    fn from(child: PromiseWithChild) -> Self {
        child.unchecked_into()
    }
}
impl PromiseWithChild {
    pub fn to_promise(self) -> Promise {
        self.unchecked_into()
    }
    pub fn as_promise(&self) -> &Promise {
        self.unchecked_ref()
    }
}
impl AsRef<CommonOptions> for SpawnOptions {
    fn as_ref(&self) -> &CommonOptions {
        self.unchecked_ref()
    }
}
impl From<SpawnOptions> for CommonOptions {
    fn from(child: SpawnOptions) -> Self {
        child.unchecked_into()
    }
}
impl SpawnOptions {
    pub fn to_common_options(self) -> CommonOptions {
        self.unchecked_into()
    }
    pub fn as_common_options(&self) -> &CommonOptions {
        self.unchecked_ref()
    }
}
impl AsRef<ProcessEnvOptions> for SpawnOptions {
    fn as_ref(&self) -> &ProcessEnvOptions {
        self.unchecked_ref()
    }
}
impl From<SpawnOptions> for ProcessEnvOptions {
    fn from(child: SpawnOptions) -> Self {
        child.unchecked_into()
    }
}
impl SpawnOptions {
    pub fn to_process_env_options(self) -> ProcessEnvOptions {
        self.unchecked_into()
    }
    pub fn as_process_env_options(&self) -> &ProcessEnvOptions {
        self.unchecked_ref()
    }
}
impl AsRef<CommonOptions> for SpawnOptionsWithStdioTuple {
    fn as_ref(&self) -> &CommonOptions {
        self.unchecked_ref()
    }
}
impl From<SpawnOptionsWithStdioTuple> for CommonOptions {
    fn from(child: SpawnOptionsWithStdioTuple) -> Self {
        child.unchecked_into()
    }
}
impl SpawnOptionsWithStdioTuple {
    pub fn to_common_options(self) -> CommonOptions {
        self.unchecked_into()
    }
    pub fn as_common_options(&self) -> &CommonOptions {
        self.unchecked_ref()
    }
}
impl AsRef<ProcessEnvOptions> for SpawnOptionsWithStdioTuple {
    fn as_ref(&self) -> &ProcessEnvOptions {
        self.unchecked_ref()
    }
}
impl From<SpawnOptionsWithStdioTuple> for ProcessEnvOptions {
    fn from(child: SpawnOptionsWithStdioTuple) -> Self {
        child.unchecked_into()
    }
}
impl SpawnOptionsWithStdioTuple {
    pub fn to_process_env_options(self) -> ProcessEnvOptions {
        self.unchecked_into()
    }
    pub fn as_process_env_options(&self) -> &ProcessEnvOptions {
        self.unchecked_ref()
    }
}
impl AsRef<SpawnOptions> for SpawnOptionsWithStdioTuple {
    fn as_ref(&self) -> &SpawnOptions {
        self.unchecked_ref()
    }
}
impl From<SpawnOptionsWithStdioTuple> for SpawnOptions {
    fn from(child: SpawnOptionsWithStdioTuple) -> Self {
        child.unchecked_into()
    }
}
impl SpawnOptionsWithStdioTuple {
    pub fn to_spawn_options(self) -> SpawnOptions {
        self.unchecked_into()
    }
    pub fn as_spawn_options(&self) -> &SpawnOptions {
        self.unchecked_ref()
    }
}
impl AsRef<CommonOptions> for SpawnOptionsWithoutStdio {
    fn as_ref(&self) -> &CommonOptions {
        self.unchecked_ref()
    }
}
impl From<SpawnOptionsWithoutStdio> for CommonOptions {
    fn from(child: SpawnOptionsWithoutStdio) -> Self {
        child.unchecked_into()
    }
}
impl SpawnOptionsWithoutStdio {
    pub fn to_common_options(self) -> CommonOptions {
        self.unchecked_into()
    }
    pub fn as_common_options(&self) -> &CommonOptions {
        self.unchecked_ref()
    }
}
impl AsRef<ProcessEnvOptions> for SpawnOptionsWithoutStdio {
    fn as_ref(&self) -> &ProcessEnvOptions {
        self.unchecked_ref()
    }
}
impl From<SpawnOptionsWithoutStdio> for ProcessEnvOptions {
    fn from(child: SpawnOptionsWithoutStdio) -> Self {
        child.unchecked_into()
    }
}
impl SpawnOptionsWithoutStdio {
    pub fn to_process_env_options(self) -> ProcessEnvOptions {
        self.unchecked_into()
    }
    pub fn as_process_env_options(&self) -> &ProcessEnvOptions {
        self.unchecked_ref()
    }
}
impl AsRef<SpawnOptions> for SpawnOptionsWithoutStdio {
    fn as_ref(&self) -> &SpawnOptions {
        self.unchecked_ref()
    }
}
impl From<SpawnOptionsWithoutStdio> for SpawnOptions {
    fn from(child: SpawnOptionsWithoutStdio) -> Self {
        child.unchecked_into()
    }
}
impl SpawnOptionsWithoutStdio {
    pub fn to_spawn_options(self) -> SpawnOptions {
        self.unchecked_into()
    }
    pub fn as_spawn_options(&self) -> &SpawnOptions {
        self.unchecked_ref()
    }
}
impl AsRef<CommonOptions> for SpawnSyncOptions {
    fn as_ref(&self) -> &CommonOptions {
        self.unchecked_ref()
    }
}
impl From<SpawnSyncOptions> for CommonOptions {
    fn from(child: SpawnSyncOptions) -> Self {
        child.unchecked_into()
    }
}
impl SpawnSyncOptions {
    pub fn to_common_options(self) -> CommonOptions {
        self.unchecked_into()
    }
    pub fn as_common_options(&self) -> &CommonOptions {
        self.unchecked_ref()
    }
}
impl AsRef<ProcessEnvOptions> for SpawnSyncOptions {
    fn as_ref(&self) -> &ProcessEnvOptions {
        self.unchecked_ref()
    }
}
impl From<SpawnSyncOptions> for ProcessEnvOptions {
    fn from(child: SpawnSyncOptions) -> Self {
        child.unchecked_into()
    }
}
impl SpawnSyncOptions {
    pub fn to_process_env_options(self) -> ProcessEnvOptions {
        self.unchecked_into()
    }
    pub fn as_process_env_options(&self) -> &ProcessEnvOptions {
        self.unchecked_ref()
    }
}
impl AsRef<CommonOptions> for SpawnSyncOptionsWithBufferEncoding {
    fn as_ref(&self) -> &CommonOptions {
        self.unchecked_ref()
    }
}
impl From<SpawnSyncOptionsWithBufferEncoding> for CommonOptions {
    fn from(child: SpawnSyncOptionsWithBufferEncoding) -> Self {
        child.unchecked_into()
    }
}
impl SpawnSyncOptionsWithBufferEncoding {
    pub fn to_common_options(self) -> CommonOptions {
        self.unchecked_into()
    }
    pub fn as_common_options(&self) -> &CommonOptions {
        self.unchecked_ref()
    }
}
impl AsRef<ProcessEnvOptions> for SpawnSyncOptionsWithBufferEncoding {
    fn as_ref(&self) -> &ProcessEnvOptions {
        self.unchecked_ref()
    }
}
impl From<SpawnSyncOptionsWithBufferEncoding> for ProcessEnvOptions {
    fn from(child: SpawnSyncOptionsWithBufferEncoding) -> Self {
        child.unchecked_into()
    }
}
impl SpawnSyncOptionsWithBufferEncoding {
    pub fn to_process_env_options(self) -> ProcessEnvOptions {
        self.unchecked_into()
    }
    pub fn as_process_env_options(&self) -> &ProcessEnvOptions {
        self.unchecked_ref()
    }
}
impl AsRef<SpawnSyncOptions> for SpawnSyncOptionsWithBufferEncoding {
    fn as_ref(&self) -> &SpawnSyncOptions {
        self.unchecked_ref()
    }
}
impl From<SpawnSyncOptionsWithBufferEncoding> for SpawnSyncOptions {
    fn from(child: SpawnSyncOptionsWithBufferEncoding) -> Self {
        child.unchecked_into()
    }
}
impl SpawnSyncOptionsWithBufferEncoding {
    pub fn to_spawn_sync_options(self) -> SpawnSyncOptions {
        self.unchecked_into()
    }
    pub fn as_spawn_sync_options(&self) -> &SpawnSyncOptions {
        self.unchecked_ref()
    }
}
impl AsRef<CommonOptions> for SpawnSyncOptionsWithStringEncoding {
    fn as_ref(&self) -> &CommonOptions {
        self.unchecked_ref()
    }
}
impl From<SpawnSyncOptionsWithStringEncoding> for CommonOptions {
    fn from(child: SpawnSyncOptionsWithStringEncoding) -> Self {
        child.unchecked_into()
    }
}
impl SpawnSyncOptionsWithStringEncoding {
    pub fn to_common_options(self) -> CommonOptions {
        self.unchecked_into()
    }
    pub fn as_common_options(&self) -> &CommonOptions {
        self.unchecked_ref()
    }
}
impl AsRef<ProcessEnvOptions> for SpawnSyncOptionsWithStringEncoding {
    fn as_ref(&self) -> &ProcessEnvOptions {
        self.unchecked_ref()
    }
}
impl From<SpawnSyncOptionsWithStringEncoding> for ProcessEnvOptions {
    fn from(child: SpawnSyncOptionsWithStringEncoding) -> Self {
        child.unchecked_into()
    }
}
impl SpawnSyncOptionsWithStringEncoding {
    pub fn to_process_env_options(self) -> ProcessEnvOptions {
        self.unchecked_into()
    }
    pub fn as_process_env_options(&self) -> &ProcessEnvOptions {
        self.unchecked_ref()
    }
}
impl AsRef<SpawnSyncOptions> for SpawnSyncOptionsWithStringEncoding {
    fn as_ref(&self) -> &SpawnSyncOptions {
        self.unchecked_ref()
    }
}
impl From<SpawnSyncOptionsWithStringEncoding> for SpawnSyncOptions {
    fn from(child: SpawnSyncOptionsWithStringEncoding) -> Self {
        child.unchecked_into()
    }
}
impl SpawnSyncOptionsWithStringEncoding {
    pub fn to_spawn_sync_options(self) -> SpawnSyncOptions {
        self.unchecked_into()
    }
    pub fn as_spawn_sync_options(&self) -> &SpawnSyncOptions {
        self.unchecked_ref()
    }
}
