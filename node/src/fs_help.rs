// generated by ts2rs 0.1.4
// https://ts2rs.ctaggart.com/

impl BigIntOptions {
    pub fn new() -> BigIntOptions {
        JsCast::unchecked_into(Object::new())
    }
}
impl BigIntStats {
    pub fn new() -> BigIntStats {
        JsCast::unchecked_into(Object::new())
    }
}
impl Dir {
    pub fn new() -> Dir {
        JsCast::unchecked_into(Object::new())
    }
}
impl Dirent {
    pub fn new() -> Dirent {
        JsCast::unchecked_into(Object::new())
    }
}
impl FSWatcher {
    pub fn new() -> FSWatcher {
        JsCast::unchecked_into(Object::new())
    }
}
impl MakeDirectoryOptions {
    pub fn new() -> MakeDirectoryOptions {
        JsCast::unchecked_into(Object::new())
    }
}
impl OpenDirOptions {
    pub fn new() -> OpenDirOptions {
        JsCast::unchecked_into(Object::new())
    }
}
impl ReadStream {
    pub fn new() -> ReadStream {
        JsCast::unchecked_into(Object::new())
    }
}
impl RmDirAsyncOptions {
    pub fn new() -> RmDirAsyncOptions {
        JsCast::unchecked_into(Object::new())
    }
}
impl RmDirOptions {
    pub fn new() -> RmDirOptions {
        JsCast::unchecked_into(Object::new())
    }
}
impl StatOptions {
    pub fn new() -> StatOptions {
        JsCast::unchecked_into(Object::new())
    }
}
impl Stats {
    pub fn new() -> Stats {
        JsCast::unchecked_into(Object::new())
    }
}
impl StatsBase {
    pub fn new() -> StatsBase {
        JsCast::unchecked_into(Object::new())
    }
}
impl WriteStream {
    pub fn new() -> WriteStream {
        JsCast::unchecked_into(Object::new())
    }
}
impl WriteVResult {
    pub fn new() -> WriteVResult {
        JsCast::unchecked_into(Object::new())
    }
}
impl AsRef<StatsBase> for BigIntStats {
    fn as_ref(&self) -> &StatsBase {
        JsCast::unchecked_ref(self)
    }
}
impl From<BigIntStats> for StatsBase {
    fn from(child: BigIntStats) -> Self {
        JsCast::unchecked_into(child)
    }
}
impl AsRef<node_js::EventEmitter> for FSWatcher {
    fn as_ref(&self) -> &node_js::EventEmitter {
        JsCast::unchecked_ref(self)
    }
}
impl From<FSWatcher> for node_js::EventEmitter {
    fn from(child: FSWatcher) -> Self {
        JsCast::unchecked_into(child)
    }
}
impl AsRef<EventEmitter> for ReadStream {
    fn as_ref(&self) -> &EventEmitter {
        JsCast::unchecked_ref(self)
    }
}
impl From<ReadStream> for EventEmitter {
    fn from(child: ReadStream) -> Self {
        JsCast::unchecked_into(child)
    }
}
impl AsRef<node_js::EventEmitter> for ReadStream {
    fn as_ref(&self) -> &node_js::EventEmitter {
        JsCast::unchecked_ref(self)
    }
}
impl From<ReadStream> for node_js::EventEmitter {
    fn from(child: ReadStream) -> Self {
        JsCast::unchecked_into(child)
    }
}
impl AsRef<node_js::ReadableStream> for ReadStream {
    fn as_ref(&self) -> &node_js::ReadableStream {
        JsCast::unchecked_ref(self)
    }
}
impl From<ReadStream> for node_js::ReadableStream {
    fn from(child: ReadStream) -> Self {
        JsCast::unchecked_into(child)
    }
}
impl AsRef<Stream> for ReadStream {
    fn as_ref(&self) -> &Stream {
        JsCast::unchecked_ref(self)
    }
}
impl From<ReadStream> for Stream {
    fn from(child: ReadStream) -> Self {
        JsCast::unchecked_into(child)
    }
}
impl AsRef<RmDirOptions> for RmDirAsyncOptions {
    fn as_ref(&self) -> &RmDirOptions {
        JsCast::unchecked_ref(self)
    }
}
impl From<RmDirAsyncOptions> for RmDirOptions {
    fn from(child: RmDirAsyncOptions) -> Self {
        JsCast::unchecked_into(child)
    }
}
impl AsRef<StatsBase> for Stats {
    fn as_ref(&self) -> &StatsBase {
        JsCast::unchecked_ref(self)
    }
}
impl From<Stats> for StatsBase {
    fn from(child: Stats) -> Self {
        JsCast::unchecked_into(child)
    }
}
impl AsRef<EventEmitter> for WriteStream {
    fn as_ref(&self) -> &EventEmitter {
        JsCast::unchecked_ref(self)
    }
}
impl From<WriteStream> for EventEmitter {
    fn from(child: WriteStream) -> Self {
        JsCast::unchecked_into(child)
    }
}
impl AsRef<node_js::EventEmitter> for WriteStream {
    fn as_ref(&self) -> &node_js::EventEmitter {
        JsCast::unchecked_ref(self)
    }
}
impl From<WriteStream> for node_js::EventEmitter {
    fn from(child: WriteStream) -> Self {
        JsCast::unchecked_into(child)
    }
}
impl AsRef<node_js::WritableStream> for WriteStream {
    fn as_ref(&self) -> &node_js::WritableStream {
        JsCast::unchecked_ref(self)
    }
}
impl From<WriteStream> for node_js::WritableStream {
    fn from(child: WriteStream) -> Self {
        JsCast::unchecked_into(child)
    }
}
impl AsRef<Stream> for WriteStream {
    fn as_ref(&self) -> &Stream {
        JsCast::unchecked_ref(self)
    }
}
impl From<WriteStream> for Stream {
    fn from(child: WriteStream) -> Self {
        JsCast::unchecked_into(child)
    }
}