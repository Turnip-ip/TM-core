searchState.loadedDescShard("wasm_bindgen_backend", 0, "A common backend for bindgen crates.\nA struct representing a diagnostic to emit to the end-user …\nA trait for converting AST structs into Tokens and adding …\nA representation of the Abstract Syntax Tree of a Rust …\nImmediately fail and return an Err, with the arguments …\nProvide a Diagnostic with the given span and message\nGenerate a <code>Diagnostic</code> from an informational message with …\nReturns the argument unchanged.\nAttempt to generate a <code>Diagnostic</code> from a vector of other …\nCalls <code>U::from(self)</code>.\nImmediately trigger a panic from this <code>Diagnostic</code>\nGenerate a <code>Diagnostic</code> from a Span and an informational …\nGenerate a <code>Diagnostic</code> from the span of any tokenizable …\nAttempt to convert a <code>Self</code> into a new <code>TokenStream</code>\nAttempt to convert a <code>Self</code> into a new <code>TokenStream</code>\nAttempt to convert a <code>Self</code> into tokens and add it to the …\nCommon utility function for manipulating syn types and …\nA by-mutable-reference arg, EG <code>&amp;mut T</code>\nA by-reference arg, EG <code>&amp;T</code>\n<code>self</code>\nA by-value arg, EG <code>T</code>\nA class constructor\nThe metadata for an Enum\nImporting a JS enum\nA rust to js interface. Allows interaction with rust …\nAn exported argument (Rust side type)\nAn exported return\nRepresents an expression that needs to be evaluated before …\nInformation about a function being imported or exported\nImporting a function\nA method for getting the value of the provided Ident or …\nThings imported from a JS module (in an <code>extern</code> block)\nAn imported argument (JS side type)\nA function being imported from JS\nThe type of a function being imported\nThe type of item being imported\nThe possible types of module to import from\nAn imported return\nThe type of a static being imported\nThe type of a static string being imported\nThe metadata for a type being imported\nA dynamically intercepted deleter\nA dynamically intercepted getter\nA dynamically intercepted setter\nImport from an inline JS snippet\nAn abstract syntax tree representing a link to a module in …\nRepresents a literal string that can be directly encoded.\nAn enum representing either a literal value (<code>Lit</code>) or an …\nA class method\nThe type of a method\nThe 3 types variations of <code>self</code>.\nImport from the named module, with relative paths …\nA standard function\nThe operation performed by a class method\nAny other kind of method\nThe kind of operation performed by a method\nAn abstract syntax tree representing a rust program. …\nImport from the named module, without interpreting paths\n<code>&amp;mut self</code>\n<code>&amp;self</code>\nA standard method, nothing special\nA method for setting the value of the provided Ident or …\nImporting a static value\nImporting a static string\nThe metadata for a String Enum\nInformation about a Struct being exported\nThe field of a struct\nImporting a type/class\nUnused, the type of an argument to / return from a function\nUnused, the location of a type for a function argument …\nThe variant of an enum\nThe arguments to the function\nCauses the Builder (See cli-support::js::binding::Builder) …\nWhether this is an <code>async</code> function\nThe custom attributes to apply to this type\nWhether to catch JS exceptions\nComments extracted from the rust source.\nThe doc comments on this struct, if provided\nThe doc comments on this field, if any\nThe doc comments on this enum, if any\nThe doc comments on this variant, if any\nThe doc comment on this import, if one is provided\nThe doc comment applied to this type, if one exists\nrust enums\nrust -&gt; js interfaces\nThe list of classes this extends, if any\nAll the fields of this struct to export\nWhether this type can be inside an <code>impl</code> block.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nThe rust function\nThe full signature of the function\nWhether to generate jsdoc documentation for this function\nWhether to generate jsdoc documentation for this field\nWhether to generate a typescript definition for this …\nWhether to generate a typescript definition for this struct\nWhether to generate a typescript definition for this field\nWhether to generate a typescript definition for this enum\nThe name of the getter shim for this field\nThe span of the <code>#[wasm_bindgen(getter_with_clone)]</code> …\nThe value to use for a <code>none</code> variant of the enum\njs -&gt; rust interfaces\nIf the rust object has a <code>fn xxx(&amp;self) -&gt; MyType</code> method, …\nIf the rust object has a <code>fn set_xxx(&amp;mut self, MyType)</code> …\nInline JS snippets\nThe name of the shim to check instanceof for this type\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns true if the Program is empty\nWhether this struct is inspectable (provides …\nWhether this method is static\nThe name of the remote function to use for the generated …\nThe class name in JS this is attached to\nThe name of this static on the JS side\nThe name of this type on the JS side\nThe name of this string enum in JS/TS code\nThe name of the struct in JS code\nThe name of the field in JS code\nThe name of this enum in JS code\nThe namespace to access the item through, if any\nThe type being returned\nPath to js_sys\nPath to js_sys\nThe type of item being imported\nThe kind of function being imported\nThe internal kind of this Operation\nName of the link function for a specific linked module\nlinked-to modules\nThe kind (static, named, regular)\nThe type of <code>self</code> (either <code>self</code>, <code>&amp;self</code>, or <code>&amp;mut self</code>)\nThe type of module being imported from, if any\nThe Rust enum’s identifiers\nThe name of the function\nThe name of this variant\nThe span of the function’s name in Rust code\nIf present, don’t generate a <code>Deref</code> impl\nWhether this value is read-only to JS\nWhether the function has a js_name attribute\nThe return type of the function, if provided\nAttributes to apply to the Rust enum\nAny custom attributes being applied to the function\nThe struct name, in Rust, this is attached to\nThe name of the rust function/method on the rust side.\nThe name rust code will use\nThe name of this static on the Rust side\nThe name of this static on the Rust side\nThe name of this type on the Rust side\nThe name of the struct in Rust code\nThe name of the field in Rust code\nThe name of this enum in Rust code\nThe visibility of this function in Rust\nThe name of the setter shim for this field\nThe shim name to use in the generated code. The ‘shim’ …\nThe name of the shim function used to access this static\nThe name of the shim function used to access this static\nWhether or not this function should be flagged as the Wasm …\nThe string to export.\nThe name of the struct this field is part of\nrust structs\nWhether the function should use structural type checking\n<code>true</code> if using the new <code>thread_local</code> representation.\nThe type of static being imported\nThe type specified by the user, which we only use to show …\nThe type of this field\ncustom typescript sections to be included in the …\nThe TS definition to generate for this type\nWhether this is an <code>unsafe</code> function\nThe backing value of this variant\nWhether the function is variadic on the JS side\nWhether this is a function with a variadict parameter\nThe JS string values of the variants\nThe Rust identifiers for the variants\nThe variants provided by this enum\nA custom prefix to add and attempt to fall back to, if the …\nThe visibility of this static in Rust\nThe visibility of this static string in Rust\nThe visibility of this type in Rust\nThe Rust enum’s visibility\nPath to wasm_bindgen\nPath to wasm_bindgen\nPath to wasm_bindgen\nPath to wasm_bindgen\nPath to wasm_bindgen\nPath to wasm_bindgen\nPath to wasm_bindgen\nPath to wasm_bindgen\nPath to wasm_bindgen\nPath to wasm_bindgen\nPath to wasm_bindgen_futures\nPath to wasm_bindgen_futures\nPath to wasm_bindgen_futures\nThe name of the class for this method, in JS\nThe kind of method this is\nThe type of the class for this method, in Rust\nSmall utility used when generating symbol names.\nReturns the argument unchanged.\nCreate a path type with a single segment from a given …\nCalls <code>U::from(self)</code>.\nCreate a global path type from the given segments. For …\nCreate an <code>Ident</code> without checking to see if it conflicts …\nCreate an <code>Ident</code>, possibly mangling it if it conflicts with …\nCreate a path type from the given segments. For example an …\nConvert an ImportFunction into the more generic Import …")