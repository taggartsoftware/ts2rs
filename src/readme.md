# ts2rs

ts2rs is a commercial application that generates [Rust WebAssembly bindings](https://github.com/rustwasm/wasm-bindgen) to JavaScript modules from TypeScript declarations. The Rust language is one of the best languages to use for targeting WebAssembly. One of its goals is that it [Plays well with JavaScript](https://www.rust-lang.org/what/wasm). ts2rs helps make seamless interop a reality by generating bindings for you.

ts2rs is not yet publicly available. For product announcements, please follow [https://medium.com/@taggartsoftware](https://medium.com/@taggartsoftware). If you are interested in a private preview, please email support@ctaggart.com. ts2rs will be available under monthly and yearly subscriptions. I am still figuring out the pricing and I am looking for feedback. ts2rs should help make it posssible to write some web applicatins and Node.js applications in Rust, saving hours or days or development time.

ts2rs is written in Rust and uses ts2rs generated bindings to the TypeScript API. More than a thousand lines of bindings were written by hand before I had it bootstrapped. It compiles to Web Assembly and is a packaged as a Node.js application. With access to a private NPM repository, you are able to install it with `npm install @taggartsoftware/ts2rs`.

### Examples
Example generated bindings will be available [on GitHub](https://github.com/taggartsoftware/ts2rs/) for a few open source projects.

### Features
- TypeScript interfaces & classes are mapped as Rust extern types.
- The TypeScript type heirarchy is mapped to series of [AsRef](https://doc.rust-lang.org/std/convert/trait.AsRef.html) and [Into](https://doc.rust-lang.org/std/convert/trait.Into.html) type conversions that use [JsCast](https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/trait.JsCast.html) to provide type safety with zero-cost.
- TypeScript doc comments become Rust doc comments.
- TypeScript enums are mapped as constants. The values are wrapped in [newtypes](https://github.com/rust-unofficial/patterns/blob/master/patterns/newtype.md).

Please create feature requests on GitHub [issues](https://github.com/taggartsoftware/ts2rs/issues) or email support@ctaggart.com.
