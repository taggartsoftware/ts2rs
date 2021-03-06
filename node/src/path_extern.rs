// generated by ts2rs 0.2.0
// https://ts2rs.ctaggart.com/

#[wasm_bindgen(module = "path")]
extern "C" {
    #[doc = "A parsed path object generated by path.parse() or consumed by path.format()."]
    pub type ParsedPath;
    #[doc = "The root of the path such as '/' or 'c:\\'"]
    #[wasm_bindgen(method, getter)]
    pub fn root(this: &ParsedPath) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_root(this: &ParsedPath, value: &str);
    #[doc = "The full directory path such as '/home/user/dir' or 'c:\\path\\dir'"]
    #[wasm_bindgen(method, getter)]
    pub fn dir(this: &ParsedPath) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_dir(this: &ParsedPath, value: &str);
    #[doc = "The file name including extension (if any) such as 'index.html'"]
    #[wasm_bindgen(method, getter)]
    pub fn base(this: &ParsedPath) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_base(this: &ParsedPath, value: &str);
    #[doc = "The file extension (if any) such as '.html'"]
    #[wasm_bindgen(method, getter)]
    pub fn ext(this: &ParsedPath) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_ext(this: &ParsedPath, value: &str);
    #[doc = "The file name without extension (if any) such as 'index'"]
    #[wasm_bindgen(method, getter)]
    pub fn name(this: &ParsedPath) -> String;
    #[wasm_bindgen(method, setter)]
    pub fn set_name(this: &ParsedPath, value: &str);
    pub type FormatInputPathObject;
    #[doc = "The root of the path such as '/' or 'c:\\'"]
    #[wasm_bindgen(method, getter)]
    pub fn root(this: &FormatInputPathObject) -> Option<String>;
    #[wasm_bindgen(method, setter)]
    pub fn set_root(this: &FormatInputPathObject, value: Option<&str>);
    #[doc = "The full directory path such as '/home/user/dir' or 'c:\\path\\dir'"]
    #[wasm_bindgen(method, getter)]
    pub fn dir(this: &FormatInputPathObject) -> Option<String>;
    #[wasm_bindgen(method, setter)]
    pub fn set_dir(this: &FormatInputPathObject, value: Option<&str>);
    #[doc = "The file name including extension (if any) such as 'index.html'"]
    #[wasm_bindgen(method, getter)]
    pub fn base(this: &FormatInputPathObject) -> Option<String>;
    #[wasm_bindgen(method, setter)]
    pub fn set_base(this: &FormatInputPathObject, value: Option<&str>);
    #[doc = "The file extension (if any) such as '.html'"]
    #[wasm_bindgen(method, getter)]
    pub fn ext(this: &FormatInputPathObject) -> Option<String>;
    #[wasm_bindgen(method, setter)]
    pub fn set_ext(this: &FormatInputPathObject, value: Option<&str>);
    #[doc = "The file name without extension (if any) such as 'index'"]
    #[wasm_bindgen(method, getter)]
    pub fn name(this: &FormatInputPathObject) -> Option<String>;
    #[wasm_bindgen(method, setter)]
    pub fn set_name(this: &FormatInputPathObject, value: Option<&str>);
    #[doc = "Normalize a string path, reducing '..' and '.' parts."]
    #[doc = "When multiple slashes are found, they're replaced by a single one; when the path contains a trailing slash, it is preserved. On Windows backslashes are used."]
    #[wasm_bindgen()]
    pub fn normalize(p: &str) -> String;
    #[doc = "Join all arguments together and normalize the resulting path."]
    #[doc = "Arguments must be strings. In v0.8, non-string arguments were silently ignored. In v0.10 and up, an exception is thrown."]
    #[wasm_bindgen()]
    pub fn join(paths: &Array) -> String;
    #[doc = "The right-most parameter is considered {to}.  Other parameters are considered an array of {from}."]
    #[doc = ""]
    #[doc = "Starting from leftmost {from} parameter, resolves {to} to an absolute path."]
    #[doc = ""]
    #[doc = "If {to} isn't already absolute, {from} arguments are prepended in right to left order,"]
    #[doc = "until an absolute path is found. If after using all {from} paths still no absolute path is found,"]
    #[doc = "the current working directory is used as well. The resulting path is normalized,"]
    #[doc = "and trailing slashes are removed unless the path gets resolved to the root directory."]
    #[wasm_bindgen()]
    pub fn resolve(path_segments: &Array) -> String;
    #[doc = "Determines whether {path} is an absolute path. An absolute path will always resolve to the same location, regardless of the working directory."]
    # [ wasm_bindgen ( js_name = isAbsolute ) ]
    pub fn is_absolute(path: &str) -> bool;
    #[doc = "Solve the relative path from {from} to {to}."]
    #[doc = "At times we have two absolute paths, and we need to derive the relative path from one to the other. This is actually the reverse transform of path.resolve."]
    #[wasm_bindgen()]
    pub fn relative(from: &str, to: &str) -> String;
    #[doc = "Return the directory name of a path. Similar to the Unix dirname command."]
    #[wasm_bindgen()]
    pub fn dirname(p: &str) -> String;
    #[doc = "Return the last portion of a path. Similar to the Unix basename command."]
    #[doc = "Often used to extract the file name from a fully qualified path."]
    #[wasm_bindgen()]
    pub fn basename(p: &str, ext: Option<&str>) -> String;
    #[doc = "Return the extension of the path, from the last '.' to end of string in the last portion of the path."]
    #[doc = "If there is no '.' in the last portion of the path or the first character of it is '.', then it returns an empty string"]
    #[wasm_bindgen()]
    pub fn extname(p: &str) -> String;
    #[wasm_bindgen(js_name = "sep")]
    #[doc = "The platform-specific file separator. '\\\\' or '/'."]
    pub static SEP: JsValue;
    #[wasm_bindgen(js_name = "delimiter")]
    #[doc = "The platform-specific file delimiter. ';' or ':'."]
    pub static DELIMITER: JsValue;
    #[doc = "Returns an object from a path string - the opposite of format()."]
    #[wasm_bindgen()]
    pub fn parse(path_string: &str) -> ParsedPath;
    #[doc = "Returns a path string from an object - the opposite of parse()."]
    #[wasm_bindgen()]
    pub fn format(path_object: &FormatInputPathObject) -> String;
}
