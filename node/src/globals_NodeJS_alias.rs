// generated by ts2rs 0.2.0
// https://ts2rs.ctaggart.com/

pub type BeforeExitListener = Closure<dyn FnMut(f64)>;
pub type DisconnectListener = Closure<dyn FnMut()>;
pub type ExitListener = Closure<dyn FnMut(f64)>;
pub type RejectionHandledListener = Closure<dyn FnMut(Promise)>;
pub type UncaughtExceptionListener = Closure<dyn FnMut(Error)>;
pub type UnhandledRejectionListener = Closure<dyn FnMut(JsValue, Promise)>;
pub type WarningListener = Closure<dyn FnMut(Error)>;
pub type MessageListener = Closure<dyn FnMut(JsValue, JsValue)>;
pub type SignalsListener = Closure<dyn FnMut(Signals)>;
pub type NewListenerListener = Closure<dyn FnMut(JsValue, Function)>;
pub type RemoveListenerListener = Closure<dyn FnMut(JsValue, Function)>;
pub type MultipleResolveListener = Closure<dyn FnMut(MultipleResolveType, Promise, JsValue)>;