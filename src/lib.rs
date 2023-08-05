/**
 * The Chrome DevTools Protocol.
*/
pub mod protocol {

    pub type Integer = i32;

    /**
     * This domain is deprecated - use Runtime or Log instead.
    */
    pub mod console {
        use super::Integer;
        use serde::{self, Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize)]
        pub enum ConsoleMessageSource {
            #[serde(rename = "xml")]
            XML,

            #[serde(rename = "javascript")]
            Javascript,

            #[serde(rename = "network")]
            Network,

            #[serde(rename = "console-api")]
            ConsoleAPI,

            #[serde(rename = "storage")]
            Storage,

            #[serde(rename = "appcache")]
            Appcache,

            #[serde(rename = "rendering")]
            Rendering,

            #[serde(rename = "security")]
            Security,

            #[serde(rename = "other")]
            Other,

            #[serde(rename = "deprecation")]
            Deprecation,

            #[serde(rename = "worker")]
            Worker,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum ConsoleMessageLevel {
            #[serde(rename = "log")]
            Log,

            #[serde(rename = "warning")]
            Warning,

            #[serde(rename = "error")]
            Error,

            #[serde(rename = "debug")]
            Debug,

            #[serde(rename = "info")]
            Info,
        }

        /**
         * Console message.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ConsoleMessage {
            /**
             * Console message.
            */
            source: Option<ConsoleMessageSource>,
            /**
             * Console message.
            */
            level: Option<ConsoleMessageLevel>,
            /**
             * Console message.
            */
            text: Option<String>,
            /**
             * Console message.
            */
            url: String,
            /**
             * Console message.
            */
            line: Integer,
            /**
             * Console message.
            */
            column: Integer,
        }

        /**
         * Issued when new console message is added.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct MessageAddedEvent {
            /**
             * Issued when new console message is added.
            */
            message: Option<ConsoleMessage>,
        }
    }

    /**
     * Debugger domain exposes JavaScript debugging capabilities. It allows setting and removing
     * breakpoints, stepping through execution, exploring stack traces, etc.
    */
    pub mod debugger {
        use super::{debugger, runtime, Integer};
        use serde::{self, Deserialize, Serialize};
        /**
         * Breakpoint identifier.
        */
        pub type BreakpointId = String;

        /**
         * Call frame identifier.
        */
        pub type CallFrameId = String;

        /**
         * Location in the source code.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Location {
            /**
             * Location in the source code.
            */
            #[serde(rename = "scriptId")]
            script_id: Option<runtime::ScriptId>,
            /**
             * Location in the source code.
            */
            #[serde(rename = "lineNumber")]
            line_number: Option<Integer>,
            /**
             * Location in the source code.
            */
            #[serde(rename = "columnNumber")]
            column_number: Integer,
        }

        /**
         * Location in the source code.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ScriptPosition {
            /**
             * Location in the source code.
            */
            #[serde(rename = "lineNumber")]
            line_number: Option<Integer>,
            /**
             * Location in the source code.
            */
            #[serde(rename = "columnNumber")]
            column_number: Option<Integer>,
        }

        /**
         * Location range within one script.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct LocationRange {
            /**
             * Location range within one script.
            */
            #[serde(rename = "scriptId")]
            scriptid: Option<runtime::ScriptId>,
            /**
             * Location range within one script.
            */
            start: Option<ScriptPosition>,
            /**
             * Location range within one script.
            */
            end: Option<ScriptPosition>,
        }

        /**
         * JavaScript call frame. Array of call frames form the call stack.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct CallFrame {
            /**
             * JavaScript call frame. Array of call frames form the call stack.
            */
            #[serde(rename = "callFrameId")]
            call_frame_id: Option<CallFrameId>,
            /**
             * JavaScript call frame. Array of call frames form the call stack.
            */
            #[serde(rename = "functionName")]
            function_name: Option<String>,
            /**
             * JavaScript call frame. Array of call frames form the call stack.
            */
            #[serde(rename = "functionLocation")]
            function_location: Location,
            /**
             * JavaScript call frame. Array of call frames form the call stack.
            */
            location: Option<Location>,
            /**
             * JavaScript call frame. Array of call frames form the call stack.
            */
            url: Option<String>,
            /**
             * JavaScript call frame. Array of call frames form the call stack.
            */
            #[serde(rename = "scopeChain")]
            scope_chain: Option<Vec<Scope>>,
            /**
             * JavaScript call frame. Array of call frames form the call stack.
            */
            this: Option<runtime::RemoteObject>,
            /**
             * JavaScript call frame. Array of call frames form the call stack.
            */
            #[serde(rename = "returnValue")]
            return_value: runtime::RemoteObject,
            /**
             * JavaScript call frame. Array of call frames form the call stack.
            */
            #[serde(rename = "canBeRestarted")]
            can_be_restarted: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum ScopeType {
            #[serde(rename = "global")]
            Global,

            #[serde(rename = "local")]
            Local,

            #[serde(rename = "with")]
            With,

            #[serde(rename = "closure")]
            Closure,

            #[serde(rename = "catch")]
            Catch,

            #[serde(rename = "block")]
            Block,

            #[serde(rename = "script")]
            Script,

            #[serde(rename = "eval")]
            Eval,

            #[serde(rename = "module")]
            Module,

            #[serde(rename = "wasm-expression-stack")]
            WasmExpressionStack,
        }

        /**
         * Scope description.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Scope {
            /**
             * Scope description.
            */
            #[serde(rename = "type")]
            r#type: Option<ScopeType>,
            /**
             * Scope description.
            */
            object: Option<runtime::RemoteObject>,
            /**
             * Scope description.
            */
            name: String,
            /**
             * Scope description.
            */
            #[serde(rename = "startLocation")]
            startlocation: Location,
            /**
             * Scope description.
            */
            #[serde(rename = "endLocation")]
            endlocation: Location,
        }

        /**
         * Search match for resource.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SearchMatch {
            /**
             * Search match for resource.
            */
            #[serde(rename = "lineNumber")]
            line_number: Option<f64>,
            /**
             * Search match for resource.
            */
            #[serde(rename = "lineContent")]
            line_content: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum BreakLocationType {
            #[serde(rename = "debuggerStatement")]
            DebuggerStatement,

            #[serde(rename = "call")]
            Call,

            #[serde(rename = "return")]
            Return,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct BreakLocation {
            #[serde(rename = "scriptId")]
            script_id: Option<runtime::ScriptId>,

            #[serde(rename = "lineNumber")]
            line_number: Option<Integer>,

            #[serde(rename = "columnNumber")]
            column_number: Integer,

            #[serde(rename = "type")]
            r#type: BreakLocationType,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct WasmDisassemblyChunk {
            lines: Option<Vec<String>>,

            #[serde(rename = "bytecodeOffsets")]
            bytecodeoffsets: Option<Vec<Integer>>,
        }

        /**
         * Enum of possible script languages.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum ScriptLanguage {
            #[serde(rename = "JavaScript")]
            JavaScript,
            #[serde(rename = "WebAssembly")]
            WebAssembly,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum DebugSymbolsType {
            #[serde(rename = "None")]
            None,

            #[serde(rename = "SourceMap")]
            SourceMap,

            #[serde(rename = "EmbeddedDWARF")]
            EmbeddedDWARF,

            #[serde(rename = "ExternalDWARF")]
            ExternalDWARF,
        }

        /**
         * Debug symbols available for a wasm script.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct DebugSymbols {
            /**
             * Debug symbols available for a wasm script.
            */
            #[serde(rename = "type")]
            r#type: Option<DebugSymbolsType>,
            /**
             * Debug symbols available for a wasm script.
            */
            #[serde(rename = "externalURL")]
            externalurl: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum ContinueToLocationRequestTargetCallFrames {
            #[serde(rename = "any")]
            Any,

            #[serde(rename = "current")]
            Current,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ContinueToLocationRequest {
            location: Option<Location>,

            #[serde(rename = "targetCallFrames")]
            targetcallframes: ContinueToLocationRequestTargetCallFrames,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct EnableRequest {
            #[serde(rename = "maxScriptsCacheSize")]
            max_scripts_cache_size: f64,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct EnableResponse {
            #[serde(rename = "debuggerId")]
            debugger_id: Option<runtime::UniqueDebuggerId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvaluateOnCallFrameRequest {
            #[serde(rename = "callFrameId")]
            call_frame_id: Option<CallFrameId>,

            expression: Option<String>,

            #[serde(rename = "objectGroup")]
            object_group: String,

            #[serde(rename = "includeCommandLineAPI")]
            include_command_line_api: bool,

            silent: bool,

            #[serde(rename = "returnByValue")]
            return_by_value: bool,

            #[serde(rename = "generatePreview")]
            generate_preview: bool,

            #[serde(rename = "throwOnSideEffect")]
            throw_on_side_effect: bool,

            timeout: runtime::TimeDelta,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvaluateOnCallFrameResponse {
            result: Option<runtime::RemoteObject>,

            #[serde(rename = "exceptionDetails")]
            exceptiondetails: runtime::ExceptionDetails,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetPossibleBreakpointsRequest {
            start: Option<Location>,

            end: Location,

            #[serde(rename = "restrictToFunction")]
            restricttofunction: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetPossibleBreakpointsResponse {
            locations: Option<Vec<BreakLocation>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetScriptSourceRequest {
            #[serde(rename = "scriptId")]
            script_id: Option<runtime::ScriptId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetScriptSourceResponse {
            #[serde(rename = "scriptSource")]
            script_source: Option<String>,

            bytecode: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct DisassembleWasmModuleRequest {
            #[serde(rename = "scriptId")]
            script_id: Option<runtime::ScriptId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct DisassembleWasmModuleResponse {
            #[serde(rename = "streamId")]
            stream_id: String,

            #[serde(rename = "totalNumberOfLines")]
            total_number_of_lines: Option<Integer>,

            #[serde(rename = "functionBodyOffsets")]
            function_body_offsets: Option<Vec<Integer>>,

            chunk: Option<WasmDisassemblyChunk>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct NextWasmDisassemblyChunkRequest {
            #[serde(rename = "streamId")]
            stream_id: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct NextWasmDisassemblyChunkResponse {
            chunk: Option<WasmDisassemblyChunk>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetWasmBytecodeRequest {
            #[serde(rename = "scriptId")]
            script_id: Option<runtime::ScriptId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetWasmBytecodeResponse {
            bytecode: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetStackTraceRequest {
            #[serde(rename = "stackTraceId")]
            stack_trace_id: Option<runtime::StackTraceId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetStackTraceResponse {
            #[serde(rename = "stackTrace")]
            stack_trace: Option<runtime::StackTrace>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct PauseOnAsyncCallRequest {
            #[serde(rename = "parentStackTraceId")]
            parent_stack_trace_id: Option<runtime::StackTraceId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct RemoveBreakpointRequest {
            #[serde(rename = "breakpointId")]
            breakpoint_id: Option<BreakpointId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum RestartFrameRequestMode {
            #[serde(rename = "StepInto")]
            StepInto,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct RestartFrameRequest {
            #[serde(rename = "callFrameId")]
            call_frame_id: Option<CallFrameId>,

            mode: RestartFrameRequestMode,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct RestartFrameResponse {
            #[serde(rename = "callFrames")]
            call_frames: Option<Vec<CallFrame>>,

            #[serde(rename = "asyncStackTrace")]
            async_stack_trace: runtime::StackTrace,

            #[serde(rename = "asyncStackTraceId")]
            async_stack_trace_id: runtime::StackTraceId,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ResumeRequest {
            #[serde(rename = "terminateOnResume")]
            terminate_on_resume: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SearchInContentRequest {
            #[serde(rename = "scriptId")]
            script_id: Option<runtime::ScriptId>,

            query: Option<String>,

            #[serde(rename = "caseSensitive")]
            case_sensitive: bool,

            #[serde(rename = "isRegex")]
            is_regex: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SearchInContentResponse {
            result: Option<Vec<SearchMatch>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetAsyncCallStackDepthRequest {
            #[serde(rename = "maxDepth")]
            max_depth: Option<Integer>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetBlackboxPatternsRequest {
            patterns: Option<Vec<String>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetBlackboxedRangesRequest {
            #[serde(rename = "scriptId")]
            script_id: Option<runtime::ScriptId>,

            positions: Option<Vec<ScriptPosition>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetBreakpointRequest {
            location: Option<Location>,

            condition: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetBreakpointResponse {
            #[serde(rename = "breakpointId")]
            breakpoint_id: Option<BreakpointId>,

            #[serde(rename = "actualLocation")]
            actual_location: Option<Location>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum SetInstrumentationBreakpointRequestInstrumentation {
            #[serde(rename = "beforeScriptExecution")]
            BeforeScriptExecution,

            #[serde(rename = "beforeScriptWithSourceMapExecution")]
            BeforeScriptWithSourceMapExecution,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetInstrumentationBreakpointRequest {
            instrumentation: Option<SetInstrumentationBreakpointRequestInstrumentation>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetInstrumentationBreakpointResponse {
            #[serde(rename = "breakpointId")]
            breakpoint_id: Option<BreakpointId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetBreakpointByUrlRequest {
            #[serde(rename = "lineNumber")]
            line_number: Option<Integer>,

            url: String,

            #[serde(rename = "urlRegex")]
            url_regex: String,

            #[serde(rename = "scriptHash")]
            script_hash: String,

            #[serde(rename = "columnNumber")]
            column_number: Integer,

            condition: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetBreakpointByUrlResponse {
            #[serde(rename = "breakpointId")]
            breakpoint_id: Option<BreakpointId>,

            locations: Option<Vec<Location>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetBreakpointOnFunctionCallRequest {
            #[serde(rename = "objectId")]
            object_id: Option<runtime::RemoteObjectId>,

            condition: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetBreakpointOnFunctionCallResponse {
            #[serde(rename = "breakpointId")]
            breakpoint_id: Option<BreakpointId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetBreakpointsActiveRequest {
            active: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum SetPauseOnExceptionsRequestState {
            #[serde(rename = "none")]
            None,

            #[serde(rename = "caught")]
            Caught,

            #[serde(rename = "uncaught")]
            Uncaught,

            #[serde(rename = "all")]
            All,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetPauseOnExceptionsRequest {
            state: Option<SetPauseOnExceptionsRequestState>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetReturnValueRequest {
            #[serde(rename = "newValue")]
            new_value: Option<runtime::CallArgument>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum SetScriptSourceResponseStatus {
            #[serde(rename = "Ok")]
            Ok,

            #[serde(rename = "CompileError")]
            CompileError,

            #[serde(rename = "BlockedByActiveGenerator")]
            BlockedByActiveGenerator,

            #[serde(rename = "BlockedByActiveFunction")]
            BlockedByActiveFunction,

            #[serde(rename = "BlockedByTopLevelEsModuleChange")]
            BlockedByTopLevelEsModuleChange,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetScriptSourceRequest {
            #[serde(rename = "scriptId")]
            script_id: Option<runtime::ScriptId>,

            #[serde(rename = "scriptSource")]
            script_source: Option<String>,

            #[serde(rename = "dryRun")]
            dry_run: bool,

            #[serde(rename = "allowTopFrameEditing")]
            allow_top_frame_editing: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetScriptSourceResponse {
            #[serde(rename = "callFrames")]
            call_frames: Vec<CallFrame>,

            #[serde(rename = "stackChanged")]
            stack_changed: bool,

            #[serde(rename = "asyncStackTrace")]
            async_stack_trace: runtime::StackTrace,

            #[serde(rename = "asyncStackTraceId")]
            async_stack_trace_id: runtime::StackTraceId,

            status: Option<SetScriptSourceResponseStatus>,

            #[serde(rename = "exceptionDetails")]
            exception_details: runtime::ExceptionDetails,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetSkipAllPausesRequest {
            skip: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetVariableValueRequest {
            #[serde(rename = "scopeNumber")]
            scope_number: Option<Integer>,

            #[serde(rename = "variableName")]
            variable_name: Option<String>,

            #[serde(rename = "newValue")]
            new_value: Option<runtime::CallArgument>,

            #[serde(rename = "callFrameId")]
            call_frame_id: Option<CallFrameId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct StepIntoRequest {
            #[serde(rename = "breakOnAsyncCall")]
            break_on_async_call: bool,

            #[serde(rename = "skipList")]
            skip_list: Vec<LocationRange>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct StepOverRequest {
            #[serde(rename = "skipList")]
            skip_list: Vec<LocationRange>,
        }

        /**
         * Fired when breakpoint is resolved to an actual script and location.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct BreakpointResolvedEvent {
            /**
             * Fired when breakpoint is resolved to an actual script and location.
            */
            #[serde(rename = "breakpointId")]
            breakpoint_id: Option<BreakpointId>,
            /**
             * Fired when breakpoint is resolved to an actual script and location.
            */
            location: Option<Location>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum PausedEventReason {
            #[serde(rename = "ambiguous")]
            Ambiguous,

            #[serde(rename = "assert")]
            Assert,

            #[serde(rename = "CSPViolation")]
            CSPViolation,

            #[serde(rename = "debugCommand")]
            DebugCommand,

            #[serde(rename = "DOM")]
            DOM,

            #[serde(rename = "EventListener")]
            EventListener,

            #[serde(rename = "exception")]
            Exception,

            #[serde(rename = "instrumentation")]
            Instrumentation,

            #[serde(rename = "OOM")]
            OOM,

            #[serde(rename = "other")]
            Other,

            #[serde(rename = "promiseRejection")]
            PromiseRejection,

            #[serde(rename = "XHR")]
            XHR,

            #[serde(rename = "step")]
            Step,
        }

        /**
         * Fired when the virtual machine stopped on breakpoint or exception or any other stop criteria.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PausedEvent {
            /**
             * Fired when the virtual machine stopped on breakpoint or exception or any other stop criteria.
            */
            #[serde(rename = "callFrames")]
            call_frames: Option<Vec<CallFrame>>,
            /**
             * Fired when the virtual machine stopped on breakpoint or exception or any other stop criteria.
            */
            reason: Option<PausedEventReason>,
            /**
             * Fired when the virtual machine stopped on breakpoint or exception or any other stop criteria.
            */
            data: serde_json::Value,
            /**
             * Fired when the virtual machine stopped on breakpoint or exception or any other stop criteria.
            */
            #[serde(rename = "hitBreakpoints")]
            hit_breakpoints: Vec<String>,
            /**
             * Fired when the virtual machine stopped on breakpoint or exception or any other stop criteria.
            */
            #[serde(rename = "asyncStackTrace")]
            async_stack_trace: runtime::StackTrace,
            /**
             * Fired when the virtual machine stopped on breakpoint or exception or any other stop criteria.
            */
            #[serde(rename = "asyncStackTraceId")]
            async_stack_trace_id: runtime::StackTraceId,
            /**
             * Fired when the virtual machine stopped on breakpoint or exception or any other stop criteria.
            */
            #[serde(rename = "asyncCallStackTraceId")]
            async_call_stack_trace_id: runtime::StackTraceId,
        }

        /**
         * Fired when virtual machine fails to parse the script.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ScriptFailedToParseEvent {
            /**
             * Fired when virtual machine fails to parse the script.
            */
            #[serde(rename = "scriptId")]
            script_id: Option<runtime::ScriptId>,
            /**
             * Fired when virtual machine fails to parse the script.
            */
            url: Option<String>,
            /**
             * Fired when virtual machine fails to parse the script.
            */
            #[serde(rename = "startLine")]
            start_line: Option<Integer>,
            /**
             * Fired when virtual machine fails to parse the script.
            */
            #[serde(rename = "startColumn")]
            start_column: Option<Integer>,
            /**
             * Fired when virtual machine fails to parse the script.
            */
            #[serde(rename = "endLine")]
            end_line: Option<Integer>,
            /**
             * Fired when virtual machine fails to parse the script.
            */
            #[serde(rename = "endColumn")]
            end_column: Option<Integer>,
            /**
             * Fired when virtual machine fails to parse the script.
            */
            #[serde(rename = "executionContextId")]
            execution_context_id: Option<runtime::ExecutionContextId>,
            /**
             * Fired when virtual machine fails to parse the script.
            */
            hash: Option<String>,
            /**
             * Fired when virtual machine fails to parse the script.
            */
            #[serde(rename = "executionContextAuxData")]
            execution_context_aux_data: serde_json::Value,
            /**
             * Fired when virtual machine fails to parse the script.
            */
            #[serde(rename = "sourceMapURL")]
            source_map_url: String,
            /**
             * Fired when virtual machine fails to parse the script.
            */
            #[serde(rename = "hasSourceURL")]
            has_source_url: bool,
            /**
             * Fired when virtual machine fails to parse the script.
            */
            #[serde(rename = "isModule")]
            is_module: bool,
            /**
             * Fired when virtual machine fails to parse the script.
            */
            length: Integer,
            /**
             * Fired when virtual machine fails to parse the script.
            */
            #[serde(rename = "stackTrace")]
            stack_trace: runtime::StackTrace,
            /**
             * Fired when virtual machine fails to parse the script.
            */
            #[serde(rename = "codeOffset")]
            code_offset: Integer,
            /**
             * Fired when virtual machine fails to parse the script.
            */
            #[serde(rename = "scriptLanguage")]
            script_language: debugger::ScriptLanguage,
            /**
             * Fired when virtual machine fails to parse the script.
            */
            #[serde(rename = "embedderName")]
            embedder_name: String,
        }

        /**
         * Fired when virtual machine parses script. This event is also fired for all known and uncollected
         * scripts upon enabling debugger.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ScriptParsedEvent {
            /**
             * Fired when virtual machine parses script. This event is also fired for all known and uncollected
             * scripts upon enabling debugger.
            */
            #[serde(rename = "scriptId")]
            script_id: Option<runtime::ScriptId>,
            /**
             * Fired when virtual machine parses script. This event is also fired for all known and uncollected
             * scripts upon enabling debugger.
            */
            url: Option<String>,
            /**
             * Fired when virtual machine parses script. This event is also fired for all known and uncollected
             * scripts upon enabling debugger.
            */
            #[serde(rename = "startLine")]
            start_line: Option<Integer>,
            /**
             * Fired when virtual machine parses script. This event is also fired for all known and uncollected
             * scripts upon enabling debugger.
            */
            #[serde(rename = "startColumn")]
            start_column: Option<Integer>,
            /**
             * Fired when virtual machine parses script. This event is also fired for all known and uncollected
             * scripts upon enabling debugger.
            */
            #[serde(rename = "endLine")]
            end_line: Option<Integer>,
            /**
             * Fired when virtual machine parses script. This event is also fired for all known and uncollected
             * scripts upon enabling debugger.
            */
            #[serde(rename = "endColumn")]
            end_column: Option<Integer>,
            /**
             * Fired when virtual machine parses script. This event is also fired for all known and uncollected
             * scripts upon enabling debugger.
            */
            #[serde(rename = "executionContextId")]
            execution_context_id: Option<runtime::ExecutionContextId>,
            /**
             * Fired when virtual machine parses script. This event is also fired for all known and uncollected
             * scripts upon enabling debugger.
            */
            hash: Option<String>,
            /**
             * Fired when virtual machine parses script. This event is also fired for all known and uncollected
             * scripts upon enabling debugger.
            */
            #[serde(rename = "executionContextAuxData")]
            execution_context_aux_data: serde_json::Value,
            /**
             * Fired when virtual machine parses script. This event is also fired for all known and uncollected
             * scripts upon enabling debugger.
            */
            #[serde(rename = "isLiveEdit")]
            is_live_edit: bool,
            /**
             * Fired when virtual machine parses script. This event is also fired for all known and uncollected
             * scripts upon enabling debugger.
            */
            #[serde(rename = "sourceMapURL")]
            source_map_url: String,
            /**
             * Fired when virtual machine parses script. This event is also fired for all known and uncollected
             * scripts upon enabling debugger.
            */
            #[serde(rename = "hasSourceURL")]
            has_source_url: bool,
            /**
             * Fired when virtual machine parses script. This event is also fired for all known and uncollected
             * scripts upon enabling debugger.
            */
            #[serde(rename = "isModule")]
            is_module: bool,
            /**
             * Fired when virtual machine parses script. This event is also fired for all known and uncollected
             * scripts upon enabling debugger.
            */
            length: Integer,
            /**
             * Fired when virtual machine parses script. This event is also fired for all known and uncollected
             * scripts upon enabling debugger.
            */
            #[serde(rename = "stackTrace")]
            stack_trace: runtime::StackTrace,
            /**
             * Fired when virtual machine parses script. This event is also fired for all known and uncollected
             * scripts upon enabling debugger.
            */
            #[serde(rename = "codeOffset")]
            code_offset: Integer,
            /**
             * Fired when virtual machine parses script. This event is also fired for all known and uncollected
             * scripts upon enabling debugger.
            */
            #[serde(rename = "scriptLanguage")]
            script_language: debugger::ScriptLanguage,
            /**
             * Fired when virtual machine parses script. This event is also fired for all known and uncollected
             * scripts upon enabling debugger.
            */
            #[serde(rename = "debugSymbols")]
            debug_symbols: debugger::DebugSymbols,
            /**
             * Fired when virtual machine parses script. This event is also fired for all known and uncollected
             * scripts upon enabling debugger.
            */
            #[serde(rename = "embedderName")]
            embedder_name: String,
        }
    }

    pub mod heap_profiler {
        use super::{runtime, Integer};
        use serde::{self, Deserialize, Serialize};
        /**
         * Heap snapshot object id.
        */
        pub type HeapSnapshotObjectId = String;

        /**
         * Sampling Heap Profile node. Holds callsite information, allocation statistics and child nodes.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SamplingHeapProfileNode {
            /**
             * Sampling Heap Profile node. Holds callsite information, allocation statistics and child nodes.
            */
            #[serde(rename = "callFrame")]
            call_frame: Option<runtime::CallFrame>,
            /**
             * Sampling Heap Profile node. Holds callsite information, allocation statistics and child nodes.
            */
            #[serde(rename = "selfSize")]
            self_size: Option<f64>,
            /**
             * Sampling Heap Profile node. Holds callsite information, allocation statistics and child nodes.
            */
            id: Option<Integer>,
            /**
             * Sampling Heap Profile node. Holds callsite information, allocation statistics and child nodes.
            */
            children: Option<Vec<SamplingHeapProfileNode>>,
        }

        /**
         * A single sample from a sampling profile.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SamplingHeapProfileSample {
            /**
             * A single sample from a sampling profile.
            */
            size: Option<f64>,
            /**
             * A single sample from a sampling profile.
            */
            #[serde(rename = "nodeId")]
            nodeid: Option<Integer>,
            /**
             * A single sample from a sampling profile.
            */
            ordinal: Option<f64>,
        }

        /**
         * Sampling profile.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SamplingHeapProfile {
            /**
             * Sampling profile.
            */
            head: Option<SamplingHeapProfileNode>,
            /**
             * Sampling profile.
            */
            samples: Option<Vec<SamplingHeapProfileSample>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct AddInspectedHeapObjectRequest {
            #[serde(rename = "heapObjectId")]
            heap_object_id: Option<HeapSnapshotObjectId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetHeapObjectIdRequest {
            #[serde(rename = "objectId")]
            object_id: Option<runtime::RemoteObjectId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetHeapObjectIdResponse {
            #[serde(rename = "heapSnapshotObjectId")]
            heap_snapshot_object_id: Option<HeapSnapshotObjectId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetObjectByHeapObjectIdRequest {
            #[serde(rename = "objectId")]
            object_id: Option<HeapSnapshotObjectId>,

            #[serde(rename = "objectGroup")]
            object_group: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetObjectByHeapObjectIdResponse {
            result: Option<runtime::RemoteObject>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetSamplingProfileResponse {
            profile: Option<SamplingHeapProfile>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct StartSamplingRequest {
            #[serde(rename = "samplingInterval")]
            sampling_interval: f64,

            #[serde(rename = "includeObjectsCollectedByMajorGC")]
            include_objects_collected_by_major_gc: bool,

            #[serde(rename = "includeObjectsCollectedByMinorGC")]
            include_objects_collected_by_minor_gc: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct StartTrackingHeapObjectsRequest {
            #[serde(rename = "trackAllocations")]
            track_allocations: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct StopSamplingResponse {
            profile: Option<SamplingHeapProfile>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct StopTrackingHeapObjectsRequest {
            #[serde(rename = "reportProgress")]
            report_progress: bool,

            #[serde(rename = "treatGlobalObjectsAsRoots")]
            treat_global_objects_as_roots: bool,

            #[serde(rename = "captureNumericValue")]
            capture_numeric_value: bool,

            #[serde(rename = "exposeInternals")]
            expose_internals: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct TakeHeapSnapshotRequest {
            #[serde(rename = "reportProgress")]
            report_progress: bool,

            #[serde(rename = "treatGlobalObjectsAsRoots")]
            treat_global_objects_as_roots: bool,

            #[serde(rename = "captureNumericValue")]
            capture_numeric_value: bool,

            #[serde(rename = "exposeInternals")]
            expose_internals: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct AddHeapSnapshotChunkEvent {
            chunk: Option<String>,
        }

        /**
         * If heap objects tracking has been started then backend may send update for one or more fragments
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct HeapStatsUpdateEvent {
            /**
             * If heap objects tracking has been started then backend may send update for one or more fragments
            */
            #[serde(rename = "statsUpdate")]
            stats_update: Option<Vec<Integer>>,
        }

        /**
         * If heap objects tracking has been started then backend regularly sends a current value for last
         * seen object id and corresponding timestamp. If the were changes in the heap since last event
         * then one or more heapStatsUpdate events will be sent before a new lastSeenObjectId event.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct LastSeenObjectIdEvent {
            /**
             * If heap objects tracking has been started then backend regularly sends a current value for last
             * seen object id and corresponding timestamp. If the were changes in the heap since last event
             * then one or more heapStatsUpdate events will be sent before a new lastSeenObjectId event.
            */
            #[serde(rename = "lastSeenObjectId")]
            last_seen_object_id: Option<Integer>,
            /**
             * If heap objects tracking has been started then backend regularly sends a current value for last
             * seen object id and corresponding timestamp. If the were changes in the heap since last event
             * then one or more heapStatsUpdate events will be sent before a new lastSeenObjectId event.
            */
            timestamp: Option<f64>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ReportHeapSnapshotProgressEvent {
            done: Option<Integer>,

            total: Option<Integer>,

            finished: bool,
        }
    }

    pub mod profiler {
        use super::{debugger, runtime, Integer};
        use serde::{self, Deserialize, Serialize};
        /**
         * Profile node. Holds callsite information, execution statistics and child nodes.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ProfileNode {
            /**
             * Profile node. Holds callsite information, execution statistics and child nodes.
            */
            id: Option<Integer>,
            /**
             * Profile node. Holds callsite information, execution statistics and child nodes.
            */
            #[serde(rename = "callFrame")]
            call_frame: Option<runtime::CallFrame>,
            /**
             * Profile node. Holds callsite information, execution statistics and child nodes.
            */
            #[serde(rename = "hitCount")]
            hit_count: Integer,
            /**
             * Profile node. Holds callsite information, execution statistics and child nodes.
            */
            children: Vec<Integer>,
            /**
             * Profile node. Holds callsite information, execution statistics and child nodes.
            */
            #[serde(rename = "deoptReason")]
            deopt_reason: String,
            /**
             * Profile node. Holds callsite information, execution statistics and child nodes.
            */
            #[serde(rename = "positionTicks")]
            position_ticks: Vec<PositionTickInfo>,
        }

        /**
         * Profile.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Profile {
            /**
             * Profile.
            */
            nodes: Option<Vec<ProfileNode>>,
            /**
             * Profile.
            */
            #[serde(rename = "startTime")]
            start_time: Option<f64>,
            /**
             * Profile.
            */
            #[serde(rename = "endTime")]
            end_time: Option<f64>,
            /**
             * Profile.
            */
            samples: Vec<Integer>,
            /**
             * Profile.
            */
            #[serde(rename = "timeDeltas")]
            time_deltas: Vec<Integer>,
        }

        /**
         * Specifies a number of samples attributed to a certain source position.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PositionTickInfo {
            /**
             * Specifies a number of samples attributed to a certain source position.
            */
            line: Option<Integer>,
            /**
             * Specifies a number of samples attributed to a certain source position.
            */
            ticks: Option<Integer>,
        }

        /**
         * Coverage data for a source range.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct CoverageRange {
            /**
             * Coverage data for a source range.
            */
            #[serde(rename = "startOffset")]
            start_offset: Option<Integer>,
            /**
             * Coverage data for a source range.
            */
            #[serde(rename = "endOffset")]
            end_offset: Option<Integer>,
            /**
             * Coverage data for a source range.
            */
            count: Option<Integer>,
        }

        /**
         * Coverage data for a JavaScript function.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FunctionCoverage {
            /**
             * Coverage data for a JavaScript function.
            */
            #[serde(rename = "functionName")]
            function_name: Option<String>,
            /**
             * Coverage data for a JavaScript function.
            */
            ranges: Option<Vec<CoverageRange>>,
            /**
             * Coverage data for a JavaScript function.
            */
            #[serde(rename = "isBlockCoverage")]
            is_block_coverage: Option<bool>,
        }

        /**
         * Coverage data for a JavaScript script.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ScriptCoverage {
            /**
             * Coverage data for a JavaScript script.
            */
            #[serde(rename = "scriptId")]
            scriptid: Option<runtime::ScriptId>,
            /**
             * Coverage data for a JavaScript script.
            */
            url: Option<String>,
            /**
             * Coverage data for a JavaScript script.
            */
            functions: Option<Vec<FunctionCoverage>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetBestEffortCoverageResponse {
            result: Option<Vec<ScriptCoverage>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetSamplingIntervalRequest {
            interval: Option<Integer>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct StartPreciseCoverageRequest {
            #[serde(rename = "callCount")]
            call_count: bool,

            detailed: bool,

            #[serde(rename = "allowTriggeredUpdates")]
            allow_triggered_updates: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct StartPreciseCoverageResponse {
            timestamp: Option<f64>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct StopResponse {
            profile: Option<Profile>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct TakePreciseCoverageResponse {
            result: Option<Vec<ScriptCoverage>>,

            timestamp: Option<f64>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ConsoleProfileFinishedEvent {
            id: Option<String>,

            location: Option<debugger::Location>,

            profile: Option<Profile>,

            title: String,
        }

        /**
         * Sent when new profile recording is started using console.profile() call.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ConsoleProfileStartedEvent {
            /**
             * Sent when new profile recording is started using console.profile() call.
            */
            id: Option<String>,
            /**
             * Sent when new profile recording is started using console.profile() call.
            */
            location: Option<debugger::Location>,
            /**
             * Sent when new profile recording is started using console.profile() call.
            */
            title: String,
        }

        /**
         * Reports coverage delta since the last poll (either from an event like this, or from
         * `takePreciseCoverage` for the current isolate. May only be sent if precise code
         * coverage has been started. This event can be trigged by the embedder to, for example,
         * trigger collection of coverage data immediately at a certain point in time.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PreciseCoverageDeltaUpdateEvent {
            /**
             * Reports coverage delta since the last poll (either from an event like this, or from
             * `takePreciseCoverage` for the current isolate. May only be sent if precise code
             * coverage has been started. This event can be trigged by the embedder to, for example,
             * trigger collection of coverage data immediately at a certain point in time.
            */
            timestamp: Option<f64>,
            /**
             * Reports coverage delta since the last poll (either from an event like this, or from
             * `takePreciseCoverage` for the current isolate. May only be sent if precise code
             * coverage has been started. This event can be trigged by the embedder to, for example,
             * trigger collection of coverage data immediately at a certain point in time.
            */
            occasion: Option<String>,
            /**
             * Reports coverage delta since the last poll (either from an event like this, or from
             * `takePreciseCoverage` for the current isolate. May only be sent if precise code
             * coverage has been started. This event can be trigged by the embedder to, for example,
             * trigger collection of coverage data immediately at a certain point in time.
            */
            result: Option<Vec<ScriptCoverage>>,
        }
    }

    /**
     * Runtime domain exposes JavaScript runtime by means of remote evaluation and mirror objects.
     * Evaluation results are returned as mirror object that expose object type, string representation
     * and unique identifier that can be used for further object reference. Original objects are
     * maintained in memory unless they are either explicitly released or are released along with the
     * other objects in their object group.
    */
    pub mod runtime {
        use super::Integer;
        use serde::{self, Deserialize, Serialize};
        /**
         * Unique script identifier.
        */
        pub type ScriptId = String;

        #[derive(Debug, Serialize, Deserialize)]
        pub enum SerializationOptionsSerialization {
            #[serde(rename = "deep")]
            Deep,

            #[serde(rename = "json")]
            Json,

            #[serde(rename = "idOnly")]
            IdOnly,
        }

        /**
         * Represents options for serialization. Overrides `generatePreview`, `returnByValue` and
         * `generateWebDriverValue`.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SerializationOptions {
            /**
             * Represents options for serialization. Overrides `generatePreview`, `returnByValue` and
             * `generateWebDriverValue`.
            */
            serialization: Option<SerializationOptionsSerialization>,
            /**
             * Represents options for serialization. Overrides `generatePreview`, `returnByValue` and
             * `generateWebDriverValue`.
            */
            #[serde(rename = "maxDepth")]
            max_depth: Integer,
            /**
             * Represents options for serialization. Overrides `generatePreview`, `returnByValue` and
             * `generateWebDriverValue`.
            */
            #[serde(rename = "additionalParameters")]
            additional_parameters: serde_json::Value,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum DeepSerializedValueType {
            #[serde(rename = "undefined")]
            Undefined,

            #[serde(rename = "null")]
            Null,

            #[serde(rename = "string")]
            String,

            #[serde(rename = "number")]
            Number,

            #[serde(rename = "boolean")]
            Boolean,

            #[serde(rename = "bigint")]
            Bigint,

            #[serde(rename = "regexp")]
            Regexp,

            #[serde(rename = "date")]
            Date,

            #[serde(rename = "symbol")]
            Symbol,

            #[serde(rename = "array")]
            Array,

            #[serde(rename = "object")]
            Object,

            #[serde(rename = "function")]
            Function,

            #[serde(rename = "map")]
            Map,

            #[serde(rename = "set")]
            Set,

            #[serde(rename = "weakmap")]
            Weakmap,

            #[serde(rename = "weakset")]
            Weakset,

            #[serde(rename = "error")]
            Error,

            #[serde(rename = "proxy")]
            Proxy,

            #[serde(rename = "promise")]
            Promise,

            #[serde(rename = "typedarray")]
            Typedarray,

            #[serde(rename = "arraybuffer")]
            Arraybuffer,

            #[serde(rename = "node")]
            Node,

            #[serde(rename = "window")]
            Window,
        }

        /**
         * Represents deep serialized value.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct DeepSerializedValue {
            /**
             * Represents deep serialized value.
            */
            #[serde(rename = "type")]
            r#type: Option<DeepSerializedValueType>,
            /**
             * Represents deep serialized value.
            */
            value: serde_json::Value,
            /**
             * Represents deep serialized value.
            */
            #[serde(rename = "objectId")]
            objectid: String,
            /**
             * Represents deep serialized value.
            */
            #[serde(rename = "weakLocalObjectReference")]
            weaklocalobjectreference: Integer,
        }

        /**
         * Unique object identifier.
        */
        pub type RemoteObjectId = String;

        /**
         * Primitive value which cannot be JSON-stringified. Includes values `-0`, `NaN`, `Infinity`,
         * `-Infinity`, and bigint literals.
        */
        pub type UnserializableValue = String;

        #[derive(Debug, Serialize, Deserialize)]
        pub enum RemoteObjectType {
            #[serde(rename = "object")]
            Object,

            #[serde(rename = "function")]
            Function,

            #[serde(rename = "undefined")]
            Undefined,

            #[serde(rename = "string")]
            String,

            #[serde(rename = "number")]
            Number,

            #[serde(rename = "boolean")]
            Boolean,

            #[serde(rename = "symbol")]
            Symbol,

            #[serde(rename = "bigint")]
            Bigint,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum RemoteObjectSubtype {
            #[serde(rename = "array")]
            Array,

            #[serde(rename = "null")]
            Null,

            #[serde(rename = "node")]
            Node,

            #[serde(rename = "regexp")]
            Regexp,

            #[serde(rename = "date")]
            Date,

            #[serde(rename = "map")]
            Map,

            #[serde(rename = "set")]
            Set,

            #[serde(rename = "weakmap")]
            Weakmap,

            #[serde(rename = "weakset")]
            Weakset,

            #[serde(rename = "iterator")]
            Iterator,

            #[serde(rename = "generator")]
            Generator,

            #[serde(rename = "error")]
            Error,

            #[serde(rename = "proxy")]
            Proxy,

            #[serde(rename = "promise")]
            Promise,

            #[serde(rename = "typedarray")]
            Typedarray,

            #[serde(rename = "arraybuffer")]
            Arraybuffer,

            #[serde(rename = "dataview")]
            Dataview,

            #[serde(rename = "webassemblymemory")]
            Webassemblymemory,

            #[serde(rename = "wasmvalue")]
            Wasmvalue,
        }

        /**
         * Mirror object referencing original JavaScript object.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct RemoteObject {
            /**
             * Mirror object referencing original JavaScript object.
            */
            #[serde(rename = "type")]
            r#type: Option<RemoteObjectType>,
            /**
             * Mirror object referencing original JavaScript object.
            */
            subtype: RemoteObjectSubtype,
            /**
             * Mirror object referencing original JavaScript object.
            */
            #[serde(rename = "className")]
            class_name: String,
            /**
             * Mirror object referencing original JavaScript object.
            */
            value: serde_json::Value,
            /**
             * Mirror object referencing original JavaScript object.
            */
            #[serde(rename = "unserializableValue")]
            unserializable_value: UnserializableValue,
            /**
             * Mirror object referencing original JavaScript object.
            */
            description: String,
            /**
             * Mirror object referencing original JavaScript object.
            */
            #[serde(rename = "webDriverValue")]
            web_driver_value: DeepSerializedValue,
            /**
             * Mirror object referencing original JavaScript object.
            */
            #[serde(rename = "deepSerializedValue")]
            deep_serialized_value: DeepSerializedValue,
            /**
             * Mirror object referencing original JavaScript object.
            */
            #[serde(rename = "objectId")]
            object_id: RemoteObjectId,
            /**
             * Mirror object referencing original JavaScript object.
            */
            preview: ObjectPreview,
            /**
             * Mirror object referencing original JavaScript object.
            */
            #[serde(rename = "customPreview")]
            custom_preview: CustomPreview,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CustomPreview {
            header: Option<String>,

            #[serde(rename = "bodyGetterId")]
            bodygetterid: RemoteObjectId,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum ObjectPreviewType {
            #[serde(rename = "object")]
            Object,

            #[serde(rename = "function")]
            Function,

            #[serde(rename = "undefined")]
            Undefined,

            #[serde(rename = "string")]
            String,

            #[serde(rename = "number")]
            Number,

            #[serde(rename = "boolean")]
            Boolean,

            #[serde(rename = "symbol")]
            Symbol,

            #[serde(rename = "bigint")]
            Bigint,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum ObjectPreviewSubtype {
            #[serde(rename = "array")]
            Array,

            #[serde(rename = "null")]
            Null,

            #[serde(rename = "node")]
            Node,

            #[serde(rename = "regexp")]
            Regexp,

            #[serde(rename = "date")]
            Date,

            #[serde(rename = "map")]
            Map,

            #[serde(rename = "set")]
            Set,

            #[serde(rename = "weakmap")]
            Weakmap,

            #[serde(rename = "weakset")]
            Weakset,

            #[serde(rename = "iterator")]
            Iterator,

            #[serde(rename = "generator")]
            Generator,

            #[serde(rename = "error")]
            Error,

            #[serde(rename = "proxy")]
            Proxy,

            #[serde(rename = "promise")]
            Promise,

            #[serde(rename = "typedarray")]
            Typedarray,

            #[serde(rename = "arraybuffer")]
            Arraybuffer,

            #[serde(rename = "dataview")]
            Dataview,

            #[serde(rename = "webassemblymemory")]
            Webassemblymemory,

            #[serde(rename = "wasmvalue")]
            Wasmvalue,
        }

        /**
         * Object containing abbreviated remote object value.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ObjectPreview {
            /**
             * Object containing abbreviated remote object value.
            */
            #[serde(rename = "type")]
            r#type: Option<ObjectPreviewType>,
            /**
             * Object containing abbreviated remote object value.
            */
            subtype: ObjectPreviewSubtype,
            /**
             * Object containing abbreviated remote object value.
            */
            description: String,
            /**
             * Object containing abbreviated remote object value.
            */
            overflow: Option<bool>,
            /**
             * Object containing abbreviated remote object value.
            */
            properties: Option<Vec<PropertyPreview>>,
            /**
             * Object containing abbreviated remote object value.
            */
            entries: Vec<EntryPreview>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum PropertyPreviewType {
            #[serde(rename = "object")]
            Object,

            #[serde(rename = "function")]
            Function,

            #[serde(rename = "undefined")]
            Undefined,

            #[serde(rename = "string")]
            String,

            #[serde(rename = "number")]
            Number,

            #[serde(rename = "boolean")]
            Boolean,

            #[serde(rename = "symbol")]
            Symbol,

            #[serde(rename = "accessor")]
            Accessor,

            #[serde(rename = "bigint")]
            Bigint,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum PropertyPreviewSubtype {
            #[serde(rename = "array")]
            Array,

            #[serde(rename = "null")]
            Null,

            #[serde(rename = "node")]
            Node,

            #[serde(rename = "regexp")]
            Regexp,

            #[serde(rename = "date")]
            Date,

            #[serde(rename = "map")]
            Map,

            #[serde(rename = "set")]
            Set,

            #[serde(rename = "weakmap")]
            Weakmap,

            #[serde(rename = "weakset")]
            Weakset,

            #[serde(rename = "iterator")]
            Iterator,

            #[serde(rename = "generator")]
            Generator,

            #[serde(rename = "error")]
            Error,

            #[serde(rename = "proxy")]
            Proxy,

            #[serde(rename = "promise")]
            Promise,

            #[serde(rename = "typedarray")]
            Typedarray,

            #[serde(rename = "arraybuffer")]
            Arraybuffer,

            #[serde(rename = "dataview")]
            Dataview,

            #[serde(rename = "webassemblymemory")]
            Webassemblymemory,

            #[serde(rename = "wasmvalue")]
            Wasmvalue,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct PropertyPreview {
            name: Option<String>,

            #[serde(rename = "type")]
            r#type: Option<PropertyPreviewType>,

            value: String,

            #[serde(rename = "valuePreview")]
            valuepreview: ObjectPreview,

            subtype: PropertyPreviewSubtype,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct EntryPreview {
            key: ObjectPreview,

            value: Option<ObjectPreview>,
        }

        /**
         * Object property descriptor.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PropertyDescriptor {
            /**
             * Object property descriptor.
            */
            name: Option<String>,
            /**
             * Object property descriptor.
            */
            value: RemoteObject,
            /**
             * Object property descriptor.
            */
            writable: bool,
            /**
             * Object property descriptor.
            */
            get: RemoteObject,
            /**
             * Object property descriptor.
            */
            set: RemoteObject,
            /**
             * Object property descriptor.
            */
            configurable: Option<bool>,
            /**
             * Object property descriptor.
            */
            enumerable: Option<bool>,
            /**
             * Object property descriptor.
            */
            #[serde(rename = "wasThrown")]
            wasthrown: bool,
            /**
             * Object property descriptor.
            */
            #[serde(rename = "isOwn")]
            isown: bool,
            /**
             * Object property descriptor.
            */
            symbol: RemoteObject,
        }

        /**
         * Object internal property descriptor. This property isn't normally visible in JavaScript code.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InternalPropertyDescriptor {
            /**
             * Object internal property descriptor. This property isn't normally visible in JavaScript code.
            */
            name: Option<String>,
            /**
             * Object internal property descriptor. This property isn't normally visible in JavaScript code.
            */
            value: RemoteObject,
        }

        /**
         * Object private field descriptor.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PrivatePropertyDescriptor {
            /**
             * Object private field descriptor.
            */
            name: Option<String>,
            /**
             * Object private field descriptor.
            */
            value: RemoteObject,
            /**
             * Object private field descriptor.
            */
            get: RemoteObject,
            /**
             * Object private field descriptor.
            */
            set: RemoteObject,
        }

        /**
         * Represents function call argument. Either remote object id `objectId`, primitive `value`,
         * unserializable primitive value or neither of (for undefined) them should be specified.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct CallArgument {
            /**
             * Represents function call argument. Either remote object id `objectId`, primitive `value`,
             * unserializable primitive value or neither of (for undefined) them should be specified.
            */
            value: serde_json::Value,
            /**
             * Represents function call argument. Either remote object id `objectId`, primitive `value`,
             * unserializable primitive value or neither of (for undefined) them should be specified.
            */
            #[serde(rename = "unserializableValue")]
            unserializable_value: UnserializableValue,
            /**
             * Represents function call argument. Either remote object id `objectId`, primitive `value`,
             * unserializable primitive value or neither of (for undefined) them should be specified.
            */
            #[serde(rename = "objectId")]
            object_id: RemoteObjectId,
        }

        /**
         * Id of an execution context.
        */
        pub type ExecutionContextId = Integer;

        /**
         * Description of an isolated world.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ExecutionContextDescription {
            /**
             * Description of an isolated world.
            */
            id: Option<ExecutionContextId>,
            /**
             * Description of an isolated world.
            */
            origin: Option<String>,
            /**
             * Description of an isolated world.
            */
            name: Option<String>,
            /**
             * Description of an isolated world.
            */
            #[serde(rename = "uniqueId")]
            uniqueid: Option<String>,
            /**
             * Description of an isolated world.
            */
            #[serde(rename = "auxData")]
            auxdata: serde_json::Value,
        }

        /**
         * Detailed information about exception (or error) that was thrown during script compilation or
         * execution.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ExceptionDetails {
            /**
             * Detailed information about exception (or error) that was thrown during script compilation or
             * execution.
            */
            #[serde(rename = "exceptionId")]
            exception_id: Option<Integer>,
            /**
             * Detailed information about exception (or error) that was thrown during script compilation or
             * execution.
            */
            text: Option<String>,
            /**
             * Detailed information about exception (or error) that was thrown during script compilation or
             * execution.
            */
            #[serde(rename = "lineNumber")]
            line_number: Option<Integer>,
            /**
             * Detailed information about exception (or error) that was thrown during script compilation or
             * execution.
            */
            #[serde(rename = "columnNumber")]
            column_number: Option<Integer>,
            /**
             * Detailed information about exception (or error) that was thrown during script compilation or
             * execution.
            */
            #[serde(rename = "scriptId")]
            script_id: ScriptId,
            /**
             * Detailed information about exception (or error) that was thrown during script compilation or
             * execution.
            */
            url: String,
            /**
             * Detailed information about exception (or error) that was thrown during script compilation or
             * execution.
            */
            #[serde(rename = "stackTrace")]
            stack_trace: StackTrace,
            /**
             * Detailed information about exception (or error) that was thrown during script compilation or
             * execution.
            */
            exception: RemoteObject,
            /**
             * Detailed information about exception (or error) that was thrown during script compilation or
             * execution.
            */
            #[serde(rename = "executionContextId")]
            execution_context_id: ExecutionContextId,
            /**
             * Detailed information about exception (or error) that was thrown during script compilation or
             * execution.
            */
            #[serde(rename = "exceptionMetaData")]
            exception_meta_data: serde_json::Value,
        }

        /**
         * Number of milliseconds since epoch.
        */
        pub type Timestamp = f64;

        /**
         * Number of milliseconds.
        */
        pub type TimeDelta = f64;

        /**
         * Stack entry for runtime errors and assertions.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct CallFrame {
            /**
             * Stack entry for runtime errors and assertions.
            */
            #[serde(rename = "functionName")]
            function_name: Option<String>,
            /**
             * Stack entry for runtime errors and assertions.
            */
            #[serde(rename = "scriptId")]
            script_id: Option<ScriptId>,
            /**
             * Stack entry for runtime errors and assertions.
            */
            url: Option<String>,
            /**
             * Stack entry for runtime errors and assertions.
            */
            #[serde(rename = "lineNumber")]
            line_number: Option<Integer>,
            /**
             * Stack entry for runtime errors and assertions.
            */
            #[serde(rename = "columnNumber")]
            column_number: Option<Integer>,
        }

        /**
         * Call frames for assertions or error messages.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct StackTrace {
            /**
             * Call frames for assertions or error messages.
            */
            description: String,
            /**
             * Call frames for assertions or error messages.
            */
            #[serde(rename = "callFrames")]
            callframes: Option<Vec<CallFrame>>,
            /**
             * Call frames for assertions or error messages.
            */
            parent: Box<StackTrace>,
            /**
             * Call frames for assertions or error messages.
            */
            #[serde(rename = "parentId")]
            parentid: StackTraceId,
        }

        /**
         * Unique identifier of current debugger.
        */
        pub type UniqueDebuggerId = String;

        /**
         * If `debuggerId` is set stack trace comes from another debugger and can be resolved there. This
         * allows to track cross-debugger calls. See `Runtime.StackTrace` and `Debugger.paused` for usages.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct StackTraceId {
            /**
             * If `debuggerId` is set stack trace comes from another debugger and can be resolved there. This
             * allows to track cross-debugger calls. See `Runtime.StackTrace` and `Debugger.paused` for usages.
            */
            id: Option<String>,
            /**
             * If `debuggerId` is set stack trace comes from another debugger and can be resolved there. This
             * allows to track cross-debugger calls. See `Runtime.StackTrace` and `Debugger.paused` for usages.
            */
            #[serde(rename = "debuggerId")]
            debuggerid: UniqueDebuggerId,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct AwaitPromiseRequest {
            #[serde(rename = "promiseObjectId")]
            promise_object_id: Option<RemoteObjectId>,

            #[serde(rename = "returnByValue")]
            return_by_value: bool,

            #[serde(rename = "generatePreview")]
            generate_preview: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct AwaitPromiseResponse {
            result: Option<RemoteObject>,

            #[serde(rename = "exceptionDetails")]
            exceptiondetails: ExceptionDetails,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CallFunctionOnRequest {
            #[serde(rename = "functionDeclaration")]
            function_declaration: Option<String>,

            #[serde(rename = "objectId")]
            object_id: RemoteObjectId,

            arguments: Vec<CallArgument>,

            silent: bool,

            #[serde(rename = "returnByValue")]
            return_by_value: bool,

            #[serde(rename = "generatePreview")]
            generate_preview: bool,

            #[serde(rename = "userGesture")]
            user_gesture: bool,

            #[serde(rename = "awaitPromise")]
            await_promise: bool,

            #[serde(rename = "executionContextId")]
            execution_context_id: ExecutionContextId,

            #[serde(rename = "objectGroup")]
            object_group: String,

            #[serde(rename = "throwOnSideEffect")]
            throw_on_side_effect: bool,

            #[serde(rename = "uniqueContextId")]
            unique_context_id: String,

            #[serde(rename = "generateWebDriverValue")]
            generate_web_driver_value: bool,

            #[serde(rename = "serializationOptions")]
            serialization_options: SerializationOptions,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CallFunctionOnResponse {
            result: Option<RemoteObject>,

            #[serde(rename = "exceptionDetails")]
            exceptiondetails: ExceptionDetails,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CompileScriptRequest {
            expression: Option<String>,

            #[serde(rename = "sourceURL")]
            source_url: Option<String>,

            #[serde(rename = "persistScript")]
            persist_script: Option<bool>,

            #[serde(rename = "executionContextId")]
            execution_context_id: ExecutionContextId,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CompileScriptResponse {
            #[serde(rename = "scriptId")]
            script_id: ScriptId,

            #[serde(rename = "exceptionDetails")]
            exception_details: ExceptionDetails,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvaluateRequest {
            expression: Option<String>,

            #[serde(rename = "objectGroup")]
            object_group: String,

            #[serde(rename = "includeCommandLineAPI")]
            include_command_line_api: bool,

            silent: bool,

            #[serde(rename = "contextId")]
            context_id: ExecutionContextId,

            #[serde(rename = "returnByValue")]
            return_by_value: bool,

            #[serde(rename = "generatePreview")]
            generate_preview: bool,

            #[serde(rename = "userGesture")]
            user_gesture: bool,

            #[serde(rename = "awaitPromise")]
            await_promise: bool,

            #[serde(rename = "throwOnSideEffect")]
            throw_on_side_effect: bool,

            timeout: TimeDelta,

            #[serde(rename = "disableBreaks")]
            disable_breaks: bool,

            #[serde(rename = "replMode")]
            repl_mode: bool,

            #[serde(rename = "allowUnsafeEvalBlockedByCSP")]
            allow_unsafe_eval_blocked_by_csp: bool,

            #[serde(rename = "uniqueContextId")]
            unique_context_id: String,

            #[serde(rename = "generateWebDriverValue")]
            generate_web_driver_value: bool,

            #[serde(rename = "serializationOptions")]
            serialization_options: SerializationOptions,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct EvaluateResponse {
            result: Option<RemoteObject>,

            #[serde(rename = "exceptionDetails")]
            exceptiondetails: ExceptionDetails,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetIsolateIdResponse {
            id: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetHeapUsageResponse {
            #[serde(rename = "usedSize")]
            used_size: Option<f64>,

            #[serde(rename = "totalSize")]
            total_size: Option<f64>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetPropertiesRequest {
            #[serde(rename = "objectId")]
            object_id: Option<RemoteObjectId>,

            #[serde(rename = "ownProperties")]
            own_properties: bool,

            #[serde(rename = "accessorPropertiesOnly")]
            accessor_properties_only: bool,

            #[serde(rename = "generatePreview")]
            generate_preview: bool,

            #[serde(rename = "nonIndexedPropertiesOnly")]
            non_indexed_properties_only: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetPropertiesResponse {
            result: Option<Vec<PropertyDescriptor>>,

            #[serde(rename = "internalProperties")]
            internal_properties: Vec<InternalPropertyDescriptor>,

            #[serde(rename = "privateProperties")]
            private_properties: Vec<PrivatePropertyDescriptor>,

            #[serde(rename = "exceptionDetails")]
            exception_details: ExceptionDetails,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GlobalLexicalScopeNamesRequest {
            #[serde(rename = "executionContextId")]
            execution_context_id: ExecutionContextId,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GlobalLexicalScopeNamesResponse {
            names: Option<Vec<String>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct QueryObjectsRequest {
            #[serde(rename = "prototypeObjectId")]
            prototype_object_id: Option<RemoteObjectId>,

            #[serde(rename = "objectGroup")]
            object_group: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct QueryObjectsResponse {
            objects: Option<RemoteObject>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ReleaseObjectRequest {
            #[serde(rename = "objectId")]
            object_id: Option<RemoteObjectId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ReleaseObjectGroupRequest {
            #[serde(rename = "objectGroup")]
            object_group: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct RunScriptRequest {
            #[serde(rename = "scriptId")]
            script_id: Option<ScriptId>,

            #[serde(rename = "executionContextId")]
            execution_context_id: ExecutionContextId,

            #[serde(rename = "objectGroup")]
            object_group: String,

            silent: bool,

            #[serde(rename = "includeCommandLineAPI")]
            include_command_line_api: bool,

            #[serde(rename = "returnByValue")]
            return_by_value: bool,

            #[serde(rename = "generatePreview")]
            generate_preview: bool,

            #[serde(rename = "awaitPromise")]
            await_promise: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct RunScriptResponse {
            result: Option<RemoteObject>,

            #[serde(rename = "exceptionDetails")]
            exceptiondetails: ExceptionDetails,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetAsyncCallStackDepthRequest {
            #[serde(rename = "maxDepth")]
            max_depth: Option<Integer>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetCustomObjectFormatterEnabledRequest {
            enabled: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetMaxCallStackSizeToCaptureRequest {
            size: Option<Integer>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct AddBindingRequest {
            name: Option<String>,

            #[serde(rename = "executionContextId")]
            execution_context_id: ExecutionContextId,

            #[serde(rename = "executionContextName")]
            execution_context_name: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct RemoveBindingRequest {
            name: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetExceptionDetailsRequest {
            #[serde(rename = "errorObjectId")]
            error_object_id: Option<RemoteObjectId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetExceptionDetailsResponse {
            #[serde(rename = "exceptionDetails")]
            exception_details: ExceptionDetails,
        }

        /**
         * Notification is issued every time when binding is called.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct BindingCalledEvent {
            /**
             * Notification is issued every time when binding is called.
            */
            name: Option<String>,
            /**
             * Notification is issued every time when binding is called.
            */
            payload: Option<String>,
            /**
             * Notification is issued every time when binding is called.
            */
            #[serde(rename = "executionContextId")]
            executioncontextid: Option<ExecutionContextId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum ConsoleAPICalledEventType {
            #[serde(rename = "log")]
            Log,

            #[serde(rename = "debug")]
            Debug,

            #[serde(rename = "info")]
            Info,

            #[serde(rename = "error")]
            Error,

            #[serde(rename = "warning")]
            Warning,

            #[serde(rename = "dir")]
            Dir,

            #[serde(rename = "dirxml")]
            DirXML,

            #[serde(rename = "table")]
            Table,

            #[serde(rename = "trace")]
            Trace,

            #[serde(rename = "clear")]
            Clear,

            #[serde(rename = "startGroup")]
            StartGroup,

            #[serde(rename = "startGroupCollapsed")]
            StartGroupCollapsed,

            #[serde(rename = "endGroup")]
            EndGroup,

            #[serde(rename = "assert")]
            Assert,

            #[serde(rename = "profile")]
            Profile,

            #[serde(rename = "profileEnd")]
            ProfileEnd,

            #[serde(rename = "count")]
            Count,

            #[serde(rename = "timeEnd")]
            TimeEnd,
        }

        /**
         * Issued when console API was called.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ConsoleAPICalledEvent {
            /**
             * Issued when console API was called.
            */
            #[serde(rename = "type")]
            r#type: Option<ConsoleAPICalledEventType>,
            /**
             * Issued when console API was called.
            */
            args: Option<Vec<RemoteObject>>,
            /**
             * Issued when console API was called.
            */
            #[serde(rename = "executionContextId")]
            executioncontextid: Option<ExecutionContextId>,
            /**
             * Issued when console API was called.
            */
            timestamp: Option<Timestamp>,
            /**
             * Issued when console API was called.
            */
            #[serde(rename = "stackTrace")]
            stacktrace: StackTrace,
            /**
             * Issued when console API was called.
            */
            context: String,
        }

        /**
         * Issued when unhandled exception was revoked.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ExceptionRevokedEvent {
            /**
             * Issued when unhandled exception was revoked.
            */
            reason: Option<String>,
            /**
             * Issued when unhandled exception was revoked.
            */
            #[serde(rename = "exceptionId")]
            exceptionid: Option<Integer>,
        }

        /**
         * Issued when exception was thrown and unhandled.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ExceptionThrownEvent {
            /**
             * Issued when exception was thrown and unhandled.
            */
            timestamp: Option<Timestamp>,
            /**
             * Issued when exception was thrown and unhandled.
            */
            #[serde(rename = "exceptionDetails")]
            exceptiondetails: Option<ExceptionDetails>,
        }

        /**
         * Issued when new execution context is created.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ExecutionContextCreatedEvent {
            /**
             * Issued when new execution context is created.
            */
            context: Option<ExecutionContextDescription>,
        }

        /**
         * Issued when execution context is destroyed.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ExecutionContextDestroyedEvent {
            /**
             * Issued when execution context is destroyed.
            */
            #[serde(rename = "executionContextId")]
            execution_context_id: Option<ExecutionContextId>,
            /**
             * Issued when execution context is destroyed.
            */
            #[serde(rename = "executionContextUniqueId")]
            execution_context_unique_id: Option<String>,
        }

        /**
         * Issued when object should be inspected (for example, as a result of inspect() command line API
         * call).
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InspectRequestedEvent {
            /**
             * Issued when object should be inspected (for example, as a result of inspect() command line API
             * call).
            */
            object: Option<RemoteObject>,
            /**
             * Issued when object should be inspected (for example, as a result of inspect() command line API
             * call).
            */
            hints: Option<serde_json::Value>,
            /**
             * Issued when object should be inspected (for example, as a result of inspect() command line API
             * call).
            */
            #[serde(rename = "executionContextId")]
            executioncontextid: ExecutionContextId,
        }
    }

    /**
     * This domain is deprecated.
    */
    pub mod schema {
        use serde::{self, Deserialize, Serialize};
        /**
         * Description of the protocol domain.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Domain {
            /**
             * Description of the protocol domain.
            */
            name: Option<String>,
            /**
             * Description of the protocol domain.
            */
            version: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetDomainsResponse {
            domains: Option<Vec<Domain>>,
        }
    }

    pub mod accessibility {
        use super::{dom, page, runtime, Integer};
        use serde::{self, Deserialize, Serialize};
        /**
         * Unique accessibility node identifier.
        */
        pub type AXNodeId = String;

        /**
         * Enum of possible property types.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum AXValueType {
            #[serde(rename = "boolean")]
            Boolean,
            #[serde(rename = "tristate")]
            Tristate,
            #[serde(rename = "booleanOrUndefined")]
            BooleanOrUndefined,
            #[serde(rename = "idref")]
            Idref,
            #[serde(rename = "idrefList")]
            IdrefList,
            #[serde(rename = "integer")]
            Integer,
            #[serde(rename = "node")]
            Node,
            #[serde(rename = "nodeList")]
            NodeList,
            #[serde(rename = "number")]
            Number,
            #[serde(rename = "string")]
            String,
            #[serde(rename = "computedString")]
            ComputedString,
            #[serde(rename = "token")]
            Token,
            #[serde(rename = "tokenList")]
            TokenList,
            #[serde(rename = "domRelation")]
            DomRelation,
            #[serde(rename = "role")]
            Role,
            #[serde(rename = "internalRole")]
            InternalRole,
            #[serde(rename = "valueUndefined")]
            ValueUndefined,
        }

        /**
         * Enum of possible property sources.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum AXValueSourceType {
            #[serde(rename = "attribute")]
            Attribute,
            #[serde(rename = "implicit")]
            Implicit,
            #[serde(rename = "style")]
            Style,
            #[serde(rename = "contents")]
            Contents,
            #[serde(rename = "placeholder")]
            Placeholder,
            #[serde(rename = "relatedElement")]
            RelatedElement,
        }

        /**
         * Enum of possible native property sources (as a subtype of a particular AXValueSourceType).
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum AXValueNativeSourceType {
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "figcaption")]
            Figcaption,
            #[serde(rename = "label")]
            Label,
            #[serde(rename = "labelfor")]
            Labelfor,
            #[serde(rename = "labelwrapped")]
            Labelwrapped,
            #[serde(rename = "legend")]
            Legend,
            #[serde(rename = "rubyannotation")]
            Rubyannotation,
            #[serde(rename = "tablecaption")]
            Tablecaption,
            #[serde(rename = "title")]
            Title,
            #[serde(rename = "other")]
            Other,
        }

        /**
         * A single source for a computed AX property.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AXValueSource {
            /**
             * A single source for a computed AX property.
            */
            #[serde(rename = "type")]
            r#type: Option<AXValueSourceType>,
            /**
             * A single source for a computed AX property.
            */
            value: AXValue,
            /**
             * A single source for a computed AX property.
            */
            attribute: String,
            /**
             * A single source for a computed AX property.
            */
            #[serde(rename = "attributeValue")]
            attributevalue: AXValue,
            /**
             * A single source for a computed AX property.
            */
            superseded: bool,
            /**
             * A single source for a computed AX property.
            */
            #[serde(rename = "nativeSource")]
            nativesource: AXValueNativeSourceType,
            /**
             * A single source for a computed AX property.
            */
            #[serde(rename = "nativeSourceValue")]
            nativesourcevalue: AXValue,
            /**
             * A single source for a computed AX property.
            */
            invalid: bool,
            /**
             * A single source for a computed AX property.
            */
            #[serde(rename = "invalidReason")]
            invalidreason: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct AXRelatedNode {
            #[serde(rename = "backendDOMNodeId")]
            backenddomnodeid: Option<dom::BackendNodeId>,

            idref: String,

            text: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct AXProperty {
            name: Option<AXPropertyName>,

            value: Option<AXValue>,
        }

        /**
         * A single computed AX property.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AXValue {
            /**
             * A single computed AX property.
            */
            #[serde(rename = "type")]
            r#type: Option<AXValueType>,
            /**
             * A single computed AX property.
            */
            value: serde_json::Value,
            /**
             * A single computed AX property.
            */
            #[serde(rename = "relatedNodes")]
            relatednodes: Vec<AXRelatedNode>,
            /**
             * A single computed AX property.
            */
            sources: Vec<AXValueSource>,
        }

        /**
         * Values of AXProperty name:
         * - from 'busy' to 'roledescription': states which apply to every AX node
         * - from 'live' to 'root': attributes which apply to nodes in live regions
         * - from 'autocomplete' to 'valuetext': attributes which apply to widgets
         * - from 'checked' to 'selected': states which apply to widgets
         * - from 'activedescendant' to 'owns' - relationships between elements other than parent/child/sibling.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum AXPropertyName {
            #[serde(rename = "busy")]
            Busy,
            #[serde(rename = "disabled")]
            Disabled,
            #[serde(rename = "editable")]
            Editable,
            #[serde(rename = "focusable")]
            Focusable,
            #[serde(rename = "focused")]
            Focused,
            #[serde(rename = "hidden")]
            Hidden,
            #[serde(rename = "hiddenRoot")]
            HiddenRoot,
            #[serde(rename = "invalid")]
            Invalid,
            #[serde(rename = "keyshortcuts")]
            Keyshortcuts,
            #[serde(rename = "settable")]
            Settable,
            #[serde(rename = "roledescription")]
            Roledescription,
            #[serde(rename = "live")]
            Live,
            #[serde(rename = "atomic")]
            Atomic,
            #[serde(rename = "relevant")]
            Relevant,
            #[serde(rename = "root")]
            Root,
            #[serde(rename = "autocomplete")]
            Autocomplete,
            #[serde(rename = "hasPopup")]
            HasPopup,
            #[serde(rename = "level")]
            Level,
            #[serde(rename = "multiselectable")]
            Multiselectable,
            #[serde(rename = "orientation")]
            Orientation,
            #[serde(rename = "multiline")]
            Multiline,
            #[serde(rename = "readonly")]
            Readonly,
            #[serde(rename = "required")]
            Required,
            #[serde(rename = "valuemin")]
            Valuemin,
            #[serde(rename = "valuemax")]
            Valuemax,
            #[serde(rename = "valuetext")]
            Valuetext,
            #[serde(rename = "checked")]
            Checked,
            #[serde(rename = "expanded")]
            Expanded,
            #[serde(rename = "modal")]
            Modal,
            #[serde(rename = "pressed")]
            Pressed,
            #[serde(rename = "selected")]
            Selected,
            #[serde(rename = "activedescendant")]
            Activedescendant,
            #[serde(rename = "controls")]
            Controls,
            #[serde(rename = "describedby")]
            Describedby,
            #[serde(rename = "details")]
            Details,
            #[serde(rename = "errormessage")]
            Errormessage,
            #[serde(rename = "flowto")]
            Flowto,
            #[serde(rename = "labelledby")]
            Labelledby,
            #[serde(rename = "owns")]
            Owns,
        }

        /**
         * A node in the accessibility tree.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AXNode {
            /**
             * A node in the accessibility tree.
            */
            #[serde(rename = "nodeId")]
            node_id: Option<AXNodeId>,
            /**
             * A node in the accessibility tree.
            */
            ignored: Option<bool>,
            /**
             * A node in the accessibility tree.
            */
            #[serde(rename = "ignoredReasons")]
            ignored_reasons: Vec<AXProperty>,
            /**
             * A node in the accessibility tree.
            */
            role: AXValue,
            /**
             * A node in the accessibility tree.
            */
            #[serde(rename = "chromeRole")]
            chrome_role: AXValue,
            /**
             * A node in the accessibility tree.
            */
            name: AXValue,
            /**
             * A node in the accessibility tree.
            */
            description: AXValue,
            /**
             * A node in the accessibility tree.
            */
            value: AXValue,
            /**
             * A node in the accessibility tree.
            */
            properties: Vec<AXProperty>,
            /**
             * A node in the accessibility tree.
            */
            #[serde(rename = "parentId")]
            parent_id: AXNodeId,
            /**
             * A node in the accessibility tree.
            */
            #[serde(rename = "childIds")]
            child_ids: Vec<AXNodeId>,
            /**
             * A node in the accessibility tree.
            */
            #[serde(rename = "backendDOMNodeId")]
            backend_domnode_id: dom::BackendNodeId,
            /**
             * A node in the accessibility tree.
            */
            #[serde(rename = "frameId")]
            frame_id: page::FrameId,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetPartialAXTreeRequest {
            #[serde(rename = "nodeId")]
            node_id: dom::NodeId,

            #[serde(rename = "backendNodeId")]
            backend_node_id: dom::BackendNodeId,

            #[serde(rename = "objectId")]
            object_id: runtime::RemoteObjectId,

            #[serde(rename = "fetchRelatives")]
            fetch_relatives: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetPartialAXTreeResponse {
            nodes: Option<Vec<AXNode>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetFullAXTreeRequest {
            depth: Integer,

            #[serde(rename = "frameId")]
            frameid: page::FrameId,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetFullAXTreeResponse {
            nodes: Option<Vec<AXNode>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetRootAXNodeRequest {
            #[serde(rename = "frameId")]
            frame_id: page::FrameId,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetRootAXNodeResponse {
            node: Option<AXNode>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetAXNodeAndAncestorsRequest {
            #[serde(rename = "nodeId")]
            node_id: dom::NodeId,

            #[serde(rename = "backendNodeId")]
            backend_node_id: dom::BackendNodeId,

            #[serde(rename = "objectId")]
            object_id: runtime::RemoteObjectId,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetAXNodeAndAncestorsResponse {
            nodes: Option<Vec<AXNode>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetChildAXNodesRequest {
            id: Option<AXNodeId>,

            #[serde(rename = "frameId")]
            frameid: page::FrameId,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetChildAXNodesResponse {
            nodes: Option<Vec<AXNode>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct QueryAXTreeRequest {
            #[serde(rename = "nodeId")]
            node_id: dom::NodeId,

            #[serde(rename = "backendNodeId")]
            backend_node_id: dom::BackendNodeId,

            #[serde(rename = "objectId")]
            object_id: runtime::RemoteObjectId,

            #[serde(rename = "accessibleName")]
            accessible_name: String,

            role: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct QueryAXTreeResponse {
            nodes: Option<Vec<AXNode>>,
        }

        /**
         * The loadComplete event mirrors the load complete event sent by the browser to assistive
         * technology when the web page has finished loading.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct LoadCompleteEvent {
            /**
             * The loadComplete event mirrors the load complete event sent by the browser to assistive
             * technology when the web page has finished loading.
            */
            root: Option<AXNode>,
        }

        /**
         * The nodesUpdated event is sent every time a previously requested node has changed the in tree.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct NodesUpdatedEvent {
            /**
             * The nodesUpdated event is sent every time a previously requested node has changed the in tree.
            */
            nodes: Option<Vec<AXNode>>,
        }
    }

    pub mod animation {
        use super::{dom, runtime};
        use serde::{self, Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize)]
        pub enum AnimationType {
            #[serde(rename = "CSSTransition")]
            CSSTransition,

            #[serde(rename = "CSSAnimation")]
            CSSAnimation,

            #[serde(rename = "WebAnimation")]
            WebAnimation,
        }

        /**
         * Animation instance.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Animation {
            /**
             * Animation instance.
            */
            id: Option<String>,
            /**
             * Animation instance.
            */
            name: Option<String>,
            /**
             * Animation instance.
            */
            #[serde(rename = "pausedState")]
            paused_state: Option<bool>,
            /**
             * Animation instance.
            */
            #[serde(rename = "playState")]
            play_state: Option<String>,
            /**
             * Animation instance.
            */
            #[serde(rename = "playbackRate")]
            playback_rate: Option<f64>,
            /**
             * Animation instance.
            */
            #[serde(rename = "startTime")]
            start_time: Option<f64>,
            /**
             * Animation instance.
            */
            #[serde(rename = "currentTime")]
            current_time: Option<f64>,
            /**
             * Animation instance.
            */
            #[serde(rename = "type")]
            r#type: Option<AnimationType>,
            /**
             * Animation instance.
            */
            source: AnimationEffect,
            /**
             * Animation instance.
            */
            #[serde(rename = "cssId")]
            css_id: String,
        }

        /**
         * AnimationEffect instance
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AnimationEffect {
            /**
             * AnimationEffect instance
            */
            delay: Option<f64>,
            /**
             * AnimationEffect instance
            */
            #[serde(rename = "endDelay")]
            enddelay: Option<f64>,
            /**
             * AnimationEffect instance
            */
            #[serde(rename = "iterationStart")]
            iterationstart: Option<f64>,
            /**
             * AnimationEffect instance
            */
            iterations: Option<f64>,
            /**
             * AnimationEffect instance
            */
            duration: Option<f64>,
            /**
             * AnimationEffect instance
            */
            direction: Option<String>,
            /**
             * AnimationEffect instance
            */
            fill: Option<String>,
            /**
             * AnimationEffect instance
            */
            #[serde(rename = "backendNodeId")]
            backendnodeid: dom::BackendNodeId,
            /**
             * AnimationEffect instance
            */
            #[serde(rename = "keyframesRule")]
            keyframesrule: KeyframesRule,
            /**
             * AnimationEffect instance
            */
            easing: Option<String>,
        }

        /**
         * Keyframes Rule
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct KeyframesRule {
            /**
             * Keyframes Rule
            */
            name: String,
            /**
             * Keyframes Rule
            */
            keyframes: Option<Vec<KeyframeStyle>>,
        }

        /**
         * Keyframe Style
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct KeyframeStyle {
            /**
             * Keyframe Style
            */
            offset: Option<String>,
            /**
             * Keyframe Style
            */
            easing: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetCurrentTimeRequest {
            id: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetCurrentTimeResponse {
            #[serde(rename = "currentTime")]
            current_time: Option<f64>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetPlaybackRateResponse {
            #[serde(rename = "playbackRate")]
            playback_rate: Option<f64>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ReleaseAnimationsRequest {
            animations: Option<Vec<String>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ResolveAnimationRequest {
            #[serde(rename = "animationId")]
            animation_id: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ResolveAnimationResponse {
            #[serde(rename = "remoteObject")]
            remote_object: Option<runtime::RemoteObject>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SeekAnimationsRequest {
            animations: Option<Vec<String>>,

            #[serde(rename = "currentTime")]
            currenttime: Option<f64>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetPausedRequest {
            animations: Option<Vec<String>>,

            paused: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetPlaybackRateRequest {
            #[serde(rename = "playbackRate")]
            playback_rate: Option<f64>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetTimingRequest {
            #[serde(rename = "animationId")]
            animationid: Option<String>,

            duration: Option<f64>,

            delay: Option<f64>,
        }

        /**
         * Event for when an animation has been cancelled.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AnimationCanceledEvent {
            /**
             * Event for when an animation has been cancelled.
            */
            id: Option<String>,
        }

        /**
         * Event for each animation that has been created.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AnimationCreatedEvent {
            /**
             * Event for each animation that has been created.
            */
            id: Option<String>,
        }

        /**
         * Event for animation that has been started.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AnimationStartedEvent {
            /**
             * Event for animation that has been started.
            */
            animation: Option<Animation>,
        }
    }

    /**
     * Audits domain allows investigation of page violations and possible improvements.
    */
    pub mod audits {
        use super::{dom, network, page, runtime, Integer};
        use serde::{self, Deserialize, Serialize};
        /**
         * Information about a cookie that is affected by an inspector issue.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AffectedCookie {
            /**
             * Information about a cookie that is affected by an inspector issue.
            */
            name: Option<String>,
            /**
             * Information about a cookie that is affected by an inspector issue.
            */
            path: Option<String>,
            /**
             * Information about a cookie that is affected by an inspector issue.
            */
            domain: Option<String>,
        }

        /**
         * Information about a request that is affected by an inspector issue.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AffectedRequest {
            /**
             * Information about a request that is affected by an inspector issue.
            */
            #[serde(rename = "requestId")]
            request_id: Option<network::RequestId>,
            /**
             * Information about a request that is affected by an inspector issue.
            */
            url: String,
        }

        /**
         * Information about the frame affected by an inspector issue.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AffectedFrame {
            /**
             * Information about the frame affected by an inspector issue.
            */
            #[serde(rename = "frameId")]
            frame_id: Option<page::FrameId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum CookieExclusionReason {
            #[serde(rename = "ExcludeSameSiteUnspecifiedTreatedAsLax")]
            ExcludeSameSiteUnspecifiedTreatedAsLax,
            #[serde(rename = "ExcludeSameSiteNoneInsecure")]
            ExcludeSameSiteNoneInsecure,
            #[serde(rename = "ExcludeSameSiteLax")]
            ExcludeSameSiteLax,
            #[serde(rename = "ExcludeSameSiteStrict")]
            ExcludeSameSiteStrict,
            #[serde(rename = "ExcludeInvalidSameParty")]
            ExcludeInvalidSameParty,
            #[serde(rename = "ExcludeSamePartyCrossPartyContext")]
            ExcludeSamePartyCrossPartyContext,
            #[serde(rename = "ExcludeDomainNonASCII")]
            ExcludeDomainNonASCII,
            #[serde(rename = "ExcludeThirdPartyCookieBlockedInFirstPartySet")]
            ExcludeThirdPartyCookieBlockedInFirstPartySet,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum CookieWarningReason {
            #[serde(rename = "WarnSameSiteUnspecifiedCrossSiteContext")]
            WarnSameSiteUnspecifiedCrossSiteContext,
            #[serde(rename = "WarnSameSiteNoneInsecure")]
            WarnSameSiteNoneInsecure,
            #[serde(rename = "WarnSameSiteUnspecifiedLaxAllowUnsafe")]
            WarnSameSiteUnspecifiedLaxAllowUnsafe,
            #[serde(rename = "WarnSameSiteStrictLaxDowngradeStrict")]
            WarnSameSiteStrictLaxDowngradeStrict,
            #[serde(rename = "WarnSameSiteStrictCrossDowngradeStrict")]
            WarnSameSiteStrictCrossDowngradeStrict,
            #[serde(rename = "WarnSameSiteStrictCrossDowngradeLax")]
            WarnSameSiteStrictCrossDowngradeLax,
            #[serde(rename = "WarnSameSiteLaxCrossDowngradeStrict")]
            WarnSameSiteLaxCrossDowngradeStrict,
            #[serde(rename = "WarnSameSiteLaxCrossDowngradeLax")]
            WarnSameSiteLaxCrossDowngradeLax,
            #[serde(rename = "WarnAttributeValueExceedsMaxSize")]
            WarnAttributeValueExceedsMaxSize,
            #[serde(rename = "WarnDomainNonASCII")]
            WarnDomainNonASCII,
            #[serde(rename = "WarnThirdPartyPhaseout")]
            WarnThirdPartyPhaseout,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum CookieOperation {
            #[serde(rename = "SetCookie")]
            SetCookie,
            #[serde(rename = "ReadCookie")]
            ReadCookie,
        }

        /**
         * This information is currently necessary, as the front-end has a difficult
         * time finding a specific cookie. With this, we can convey specific error
         * information without the cookie.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct CookieIssueDetails {
            /**
             * This information is currently necessary, as the front-end has a difficult
             * time finding a specific cookie. With this, we can convey specific error
             * information without the cookie.
            */
            cookie: AffectedCookie,
            /**
             * This information is currently necessary, as the front-end has a difficult
             * time finding a specific cookie. With this, we can convey specific error
             * information without the cookie.
            */
            #[serde(rename = "rawCookieLine")]
            raw_cookie_line: String,
            /**
             * This information is currently necessary, as the front-end has a difficult
             * time finding a specific cookie. With this, we can convey specific error
             * information without the cookie.
            */
            #[serde(rename = "cookieWarningReasons")]
            cookie_warning_reasons: Option<Vec<CookieWarningReason>>,
            /**
             * This information is currently necessary, as the front-end has a difficult
             * time finding a specific cookie. With this, we can convey specific error
             * information without the cookie.
            */
            #[serde(rename = "cookieExclusionReasons")]
            cookie_exclusion_reasons: Option<Vec<CookieExclusionReason>>,
            /**
             * This information is currently necessary, as the front-end has a difficult
             * time finding a specific cookie. With this, we can convey specific error
             * information without the cookie.
            */
            operation: Option<CookieOperation>,
            /**
             * This information is currently necessary, as the front-end has a difficult
             * time finding a specific cookie. With this, we can convey specific error
             * information without the cookie.
            */
            #[serde(rename = "siteForCookies")]
            site_for_cookies: String,
            /**
             * This information is currently necessary, as the front-end has a difficult
             * time finding a specific cookie. With this, we can convey specific error
             * information without the cookie.
            */
            #[serde(rename = "cookieUrl")]
            cookie_url: String,
            /**
             * This information is currently necessary, as the front-end has a difficult
             * time finding a specific cookie. With this, we can convey specific error
             * information without the cookie.
            */
            request: AffectedRequest,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum MixedContentResolutionStatus {
            #[serde(rename = "MixedContentBlocked")]
            MixedContentBlocked,
            #[serde(rename = "MixedContentAutomaticallyUpgraded")]
            MixedContentAutomaticallyUpgraded,
            #[serde(rename = "MixedContentWarning")]
            MixedContentWarning,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum MixedContentResourceType {
            #[serde(rename = "AttributionSrc")]
            AttributionSrc,
            #[serde(rename = "Audio")]
            Audio,
            #[serde(rename = "Beacon")]
            Beacon,
            #[serde(rename = "CSPReport")]
            CSPReport,
            #[serde(rename = "Download")]
            Download,
            #[serde(rename = "EventSource")]
            EventSource,
            #[serde(rename = "Favicon")]
            Favicon,
            #[serde(rename = "Font")]
            Font,
            #[serde(rename = "Form")]
            Form,
            #[serde(rename = "Frame")]
            Frame,
            #[serde(rename = "Image")]
            Image,
            #[serde(rename = "Import")]
            Import,
            #[serde(rename = "Manifest")]
            Manifest,
            #[serde(rename = "Ping")]
            Ping,
            #[serde(rename = "PluginData")]
            PluginData,
            #[serde(rename = "PluginResource")]
            PluginResource,
            #[serde(rename = "Prefetch")]
            Prefetch,
            #[serde(rename = "Resource")]
            Resource,
            #[serde(rename = "Script")]
            Script,
            #[serde(rename = "ServiceWorker")]
            ServiceWorker,
            #[serde(rename = "SharedWorker")]
            SharedWorker,
            #[serde(rename = "Stylesheet")]
            Stylesheet,
            #[serde(rename = "Track")]
            Track,
            #[serde(rename = "Video")]
            Video,
            #[serde(rename = "Worker")]
            Worker,
            #[serde(rename = "XMLHttpRequest")]
            XMLHttpRequest,
            #[serde(rename = "XSLT")]
            XSLT,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct MixedContentIssueDetails {
            #[serde(rename = "resourceType")]
            resource_type: MixedContentResourceType,

            #[serde(rename = "resolutionStatus")]
            resolution_status: Option<MixedContentResolutionStatus>,

            #[serde(rename = "insecureURL")]
            insecure_url: Option<String>,

            #[serde(rename = "mainResourceURL")]
            main_resource_url: Option<String>,

            request: AffectedRequest,

            frame: AffectedFrame,
        }

        /**
         * Enum indicating the reason a response has been blocked. These reasons are
         * refinements of the net error BLOCKED_BY_RESPONSE.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum BlockedByResponseReason {
            #[serde(rename = "CoepFrameResourceNeedsCoepHeader")]
            CoepFrameResourceNeedsCoepHeader,
            #[serde(rename = "CoopSandboxedIFrameCannotNavigateToCoopPage")]
            CoopSandboxedIFrameCannotNavigateToCoopPage,
            #[serde(rename = "CorpNotSameOrigin")]
            CorpNotSameOrigin,
            #[serde(rename = "CorpNotSameOriginAfterDefaultedToSameOriginByCoep")]
            CorpNotSameOriginAfterDefaultedToSameOriginByCoep,
            #[serde(rename = "CorpNotSameSite")]
            CorpNotSameSite,
        }

        /**
         * Details for a request that has been blocked with the BLOCKED_BY_RESPONSE
         * code. Currently only used for COEP/COOP, but may be extended to include
         * some CSP errors in the future.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct BlockedByResponseIssueDetails {
            /**
             * Details for a request that has been blocked with the BLOCKED_BY_RESPONSE
             * code. Currently only used for COEP/COOP, but may be extended to include
             * some CSP errors in the future.
            */
            request: Option<AffectedRequest>,
            /**
             * Details for a request that has been blocked with the BLOCKED_BY_RESPONSE
             * code. Currently only used for COEP/COOP, but may be extended to include
             * some CSP errors in the future.
            */
            #[serde(rename = "parentFrame")]
            parentframe: AffectedFrame,
            /**
             * Details for a request that has been blocked with the BLOCKED_BY_RESPONSE
             * code. Currently only used for COEP/COOP, but may be extended to include
             * some CSP errors in the future.
            */
            #[serde(rename = "blockedFrame")]
            blockedframe: AffectedFrame,
            /**
             * Details for a request that has been blocked with the BLOCKED_BY_RESPONSE
             * code. Currently only used for COEP/COOP, but may be extended to include
             * some CSP errors in the future.
            */
            reason: Option<BlockedByResponseReason>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum HeavyAdResolutionStatus {
            #[serde(rename = "HeavyAdBlocked")]
            HeavyAdBlocked,
            #[serde(rename = "HeavyAdWarning")]
            HeavyAdWarning,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum HeavyAdReason {
            #[serde(rename = "NetworkTotalLimit")]
            NetworkTotalLimit,
            #[serde(rename = "CpuTotalLimit")]
            CpuTotalLimit,
            #[serde(rename = "CpuPeakLimit")]
            CpuPeakLimit,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct HeavyAdIssueDetails {
            resolution: Option<HeavyAdResolutionStatus>,

            reason: Option<HeavyAdReason>,

            frame: Option<AffectedFrame>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum ContentSecurityPolicyViolationType {
            #[serde(rename = "kInlineViolation")]
            KInlineViolation,
            #[serde(rename = "kEvalViolation")]
            KEvalViolation,
            #[serde(rename = "kURLViolation")]
            KURLViolation,
            #[serde(rename = "kTrustedTypesSinkViolation")]
            KTrustedTypesSinkViolation,
            #[serde(rename = "kTrustedTypesPolicyViolation")]
            KTrustedTypesPolicyViolation,
            #[serde(rename = "kWasmEvalViolation")]
            KWasmEvalViolation,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SourceCodeLocation {
            #[serde(rename = "scriptId")]
            script_id: runtime::ScriptId,

            url: Option<String>,

            #[serde(rename = "lineNumber")]
            line_number: Option<Integer>,

            #[serde(rename = "columnNumber")]
            column_number: Option<Integer>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ContentSecurityPolicyIssueDetails {
            #[serde(rename = "blockedURL")]
            blocked_url: String,

            #[serde(rename = "violatedDirective")]
            violated_directive: Option<String>,

            #[serde(rename = "isReportOnly")]
            is_report_only: Option<bool>,

            #[serde(rename = "contentSecurityPolicyViolationType")]
            content_security_policy_violation_type: Option<ContentSecurityPolicyViolationType>,

            #[serde(rename = "frameAncestor")]
            frame_ancestor: AffectedFrame,

            #[serde(rename = "sourceCodeLocation")]
            source_code_location: SourceCodeLocation,

            #[serde(rename = "violatingNodeId")]
            violating_node_id: dom::BackendNodeId,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum SharedArrayBufferIssueType {
            #[serde(rename = "TransferIssue")]
            TransferIssue,
            #[serde(rename = "CreationIssue")]
            CreationIssue,
        }

        /**
         * Details for a issue arising from an SAB being instantiated in, or
         * transferred to a context that is not cross-origin isolated.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SharedArrayBufferIssueDetails {
            /**
             * Details for a issue arising from an SAB being instantiated in, or
             * transferred to a context that is not cross-origin isolated.
            */
            #[serde(rename = "sourceCodeLocation")]
            source_code_location: Option<SourceCodeLocation>,
            /**
             * Details for a issue arising from an SAB being instantiated in, or
             * transferred to a context that is not cross-origin isolated.
            */
            #[serde(rename = "isWarning")]
            is_warning: Option<bool>,
            /**
             * Details for a issue arising from an SAB being instantiated in, or
             * transferred to a context that is not cross-origin isolated.
            */
            #[serde(rename = "type")]
            r#type: Option<SharedArrayBufferIssueType>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct LowTextContrastIssueDetails {
            #[serde(rename = "violatingNodeId")]
            violating_node_id: Option<dom::BackendNodeId>,

            #[serde(rename = "violatingNodeSelector")]
            violating_node_selector: Option<String>,

            #[serde(rename = "contrastRatio")]
            contrast_ratio: Option<f64>,

            #[serde(rename = "thresholdAA")]
            threshold_aa: Option<f64>,

            #[serde(rename = "thresholdAAA")]
            threshold_aaa: Option<f64>,

            #[serde(rename = "fontSize")]
            font_size: Option<String>,

            #[serde(rename = "fontWeight")]
            font_weight: Option<String>,
        }

        /**
         * Details for a CORS related issue, e.g. a warning or error related to
         * CORS RFC1918 enforcement.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct CorsIssueDetails {
            /**
             * Details for a CORS related issue, e.g. a warning or error related to
             * CORS RFC1918 enforcement.
            */
            #[serde(rename = "corsErrorStatus")]
            cors_error_status: Option<network::CorsErrorStatus>,
            /**
             * Details for a CORS related issue, e.g. a warning or error related to
             * CORS RFC1918 enforcement.
            */
            #[serde(rename = "isWarning")]
            is_warning: Option<bool>,
            /**
             * Details for a CORS related issue, e.g. a warning or error related to
             * CORS RFC1918 enforcement.
            */
            request: Option<AffectedRequest>,
            /**
             * Details for a CORS related issue, e.g. a warning or error related to
             * CORS RFC1918 enforcement.
            */
            location: SourceCodeLocation,
            /**
             * Details for a CORS related issue, e.g. a warning or error related to
             * CORS RFC1918 enforcement.
            */
            #[serde(rename = "initiatorOrigin")]
            initiator_origin: String,
            /**
             * Details for a CORS related issue, e.g. a warning or error related to
             * CORS RFC1918 enforcement.
            */
            #[serde(rename = "resourceIPAddressSpace")]
            resource_ipaddress_space: network::IPAddressSpace,
            /**
             * Details for a CORS related issue, e.g. a warning or error related to
             * CORS RFC1918 enforcement.
            */
            #[serde(rename = "clientSecurityState")]
            client_security_state: network::ClientSecurityState,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum AttributionReportingIssueType {
            #[serde(rename = "PermissionPolicyDisabled")]
            PermissionPolicyDisabled,
            #[serde(rename = "UntrustworthyReportingOrigin")]
            UntrustworthyReportingOrigin,
            #[serde(rename = "InsecureContext")]
            InsecureContext,
            #[serde(rename = "InvalidHeader")]
            InvalidHeader,
            #[serde(rename = "InvalidRegisterTriggerHeader")]
            InvalidRegisterTriggerHeader,
            #[serde(rename = "SourceAndTriggerHeaders")]
            SourceAndTriggerHeaders,
            #[serde(rename = "SourceIgnored")]
            SourceIgnored,
            #[serde(rename = "TriggerIgnored")]
            TriggerIgnored,
            #[serde(rename = "OsSourceIgnored")]
            OsSourceIgnored,
            #[serde(rename = "OsTriggerIgnored")]
            OsTriggerIgnored,
            #[serde(rename = "InvalidRegisterOsSourceHeader")]
            InvalidRegisterOsSourceHeader,
            #[serde(rename = "InvalidRegisterOsTriggerHeader")]
            InvalidRegisterOsTriggerHeader,
            #[serde(rename = "WebAndOsHeaders")]
            WebAndOsHeaders,
            #[serde(rename = "NoWebOrOsSupport")]
            NoWebOrOsSupport,
            #[serde(rename = "NavigationRegistrationWithoutTransientUserActivation")]
            NavigationRegistrationWithoutTransientUserActivation,
        }

        /**
         * Details for issues around "Attribution Reporting API" usage.
         * Explainer: https://github.com/WICG/attribution-reporting-api
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AttributionReportingIssueDetails {
            /**
             * Details for issues around "Attribution Reporting API" usage.
             * Explainer: https://github.com/WICG/attribution-reporting-api
            */
            #[serde(rename = "violationType")]
            violation_type: Option<AttributionReportingIssueType>,
            /**
             * Details for issues around "Attribution Reporting API" usage.
             * Explainer: https://github.com/WICG/attribution-reporting-api
            */
            request: AffectedRequest,
            /**
             * Details for issues around "Attribution Reporting API" usage.
             * Explainer: https://github.com/WICG/attribution-reporting-api
            */
            #[serde(rename = "violatingNodeId")]
            violating_node_id: dom::BackendNodeId,
            /**
             * Details for issues around "Attribution Reporting API" usage.
             * Explainer: https://github.com/WICG/attribution-reporting-api
            */
            #[serde(rename = "invalidParameter")]
            invalid_parameter: String,
        }

        /**
         * Details for issues about documents in Quirks Mode
         * or Limited Quirks Mode that affects page layouting.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct QuirksModeIssueDetails {
            /**
             * Details for issues about documents in Quirks Mode
             * or Limited Quirks Mode that affects page layouting.
            */
            #[serde(rename = "isLimitedQuirksMode")]
            is_limited_quirks_mode: Option<bool>,
            /**
             * Details for issues about documents in Quirks Mode
             * or Limited Quirks Mode that affects page layouting.
            */
            #[serde(rename = "documentNodeId")]
            document_node_id: Option<dom::BackendNodeId>,
            /**
             * Details for issues about documents in Quirks Mode
             * or Limited Quirks Mode that affects page layouting.
            */
            url: Option<String>,
            /**
             * Details for issues about documents in Quirks Mode
             * or Limited Quirks Mode that affects page layouting.
            */
            #[serde(rename = "frameId")]
            frame_id: Option<page::FrameId>,
            /**
             * Details for issues about documents in Quirks Mode
             * or Limited Quirks Mode that affects page layouting.
            */
            #[serde(rename = "loaderId")]
            loader_id: Option<network::LoaderId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct NavigatorUserAgentIssueDetails {
            url: Option<String>,

            location: SourceCodeLocation,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum GenericIssueErrorType {
            #[serde(rename = "CrossOriginPortalPostMessageError")]
            CrossOriginPortalPostMessageError,
            #[serde(rename = "FormLabelForNameError")]
            FormLabelForNameError,
            #[serde(rename = "FormDuplicateIdForInputError")]
            FormDuplicateIdForInputError,
            #[serde(rename = "FormInputWithNoLabelError")]
            FormInputWithNoLabelError,
            #[serde(rename = "FormAutocompleteAttributeEmptyError")]
            FormAutocompleteAttributeEmptyError,
            #[serde(rename = "FormEmptyIdAndNameAttributesForInputError")]
            FormEmptyIdAndNameAttributesForInputError,
            #[serde(rename = "FormAriaLabelledByToNonExistingId")]
            FormAriaLabelledByToNonExistingId,
            #[serde(rename = "FormInputAssignedAutocompleteValueToIdOrNameAttributeError")]
            FormInputAssignedAutocompleteValueToIdOrNameAttributeError,
            #[serde(rename = "FormLabelHasNeitherForNorNestedInput")]
            FormLabelHasNeitherForNorNestedInput,
            #[serde(rename = "FormLabelForMatchesNonExistingIdError")]
            FormLabelForMatchesNonExistingIdError,
            #[serde(rename = "FormInputHasWrongButWellIntendedAutocompleteValueError")]
            FormInputHasWrongButWellIntendedAutocompleteValueError,
            #[serde(rename = "ResponseWasBlockedByORB")]
            ResponseWasBlockedByORB,
        }

        /**
         * Depending on the concrete errorType, different properties are set.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct GenericIssueDetails {
            /**
             * Depending on the concrete errorType, different properties are set.
            */
            #[serde(rename = "errorType")]
            error_type: Option<GenericIssueErrorType>,
            /**
             * Depending on the concrete errorType, different properties are set.
            */
            #[serde(rename = "frameId")]
            frame_id: page::FrameId,
            /**
             * Depending on the concrete errorType, different properties are set.
            */
            #[serde(rename = "violatingNodeId")]
            violating_node_id: dom::BackendNodeId,
            /**
             * Depending on the concrete errorType, different properties are set.
            */
            #[serde(rename = "violatingNodeAttribute")]
            violating_node_attribute: String,
            /**
             * Depending on the concrete errorType, different properties are set.
            */
            request: AffectedRequest,
        }

        /**
         * This issue tracks information needed to print a deprecation message.
         * https://source.chromium.org/chromium/chromium/src/+/main:third_party/blink/renderer/core/frame/third_party/blink/renderer/core/frame/deprecation/README.md
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct DeprecationIssueDetails {
            /**
             * This issue tracks information needed to print a deprecation message.
             * https://source.chromium.org/chromium/chromium/src/+/main:third_party/blink/renderer/core/frame/third_party/blink/renderer/core/frame/deprecation/README.md
            */
            #[serde(rename = "affectedFrame")]
            affected_frame: AffectedFrame,
            /**
             * This issue tracks information needed to print a deprecation message.
             * https://source.chromium.org/chromium/chromium/src/+/main:third_party/blink/renderer/core/frame/third_party/blink/renderer/core/frame/deprecation/README.md
            */
            #[serde(rename = "sourceCodeLocation")]
            source_code_location: Option<SourceCodeLocation>,
            /**
             * This issue tracks information needed to print a deprecation message.
             * https://source.chromium.org/chromium/chromium/src/+/main:third_party/blink/renderer/core/frame/third_party/blink/renderer/core/frame/deprecation/README.md
            */
            #[serde(rename = "type")]
            r#type: Option<String>,
        }

        /**
         * This issue warns about sites in the redirect chain of a finished navigation
         * that may be flagged as trackers and have their state cleared if they don't
         * receive a user interaction. Note that in this context 'site' means eTLD+1.
         * For example, if the URL `https://example.test:80/bounce` was in the
         * redirect chain, the site reported would be `example.test`.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct BounceTrackingIssueDetails {
            /**
             * This issue warns about sites in the redirect chain of a finished navigation
             * that may be flagged as trackers and have their state cleared if they don't
             * receive a user interaction. Note that in this context 'site' means eTLD+1.
             * For example, if the URL `https://example.test:80/bounce` was in the
             * redirect chain, the site reported would be `example.test`.
            */
            #[serde(rename = "trackingSites")]
            tracking_sites: Option<Vec<String>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum ClientHintIssueReason {
            #[serde(rename = "MetaTagAllowListInvalidOrigin")]
            MetaTagAllowListInvalidOrigin,
            #[serde(rename = "MetaTagModifiedHTML")]
            MetaTagModifiedHTML,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct FederatedAuthRequestIssueDetails {
            #[serde(rename = "federatedAuthRequestIssueReason")]
            federated_auth_request_issue_reason: Option<FederatedAuthRequestIssueReason>,
        }

        /**
         * Represents the failure reason when a federated authentication reason fails.
         * Should be updated alongside RequestIdTokenStatus in
         * third_party/blink/public/mojom/devtools/inspector_issue.mojom to include
         * all cases except for success.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum FederatedAuthRequestIssueReason {
            #[serde(rename = "ShouldEmbargo")]
            ShouldEmbargo,
            #[serde(rename = "TooManyRequests")]
            TooManyRequests,
            #[serde(rename = "WellKnownHttpNotFound")]
            WellKnownHttpNotFound,
            #[serde(rename = "WellKnownNoResponse")]
            WellKnownNoResponse,
            #[serde(rename = "WellKnownInvalidResponse")]
            WellKnownInvalidResponse,
            #[serde(rename = "WellKnownListEmpty")]
            WellKnownListEmpty,
            #[serde(rename = "WellKnownInvalidContentType")]
            WellKnownInvalidContentType,
            #[serde(rename = "ConfigNotInWellKnown")]
            ConfigNotInWellKnown,
            #[serde(rename = "WellKnownTooBig")]
            WellKnownTooBig,
            #[serde(rename = "ConfigHttpNotFound")]
            ConfigHttpNotFound,
            #[serde(rename = "ConfigNoResponse")]
            ConfigNoResponse,
            #[serde(rename = "ConfigInvalidResponse")]
            ConfigInvalidResponse,
            #[serde(rename = "ConfigInvalidContentType")]
            ConfigInvalidContentType,
            #[serde(rename = "ClientMetadataHttpNotFound")]
            ClientMetadataHttpNotFound,
            #[serde(rename = "ClientMetadataNoResponse")]
            ClientMetadataNoResponse,
            #[serde(rename = "ClientMetadataInvalidResponse")]
            ClientMetadataInvalidResponse,
            #[serde(rename = "ClientMetadataInvalidContentType")]
            ClientMetadataInvalidContentType,
            #[serde(rename = "DisabledInSettings")]
            DisabledInSettings,
            #[serde(rename = "ErrorFetchingSignin")]
            ErrorFetchingSignin,
            #[serde(rename = "InvalidSigninResponse")]
            InvalidSigninResponse,
            #[serde(rename = "AccountsHttpNotFound")]
            AccountsHttpNotFound,
            #[serde(rename = "AccountsNoResponse")]
            AccountsNoResponse,
            #[serde(rename = "AccountsInvalidResponse")]
            AccountsInvalidResponse,
            #[serde(rename = "AccountsListEmpty")]
            AccountsListEmpty,
            #[serde(rename = "AccountsInvalidContentType")]
            AccountsInvalidContentType,
            #[serde(rename = "IdTokenHttpNotFound")]
            IdTokenHttpNotFound,
            #[serde(rename = "IdTokenNoResponse")]
            IdTokenNoResponse,
            #[serde(rename = "IdTokenInvalidResponse")]
            IdTokenInvalidResponse,
            #[serde(rename = "IdTokenInvalidRequest")]
            IdTokenInvalidRequest,
            #[serde(rename = "IdTokenInvalidContentType")]
            IdTokenInvalidContentType,
            #[serde(rename = "ErrorIdToken")]
            ErrorIdToken,
            #[serde(rename = "Canceled")]
            Canceled,
            #[serde(rename = "RpPageNotVisible")]
            RpPageNotVisible,
            #[serde(rename = "SilentMediationFailure")]
            SilentMediationFailure,
            #[serde(rename = "ThirdPartyCookiesBlocked")]
            ThirdPartyCookiesBlocked,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct FederatedAuthUserInfoRequestIssueDetails {
            #[serde(rename = "federatedAuthUserInfoRequestIssueReason")]
            federated_auth_user_info_request_issue_reason:
                Option<FederatedAuthUserInfoRequestIssueReason>,
        }

        /**
         * Represents the failure reason when a getUserInfo() call fails.
         * Should be updated alongside FederatedAuthUserInfoRequestResult in
         * third_party/blink/public/mojom/devtools/inspector_issue.mojom.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum FederatedAuthUserInfoRequestIssueReason {
            #[serde(rename = "NotSameOrigin")]
            NotSameOrigin,
            #[serde(rename = "NotIframe")]
            NotIframe,
            #[serde(rename = "NotPotentiallyTrustworthy")]
            NotPotentiallyTrustworthy,
            #[serde(rename = "NoApiPermission")]
            NoApiPermission,
            #[serde(rename = "NotSignedInWithIdp")]
            NotSignedInWithIdp,
            #[serde(rename = "NoAccountSharingPermission")]
            NoAccountSharingPermission,
            #[serde(rename = "InvalidConfigOrWellKnown")]
            InvalidConfigOrWellKnown,
            #[serde(rename = "InvalidAccountsResponse")]
            InvalidAccountsResponse,
            #[serde(rename = "NoReturningUserFromFetchedAccounts")]
            NoReturningUserFromFetchedAccounts,
        }

        /**
         * This issue tracks client hints related issues. It's used to deprecate old
         * features, encourage the use of new ones, and provide general guidance.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ClientHintIssueDetails {
            /**
             * This issue tracks client hints related issues. It's used to deprecate old
             * features, encourage the use of new ones, and provide general guidance.
            */
            #[serde(rename = "sourceCodeLocation")]
            source_code_location: Option<SourceCodeLocation>,
            /**
             * This issue tracks client hints related issues. It's used to deprecate old
             * features, encourage the use of new ones, and provide general guidance.
            */
            #[serde(rename = "clientHintIssueReason")]
            client_hint_issue_reason: Option<ClientHintIssueReason>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct FailedRequestInfo {
            url: Option<String>,

            #[serde(rename = "failureMessage")]
            failure_message: Option<String>,

            #[serde(rename = "requestId")]
            request_id: network::RequestId,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum StyleSheetLoadingIssueReason {
            #[serde(rename = "LateImportRule")]
            LateImportRule,
            #[serde(rename = "RequestFailed")]
            RequestFailed,
        }

        /**
         * This issue warns when a referenced stylesheet couldn't be loaded.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct StylesheetLoadingIssueDetails {
            /**
             * This issue warns when a referenced stylesheet couldn't be loaded.
            */
            #[serde(rename = "sourceCodeLocation")]
            source_code_location: Option<SourceCodeLocation>,
            /**
             * This issue warns when a referenced stylesheet couldn't be loaded.
            */
            #[serde(rename = "styleSheetLoadingIssueReason")]
            style_sheet_loading_issue_reason: Option<StyleSheetLoadingIssueReason>,
            /**
             * This issue warns when a referenced stylesheet couldn't be loaded.
            */
            #[serde(rename = "failedRequestInfo")]
            failed_request_info: FailedRequestInfo,
        }

        /**
         * A unique identifier for the type of issue. Each type may use one of the
         * optional fields in InspectorIssueDetails to convey more specific
         * information about the kind of issue.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum InspectorIssueCode {
            #[serde(rename = "CookieIssue")]
            CookieIssue,
            #[serde(rename = "MixedContentIssue")]
            MixedContentIssue,
            #[serde(rename = "BlockedByResponseIssue")]
            BlockedByResponseIssue,
            #[serde(rename = "HeavyAdIssue")]
            HeavyAdIssue,
            #[serde(rename = "ContentSecurityPolicyIssue")]
            ContentSecurityPolicyIssue,
            #[serde(rename = "SharedArrayBufferIssue")]
            SharedArrayBufferIssue,
            #[serde(rename = "LowTextContrastIssue")]
            LowTextContrastIssue,
            #[serde(rename = "CorsIssue")]
            CorsIssue,
            #[serde(rename = "AttributionReportingIssue")]
            AttributionReportingIssue,
            #[serde(rename = "QuirksModeIssue")]
            QuirksModeIssue,
            #[serde(rename = "NavigatorUserAgentIssue")]
            NavigatorUserAgentIssue,
            #[serde(rename = "GenericIssue")]
            GenericIssue,
            #[serde(rename = "DeprecationIssue")]
            DeprecationIssue,
            #[serde(rename = "ClientHintIssue")]
            ClientHintIssue,
            #[serde(rename = "FederatedAuthRequestIssue")]
            FederatedAuthRequestIssue,
            #[serde(rename = "BounceTrackingIssue")]
            BounceTrackingIssue,
            #[serde(rename = "StylesheetLoadingIssue")]
            StylesheetLoadingIssue,
            #[serde(rename = "FederatedAuthUserInfoRequestIssue")]
            FederatedAuthUserInfoRequestIssue,
        }

        /**
         * This struct holds a list of optional fields with additional information
         * specific to the kind of issue. When adding a new issue code, please also
         * add a new optional field to this type.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InspectorIssueDetails {
            /**
             * This struct holds a list of optional fields with additional information
             * specific to the kind of issue. When adding a new issue code, please also
             * add a new optional field to this type.
            */
            #[serde(rename = "cookieIssueDetails")]
            cookie_issue_details: CookieIssueDetails,
            /**
             * This struct holds a list of optional fields with additional information
             * specific to the kind of issue. When adding a new issue code, please also
             * add a new optional field to this type.
            */
            #[serde(rename = "mixedContentIssueDetails")]
            mixed_content_issue_details: MixedContentIssueDetails,
            /**
             * This struct holds a list of optional fields with additional information
             * specific to the kind of issue. When adding a new issue code, please also
             * add a new optional field to this type.
            */
            #[serde(rename = "blockedByResponseIssueDetails")]
            blocked_by_response_issue_details: BlockedByResponseIssueDetails,
            /**
             * This struct holds a list of optional fields with additional information
             * specific to the kind of issue. When adding a new issue code, please also
             * add a new optional field to this type.
            */
            #[serde(rename = "heavyAdIssueDetails")]
            heavy_ad_issue_details: HeavyAdIssueDetails,
            /**
             * This struct holds a list of optional fields with additional information
             * specific to the kind of issue. When adding a new issue code, please also
             * add a new optional field to this type.
            */
            #[serde(rename = "contentSecurityPolicyIssueDetails")]
            content_security_policy_issue_details: ContentSecurityPolicyIssueDetails,
            /**
             * This struct holds a list of optional fields with additional information
             * specific to the kind of issue. When adding a new issue code, please also
             * add a new optional field to this type.
            */
            #[serde(rename = "sharedArrayBufferIssueDetails")]
            shared_array_buffer_issue_details: SharedArrayBufferIssueDetails,
            /**
             * This struct holds a list of optional fields with additional information
             * specific to the kind of issue. When adding a new issue code, please also
             * add a new optional field to this type.
            */
            #[serde(rename = "lowTextContrastIssueDetails")]
            low_text_contrast_issue_details: LowTextContrastIssueDetails,
            /**
             * This struct holds a list of optional fields with additional information
             * specific to the kind of issue. When adding a new issue code, please also
             * add a new optional field to this type.
            */
            #[serde(rename = "corsIssueDetails")]
            cors_issue_details: CorsIssueDetails,
            /**
             * This struct holds a list of optional fields with additional information
             * specific to the kind of issue. When adding a new issue code, please also
             * add a new optional field to this type.
            */
            #[serde(rename = "attributionReportingIssueDetails")]
            attribution_reporting_issue_details: AttributionReportingIssueDetails,
            /**
             * This struct holds a list of optional fields with additional information
             * specific to the kind of issue. When adding a new issue code, please also
             * add a new optional field to this type.
            */
            #[serde(rename = "quirksModeIssueDetails")]
            quirks_mode_issue_details: QuirksModeIssueDetails,
            /**
             * This struct holds a list of optional fields with additional information
             * specific to the kind of issue. When adding a new issue code, please also
             * add a new optional field to this type.
            */
            #[serde(rename = "navigatorUserAgentIssueDetails")]
            navigator_user_agent_issue_details: NavigatorUserAgentIssueDetails,
            /**
             * This struct holds a list of optional fields with additional information
             * specific to the kind of issue. When adding a new issue code, please also
             * add a new optional field to this type.
            */
            #[serde(rename = "genericIssueDetails")]
            generic_issue_details: GenericIssueDetails,
            /**
             * This struct holds a list of optional fields with additional information
             * specific to the kind of issue. When adding a new issue code, please also
             * add a new optional field to this type.
            */
            #[serde(rename = "deprecationIssueDetails")]
            deprecation_issue_details: DeprecationIssueDetails,
            /**
             * This struct holds a list of optional fields with additional information
             * specific to the kind of issue. When adding a new issue code, please also
             * add a new optional field to this type.
            */
            #[serde(rename = "clientHintIssueDetails")]
            client_hint_issue_details: ClientHintIssueDetails,
            /**
             * This struct holds a list of optional fields with additional information
             * specific to the kind of issue. When adding a new issue code, please also
             * add a new optional field to this type.
            */
            #[serde(rename = "federatedAuthRequestIssueDetails")]
            federated_auth_request_issue_details: FederatedAuthRequestIssueDetails,
            /**
             * This struct holds a list of optional fields with additional information
             * specific to the kind of issue. When adding a new issue code, please also
             * add a new optional field to this type.
            */
            #[serde(rename = "bounceTrackingIssueDetails")]
            bounce_tracking_issue_details: BounceTrackingIssueDetails,
            /**
             * This struct holds a list of optional fields with additional information
             * specific to the kind of issue. When adding a new issue code, please also
             * add a new optional field to this type.
            */
            #[serde(rename = "stylesheetLoadingIssueDetails")]
            stylesheet_loading_issue_details: StylesheetLoadingIssueDetails,
            /**
             * This struct holds a list of optional fields with additional information
             * specific to the kind of issue. When adding a new issue code, please also
             * add a new optional field to this type.
            */
            #[serde(rename = "federatedAuthUserInfoRequestIssueDetails")]
            federated_auth_user_info_request_issue_details:
                FederatedAuthUserInfoRequestIssueDetails,
        }

        /**
         * A unique id for a DevTools inspector issue. Allows other entities (e.g.
         * exceptions, CDP message, console messages, etc.) to reference an issue.
        */
        pub type IssueId = String;

        /**
         * An inspector issue reported from the back-end.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InspectorIssue {
            /**
             * An inspector issue reported from the back-end.
            */
            code: Option<InspectorIssueCode>,
            /**
             * An inspector issue reported from the back-end.
            */
            details: Option<InspectorIssueDetails>,
            /**
             * An inspector issue reported from the back-end.
            */
            #[serde(rename = "issueId")]
            issueid: IssueId,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum GetEncodedResponseRequestEncoding {
            #[serde(rename = "webp")]
            Webp,

            #[serde(rename = "jpeg")]
            Jpeg,

            #[serde(rename = "png")]
            Png,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetEncodedResponseRequest {
            #[serde(rename = "requestId")]
            request_id: Option<network::RequestId>,

            encoding: Option<GetEncodedResponseRequestEncoding>,

            quality: f64,

            #[serde(rename = "sizeOnly")]
            size_only: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetEncodedResponseResponse {
            body: String,

            #[serde(rename = "originalSize")]
            original_size: Option<Integer>,

            #[serde(rename = "encodedSize")]
            encoded_size: Option<Integer>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CheckContrastRequest {
            #[serde(rename = "reportAAA")]
            report_aaa: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CheckFormsIssuesResponse {
            #[serde(rename = "formIssues")]
            form_issues: Option<Vec<GenericIssueDetails>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct IssueAddedEvent {
            issue: Option<InspectorIssue>,
        }
    }

    /**
     * Defines commands and events for Autofill.
    */
    pub mod autofill {
        use super::{dom, page};
        use serde::{self, Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CreditCard {
            number: Option<String>,

            name: Option<String>,

            #[serde(rename = "expiryMonth")]
            expirymonth: Option<String>,

            #[serde(rename = "expiryYear")]
            expiryyear: Option<String>,

            cvc: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct AddressField {
            name: Option<String>,

            value: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Address {
            fields: Option<Vec<AddressField>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct TriggerRequest {
            #[serde(rename = "fieldId")]
            field_id: Option<dom::BackendNodeId>,

            #[serde(rename = "frameId")]
            frame_id: page::FrameId,

            card: Option<CreditCard>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetAddressesRequest {
            addresses: Option<Vec<Address>>,
        }
    }

    /**
     * Defines events for background web platform features.
    */
    pub mod background_service {
        use super::{network, service_worker};
        use serde::{self, Deserialize, Serialize};
        /**
         * The Background Service that will be associated with the commands/events.
         * Every Background Service operates independently, but they share the same
         * API.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum ServiceName {
            #[serde(rename = "backgroundFetch")]
            BackgroundFetch,
            #[serde(rename = "backgroundSync")]
            BackgroundSync,
            #[serde(rename = "pushMessaging")]
            PushMessaging,
            #[serde(rename = "notifications")]
            Notifications,
            #[serde(rename = "paymentHandler")]
            PaymentHandler,
            #[serde(rename = "periodicBackgroundSync")]
            PeriodicBackgroundSync,
        }

        /**
         * A key-value pair for additional event information to pass along.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EventMetadata {
            /**
             * A key-value pair for additional event information to pass along.
            */
            key: Option<String>,
            /**
             * A key-value pair for additional event information to pass along.
            */
            value: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct BackgroundServiceEvent {
            timestamp: Option<network::TimeSinceEpoch>,

            origin: Option<String>,

            #[serde(rename = "serviceWorkerRegistrationId")]
            service_worker_registration_id: Option<service_worker::RegistrationID>,

            service: Option<ServiceName>,

            #[serde(rename = "eventName")]
            event_name: Option<String>,

            #[serde(rename = "instanceId")]
            instance_id: Option<String>,

            #[serde(rename = "eventMetadata")]
            event_metadata: Option<Vec<EventMetadata>>,

            #[serde(rename = "storageKey")]
            storage_key: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct StartObservingRequest {
            service: Option<ServiceName>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct StopObservingRequest {
            service: Option<ServiceName>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetRecordingRequest {
            #[serde(rename = "shouldRecord")]
            should_record: Option<bool>,

            service: Option<ServiceName>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ClearEventsRequest {
            service: Option<ServiceName>,
        }

        /**
         * Called when the recording state for the service has been updated.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct RecordingStateChangedEvent {
            /**
             * Called when the recording state for the service has been updated.
            */
            #[serde(rename = "isRecording")]
            is_recording: Option<bool>,
            /**
             * Called when the recording state for the service has been updated.
            */
            service: Option<ServiceName>,
        }

        /**
         * Called with all existing backgroundServiceEvents when enabled, and all new
         * events afterwards if enabled and recording.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct BackgroundServiceEventReceivedEvent {
            /**
             * Called with all existing backgroundServiceEvents when enabled, and all new
             * events afterwards if enabled and recording.
            */
            #[serde(rename = "backgroundServiceEvent")]
            background_service_event: Option<BackgroundServiceEvent>,
        }
    }

    /**
     * The Browser domain defines methods and events for browser managing.
    */
    pub mod browser {
        use super::{page, target, Integer};
        use serde::{self, Deserialize, Serialize};

        pub type BrowserContextID = String;

        pub type WindowID = Integer;

        /**
         * The state of the browser window.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum WindowState {
            #[serde(rename = "normal")]
            Normal,
            #[serde(rename = "minimized")]
            Minimized,
            #[serde(rename = "maximized")]
            Maximized,
            #[serde(rename = "fullscreen")]
            Fullscreen,
        }

        /**
         * Browser window bounds information
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Bounds {
            /**
             * Browser window bounds information
            */
            left: Integer,
            /**
             * Browser window bounds information
            */
            top: Integer,
            /**
             * Browser window bounds information
            */
            width: Integer,
            /**
             * Browser window bounds information
            */
            height: Integer,
            /**
             * Browser window bounds information
            */
            #[serde(rename = "windowState")]
            windowstate: WindowState,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum PermissionType {
            #[serde(rename = "accessibilityEvents")]
            AccessibilityEvents,
            #[serde(rename = "audioCapture")]
            AudioCapture,
            #[serde(rename = "backgroundSync")]
            BackgroundSync,
            #[serde(rename = "backgroundFetch")]
            BackgroundFetch,
            #[serde(rename = "clipboardReadWrite")]
            ClipboardReadWrite,
            #[serde(rename = "clipboardSanitizedWrite")]
            ClipboardSanitizedWrite,
            #[serde(rename = "displayCapture")]
            DisplayCapture,
            #[serde(rename = "durableStorage")]
            DurableStorage,
            #[serde(rename = "flash")]
            Flash,
            #[serde(rename = "geolocation")]
            Geolocation,
            #[serde(rename = "idleDetection")]
            IdleDetection,
            #[serde(rename = "localFonts")]
            LocalFonts,
            #[serde(rename = "midi")]
            Midi,
            #[serde(rename = "midiSysex")]
            MidiSysex,
            #[serde(rename = "nfc")]
            Nfc,
            #[serde(rename = "notifications")]
            Notifications,
            #[serde(rename = "paymentHandler")]
            PaymentHandler,
            #[serde(rename = "periodicBackgroundSync")]
            PeriodicBackgroundSync,
            #[serde(rename = "protectedMediaIdentifier")]
            ProtectedMediaIdentifier,
            #[serde(rename = "sensors")]
            Sensors,
            #[serde(rename = "storageAccess")]
            StorageAccess,
            #[serde(rename = "topLevelStorageAccess")]
            TopLevelStorageAccess,
            #[serde(rename = "videoCapture")]
            VideoCapture,
            #[serde(rename = "videoCapturePanTiltZoom")]
            VideoCapturePanTiltZoom,
            #[serde(rename = "wakeLockScreen")]
            WakeLockScreen,
            #[serde(rename = "wakeLockSystem")]
            WakeLockSystem,
            #[serde(rename = "windowManagement")]
            WindowManagement,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum PermissionSetting {
            #[serde(rename = "granted")]
            Granted,
            #[serde(rename = "denied")]
            Denied,
            #[serde(rename = "prompt")]
            Prompt,
        }

        /**
         * Definition of PermissionDescriptor defined in the Permissions API:
         * https://w3c.github.io/permissions/#dictdef-permissiondescriptor.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PermissionDescriptor {
            /**
             * Definition of PermissionDescriptor defined in the Permissions API:
             * https://w3c.github.io/permissions/#dictdef-permissiondescriptor.
            */
            name: Option<String>,
            /**
             * Definition of PermissionDescriptor defined in the Permissions API:
             * https://w3c.github.io/permissions/#dictdef-permissiondescriptor.
            */
            sysex: bool,
            /**
             * Definition of PermissionDescriptor defined in the Permissions API:
             * https://w3c.github.io/permissions/#dictdef-permissiondescriptor.
            */
            #[serde(rename = "userVisibleOnly")]
            user_visible_only: bool,
            /**
             * Definition of PermissionDescriptor defined in the Permissions API:
             * https://w3c.github.io/permissions/#dictdef-permissiondescriptor.
            */
            #[serde(rename = "allowWithoutSanitization")]
            allow_without_sanitization: bool,
            /**
             * Definition of PermissionDescriptor defined in the Permissions API:
             * https://w3c.github.io/permissions/#dictdef-permissiondescriptor.
            */
            #[serde(rename = "panTiltZoom")]
            pan_tilt_zoom: bool,
        }

        /**
         * Browser command ids used by executeBrowserCommand.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum BrowserCommandId {
            #[serde(rename = "openTabSearch")]
            OpenTabSearch,
            #[serde(rename = "closeTabSearch")]
            CloseTabSearch,
        }

        /**
         * Chrome histogram bucket.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Bucket {
            /**
             * Chrome histogram bucket.
            */
            low: Option<Integer>,
            /**
             * Chrome histogram bucket.
            */
            high: Option<Integer>,
            /**
             * Chrome histogram bucket.
            */
            count: Option<Integer>,
        }

        /**
         * Chrome histogram.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Histogram {
            /**
             * Chrome histogram.
            */
            name: Option<String>,
            /**
             * Chrome histogram.
            */
            sum: Option<Integer>,
            /**
             * Chrome histogram.
            */
            count: Option<Integer>,
            /**
             * Chrome histogram.
            */
            buckets: Option<Vec<Bucket>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetPermissionRequest {
            permission: Option<PermissionDescriptor>,

            setting: Option<PermissionSetting>,

            origin: String,

            #[serde(rename = "browserContextId")]
            browsercontextid: BrowserContextID,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GrantPermissionsRequest {
            permissions: Option<Vec<PermissionType>>,

            origin: String,

            #[serde(rename = "browserContextId")]
            browsercontextid: BrowserContextID,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ResetPermissionsRequest {
            #[serde(rename = "browserContextId")]
            browser_context_id: BrowserContextID,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum SetDownloadBehaviorRequestBehavior {
            #[serde(rename = "deny")]
            Deny,

            #[serde(rename = "allow")]
            Allow,

            #[serde(rename = "allowAndName")]
            AllowAndName,

            #[serde(rename = "default")]
            Default,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetDownloadBehaviorRequest {
            behavior: Option<SetDownloadBehaviorRequestBehavior>,

            #[serde(rename = "browserContextId")]
            browser_context_id: BrowserContextID,

            #[serde(rename = "downloadPath")]
            download_path: String,

            #[serde(rename = "eventsEnabled")]
            events_enabled: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CancelDownloadRequest {
            guid: Option<String>,

            #[serde(rename = "browserContextId")]
            browsercontextid: BrowserContextID,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetVersionResponse {
            #[serde(rename = "protocolVersion")]
            protocol_version: Option<String>,

            product: Option<String>,

            revision: Option<String>,

            #[serde(rename = "userAgent")]
            user_agent: Option<String>,

            #[serde(rename = "jsVersion")]
            js_version: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetBrowserCommandLineResponse {
            arguments: Option<Vec<String>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetHistogramsRequest {
            query: String,

            delta: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetHistogramsResponse {
            histograms: Option<Vec<Histogram>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetHistogramRequest {
            name: Option<String>,

            delta: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetHistogramResponse {
            histogram: Option<Histogram>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetWindowBoundsRequest {
            #[serde(rename = "windowId")]
            window_id: Option<WindowID>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetWindowBoundsResponse {
            bounds: Option<Bounds>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetWindowForTargetRequest {
            #[serde(rename = "targetId")]
            target_id: target::TargetID,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetWindowForTargetResponse {
            #[serde(rename = "windowId")]
            window_id: Option<WindowID>,

            bounds: Option<Bounds>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetWindowBoundsRequest {
            #[serde(rename = "windowId")]
            window_id: Option<WindowID>,

            bounds: Option<Bounds>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetDockTileRequest {
            #[serde(rename = "badgeLabel")]
            badge_label: String,

            image: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ExecuteBrowserCommandRequest {
            #[serde(rename = "commandId")]
            command_id: Option<BrowserCommandId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct AddPrivacySandboxEnrollmentOverrideRequest {
            url: Option<String>,
        }

        /**
         * Fired when page is about to start a download.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct DownloadWillBeginEvent {
            /**
             * Fired when page is about to start a download.
            */
            #[serde(rename = "frameId")]
            frame_id: Option<page::FrameId>,
            /**
             * Fired when page is about to start a download.
            */
            guid: Option<String>,
            /**
             * Fired when page is about to start a download.
            */
            url: Option<String>,
            /**
             * Fired when page is about to start a download.
            */
            #[serde(rename = "suggestedFilename")]
            suggested_filename: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum DownloadProgressEventState {
            #[serde(rename = "inProgress")]
            InProgress,

            #[serde(rename = "completed")]
            Completed,

            #[serde(rename = "canceled")]
            Canceled,
        }

        /**
         * Fired when download makes progress. Last call has |done| == true.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct DownloadProgressEvent {
            /**
             * Fired when download makes progress. Last call has |done| == true.
            */
            guid: Option<String>,
            /**
             * Fired when download makes progress. Last call has |done| == true.
            */
            #[serde(rename = "totalBytes")]
            totalbytes: Option<f64>,
            /**
             * Fired when download makes progress. Last call has |done| == true.
            */
            #[serde(rename = "receivedBytes")]
            receivedbytes: Option<f64>,
            /**
             * Fired when download makes progress. Last call has |done| == true.
            */
            state: Option<DownloadProgressEventState>,
        }
    }

    /**
     * This domain exposes CSS read/write operations. All CSS objects (stylesheets, rules, and styles)
     * have an associated `id` used in subsequent operations on the related object. Each object type has
     * a specific `id` structure, and those are not interchangeable between objects of different kinds.
     * CSS objects can be loaded using the `get*ForNode()` calls (which accept a DOM node id). A client
     * can also keep track of stylesheets via the `styleSheetAdded`/`styleSheetRemoved` events and
     * subsequently load the required stylesheet contents using the `getStyleSheet[Text]()` methods.
    */
    pub mod css {
        use super::{dom, page, Integer};
        use serde::{self, Deserialize, Serialize};

        pub type StyleSheetId = String;

        /**
         * Stylesheet type: "injected" for stylesheets injected via extension, "user-agent" for user-agent
         * stylesheets, "inspector" for stylesheets created by the inspector (i.e. those holding the "via
         * inspector" rules), "regular" for regular stylesheets.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum StyleSheetOrigin {
            #[serde(rename = "injected")]
            Injected,
            #[serde(rename = "user-agent")]
            UserAgent,
            #[serde(rename = "inspector")]
            Inspector,
            #[serde(rename = "regular")]
            Regular,
        }

        /**
         * CSS rule collection for a single pseudo style.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PseudoElementMatches {
            /**
             * CSS rule collection for a single pseudo style.
            */
            #[serde(rename = "pseudoType")]
            pseudo_type: Option<dom::PseudoType>,
            /**
             * CSS rule collection for a single pseudo style.
            */
            #[serde(rename = "pseudoIdentifier")]
            pseudo_identifier: String,
            /**
             * CSS rule collection for a single pseudo style.
            */
            matches: Option<Vec<RuleMatch>>,
        }

        /**
         * Inherited CSS rule collection from ancestor node.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InheritedStyleEntry {
            /**
             * Inherited CSS rule collection from ancestor node.
            */
            #[serde(rename = "inlineStyle")]
            inline_style: CSSStyle,
            /**
             * Inherited CSS rule collection from ancestor node.
            */
            #[serde(rename = "matchedCSSRules")]
            matched_cssrules: Option<Vec<RuleMatch>>,
        }

        /**
         * Inherited pseudo element matches from pseudos of an ancestor node.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InheritedPseudoElementMatches {
            /**
             * Inherited pseudo element matches from pseudos of an ancestor node.
            */
            #[serde(rename = "pseudoElements")]
            pseudo_elements: Option<Vec<PseudoElementMatches>>,
        }

        /**
         * Match data for a CSS rule.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct RuleMatch {
            /**
             * Match data for a CSS rule.
            */
            rule: Option<CSSRule>,
            /**
             * Match data for a CSS rule.
            */
            #[serde(rename = "matchingSelectors")]
            matchingselectors: Option<Vec<Integer>>,
        }

        /**
         * Data for a simple selector (these are delimited by commas in a selector list).
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Value {
            /**
             * Data for a simple selector (these are delimited by commas in a selector list).
            */
            text: Option<String>,
            /**
             * Data for a simple selector (these are delimited by commas in a selector list).
            */
            range: SourceRange,
            /**
             * Data for a simple selector (these are delimited by commas in a selector list).
            */
            specificity: Specificity,
        }

        /**
         * Specificity:
         * https://drafts.csswg.org/selectors/#specificity-rules
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Specificity {
            /**
             * Specificity:
             * https://drafts.csswg.org/selectors/#specificity-rules
            */
            a: Option<Integer>,
            /**
             * Specificity:
             * https://drafts.csswg.org/selectors/#specificity-rules
            */
            b: Option<Integer>,
            /**
             * Specificity:
             * https://drafts.csswg.org/selectors/#specificity-rules
            */
            c: Option<Integer>,
        }

        /**
         * Selector list data.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SelectorList {
            /**
             * Selector list data.
            */
            selectors: Option<Vec<Value>>,
            /**
             * Selector list data.
            */
            text: Option<String>,
        }

        /**
         * CSS stylesheet metainformation.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct CSSStyleSheetHeader {
            /**
             * CSS stylesheet metainformation.
            */
            #[serde(rename = "styleSheetId")]
            style_sheet_id: Option<StyleSheetId>,
            /**
             * CSS stylesheet metainformation.
            */
            #[serde(rename = "frameId")]
            frame_id: Option<page::FrameId>,
            /**
             * CSS stylesheet metainformation.
            */
            #[serde(rename = "sourceURL")]
            source_url: Option<String>,
            /**
             * CSS stylesheet metainformation.
            */
            #[serde(rename = "sourceMapURL")]
            source_map_url: String,
            /**
             * CSS stylesheet metainformation.
            */
            origin: Option<StyleSheetOrigin>,
            /**
             * CSS stylesheet metainformation.
            */
            title: Option<String>,
            /**
             * CSS stylesheet metainformation.
            */
            #[serde(rename = "ownerNode")]
            owner_node: dom::BackendNodeId,
            /**
             * CSS stylesheet metainformation.
            */
            disabled: Option<bool>,
            /**
             * CSS stylesheet metainformation.
            */
            #[serde(rename = "hasSourceURL")]
            has_source_url: bool,
            /**
             * CSS stylesheet metainformation.
            */
            #[serde(rename = "isInline")]
            is_inline: Option<bool>,
            /**
             * CSS stylesheet metainformation.
            */
            #[serde(rename = "isMutable")]
            is_mutable: Option<bool>,
            /**
             * CSS stylesheet metainformation.
            */
            #[serde(rename = "isConstructed")]
            is_constructed: Option<bool>,
            /**
             * CSS stylesheet metainformation.
            */
            #[serde(rename = "startLine")]
            start_line: Option<f64>,
            /**
             * CSS stylesheet metainformation.
            */
            #[serde(rename = "startColumn")]
            start_column: Option<f64>,
            /**
             * CSS stylesheet metainformation.
            */
            length: Option<f64>,
            /**
             * CSS stylesheet metainformation.
            */
            #[serde(rename = "endLine")]
            end_line: Option<f64>,
            /**
             * CSS stylesheet metainformation.
            */
            #[serde(rename = "endColumn")]
            end_column: Option<f64>,
            /**
             * CSS stylesheet metainformation.
            */
            #[serde(rename = "loadingFailed")]
            loading_failed: bool,
        }

        /**
         * CSS rule representation.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct CSSRule {
            /**
             * CSS rule representation.
            */
            #[serde(rename = "styleSheetId")]
            stylesheetid: StyleSheetId,
            /**
             * CSS rule representation.
            */
            #[serde(rename = "selectorList")]
            selectorlist: Option<SelectorList>,
            /**
             * CSS rule representation.
            */
            #[serde(rename = "nestingSelectors")]
            nestingselectors: Vec<String>,
            /**
             * CSS rule representation.
            */
            origin: Option<StyleSheetOrigin>,
            /**
             * CSS rule representation.
            */
            style: Option<CSSStyle>,
            /**
             * CSS rule representation.
            */
            media: Vec<CSSMedia>,
            /**
             * CSS rule representation.
            */
            #[serde(rename = "containerQueries")]
            containerqueries: Vec<CSSContainerQuery>,
            /**
             * CSS rule representation.
            */
            supports: Vec<CSSSupports>,
            /**
             * CSS rule representation.
            */
            layers: Vec<CSSLayer>,
            /**
             * CSS rule representation.
            */
            scopes: Vec<CSSScope>,
            /**
             * CSS rule representation.
            */
            #[serde(rename = "ruleTypes")]
            ruletypes: Vec<CSSRuleType>,
        }

        /**
         * Enum indicating the type of a CSS rule, used to represent the order of a style rule's ancestors.
         * This list only contains rule types that are collected during the ancestor rule collection.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum CSSRuleType {
            #[serde(rename = "MediaRule")]
            MediaRule,
            #[serde(rename = "SupportsRule")]
            SupportsRule,
            #[serde(rename = "ContainerRule")]
            ContainerRule,
            #[serde(rename = "LayerRule")]
            LayerRule,
            #[serde(rename = "ScopeRule")]
            ScopeRule,
            #[serde(rename = "StyleRule")]
            StyleRule,
        }

        /**
         * CSS coverage information.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct RuleUsage {
            /**
             * CSS coverage information.
            */
            #[serde(rename = "styleSheetId")]
            style_sheet_id: Option<StyleSheetId>,
            /**
             * CSS coverage information.
            */
            #[serde(rename = "startOffset")]
            start_offset: Option<f64>,
            /**
             * CSS coverage information.
            */
            #[serde(rename = "endOffset")]
            end_offset: Option<f64>,
            /**
             * CSS coverage information.
            */
            used: Option<bool>,
        }

        /**
         * Text range within a resource. All numbers are zero-based.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SourceRange {
            /**
             * Text range within a resource. All numbers are zero-based.
            */
            #[serde(rename = "startLine")]
            start_line: Option<Integer>,
            /**
             * Text range within a resource. All numbers are zero-based.
            */
            #[serde(rename = "startColumn")]
            start_column: Option<Integer>,
            /**
             * Text range within a resource. All numbers are zero-based.
            */
            #[serde(rename = "endLine")]
            end_line: Option<Integer>,
            /**
             * Text range within a resource. All numbers are zero-based.
            */
            #[serde(rename = "endColumn")]
            end_column: Option<Integer>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ShorthandEntry {
            name: Option<String>,

            value: Option<String>,

            important: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CSSComputedStyleProperty {
            name: Option<String>,

            value: Option<String>,
        }

        /**
         * CSS style representation.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct CSSStyle {
            /**
             * CSS style representation.
            */
            #[serde(rename = "styleSheetId")]
            style_sheet_id: StyleSheetId,
            /**
             * CSS style representation.
            */
            #[serde(rename = "cssProperties")]
            css_properties: Option<Vec<CSSProperty>>,
            /**
             * CSS style representation.
            */
            #[serde(rename = "shorthandEntries")]
            shorthand_entries: Option<Vec<ShorthandEntry>>,
            /**
             * CSS style representation.
            */
            #[serde(rename = "cssText")]
            css_text: String,
            /**
             * CSS style representation.
            */
            range: SourceRange,
        }

        /**
         * CSS property declaration data.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct CSSProperty {
            /**
             * CSS property declaration data.
            */
            name: Option<String>,
            /**
             * CSS property declaration data.
            */
            value: Option<String>,
            /**
             * CSS property declaration data.
            */
            important: bool,
            /**
             * CSS property declaration data.
            */
            implicit: bool,
            /**
             * CSS property declaration data.
            */
            text: String,
            /**
             * CSS property declaration data.
            */
            #[serde(rename = "parsedOk")]
            parsedok: bool,
            /**
             * CSS property declaration data.
            */
            disabled: bool,
            /**
             * CSS property declaration data.
            */
            range: SourceRange,
            /**
             * CSS property declaration data.
            */
            #[serde(rename = "longhandProperties")]
            longhandproperties: Vec<CSSProperty>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum CSSMediaSource {
            #[serde(rename = "mediaRule")]
            MediaRule,

            #[serde(rename = "importRule")]
            ImportRule,

            #[serde(rename = "linkedSheet")]
            LinkedSheet,

            #[serde(rename = "inlineSheet")]
            InlineSheet,
        }

        /**
         * CSS media rule descriptor.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct CSSMedia {
            /**
             * CSS media rule descriptor.
            */
            text: Option<String>,
            /**
             * CSS media rule descriptor.
            */
            source: Option<CSSMediaSource>,
            /**
             * CSS media rule descriptor.
            */
            #[serde(rename = "sourceURL")]
            sourceurl: String,
            /**
             * CSS media rule descriptor.
            */
            range: SourceRange,
            /**
             * CSS media rule descriptor.
            */
            #[serde(rename = "styleSheetId")]
            stylesheetid: StyleSheetId,
            /**
             * CSS media rule descriptor.
            */
            #[serde(rename = "mediaList")]
            medialist: Vec<MediaQuery>,
        }

        /**
         * Media query descriptor.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct MediaQuery {
            /**
             * Media query descriptor.
            */
            expressions: Option<Vec<MediaQueryExpression>>,
            /**
             * Media query descriptor.
            */
            active: Option<bool>,
        }

        /**
         * Media query expression descriptor.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct MediaQueryExpression {
            /**
             * Media query expression descriptor.
            */
            value: Option<f64>,
            /**
             * Media query expression descriptor.
            */
            unit: Option<String>,
            /**
             * Media query expression descriptor.
            */
            feature: Option<String>,
            /**
             * Media query expression descriptor.
            */
            #[serde(rename = "valueRange")]
            valuerange: SourceRange,
            /**
             * Media query expression descriptor.
            */
            #[serde(rename = "computedLength")]
            computedlength: f64,
        }

        /**
         * CSS container query rule descriptor.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct CSSContainerQuery {
            /**
             * CSS container query rule descriptor.
            */
            text: Option<String>,
            /**
             * CSS container query rule descriptor.
            */
            range: SourceRange,
            /**
             * CSS container query rule descriptor.
            */
            #[serde(rename = "styleSheetId")]
            stylesheetid: StyleSheetId,
            /**
             * CSS container query rule descriptor.
            */
            name: String,
            /**
             * CSS container query rule descriptor.
            */
            #[serde(rename = "physicalAxes")]
            physicalaxes: dom::PhysicalAxes,
            /**
             * CSS container query rule descriptor.
            */
            #[serde(rename = "logicalAxes")]
            logicalaxes: dom::LogicalAxes,
        }

        /**
         * CSS Supports at-rule descriptor.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct CSSSupports {
            /**
             * CSS Supports at-rule descriptor.
            */
            text: Option<String>,
            /**
             * CSS Supports at-rule descriptor.
            */
            active: Option<bool>,
            /**
             * CSS Supports at-rule descriptor.
            */
            range: SourceRange,
            /**
             * CSS Supports at-rule descriptor.
            */
            #[serde(rename = "styleSheetId")]
            stylesheetid: StyleSheetId,
        }

        /**
         * CSS Scope at-rule descriptor.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct CSSScope {
            /**
             * CSS Scope at-rule descriptor.
            */
            text: Option<String>,
            /**
             * CSS Scope at-rule descriptor.
            */
            range: SourceRange,
            /**
             * CSS Scope at-rule descriptor.
            */
            #[serde(rename = "styleSheetId")]
            stylesheetid: StyleSheetId,
        }

        /**
         * CSS Layer at-rule descriptor.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct CSSLayer {
            /**
             * CSS Layer at-rule descriptor.
            */
            text: Option<String>,
            /**
             * CSS Layer at-rule descriptor.
            */
            range: SourceRange,
            /**
             * CSS Layer at-rule descriptor.
            */
            #[serde(rename = "styleSheetId")]
            stylesheetid: StyleSheetId,
        }

        /**
         * CSS Layer data.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct CSSLayerData {
            /**
             * CSS Layer data.
            */
            name: Option<String>,
            /**
             * CSS Layer data.
            */
            #[serde(rename = "subLayers")]
            sublayers: Vec<CSSLayerData>,
            /**
             * CSS Layer data.
            */
            order: Option<f64>,
        }

        /**
         * Information about amount of glyphs that were rendered with given font.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PlatformFontUsage {
            /**
             * Information about amount of glyphs that were rendered with given font.
            */
            #[serde(rename = "familyName")]
            family_name: Option<String>,
            /**
             * Information about amount of glyphs that were rendered with given font.
            */
            #[serde(rename = "isCustomFont")]
            is_custom_font: Option<bool>,
            /**
             * Information about amount of glyphs that were rendered with given font.
            */
            #[serde(rename = "glyphCount")]
            glyph_count: Option<f64>,
        }

        /**
         * Information about font variation axes for variable fonts
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FontVariationAxis {
            /**
             * Information about font variation axes for variable fonts
            */
            tag: Option<String>,
            /**
             * Information about font variation axes for variable fonts
            */
            name: Option<String>,
            /**
             * Information about font variation axes for variable fonts
            */
            #[serde(rename = "minValue")]
            min_value: Option<f64>,
            /**
             * Information about font variation axes for variable fonts
            */
            #[serde(rename = "maxValue")]
            max_value: Option<f64>,
            /**
             * Information about font variation axes for variable fonts
            */
            #[serde(rename = "defaultValue")]
            default_value: Option<f64>,
        }

        /**
         * Properties of a web font: https://www.w3.org/TR/2008/REC-CSS2-20080411/fonts.html#font-descriptions
         * and additional information such as platformFontFamily and fontVariationAxes.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FontFace {
            /**
             * Properties of a web font: https://www.w3.org/TR/2008/REC-CSS2-20080411/fonts.html#font-descriptions
             * and additional information such as platformFontFamily and fontVariationAxes.
            */
            #[serde(rename = "fontFamily")]
            font_family: Option<String>,
            /**
             * Properties of a web font: https://www.w3.org/TR/2008/REC-CSS2-20080411/fonts.html#font-descriptions
             * and additional information such as platformFontFamily and fontVariationAxes.
            */
            #[serde(rename = "fontStyle")]
            font_style: Option<String>,
            /**
             * Properties of a web font: https://www.w3.org/TR/2008/REC-CSS2-20080411/fonts.html#font-descriptions
             * and additional information such as platformFontFamily and fontVariationAxes.
            */
            #[serde(rename = "fontVariant")]
            font_variant: Option<String>,
            /**
             * Properties of a web font: https://www.w3.org/TR/2008/REC-CSS2-20080411/fonts.html#font-descriptions
             * and additional information such as platformFontFamily and fontVariationAxes.
            */
            #[serde(rename = "fontWeight")]
            font_weight: Option<String>,
            /**
             * Properties of a web font: https://www.w3.org/TR/2008/REC-CSS2-20080411/fonts.html#font-descriptions
             * and additional information such as platformFontFamily and fontVariationAxes.
            */
            #[serde(rename = "fontStretch")]
            font_stretch: Option<String>,
            /**
             * Properties of a web font: https://www.w3.org/TR/2008/REC-CSS2-20080411/fonts.html#font-descriptions
             * and additional information such as platformFontFamily and fontVariationAxes.
            */
            #[serde(rename = "fontDisplay")]
            font_display: Option<String>,
            /**
             * Properties of a web font: https://www.w3.org/TR/2008/REC-CSS2-20080411/fonts.html#font-descriptions
             * and additional information such as platformFontFamily and fontVariationAxes.
            */
            #[serde(rename = "unicodeRange")]
            unicode_range: Option<String>,
            /**
             * Properties of a web font: https://www.w3.org/TR/2008/REC-CSS2-20080411/fonts.html#font-descriptions
             * and additional information such as platformFontFamily and fontVariationAxes.
            */
            src: Option<String>,
            /**
             * Properties of a web font: https://www.w3.org/TR/2008/REC-CSS2-20080411/fonts.html#font-descriptions
             * and additional information such as platformFontFamily and fontVariationAxes.
            */
            #[serde(rename = "platformFontFamily")]
            platform_font_family: Option<String>,
            /**
             * Properties of a web font: https://www.w3.org/TR/2008/REC-CSS2-20080411/fonts.html#font-descriptions
             * and additional information such as platformFontFamily and fontVariationAxes.
            */
            #[serde(rename = "fontVariationAxes")]
            font_variation_axes: Vec<FontVariationAxis>,
        }

        /**
         * CSS try rule representation.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct CSSTryRule {
            /**
             * CSS try rule representation.
            */
            #[serde(rename = "styleSheetId")]
            stylesheetid: StyleSheetId,
            /**
             * CSS try rule representation.
            */
            origin: Option<StyleSheetOrigin>,
            /**
             * CSS try rule representation.
            */
            style: Option<CSSStyle>,
        }

        /**
         * CSS position-fallback rule representation.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct CSSPositionFallbackRule {
            /**
             * CSS position-fallback rule representation.
            */
            name: Option<Value>,
            /**
             * CSS position-fallback rule representation.
            */
            #[serde(rename = "tryRules")]
            tryrules: Option<Vec<CSSTryRule>>,
        }

        /**
         * CSS keyframes rule representation.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct CSSKeyframesRule {
            /**
             * CSS keyframes rule representation.
            */
            #[serde(rename = "animationName")]
            animation_name: Option<Value>,
            /**
             * CSS keyframes rule representation.
            */
            keyframes: Option<Vec<CSSKeyframeRule>>,
        }

        /**
         * CSS keyframe rule representation.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct CSSKeyframeRule {
            /**
             * CSS keyframe rule representation.
            */
            #[serde(rename = "styleSheetId")]
            style_sheet_id: StyleSheetId,
            /**
             * CSS keyframe rule representation.
            */
            origin: Option<StyleSheetOrigin>,
            /**
             * CSS keyframe rule representation.
            */
            #[serde(rename = "keyText")]
            key_text: Option<Value>,
            /**
             * CSS keyframe rule representation.
            */
            style: Option<CSSStyle>,
        }

        /**
         * A descriptor of operation to mutate style declaration text.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct StyleDeclarationEdit {
            /**
             * A descriptor of operation to mutate style declaration text.
            */
            #[serde(rename = "styleSheetId")]
            stylesheetid: Option<StyleSheetId>,
            /**
             * A descriptor of operation to mutate style declaration text.
            */
            range: Option<SourceRange>,
            /**
             * A descriptor of operation to mutate style declaration text.
            */
            text: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct AddRuleRequest {
            #[serde(rename = "styleSheetId")]
            style_sheet_id: Option<StyleSheetId>,

            #[serde(rename = "ruleText")]
            rule_text: Option<String>,

            location: Option<SourceRange>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct AddRuleResponse {
            rule: Option<CSSRule>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CollectClassNamesRequest {
            #[serde(rename = "styleSheetId")]
            style_sheet_id: Option<StyleSheetId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CollectClassNamesResponse {
            #[serde(rename = "classNames")]
            class_names: Option<Vec<String>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CreateStyleSheetRequest {
            #[serde(rename = "frameId")]
            frame_id: Option<page::FrameId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CreateStyleSheetResponse {
            #[serde(rename = "styleSheetId")]
            style_sheet_id: Option<StyleSheetId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ForcePseudoStateRequest {
            #[serde(rename = "nodeId")]
            node_id: Option<dom::NodeId>,

            #[serde(rename = "forcedPseudoClasses")]
            forced_pseudo_classes: Option<Vec<String>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetBackgroundColorsRequest {
            #[serde(rename = "nodeId")]
            node_id: Option<dom::NodeId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetBackgroundColorsResponse {
            #[serde(rename = "backgroundColors")]
            background_colors: Vec<String>,

            #[serde(rename = "computedFontSize")]
            computed_font_size: String,

            #[serde(rename = "computedFontWeight")]
            computed_font_weight: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetComputedStyleForNodeRequest {
            #[serde(rename = "nodeId")]
            node_id: Option<dom::NodeId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetComputedStyleForNodeResponse {
            #[serde(rename = "computedStyle")]
            computed_style: Option<Vec<CSSComputedStyleProperty>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetInlineStylesForNodeRequest {
            #[serde(rename = "nodeId")]
            node_id: Option<dom::NodeId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetInlineStylesForNodeResponse {
            #[serde(rename = "inlineStyle")]
            inline_style: CSSStyle,

            #[serde(rename = "attributesStyle")]
            attributes_style: CSSStyle,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetMatchedStylesForNodeRequest {
            #[serde(rename = "nodeId")]
            node_id: Option<dom::NodeId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetMatchedStylesForNodeResponse {
            #[serde(rename = "inlineStyle")]
            inline_style: CSSStyle,

            #[serde(rename = "attributesStyle")]
            attributes_style: CSSStyle,

            #[serde(rename = "matchedCSSRules")]
            matched_cssrules: Vec<RuleMatch>,

            #[serde(rename = "pseudoElements")]
            pseudo_elements: Vec<PseudoElementMatches>,

            inherited: Vec<InheritedStyleEntry>,

            #[serde(rename = "inheritedPseudoElements")]
            inherited_pseudo_elements: Vec<InheritedPseudoElementMatches>,

            #[serde(rename = "cssKeyframesRules")]
            css_keyframes_rules: Vec<CSSKeyframesRule>,

            #[serde(rename = "cssPositionFallbackRules")]
            css_position_fallback_rules: Vec<CSSPositionFallbackRule>,

            #[serde(rename = "parentLayoutNodeId")]
            parent_layout_node_id: dom::NodeId,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetMediaQueriesResponse {
            medias: Option<Vec<CSSMedia>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetPlatformFontsForNodeRequest {
            #[serde(rename = "nodeId")]
            node_id: Option<dom::NodeId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetPlatformFontsForNodeResponse {
            fonts: Option<Vec<PlatformFontUsage>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetStyleSheetTextRequest {
            #[serde(rename = "styleSheetId")]
            style_sheet_id: Option<StyleSheetId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetStyleSheetTextResponse {
            text: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetLayersForNodeRequest {
            #[serde(rename = "nodeId")]
            node_id: Option<dom::NodeId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetLayersForNodeResponse {
            #[serde(rename = "rootLayer")]
            root_layer: Option<CSSLayerData>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct TrackComputedStyleUpdatesRequest {
            #[serde(rename = "propertiesToTrack")]
            properties_to_track: Option<Vec<CSSComputedStyleProperty>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct TakeComputedStyleUpdatesResponse {
            #[serde(rename = "nodeIds")]
            node_ids: Option<Vec<dom::NodeId>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetEffectivePropertyValueForNodeRequest {
            #[serde(rename = "nodeId")]
            node_id: Option<dom::NodeId>,

            #[serde(rename = "propertyName")]
            property_name: Option<String>,

            value: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetKeyframeKeyRequest {
            #[serde(rename = "styleSheetId")]
            style_sheet_id: Option<StyleSheetId>,

            range: Option<SourceRange>,

            #[serde(rename = "keyText")]
            key_text: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetKeyframeKeyResponse {
            #[serde(rename = "keyText")]
            key_text: Option<Value>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetMediaTextRequest {
            #[serde(rename = "styleSheetId")]
            stylesheetid: Option<StyleSheetId>,

            range: Option<SourceRange>,

            text: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetMediaTextResponse {
            media: Option<CSSMedia>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetContainerQueryTextRequest {
            #[serde(rename = "styleSheetId")]
            stylesheetid: Option<StyleSheetId>,

            range: Option<SourceRange>,

            text: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetContainerQueryTextResponse {
            #[serde(rename = "containerQuery")]
            container_query: Option<CSSContainerQuery>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetSupportsTextRequest {
            #[serde(rename = "styleSheetId")]
            stylesheetid: Option<StyleSheetId>,

            range: Option<SourceRange>,

            text: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetSupportsTextResponse {
            supports: Option<CSSSupports>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetScopeTextRequest {
            #[serde(rename = "styleSheetId")]
            stylesheetid: Option<StyleSheetId>,

            range: Option<SourceRange>,

            text: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetScopeTextResponse {
            scope: Option<CSSScope>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetRuleSelectorRequest {
            #[serde(rename = "styleSheetId")]
            stylesheetid: Option<StyleSheetId>,

            range: Option<SourceRange>,

            selector: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetRuleSelectorResponse {
            #[serde(rename = "selectorList")]
            selector_list: Option<SelectorList>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetStyleSheetTextRequest {
            #[serde(rename = "styleSheetId")]
            style_sheet_id: Option<StyleSheetId>,

            text: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetStyleSheetTextResponse {
            #[serde(rename = "sourceMapURL")]
            source_map_url: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetStyleTextsRequest {
            edits: Option<Vec<StyleDeclarationEdit>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetStyleTextsResponse {
            styles: Option<Vec<CSSStyle>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct StopRuleUsageTrackingResponse {
            #[serde(rename = "ruleUsage")]
            rule_usage: Option<Vec<RuleUsage>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct TakeCoverageDeltaResponse {
            coverage: Option<Vec<RuleUsage>>,

            timestamp: Option<f64>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetLocalFontsEnabledRequest {
            enabled: Option<bool>,
        }

        /**
         * Fires whenever a web font is updated.  A non-empty font parameter indicates a successfully loaded
         * web font.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FontsUpdatedEvent {
            /**
             * Fires whenever a web font is updated.  A non-empty font parameter indicates a successfully loaded
             * web font.
            */
            font: FontFace,
        }

        /**
         * Fired whenever an active document stylesheet is added.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct StyleSheetAddedEvent {
            /**
             * Fired whenever an active document stylesheet is added.
            */
            header: Option<CSSStyleSheetHeader>,
        }

        /**
         * Fired whenever a stylesheet is changed as a result of the client operation.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct StyleSheetChangedEvent {
            /**
             * Fired whenever a stylesheet is changed as a result of the client operation.
            */
            #[serde(rename = "styleSheetId")]
            style_sheet_id: Option<StyleSheetId>,
        }

        /**
         * Fired whenever an active document stylesheet is removed.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct StyleSheetRemovedEvent {
            /**
             * Fired whenever an active document stylesheet is removed.
            */
            #[serde(rename = "styleSheetId")]
            style_sheet_id: Option<StyleSheetId>,
        }
    }

    pub mod cache_storage {
        use super::{storage, Integer};
        use serde::{self, Deserialize, Serialize};
        /**
         * Unique identifier of the Cache object.
        */
        pub type CacheId = String;

        /**
         * type of HTTP response cached
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum CachedResponseType {
            #[serde(rename = "basic")]
            Basic,
            #[serde(rename = "cors")]
            Cors,
            #[serde(rename = "default")]
            Default,
            #[serde(rename = "error")]
            Error,
            #[serde(rename = "opaqueResponse")]
            OpaqueResponse,
            #[serde(rename = "opaqueRedirect")]
            OpaqueRedirect,
        }

        /**
         * Data entry.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct DataEntry {
            /**
             * Data entry.
            */
            #[serde(rename = "requestURL")]
            request_url: Option<String>,
            /**
             * Data entry.
            */
            #[serde(rename = "requestMethod")]
            request_method: Option<String>,
            /**
             * Data entry.
            */
            #[serde(rename = "requestHeaders")]
            request_headers: Option<Vec<Header>>,
            /**
             * Data entry.
            */
            #[serde(rename = "responseTime")]
            response_time: Option<f64>,
            /**
             * Data entry.
            */
            #[serde(rename = "responseStatus")]
            response_status: Option<Integer>,
            /**
             * Data entry.
            */
            #[serde(rename = "responseStatusText")]
            response_status_text: Option<String>,
            /**
             * Data entry.
            */
            #[serde(rename = "responseType")]
            response_type: Option<CachedResponseType>,
            /**
             * Data entry.
            */
            #[serde(rename = "responseHeaders")]
            response_headers: Option<Vec<Header>>,
        }

        /**
         * Cache identifier.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Cache {
            /**
             * Cache identifier.
            */
            #[serde(rename = "cacheId")]
            cache_id: Option<CacheId>,
            /**
             * Cache identifier.
            */
            #[serde(rename = "securityOrigin")]
            security_origin: Option<String>,
            /**
             * Cache identifier.
            */
            #[serde(rename = "storageKey")]
            storage_key: Option<String>,
            /**
             * Cache identifier.
            */
            #[serde(rename = "storageBucket")]
            storage_bucket: storage::StorageBucket,
            /**
             * Cache identifier.
            */
            #[serde(rename = "cacheName")]
            cache_name: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Header {
            name: Option<String>,

            value: Option<String>,
        }

        /**
         * Cached response
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct CachedResponse {
            /**
             * Cached response
            */
            body: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct DeleteCacheRequest {
            #[serde(rename = "cacheId")]
            cache_id: Option<CacheId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct DeleteEntryRequest {
            #[serde(rename = "cacheId")]
            cache_id: Option<CacheId>,

            request: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct RequestCacheNamesRequest {
            #[serde(rename = "securityOrigin")]
            security_origin: String,

            #[serde(rename = "storageKey")]
            storage_key: String,

            #[serde(rename = "storageBucket")]
            storage_bucket: storage::StorageBucket,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct RequestCacheNamesResponse {
            caches: Option<Vec<Cache>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct RequestCachedResponseRequest {
            #[serde(rename = "cacheId")]
            cache_id: Option<CacheId>,

            #[serde(rename = "requestURL")]
            request_url: Option<String>,

            #[serde(rename = "requestHeaders")]
            request_headers: Option<Vec<Header>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct RequestCachedResponseResponse {
            response: Option<CachedResponse>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct RequestEntriesRequest {
            #[serde(rename = "cacheId")]
            cache_id: Option<CacheId>,

            #[serde(rename = "skipCount")]
            skip_count: Integer,

            #[serde(rename = "pageSize")]
            page_size: Integer,

            #[serde(rename = "pathFilter")]
            path_filter: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct RequestEntriesResponse {
            #[serde(rename = "cacheDataEntries")]
            cache_data_entries: Option<Vec<DataEntry>>,

            #[serde(rename = "returnCount")]
            return_count: Option<f64>,
        }
    }

    /**
     * A domain for interacting with Cast, Presentation API, and Remote Playback API
     * functionalities.
    */
    pub mod cast {
        use serde::{self, Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Sink {
            name: Option<String>,

            id: Option<String>,

            session: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct EnableRequest {
            #[serde(rename = "presentationUrl")]
            presentation_url: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetSinkToUseRequest {
            #[serde(rename = "sinkName")]
            sink_name: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct StartDesktopMirroringRequest {
            #[serde(rename = "sinkName")]
            sink_name: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct StartTabMirroringRequest {
            #[serde(rename = "sinkName")]
            sink_name: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct StopCastingRequest {
            #[serde(rename = "sinkName")]
            sink_name: Option<String>,
        }

        /**
         * This is fired whenever the list of available sinks changes. A sink is a
         * device or a software surface that you can cast to.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SinksUpdatedEvent {
            /**
             * This is fired whenever the list of available sinks changes. A sink is a
             * device or a software surface that you can cast to.
            */
            sinks: Option<Vec<Sink>>,
        }

        /**
         * This is fired whenever the outstanding issue/error message changes.
         * |issueMessage| is empty if there is no issue.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct IssueUpdatedEvent {
            /**
             * This is fired whenever the outstanding issue/error message changes.
             * |issueMessage| is empty if there is no issue.
            */
            #[serde(rename = "issueMessage")]
            issue_message: Option<String>,
        }
    }

    /**
     * This domain exposes DOM read/write operations. Each DOM Node is represented with its mirror object
     * that has an `id`. This `id` can be used to get additional information on the Node, resolve it into
     * the JavaScript object wrapper, etc. It is important that client receives DOM events only for the
     * nodes that are known to the client. Backend keeps track of the nodes that were sent to the client
     * and never sends the same node twice. It is client's responsibility to collect information about
     * the nodes that were sent to the client. Note that `iframe` owner elements will return
     * corresponding document elements as their child nodes.
    */
    pub mod dom {
        use super::{dom, page, runtime, Integer};
        use serde::{self, Deserialize, Serialize};
        /**
         * Unique DOM node identifier.
        */
        pub type NodeId = Integer;

        /**
         * Unique DOM node identifier used to reference a node that may not have been pushed to the
         * front-end.
        */
        pub type BackendNodeId = Integer;

        /**
         * Backend node with a friendly name.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct BackendNode {
            /**
             * Backend node with a friendly name.
            */
            #[serde(rename = "nodeType")]
            node_type: Option<Integer>,
            /**
             * Backend node with a friendly name.
            */
            #[serde(rename = "nodeName")]
            node_name: Option<String>,
            /**
             * Backend node with a friendly name.
            */
            #[serde(rename = "backendNodeId")]
            backend_node_id: Option<BackendNodeId>,
        }

        /**
         * Pseudo element type.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum PseudoType {
            #[serde(rename = "first-line")]
            FirstLine,
            #[serde(rename = "first-letter")]
            FirstLetter,
            #[serde(rename = "before")]
            Before,
            #[serde(rename = "after")]
            After,
            #[serde(rename = "marker")]
            Marker,
            #[serde(rename = "backdrop")]
            Backdrop,
            #[serde(rename = "selection")]
            Selection,
            #[serde(rename = "target-text")]
            TargetText,
            #[serde(rename = "spelling-error")]
            SpellingError,
            #[serde(rename = "grammar-error")]
            GrammarError,
            #[serde(rename = "highlight")]
            Highlight,
            #[serde(rename = "first-line-inherited")]
            FirstLineInherited,
            #[serde(rename = "scrollbar")]
            Scrollbar,
            #[serde(rename = "scrollbar-thumb")]
            ScrollbarThumb,
            #[serde(rename = "scrollbar-button")]
            ScrollbarButton,
            #[serde(rename = "scrollbar-track")]
            ScrollbarTrack,
            #[serde(rename = "scrollbar-track-piece")]
            ScrollbarTrackPiece,
            #[serde(rename = "scrollbar-corner")]
            ScrollbarCorner,
            #[serde(rename = "resizer")]
            Resizer,
            #[serde(rename = "input-list-button")]
            InputListButton,
            #[serde(rename = "view-transition")]
            ViewTransition,
            #[serde(rename = "view-transition-group")]
            ViewTransitionGroup,
            #[serde(rename = "view-transition-image-pair")]
            ViewTransitionImagePair,
            #[serde(rename = "view-transition-old")]
            ViewTransitionOld,
            #[serde(rename = "view-transition-new")]
            ViewTransitionNew,
        }

        /**
         * Shadow root type.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum ShadowRootType {
            #[serde(rename = "user-agent")]
            UserAgent,
            #[serde(rename = "open")]
            Open,
            #[serde(rename = "closed")]
            Closed,
        }

        /**
         * Document compatibility mode.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum CompatibilityMode {
            #[serde(rename = "QuirksMode")]
            QuirksMode,
            #[serde(rename = "LimitedQuirksMode")]
            LimitedQuirksMode,
            #[serde(rename = "NoQuirksMode")]
            NoQuirksMode,
        }

        /**
         * ContainerSelector physical axes
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum PhysicalAxes {
            #[serde(rename = "Horizontal")]
            Horizontal,
            #[serde(rename = "Vertical")]
            Vertical,
            #[serde(rename = "Both")]
            Both,
        }

        /**
         * ContainerSelector logical axes
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum LogicalAxes {
            #[serde(rename = "Inline")]
            Inline,
            #[serde(rename = "Block")]
            Block,
            #[serde(rename = "Both")]
            Both,
        }

        /**
         * DOM interaction is implemented in terms of mirror objects that represent the actual DOM nodes.
         * DOMNode is a base node mirror type.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Node {
            /**
             * DOM interaction is implemented in terms of mirror objects that represent the actual DOM nodes.
             * DOMNode is a base node mirror type.
            */
            #[serde(rename = "nodeId")]
            node_id: Option<NodeId>,
            /**
             * DOM interaction is implemented in terms of mirror objects that represent the actual DOM nodes.
             * DOMNode is a base node mirror type.
            */
            #[serde(rename = "parentId")]
            parent_id: NodeId,
            /**
             * DOM interaction is implemented in terms of mirror objects that represent the actual DOM nodes.
             * DOMNode is a base node mirror type.
            */
            #[serde(rename = "backendNodeId")]
            backend_node_id: Option<BackendNodeId>,
            /**
             * DOM interaction is implemented in terms of mirror objects that represent the actual DOM nodes.
             * DOMNode is a base node mirror type.
            */
            #[serde(rename = "nodeType")]
            node_type: Option<Integer>,
            /**
             * DOM interaction is implemented in terms of mirror objects that represent the actual DOM nodes.
             * DOMNode is a base node mirror type.
            */
            #[serde(rename = "nodeName")]
            node_name: Option<String>,
            /**
             * DOM interaction is implemented in terms of mirror objects that represent the actual DOM nodes.
             * DOMNode is a base node mirror type.
            */
            #[serde(rename = "localName")]
            local_name: Option<String>,
            /**
             * DOM interaction is implemented in terms of mirror objects that represent the actual DOM nodes.
             * DOMNode is a base node mirror type.
            */
            #[serde(rename = "nodeValue")]
            node_value: Option<String>,
            /**
             * DOM interaction is implemented in terms of mirror objects that represent the actual DOM nodes.
             * DOMNode is a base node mirror type.
            */
            #[serde(rename = "childNodeCount")]
            child_node_count: Integer,
            /**
             * DOM interaction is implemented in terms of mirror objects that represent the actual DOM nodes.
             * DOMNode is a base node mirror type.
            */
            children: Vec<Box<Node>>,
            /**
             * DOM interaction is implemented in terms of mirror objects that represent the actual DOM nodes.
             * DOMNode is a base node mirror type.
            */
            attributes: Vec<String>,
            /**
             * DOM interaction is implemented in terms of mirror objects that represent the actual DOM nodes.
             * DOMNode is a base node mirror type.
            */
            #[serde(rename = "documentURL")]
            document_url: String,
            /**
             * DOM interaction is implemented in terms of mirror objects that represent the actual DOM nodes.
             * DOMNode is a base node mirror type.
            */
            #[serde(rename = "baseURL")]
            base_url: String,
            /**
             * DOM interaction is implemented in terms of mirror objects that represent the actual DOM nodes.
             * DOMNode is a base node mirror type.
            */
            #[serde(rename = "publicId")]
            public_id: String,
            /**
             * DOM interaction is implemented in terms of mirror objects that represent the actual DOM nodes.
             * DOMNode is a base node mirror type.
            */
            #[serde(rename = "systemId")]
            system_id: String,
            /**
             * DOM interaction is implemented in terms of mirror objects that represent the actual DOM nodes.
             * DOMNode is a base node mirror type.
            */
            #[serde(rename = "internalSubset")]
            internal_subset: String,
            /**
             * DOM interaction is implemented in terms of mirror objects that represent the actual DOM nodes.
             * DOMNode is a base node mirror type.
            */
            #[serde(rename = "xmlVersion")]
            xml_version: String,
            /**
             * DOM interaction is implemented in terms of mirror objects that represent the actual DOM nodes.
             * DOMNode is a base node mirror type.
            */
            name: String,
            /**
             * DOM interaction is implemented in terms of mirror objects that represent the actual DOM nodes.
             * DOMNode is a base node mirror type.
            */
            value: String,
            /**
             * DOM interaction is implemented in terms of mirror objects that represent the actual DOM nodes.
             * DOMNode is a base node mirror type.
            */
            #[serde(rename = "pseudoType")]
            pseudo_type: PseudoType,
            /**
             * DOM interaction is implemented in terms of mirror objects that represent the actual DOM nodes.
             * DOMNode is a base node mirror type.
            */
            #[serde(rename = "pseudoIdentifier")]
            pseudo_identifier: String,
            /**
             * DOM interaction is implemented in terms of mirror objects that represent the actual DOM nodes.
             * DOMNode is a base node mirror type.
            */
            #[serde(rename = "shadowRootType")]
            shadow_root_type: ShadowRootType,
            /**
             * DOM interaction is implemented in terms of mirror objects that represent the actual DOM nodes.
             * DOMNode is a base node mirror type.
            */
            #[serde(rename = "frameId")]
            frame_id: page::FrameId,
            /**
             * DOM interaction is implemented in terms of mirror objects that represent the actual DOM nodes.
             * DOMNode is a base node mirror type.
            */
            #[serde(rename = "contentDocument")]
            content_document: Box<Node>,
            /**
             * DOM interaction is implemented in terms of mirror objects that represent the actual DOM nodes.
             * DOMNode is a base node mirror type.
            */
            #[serde(rename = "shadowRoots")]
            shadow_roots: Vec<Box<Node>>,
            /**
             * DOM interaction is implemented in terms of mirror objects that represent the actual DOM nodes.
             * DOMNode is a base node mirror type.
            */
            #[serde(rename = "templateContent")]
            template_content: Box<Node>,
            /**
             * DOM interaction is implemented in terms of mirror objects that represent the actual DOM nodes.
             * DOMNode is a base node mirror type.
            */
            #[serde(rename = "pseudoElements")]
            pseudo_elements: Vec<Box<Node>>,
            /**
             * DOM interaction is implemented in terms of mirror objects that represent the actual DOM nodes.
             * DOMNode is a base node mirror type.
            */
            #[serde(rename = "importedDocument")]
            imported_document: Box<Node>,
            /**
             * DOM interaction is implemented in terms of mirror objects that represent the actual DOM nodes.
             * DOMNode is a base node mirror type.
            */
            #[serde(rename = "distributedNodes")]
            distributed_nodes: Vec<BackendNode>,
            /**
             * DOM interaction is implemented in terms of mirror objects that represent the actual DOM nodes.
             * DOMNode is a base node mirror type.
            */
            #[serde(rename = "isSVG")]
            is_svg: bool,
            /**
             * DOM interaction is implemented in terms of mirror objects that represent the actual DOM nodes.
             * DOMNode is a base node mirror type.
            */
            #[serde(rename = "compatibilityMode")]
            compatibility_mode: CompatibilityMode,
            /**
             * DOM interaction is implemented in terms of mirror objects that represent the actual DOM nodes.
             * DOMNode is a base node mirror type.
            */
            #[serde(rename = "assignedSlot")]
            assigned_slot: BackendNode,
        }

        /**
         * A structure holding an RGBA color.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct RGBA {
            /**
             * A structure holding an RGBA color.
            */
            r: Option<Integer>,
            /**
             * A structure holding an RGBA color.
            */
            g: Option<Integer>,
            /**
             * A structure holding an RGBA color.
            */
            b: Option<Integer>,
            /**
             * A structure holding an RGBA color.
            */
            a: f64,
        }

        /**
         * An array of quad vertices, x immediately followed by y for each point, points clock-wise.
        */
        pub type Quad = Vec<f64>;

        /**
         * Box model.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct BoxModel {
            /**
             * Box model.
            */
            content: Option<Quad>,
            /**
             * Box model.
            */
            padding: Option<Quad>,
            /**
             * Box model.
            */
            border: Option<Quad>,
            /**
             * Box model.
            */
            margin: Option<Quad>,
            /**
             * Box model.
            */
            width: Option<Integer>,
            /**
             * Box model.
            */
            height: Option<Integer>,
            /**
             * Box model.
            */
            #[serde(rename = "shapeOutside")]
            shapeoutside: ShapeOutsideInfo,
        }

        /**
         * CSS Shape Outside details.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ShapeOutsideInfo {
            /**
             * CSS Shape Outside details.
            */
            bounds: Option<Quad>,
            /**
             * CSS Shape Outside details.
            */
            shape: Option<Vec<serde_json::Value>>,
            /**
             * CSS Shape Outside details.
            */
            #[serde(rename = "marginShape")]
            marginshape: Option<Vec<serde_json::Value>>,
        }

        /**
         * Rectangle.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Rect {
            /**
             * Rectangle.
            */
            x: Option<f64>,
            /**
             * Rectangle.
            */
            y: Option<f64>,
            /**
             * Rectangle.
            */
            width: Option<f64>,
            /**
             * Rectangle.
            */
            height: Option<f64>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CSSComputedStyleProperty {
            name: Option<String>,

            value: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CollectClassNamesFromSubtreeRequest {
            #[serde(rename = "nodeId")]
            node_id: Option<NodeId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CollectClassNamesFromSubtreeResponse {
            #[serde(rename = "classNames")]
            class_names: Option<Vec<String>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CopyToRequest {
            #[serde(rename = "nodeId")]
            node_id: Option<NodeId>,

            #[serde(rename = "targetNodeId")]
            target_node_id: Option<NodeId>,

            #[serde(rename = "insertBeforeNodeId")]
            insert_before_node_id: NodeId,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CopyToResponse {
            #[serde(rename = "nodeId")]
            node_id: Option<NodeId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct DescribeNodeRequest {
            #[serde(rename = "nodeId")]
            node_id: NodeId,

            #[serde(rename = "backendNodeId")]
            backend_node_id: BackendNodeId,

            #[serde(rename = "objectId")]
            object_id: runtime::RemoteObjectId,

            depth: Integer,

            pierce: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct DescribeNodeResponse {
            node: Option<Node>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ScrollIntoViewIfNeededRequest {
            #[serde(rename = "nodeId")]
            node_id: NodeId,

            #[serde(rename = "backendNodeId")]
            backend_node_id: BackendNodeId,

            #[serde(rename = "objectId")]
            object_id: runtime::RemoteObjectId,

            rect: Rect,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct DiscardSearchResultsRequest {
            #[serde(rename = "searchId")]
            search_id: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum EnableRequestIncludeWhitespace {
            #[serde(rename = "none")]
            None,

            #[serde(rename = "all")]
            All,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct EnableRequest {
            #[serde(rename = "includeWhitespace")]
            include_whitespace: EnableRequestIncludeWhitespace,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct FocusRequest {
            #[serde(rename = "nodeId")]
            node_id: NodeId,

            #[serde(rename = "backendNodeId")]
            backend_node_id: BackendNodeId,

            #[serde(rename = "objectId")]
            object_id: runtime::RemoteObjectId,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetAttributesRequest {
            #[serde(rename = "nodeId")]
            node_id: Option<NodeId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetAttributesResponse {
            attributes: Option<Vec<String>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetBoxModelRequest {
            #[serde(rename = "nodeId")]
            node_id: NodeId,

            #[serde(rename = "backendNodeId")]
            backend_node_id: BackendNodeId,

            #[serde(rename = "objectId")]
            object_id: runtime::RemoteObjectId,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetBoxModelResponse {
            model: Option<BoxModel>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetContentQuadsRequest {
            #[serde(rename = "nodeId")]
            node_id: NodeId,

            #[serde(rename = "backendNodeId")]
            backend_node_id: BackendNodeId,

            #[serde(rename = "objectId")]
            object_id: runtime::RemoteObjectId,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetContentQuadsResponse {
            quads: Option<Vec<Quad>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetDocumentRequest {
            depth: Integer,

            pierce: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetDocumentResponse {
            root: Option<Node>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetFlattenedDocumentRequest {
            depth: Integer,

            pierce: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetFlattenedDocumentResponse {
            nodes: Option<Vec<Node>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetNodesForSubtreeByStyleRequest {
            #[serde(rename = "nodeId")]
            node_id: Option<NodeId>,

            #[serde(rename = "computedStyles")]
            computed_styles: Option<Vec<CSSComputedStyleProperty>>,

            pierce: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetNodesForSubtreeByStyleResponse {
            #[serde(rename = "nodeIds")]
            node_ids: Option<Vec<NodeId>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetNodeForLocationRequest {
            x: Option<Integer>,

            y: Option<Integer>,

            #[serde(rename = "includeUserAgentShadowDOM")]
            includeuseragentshadowdom: bool,

            #[serde(rename = "ignorePointerEventsNone")]
            ignorepointereventsnone: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetNodeForLocationResponse {
            #[serde(rename = "backendNodeId")]
            backend_node_id: Option<BackendNodeId>,

            #[serde(rename = "frameId")]
            frame_id: Option<page::FrameId>,

            #[serde(rename = "nodeId")]
            node_id: NodeId,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetOuterHTMLRequest {
            #[serde(rename = "nodeId")]
            node_id: NodeId,

            #[serde(rename = "backendNodeId")]
            backend_node_id: BackendNodeId,

            #[serde(rename = "objectId")]
            object_id: runtime::RemoteObjectId,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetOuterHTMLResponse {
            #[serde(rename = "outerHTML")]
            outer_html: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetRelayoutBoundaryRequest {
            #[serde(rename = "nodeId")]
            node_id: Option<NodeId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetRelayoutBoundaryResponse {
            #[serde(rename = "nodeId")]
            node_id: Option<NodeId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetSearchResultsRequest {
            #[serde(rename = "searchId")]
            search_id: Option<String>,

            #[serde(rename = "fromIndex")]
            from_index: Option<Integer>,

            #[serde(rename = "toIndex")]
            to_index: Option<Integer>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetSearchResultsResponse {
            #[serde(rename = "nodeIds")]
            node_ids: Option<Vec<NodeId>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct MoveToRequest {
            #[serde(rename = "nodeId")]
            node_id: Option<NodeId>,

            #[serde(rename = "targetNodeId")]
            target_node_id: Option<NodeId>,

            #[serde(rename = "insertBeforeNodeId")]
            insert_before_node_id: NodeId,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct MoveToResponse {
            #[serde(rename = "nodeId")]
            node_id: Option<NodeId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct PerformSearchRequest {
            query: Option<String>,

            #[serde(rename = "includeUserAgentShadowDOM")]
            includeuseragentshadowdom: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct PerformSearchResponse {
            #[serde(rename = "searchId")]
            search_id: Option<String>,

            #[serde(rename = "resultCount")]
            result_count: Option<Integer>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct PushNodeByPathToFrontendRequest {
            path: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct PushNodeByPathToFrontendResponse {
            #[serde(rename = "nodeId")]
            node_id: Option<NodeId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct PushNodesByBackendIdsToFrontendRequest {
            #[serde(rename = "backendNodeIds")]
            backend_node_ids: Option<Vec<BackendNodeId>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct PushNodesByBackendIdsToFrontendResponse {
            #[serde(rename = "nodeIds")]
            node_ids: Option<Vec<NodeId>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct QuerySelectorRequest {
            #[serde(rename = "nodeId")]
            node_id: Option<NodeId>,

            selector: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct QuerySelectorResponse {
            #[serde(rename = "nodeId")]
            node_id: Option<NodeId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct QuerySelectorAllRequest {
            #[serde(rename = "nodeId")]
            node_id: Option<NodeId>,

            selector: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct QuerySelectorAllResponse {
            #[serde(rename = "nodeIds")]
            node_ids: Option<Vec<NodeId>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetTopLayerElementsResponse {
            #[serde(rename = "nodeIds")]
            node_ids: Option<Vec<NodeId>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct RemoveAttributeRequest {
            #[serde(rename = "nodeId")]
            node_id: Option<NodeId>,

            name: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct RemoveNodeRequest {
            #[serde(rename = "nodeId")]
            node_id: Option<NodeId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct RequestChildNodesRequest {
            #[serde(rename = "nodeId")]
            nodeid: Option<NodeId>,

            depth: Integer,

            pierce: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct RequestNodeRequest {
            #[serde(rename = "objectId")]
            object_id: Option<runtime::RemoteObjectId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct RequestNodeResponse {
            #[serde(rename = "nodeId")]
            node_id: Option<NodeId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ResolveNodeRequest {
            #[serde(rename = "nodeId")]
            node_id: NodeId,

            #[serde(rename = "backendNodeId")]
            backend_node_id: dom::BackendNodeId,

            #[serde(rename = "objectGroup")]
            object_group: String,

            #[serde(rename = "executionContextId")]
            execution_context_id: runtime::ExecutionContextId,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ResolveNodeResponse {
            object: Option<runtime::RemoteObject>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetAttributeValueRequest {
            #[serde(rename = "nodeId")]
            nodeid: Option<NodeId>,

            name: Option<String>,

            value: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetAttributesAsTextRequest {
            #[serde(rename = "nodeId")]
            nodeid: Option<NodeId>,

            text: Option<String>,

            name: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetFileInputFilesRequest {
            files: Option<Vec<String>>,

            #[serde(rename = "nodeId")]
            node_id: NodeId,

            #[serde(rename = "backendNodeId")]
            backend_node_id: BackendNodeId,

            #[serde(rename = "objectId")]
            object_id: runtime::RemoteObjectId,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetNodeStackTracesEnabledRequest {
            enable: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetNodeStackTracesRequest {
            #[serde(rename = "nodeId")]
            node_id: Option<NodeId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetNodeStackTracesResponse {
            creation: runtime::StackTrace,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetFileInfoRequest {
            #[serde(rename = "objectId")]
            object_id: Option<runtime::RemoteObjectId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetFileInfoResponse {
            path: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetInspectedNodeRequest {
            #[serde(rename = "nodeId")]
            node_id: Option<NodeId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetNodeNameRequest {
            #[serde(rename = "nodeId")]
            node_id: Option<NodeId>,

            name: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetNodeNameResponse {
            #[serde(rename = "nodeId")]
            node_id: Option<NodeId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetNodeValueRequest {
            #[serde(rename = "nodeId")]
            node_id: Option<NodeId>,

            value: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetOuterHTMLRequest {
            #[serde(rename = "nodeId")]
            node_id: Option<NodeId>,

            #[serde(rename = "outerHTML")]
            outer_html: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetFrameOwnerRequest {
            #[serde(rename = "frameId")]
            frame_id: Option<page::FrameId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetFrameOwnerResponse {
            #[serde(rename = "backendNodeId")]
            backend_node_id: Option<BackendNodeId>,

            #[serde(rename = "nodeId")]
            node_id: NodeId,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetContainerForNodeRequest {
            #[serde(rename = "nodeId")]
            node_id: Option<NodeId>,

            #[serde(rename = "containerName")]
            container_name: String,

            #[serde(rename = "physicalAxes")]
            physical_axes: PhysicalAxes,

            #[serde(rename = "logicalAxes")]
            logical_axes: LogicalAxes,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetContainerForNodeResponse {
            #[serde(rename = "nodeId")]
            node_id: NodeId,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetQueryingDescendantsForContainerRequest {
            #[serde(rename = "nodeId")]
            node_id: Option<NodeId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetQueryingDescendantsForContainerResponse {
            #[serde(rename = "nodeIds")]
            node_ids: Option<Vec<NodeId>>,
        }

        /**
         * Fired when `Element`'s attribute is modified.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AttributeModifiedEvent {
            /**
             * Fired when `Element`'s attribute is modified.
            */
            #[serde(rename = "nodeId")]
            nodeid: Option<NodeId>,
            /**
             * Fired when `Element`'s attribute is modified.
            */
            name: Option<String>,
            /**
             * Fired when `Element`'s attribute is modified.
            */
            value: Option<String>,
        }

        /**
         * Fired when `Element`'s attribute is removed.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AttributeRemovedEvent {
            /**
             * Fired when `Element`'s attribute is removed.
            */
            #[serde(rename = "nodeId")]
            node_id: Option<NodeId>,
            /**
             * Fired when `Element`'s attribute is removed.
            */
            name: Option<String>,
        }

        /**
         * Mirrors `DOMCharacterDataModified` event.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct CharacterDataModifiedEvent {
            /**
             * Mirrors `DOMCharacterDataModified` event.
            */
            #[serde(rename = "nodeId")]
            node_id: Option<NodeId>,
            /**
             * Mirrors `DOMCharacterDataModified` event.
            */
            #[serde(rename = "characterData")]
            character_data: Option<String>,
        }

        /**
         * Fired when `Container`'s child node count has changed.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ChildNodeCountUpdatedEvent {
            /**
             * Fired when `Container`'s child node count has changed.
            */
            #[serde(rename = "nodeId")]
            node_id: Option<NodeId>,
            /**
             * Fired when `Container`'s child node count has changed.
            */
            #[serde(rename = "childNodeCount")]
            child_node_count: Option<Integer>,
        }

        /**
         * Mirrors `DOMNodeInserted` event.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ChildNodeInsertedEvent {
            /**
             * Mirrors `DOMNodeInserted` event.
            */
            #[serde(rename = "parentNodeId")]
            parent_node_id: Option<NodeId>,
            /**
             * Mirrors `DOMNodeInserted` event.
            */
            #[serde(rename = "previousNodeId")]
            previous_node_id: Option<NodeId>,
            /**
             * Mirrors `DOMNodeInserted` event.
            */
            node: Option<Node>,
        }

        /**
         * Mirrors `DOMNodeRemoved` event.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ChildNodeRemovedEvent {
            /**
             * Mirrors `DOMNodeRemoved` event.
            */
            #[serde(rename = "parentNodeId")]
            parent_node_id: Option<NodeId>,
            /**
             * Mirrors `DOMNodeRemoved` event.
            */
            #[serde(rename = "nodeId")]
            node_id: Option<NodeId>,
        }

        /**
         * Called when distribution is changed.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct DistributedNodesUpdatedEvent {
            /**
             * Called when distribution is changed.
            */
            #[serde(rename = "insertionPointId")]
            insertion_point_id: Option<NodeId>,
            /**
             * Called when distribution is changed.
            */
            #[serde(rename = "distributedNodes")]
            distributed_nodes: Option<Vec<BackendNode>>,
        }

        /**
         * Fired when `Element`'s inline style is modified via a CSS property modification.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InlineStyleInvalidatedEvent {
            /**
             * Fired when `Element`'s inline style is modified via a CSS property modification.
            */
            #[serde(rename = "nodeIds")]
            node_ids: Option<Vec<NodeId>>,
        }

        /**
         * Called when a pseudo element is added to an element.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PseudoElementAddedEvent {
            /**
             * Called when a pseudo element is added to an element.
            */
            #[serde(rename = "parentId")]
            parent_id: Option<NodeId>,
            /**
             * Called when a pseudo element is added to an element.
            */
            #[serde(rename = "pseudoElement")]
            pseudo_element: Option<Node>,
        }

        /**
         * Called when a pseudo element is removed from an element.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PseudoElementRemovedEvent {
            /**
             * Called when a pseudo element is removed from an element.
            */
            #[serde(rename = "parentId")]
            parent_id: Option<NodeId>,
            /**
             * Called when a pseudo element is removed from an element.
            */
            #[serde(rename = "pseudoElementId")]
            pseudo_element_id: Option<NodeId>,
        }

        /**
         * Fired when backend wants to provide client with the missing DOM structure. This happens upon
         * most of the calls requesting node ids.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetChildNodesEvent {
            /**
             * Fired when backend wants to provide client with the missing DOM structure. This happens upon
             * most of the calls requesting node ids.
            */
            #[serde(rename = "parentId")]
            parent_id: Option<NodeId>,
            /**
             * Fired when backend wants to provide client with the missing DOM structure. This happens upon
             * most of the calls requesting node ids.
            */
            nodes: Option<Vec<Node>>,
        }

        /**
         * Called when shadow root is popped from the element.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ShadowRootPoppedEvent {
            /**
             * Called when shadow root is popped from the element.
            */
            #[serde(rename = "hostId")]
            host_id: Option<NodeId>,
            /**
             * Called when shadow root is popped from the element.
            */
            #[serde(rename = "rootId")]
            root_id: Option<NodeId>,
        }

        /**
         * Called when shadow root is pushed into the element.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ShadowRootPushedEvent {
            /**
             * Called when shadow root is pushed into the element.
            */
            #[serde(rename = "hostId")]
            host_id: Option<NodeId>,
            /**
             * Called when shadow root is pushed into the element.
            */
            root: Option<Node>,
        }
    }

    /**
     * DOM debugging allows setting breakpoints on particular DOM operations and events. JavaScript
     * execution will stop on these operations as if there was a regular breakpoint set.
    */
    pub mod domdebugger {
        use super::{dom, runtime, Integer};
        use serde::{self, Deserialize, Serialize};
        /**
         * DOM breakpoint type.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum DOMBreakpointType {
            #[serde(rename = "subtree-modified")]
            SubtreeModified,
            #[serde(rename = "attribute-modified")]
            AttributeModified,
            #[serde(rename = "node-removed")]
            NodeRemoved,
        }

        /**
         * CSP Violation type.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum CSPViolationType {
            #[serde(rename = "trustedtype-sink-violation")]
            TrustedtypeSinkViolation,
            #[serde(rename = "trustedtype-policy-violation")]
            TrustedtypePolicyViolation,
        }

        /**
         * Object event listener.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EventListener {
            /**
             * Object event listener.
            */
            #[serde(rename = "type")]
            r#type: Option<String>,
            /**
             * Object event listener.
            */
            #[serde(rename = "useCapture")]
            use_capture: Option<bool>,
            /**
             * Object event listener.
            */
            passive: Option<bool>,
            /**
             * Object event listener.
            */
            once: Option<bool>,
            /**
             * Object event listener.
            */
            #[serde(rename = "scriptId")]
            script_id: Option<runtime::ScriptId>,
            /**
             * Object event listener.
            */
            #[serde(rename = "lineNumber")]
            line_number: Option<Integer>,
            /**
             * Object event listener.
            */
            #[serde(rename = "columnNumber")]
            column_number: Option<Integer>,
            /**
             * Object event listener.
            */
            handler: runtime::RemoteObject,
            /**
             * Object event listener.
            */
            #[serde(rename = "originalHandler")]
            original_handler: runtime::RemoteObject,
            /**
             * Object event listener.
            */
            #[serde(rename = "backendNodeId")]
            backend_node_id: dom::BackendNodeId,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetEventListenersRequest {
            #[serde(rename = "objectId")]
            objectid: Option<runtime::RemoteObjectId>,

            depth: Integer,

            pierce: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetEventListenersResponse {
            listeners: Option<Vec<EventListener>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct RemoveDOMBreakpointRequest {
            #[serde(rename = "nodeId")]
            node_id: Option<dom::NodeId>,

            #[serde(rename = "type")]
            r#type: Option<DOMBreakpointType>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct RemoveEventListenerBreakpointRequest {
            #[serde(rename = "eventName")]
            event_name: Option<String>,

            #[serde(rename = "targetName")]
            target_name: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct RemoveInstrumentationBreakpointRequest {
            #[serde(rename = "eventName")]
            event_name: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct RemoveXHRBreakpointRequest {
            url: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetBreakOnCSPViolationRequest {
            #[serde(rename = "violationTypes")]
            violation_types: Option<Vec<CSPViolationType>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetDOMBreakpointRequest {
            #[serde(rename = "nodeId")]
            node_id: Option<dom::NodeId>,

            #[serde(rename = "type")]
            r#type: Option<DOMBreakpointType>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetEventListenerBreakpointRequest {
            #[serde(rename = "eventName")]
            event_name: Option<String>,

            #[serde(rename = "targetName")]
            target_name: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetInstrumentationBreakpointRequest {
            #[serde(rename = "eventName")]
            event_name: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetXHRBreakpointRequest {
            url: Option<String>,
        }
    }

    /**
     * EventBreakpoints permits setting breakpoints on particular operations and
     * events in targets that run JavaScript but do not have a DOM.
     * JavaScript execution will stop on these operations as if there was a regular
     * breakpoint set.
    */
    pub mod event_breakpoints {
        use serde::{self, Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetInstrumentationBreakpointRequest {
            #[serde(rename = "eventName")]
            event_name: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct RemoveInstrumentationBreakpointRequest {
            #[serde(rename = "eventName")]
            event_name: Option<String>,
        }
    }

    /**
     * This domain facilitates obtaining document snapshots with DOM, layout, and style information.
    */
    pub mod domsnapshot {
        use super::{dom, domdebugger, page, Integer};
        use serde::{self, Deserialize, Serialize};
        /**
         * A Node in the DOM tree.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct DOMNode {
            /**
             * A Node in the DOM tree.
            */
            #[serde(rename = "nodeType")]
            node_type: Option<Integer>,
            /**
             * A Node in the DOM tree.
            */
            #[serde(rename = "nodeName")]
            node_name: Option<String>,
            /**
             * A Node in the DOM tree.
            */
            #[serde(rename = "nodeValue")]
            node_value: Option<String>,
            /**
             * A Node in the DOM tree.
            */
            #[serde(rename = "textValue")]
            text_value: String,
            /**
             * A Node in the DOM tree.
            */
            #[serde(rename = "inputValue")]
            input_value: String,
            /**
             * A Node in the DOM tree.
            */
            #[serde(rename = "inputChecked")]
            input_checked: bool,
            /**
             * A Node in the DOM tree.
            */
            #[serde(rename = "optionSelected")]
            option_selected: bool,
            /**
             * A Node in the DOM tree.
            */
            #[serde(rename = "backendNodeId")]
            backend_node_id: Option<dom::BackendNodeId>,
            /**
             * A Node in the DOM tree.
            */
            #[serde(rename = "childNodeIndexes")]
            child_node_indexes: Vec<Integer>,
            /**
             * A Node in the DOM tree.
            */
            attributes: Vec<NameValue>,
            /**
             * A Node in the DOM tree.
            */
            #[serde(rename = "pseudoElementIndexes")]
            pseudo_element_indexes: Vec<Integer>,
            /**
             * A Node in the DOM tree.
            */
            #[serde(rename = "layoutNodeIndex")]
            layout_node_index: Integer,
            /**
             * A Node in the DOM tree.
            */
            #[serde(rename = "documentURL")]
            document_url: String,
            /**
             * A Node in the DOM tree.
            */
            #[serde(rename = "baseURL")]
            base_url: String,
            /**
             * A Node in the DOM tree.
            */
            #[serde(rename = "contentLanguage")]
            content_language: String,
            /**
             * A Node in the DOM tree.
            */
            #[serde(rename = "documentEncoding")]
            document_encoding: String,
            /**
             * A Node in the DOM tree.
            */
            #[serde(rename = "publicId")]
            public_id: String,
            /**
             * A Node in the DOM tree.
            */
            #[serde(rename = "systemId")]
            system_id: String,
            /**
             * A Node in the DOM tree.
            */
            #[serde(rename = "frameId")]
            frame_id: page::FrameId,
            /**
             * A Node in the DOM tree.
            */
            #[serde(rename = "contentDocumentIndex")]
            content_document_index: Integer,
            /**
             * A Node in the DOM tree.
            */
            #[serde(rename = "pseudoType")]
            pseudo_type: dom::PseudoType,
            /**
             * A Node in the DOM tree.
            */
            #[serde(rename = "shadowRootType")]
            shadow_root_type: dom::ShadowRootType,
            /**
             * A Node in the DOM tree.
            */
            #[serde(rename = "isClickable")]
            is_clickable: bool,
            /**
             * A Node in the DOM tree.
            */
            #[serde(rename = "eventListeners")]
            event_listeners: Vec<domdebugger::EventListener>,
            /**
             * A Node in the DOM tree.
            */
            #[serde(rename = "currentSourceURL")]
            current_source_url: String,
            /**
             * A Node in the DOM tree.
            */
            #[serde(rename = "originURL")]
            origin_url: String,
            /**
             * A Node in the DOM tree.
            */
            #[serde(rename = "scrollOffsetX")]
            scroll_offset_x: f64,
            /**
             * A Node in the DOM tree.
            */
            #[serde(rename = "scrollOffsetY")]
            scroll_offset_y: f64,
        }

        /**
         * Details of post layout rendered text positions. The exact layout should not be regarded as
         * stable and may change between versions.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InlineTextBox {
            /**
             * Details of post layout rendered text positions. The exact layout should not be regarded as
             * stable and may change between versions.
            */
            #[serde(rename = "boundingBox")]
            bounding_box: Option<dom::Rect>,
            /**
             * Details of post layout rendered text positions. The exact layout should not be regarded as
             * stable and may change between versions.
            */
            #[serde(rename = "startCharacterIndex")]
            start_character_index: Option<Integer>,
            /**
             * Details of post layout rendered text positions. The exact layout should not be regarded as
             * stable and may change between versions.
            */
            #[serde(rename = "numCharacters")]
            num_characters: Option<Integer>,
        }

        /**
         * Details of an element in the DOM tree with a LayoutObject.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct LayoutTreeNode {
            /**
             * Details of an element in the DOM tree with a LayoutObject.
            */
            #[serde(rename = "domNodeIndex")]
            dom_node_index: Option<Integer>,
            /**
             * Details of an element in the DOM tree with a LayoutObject.
            */
            #[serde(rename = "boundingBox")]
            bounding_box: Option<dom::Rect>,
            /**
             * Details of an element in the DOM tree with a LayoutObject.
            */
            #[serde(rename = "layoutText")]
            layout_text: String,
            /**
             * Details of an element in the DOM tree with a LayoutObject.
            */
            #[serde(rename = "inlineTextNodes")]
            inline_text_nodes: Vec<InlineTextBox>,
            /**
             * Details of an element in the DOM tree with a LayoutObject.
            */
            #[serde(rename = "styleIndex")]
            style_index: Integer,
            /**
             * Details of an element in the DOM tree with a LayoutObject.
            */
            #[serde(rename = "paintOrder")]
            paint_order: Integer,
            /**
             * Details of an element in the DOM tree with a LayoutObject.
            */
            #[serde(rename = "isStackingContext")]
            is_stacking_context: bool,
        }

        /**
         * A subset of the full ComputedStyle as defined by the request whitelist.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ComputedStyle {
            /**
             * A subset of the full ComputedStyle as defined by the request whitelist.
            */
            properties: Option<Vec<NameValue>>,
        }

        /**
         * A name/value pair.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct NameValue {
            /**
             * A name/value pair.
            */
            name: Option<String>,
            /**
             * A name/value pair.
            */
            value: Option<String>,
        }

        /**
         * Index of the string in the strings table.
        */
        pub type StringIndex = Integer;

        /**
         * Index of the string in the strings table.
        */
        pub type ArrayOfStrings = Vec<StringIndex>;

        /**
         * Data that is only present on rare nodes.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct RareStringData {
            /**
             * Data that is only present on rare nodes.
            */
            index: Option<Vec<Integer>>,
            /**
             * Data that is only present on rare nodes.
            */
            value: Option<Vec<StringIndex>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct RareBooleanData {
            index: Option<Vec<Integer>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct RareIntegerData {
            index: Option<Vec<Integer>>,

            value: Option<Vec<Integer>>,
        }

        pub type Rectangle = Vec<f64>;

        /**
         * Document snapshot.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct DocumentSnapshot {
            /**
             * Document snapshot.
            */
            #[serde(rename = "documentURL")]
            document_url: Option<StringIndex>,
            /**
             * Document snapshot.
            */
            title: Option<StringIndex>,
            /**
             * Document snapshot.
            */
            #[serde(rename = "baseURL")]
            base_url: Option<StringIndex>,
            /**
             * Document snapshot.
            */
            #[serde(rename = "contentLanguage")]
            content_language: Option<StringIndex>,
            /**
             * Document snapshot.
            */
            #[serde(rename = "encodingName")]
            encoding_name: Option<StringIndex>,
            /**
             * Document snapshot.
            */
            #[serde(rename = "publicId")]
            public_id: Option<StringIndex>,
            /**
             * Document snapshot.
            */
            #[serde(rename = "systemId")]
            system_id: Option<StringIndex>,
            /**
             * Document snapshot.
            */
            #[serde(rename = "frameId")]
            frame_id: Option<StringIndex>,
            /**
             * Document snapshot.
            */
            nodes: Option<NodeTreeSnapshot>,
            /**
             * Document snapshot.
            */
            layout: Option<LayoutTreeSnapshot>,
            /**
             * Document snapshot.
            */
            #[serde(rename = "textBoxes")]
            text_boxes: Option<TextBoxSnapshot>,
            /**
             * Document snapshot.
            */
            #[serde(rename = "scrollOffsetX")]
            scroll_offset_x: f64,
            /**
             * Document snapshot.
            */
            #[serde(rename = "scrollOffsetY")]
            scroll_offset_y: f64,
            /**
             * Document snapshot.
            */
            #[serde(rename = "contentWidth")]
            content_width: f64,
            /**
             * Document snapshot.
            */
            #[serde(rename = "contentHeight")]
            content_height: f64,
        }

        /**
         * Table containing nodes.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct NodeTreeSnapshot {
            /**
             * Table containing nodes.
            */
            #[serde(rename = "parentIndex")]
            parent_index: Vec<Integer>,
            /**
             * Table containing nodes.
            */
            #[serde(rename = "nodeType")]
            node_type: Vec<Integer>,
            /**
             * Table containing nodes.
            */
            #[serde(rename = "shadowRootType")]
            shadow_root_type: RareStringData,
            /**
             * Table containing nodes.
            */
            #[serde(rename = "nodeName")]
            node_name: Vec<StringIndex>,
            /**
             * Table containing nodes.
            */
            #[serde(rename = "nodeValue")]
            node_value: Vec<StringIndex>,
            /**
             * Table containing nodes.
            */
            #[serde(rename = "backendNodeId")]
            backend_node_id: Vec<dom::BackendNodeId>,
            /**
             * Table containing nodes.
            */
            attributes: Vec<ArrayOfStrings>,
            /**
             * Table containing nodes.
            */
            #[serde(rename = "textValue")]
            text_value: RareStringData,
            /**
             * Table containing nodes.
            */
            #[serde(rename = "inputValue")]
            input_value: RareStringData,
            /**
             * Table containing nodes.
            */
            #[serde(rename = "inputChecked")]
            input_checked: RareBooleanData,
            /**
             * Table containing nodes.
            */
            #[serde(rename = "optionSelected")]
            option_selected: RareBooleanData,
            /**
             * Table containing nodes.
            */
            #[serde(rename = "contentDocumentIndex")]
            content_document_index: RareIntegerData,
            /**
             * Table containing nodes.
            */
            #[serde(rename = "pseudoType")]
            pseudo_type: RareStringData,
            /**
             * Table containing nodes.
            */
            #[serde(rename = "pseudoIdentifier")]
            pseudo_identifier: RareStringData,
            /**
             * Table containing nodes.
            */
            #[serde(rename = "isClickable")]
            is_clickable: RareBooleanData,
            /**
             * Table containing nodes.
            */
            #[serde(rename = "currentSourceURL")]
            current_source_url: RareStringData,
            /**
             * Table containing nodes.
            */
            #[serde(rename = "originURL")]
            origin_url: RareStringData,
        }

        /**
         * Table of details of an element in the DOM tree with a LayoutObject.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct LayoutTreeSnapshot {
            /**
             * Table of details of an element in the DOM tree with a LayoutObject.
            */
            #[serde(rename = "nodeIndex")]
            node_index: Option<Vec<Integer>>,
            /**
             * Table of details of an element in the DOM tree with a LayoutObject.
            */
            styles: Option<Vec<ArrayOfStrings>>,
            /**
             * Table of details of an element in the DOM tree with a LayoutObject.
            */
            bounds: Option<Vec<Rectangle>>,
            /**
             * Table of details of an element in the DOM tree with a LayoutObject.
            */
            text: Option<Vec<StringIndex>>,
            /**
             * Table of details of an element in the DOM tree with a LayoutObject.
            */
            #[serde(rename = "stackingContexts")]
            stacking_contexts: Option<RareBooleanData>,
            /**
             * Table of details of an element in the DOM tree with a LayoutObject.
            */
            #[serde(rename = "paintOrders")]
            paint_orders: Vec<Integer>,
            /**
             * Table of details of an element in the DOM tree with a LayoutObject.
            */
            #[serde(rename = "offsetRects")]
            offset_rects: Vec<Rectangle>,
            /**
             * Table of details of an element in the DOM tree with a LayoutObject.
            */
            #[serde(rename = "scrollRects")]
            scroll_rects: Vec<Rectangle>,
            /**
             * Table of details of an element in the DOM tree with a LayoutObject.
            */
            #[serde(rename = "clientRects")]
            client_rects: Vec<Rectangle>,
            /**
             * Table of details of an element in the DOM tree with a LayoutObject.
            */
            #[serde(rename = "blendedBackgroundColors")]
            blended_background_colors: Vec<StringIndex>,
            /**
             * Table of details of an element in the DOM tree with a LayoutObject.
            */
            #[serde(rename = "textColorOpacities")]
            text_color_opacities: Vec<f64>,
        }

        /**
         * Table of details of the post layout rendered text positions. The exact layout should not be regarded as
         * stable and may change between versions.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct TextBoxSnapshot {
            /**
             * Table of details of the post layout rendered text positions. The exact layout should not be regarded as
             * stable and may change between versions.
            */
            #[serde(rename = "layoutIndex")]
            layoutindex: Option<Vec<Integer>>,
            /**
             * Table of details of the post layout rendered text positions. The exact layout should not be regarded as
             * stable and may change between versions.
            */
            bounds: Option<Vec<Rectangle>>,
            /**
             * Table of details of the post layout rendered text positions. The exact layout should not be regarded as
             * stable and may change between versions.
            */
            start: Option<Vec<Integer>>,
            /**
             * Table of details of the post layout rendered text positions. The exact layout should not be regarded as
             * stable and may change between versions.
            */
            length: Option<Vec<Integer>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetSnapshotRequest {
            #[serde(rename = "computedStyleWhitelist")]
            computed_style_whitelist: Option<Vec<String>>,

            #[serde(rename = "includeEventListeners")]
            include_event_listeners: bool,

            #[serde(rename = "includePaintOrder")]
            include_paint_order: bool,

            #[serde(rename = "includeUserAgentShadowTree")]
            include_user_agent_shadow_tree: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetSnapshotResponse {
            #[serde(rename = "domNodes")]
            dom_nodes: Option<Vec<DOMNode>>,

            #[serde(rename = "layoutTreeNodes")]
            layout_tree_nodes: Option<Vec<LayoutTreeNode>>,

            #[serde(rename = "computedStyles")]
            computed_styles: Option<Vec<ComputedStyle>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CaptureSnapshotRequest {
            #[serde(rename = "computedStyles")]
            computed_styles: Option<Vec<String>>,

            #[serde(rename = "includePaintOrder")]
            include_paint_order: bool,

            #[serde(rename = "includeDOMRects")]
            include_domrects: bool,

            #[serde(rename = "includeBlendedBackgroundColors")]
            include_blended_background_colors: bool,

            #[serde(rename = "includeTextColorOpacities")]
            include_text_color_opacities: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CaptureSnapshotResponse {
            documents: Option<Vec<DocumentSnapshot>>,

            strings: Option<Vec<String>>,
        }
    }

    /**
     * Query and modify DOM storage.
    */
    pub mod domstorage {
        use serde::{self, Deserialize, Serialize};

        pub type SerializedStorageKey = String;

        /**
         * DOM Storage identifier.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct StorageId {
            /**
             * DOM Storage identifier.
            */
            #[serde(rename = "securityOrigin")]
            security_origin: String,
            /**
             * DOM Storage identifier.
            */
            #[serde(rename = "storageKey")]
            storage_key: SerializedStorageKey,
            /**
             * DOM Storage identifier.
            */
            #[serde(rename = "isLocalStorage")]
            is_local_storage: Option<bool>,
        }

        /**
         * DOM Storage item.
        */
        pub type Item = Vec<String>;

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ClearRequest {
            #[serde(rename = "storageId")]
            storage_id: Option<StorageId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetDOMStorageItemsRequest {
            #[serde(rename = "storageId")]
            storage_id: Option<StorageId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetDOMStorageItemsResponse {
            entries: Option<Vec<Item>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct RemoveDOMStorageItemRequest {
            #[serde(rename = "storageId")]
            storage_id: Option<StorageId>,

            key: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetDOMStorageItemRequest {
            #[serde(rename = "storageId")]
            storageid: Option<StorageId>,

            key: Option<String>,

            value: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct DomStorageItemAddedEvent {
            #[serde(rename = "storageId")]
            storage_id: Option<StorageId>,

            key: Option<String>,

            #[serde(rename = "newValue")]
            new_value: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct DomStorageItemRemovedEvent {
            #[serde(rename = "storageId")]
            storage_id: Option<StorageId>,

            key: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct DomStorageItemUpdatedEvent {
            #[serde(rename = "storageId")]
            storage_id: Option<StorageId>,

            key: Option<String>,

            #[serde(rename = "oldValue")]
            old_value: Option<String>,

            #[serde(rename = "newValue")]
            new_value: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct DomStorageItemsClearedEvent {
            #[serde(rename = "storageId")]
            storage_id: Option<StorageId>,
        }
    }

    pub mod database {
        use super::Integer;
        use serde::{self, Deserialize, Serialize};
        /**
         * Unique identifier of Database object.
        */
        pub type DatabaseId = String;

        /**
         * Database object.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Database {
            /**
             * Database object.
            */
            id: Option<DatabaseId>,
            /**
             * Database object.
            */
            domain: Option<String>,
            /**
             * Database object.
            */
            name: Option<String>,
            /**
             * Database object.
            */
            version: Option<String>,
        }

        /**
         * Database error.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Error {
            /**
             * Database error.
            */
            message: Option<String>,
            /**
             * Database error.
            */
            code: Option<Integer>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ExecuteSQLRequest {
            #[serde(rename = "databaseId")]
            database_id: Option<DatabaseId>,

            query: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ExecuteSQLResponse {
            #[serde(rename = "columnNames")]
            column_names: Vec<String>,

            values: Vec<serde_json::Value>,

            #[serde(rename = "sqlError")]
            sql_error: Error,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetDatabaseTableNamesRequest {
            #[serde(rename = "databaseId")]
            database_id: Option<DatabaseId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetDatabaseTableNamesResponse {
            #[serde(rename = "tableNames")]
            table_names: Option<Vec<String>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct AddDatabaseEvent {
            database: Option<Database>,
        }
    }

    pub mod device_orientation {
        use serde::{self, Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetDeviceOrientationOverrideRequest {
            alpha: Option<f64>,

            beta: Option<f64>,

            gamma: Option<f64>,
        }
    }

    /**
     * This domain emulates different environments for the page.
    */
    pub mod emulation {
        use super::{dom, network, page, Integer};
        use serde::{self, Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize)]
        pub enum ScreenOrientationType {
            #[serde(rename = "portraitPrimary")]
            PortraitPrimary,

            #[serde(rename = "portraitSecondary")]
            PortraitSecondary,

            #[serde(rename = "landscapePrimary")]
            LandscapePrimary,

            #[serde(rename = "landscapeSecondary")]
            LandscapeSecondary,
        }

        /**
         * Screen orientation.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ScreenOrientation {
            /**
             * Screen orientation.
            */
            #[serde(rename = "type")]
            r#type: Option<ScreenOrientationType>,
            /**
             * Screen orientation.
            */
            angle: Option<Integer>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum DisplayFeatureOrientation {
            #[serde(rename = "vertical")]
            Vertical,

            #[serde(rename = "horizontal")]
            Horizontal,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct DisplayFeature {
            orientation: Option<DisplayFeatureOrientation>,

            offset: Option<Integer>,

            #[serde(rename = "maskLength")]
            masklength: Option<Integer>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct MediaFeature {
            name: Option<String>,

            value: Option<String>,
        }

        /**
         * advance: If the scheduler runs out of immediate work, the virtual time base may fast forward to
         * allow the next delayed task (if any) to run; pause: The virtual time base may not advance;
         * pauseIfNetworkFetchesPending: The virtual time base may not advance if there are any pending
         * resource fetches.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum VirtualTimePolicy {
            #[serde(rename = "advance")]
            Advance,
            #[serde(rename = "pause")]
            Pause,
            #[serde(rename = "pauseIfNetworkFetchesPending")]
            PauseIfNetworkFetchesPending,
        }

        /**
         * Used to specify User Agent Cient Hints to emulate. See https://wicg.github.io/ua-client-hints
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct UserAgentBrandVersion {
            /**
             * Used to specify User Agent Cient Hints to emulate. See https://wicg.github.io/ua-client-hints
            */
            brand: Option<String>,
            /**
             * Used to specify User Agent Cient Hints to emulate. See https://wicg.github.io/ua-client-hints
            */
            version: Option<String>,
        }

        /**
         * Used to specify User Agent Cient Hints to emulate. See https://wicg.github.io/ua-client-hints
         * Missing optional values will be filled in by the target with what it would normally use.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct UserAgentMetadata {
            /**
             * Used to specify User Agent Cient Hints to emulate. See https://wicg.github.io/ua-client-hints
             * Missing optional values will be filled in by the target with what it would normally use.
            */
            brands: Vec<UserAgentBrandVersion>,
            /**
             * Used to specify User Agent Cient Hints to emulate. See https://wicg.github.io/ua-client-hints
             * Missing optional values will be filled in by the target with what it would normally use.
            */
            #[serde(rename = "fullVersionList")]
            fullversionlist: Vec<UserAgentBrandVersion>,
            /**
             * Used to specify User Agent Cient Hints to emulate. See https://wicg.github.io/ua-client-hints
             * Missing optional values will be filled in by the target with what it would normally use.
            */
            #[serde(rename = "fullVersion")]
            fullversion: String,
            /**
             * Used to specify User Agent Cient Hints to emulate. See https://wicg.github.io/ua-client-hints
             * Missing optional values will be filled in by the target with what it would normally use.
            */
            platform: Option<String>,
            /**
             * Used to specify User Agent Cient Hints to emulate. See https://wicg.github.io/ua-client-hints
             * Missing optional values will be filled in by the target with what it would normally use.
            */
            #[serde(rename = "platformVersion")]
            platformversion: Option<String>,
            /**
             * Used to specify User Agent Cient Hints to emulate. See https://wicg.github.io/ua-client-hints
             * Missing optional values will be filled in by the target with what it would normally use.
            */
            architecture: Option<String>,
            /**
             * Used to specify User Agent Cient Hints to emulate. See https://wicg.github.io/ua-client-hints
             * Missing optional values will be filled in by the target with what it would normally use.
            */
            model: Option<String>,
            /**
             * Used to specify User Agent Cient Hints to emulate. See https://wicg.github.io/ua-client-hints
             * Missing optional values will be filled in by the target with what it would normally use.
            */
            mobile: Option<bool>,
            /**
             * Used to specify User Agent Cient Hints to emulate. See https://wicg.github.io/ua-client-hints
             * Missing optional values will be filled in by the target with what it would normally use.
            */
            bitness: String,
            /**
             * Used to specify User Agent Cient Hints to emulate. See https://wicg.github.io/ua-client-hints
             * Missing optional values will be filled in by the target with what it would normally use.
            */
            wow64: bool,
        }

        /**
         * Enum of image types that can be disabled.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum DisabledImageType {
            #[serde(rename = "avif")]
            Avif,
            #[serde(rename = "webp")]
            Webp,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CanEmulateResponse {
            result: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetFocusEmulationEnabledRequest {
            enabled: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetAutoDarkModeOverrideRequest {
            enabled: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetCPUThrottlingRateRequest {
            rate: Option<f64>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetDefaultBackgroundColorOverrideRequest {
            color: dom::RGBA,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetDeviceMetricsOverrideRequest {
            width: Option<Integer>,

            height: Option<Integer>,

            #[serde(rename = "deviceScaleFactor")]
            device_scale_factor: Option<f64>,

            mobile: Option<bool>,

            scale: f64,

            #[serde(rename = "screenWidth")]
            screen_width: Integer,

            #[serde(rename = "screenHeight")]
            screen_height: Integer,

            #[serde(rename = "positionX")]
            position_x: Integer,

            #[serde(rename = "positionY")]
            position_y: Integer,

            #[serde(rename = "dontSetVisibleSize")]
            dont_set_visible_size: bool,

            #[serde(rename = "screenOrientation")]
            screen_orientation: ScreenOrientation,

            viewport: page::Viewport,

            #[serde(rename = "displayFeature")]
            display_feature: DisplayFeature,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetScrollbarsHiddenRequest {
            hidden: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetDocumentCookieDisabledRequest {
            disabled: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum SetEmitTouchEventsForMouseRequestConfiguration {
            #[serde(rename = "mobile")]
            Mobile,

            #[serde(rename = "desktop")]
            Desktop,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetEmitTouchEventsForMouseRequest {
            enabled: Option<bool>,

            configuration: SetEmitTouchEventsForMouseRequestConfiguration,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetEmulatedMediaRequest {
            media: String,

            features: Vec<MediaFeature>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum SetEmulatedVisionDeficiencyRequestType {
            #[serde(rename = "none")]
            None,

            #[serde(rename = "blurredVision")]
            BlurredVision,

            #[serde(rename = "reducedContrast")]
            ReducedContrast,

            #[serde(rename = "achromatopsia")]
            Achromatopsia,

            #[serde(rename = "deuteranopia")]
            Deuteranopia,

            #[serde(rename = "protanopia")]
            Protanopia,

            #[serde(rename = "tritanopia")]
            Tritanopia,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetEmulatedVisionDeficiencyRequest {
            #[serde(rename = "type")]
            r#type: Option<SetEmulatedVisionDeficiencyRequestType>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetGeolocationOverrideRequest {
            latitude: f64,

            longitude: f64,

            accuracy: f64,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetIdleOverrideRequest {
            #[serde(rename = "isUserActive")]
            is_user_active: Option<bool>,

            #[serde(rename = "isScreenUnlocked")]
            is_screen_unlocked: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetNavigatorOverridesRequest {
            platform: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetPageScaleFactorRequest {
            #[serde(rename = "pageScaleFactor")]
            page_scale_factor: Option<f64>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetScriptExecutionDisabledRequest {
            value: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetTouchEmulationEnabledRequest {
            enabled: Option<bool>,

            #[serde(rename = "maxTouchPoints")]
            maxtouchpoints: Integer,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetVirtualTimePolicyRequest {
            policy: Option<VirtualTimePolicy>,

            budget: f64,

            #[serde(rename = "maxVirtualTimeTaskStarvationCount")]
            maxvirtualtimetaskstarvationcount: Integer,

            #[serde(rename = "initialVirtualTime")]
            initialvirtualtime: network::TimeSinceEpoch,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetVirtualTimePolicyResponse {
            #[serde(rename = "virtualTimeTicksBase")]
            virtual_time_ticks_base: Option<f64>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetLocaleOverrideRequest {
            locale: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetTimezoneOverrideRequest {
            #[serde(rename = "timezoneId")]
            timezone_id: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetVisibleSizeRequest {
            width: Option<Integer>,

            height: Option<Integer>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetDisabledImageTypesRequest {
            #[serde(rename = "imageTypes")]
            image_types: Option<Vec<DisabledImageType>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetHardwareConcurrencyOverrideRequest {
            #[serde(rename = "hardwareConcurrency")]
            hardware_concurrency: Option<Integer>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetUserAgentOverrideRequest {
            #[serde(rename = "userAgent")]
            user_agent: Option<String>,

            #[serde(rename = "acceptLanguage")]
            accept_language: String,

            platform: String,

            #[serde(rename = "userAgentMetadata")]
            user_agent_metadata: UserAgentMetadata,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetAutomationOverrideRequest {
            enabled: Option<bool>,
        }
    }

    /**
     * This domain provides experimental commands only supported in headless mode.
    */
    pub mod headless_experimental {
        use super::Integer;
        use serde::{self, Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize)]
        pub enum ScreenshotParamsFormat {
            #[serde(rename = "jpeg")]
            Jpeg,

            #[serde(rename = "png")]
            Png,

            #[serde(rename = "webp")]
            Webp,
        }

        /**
         * Encoding options for a screenshot.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ScreenshotParams {
            /**
             * Encoding options for a screenshot.
            */
            format: ScreenshotParamsFormat,
            /**
             * Encoding options for a screenshot.
            */
            quality: Integer,
            /**
             * Encoding options for a screenshot.
            */
            #[serde(rename = "optimizeForSpeed")]
            optimizeforspeed: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct BeginFrameRequest {
            #[serde(rename = "frameTimeTicks")]
            frame_time_ticks: f64,

            interval: f64,

            #[serde(rename = "noDisplayUpdates")]
            no_display_updates: bool,

            screenshot: ScreenshotParams,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct BeginFrameResponse {
            #[serde(rename = "hasDamage")]
            has_damage: Option<bool>,

            #[serde(rename = "screenshotData")]
            screenshot_data: String,
        }
    }

    /**
     * Input/Output operations for streams produced by DevTools.
    */
    pub mod io {
        use super::{runtime, Integer};
        use serde::{self, Deserialize, Serialize};
        /**
         * This is either obtained from another method or specified as `blob:<uuid>` where
         * `<uuid>` is an UUID of a Blob.
        */
        pub type StreamHandle = String;

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CloseRequest {
            handle: Option<StreamHandle>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ReadRequest {
            handle: Option<StreamHandle>,

            offset: Integer,

            size: Integer,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ReadResponse {
            #[serde(rename = "base64Encoded")]
            base64encoded: bool,

            data: Option<String>,

            eof: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ResolveBlobRequest {
            #[serde(rename = "objectId")]
            object_id: Option<runtime::RemoteObjectId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ResolveBlobResponse {
            uuid: Option<String>,
        }
    }

    pub mod indexed_db {
        use super::{runtime, storage, Integer};
        use serde::{self, Deserialize, Serialize};
        /**
         * Database with an array of object stores.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct DatabaseWithObjectStores {
            /**
             * Database with an array of object stores.
            */
            name: Option<String>,
            /**
             * Database with an array of object stores.
            */
            version: Option<f64>,
            /**
             * Database with an array of object stores.
            */
            #[serde(rename = "objectStores")]
            objectstores: Option<Vec<ObjectStore>>,
        }

        /**
         * Object store.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ObjectStore {
            /**
             * Object store.
            */
            name: Option<String>,
            /**
             * Object store.
            */
            #[serde(rename = "keyPath")]
            keypath: Option<KeyPath>,
            /**
             * Object store.
            */
            #[serde(rename = "autoIncrement")]
            autoincrement: Option<bool>,
            /**
             * Object store.
            */
            indexes: Option<Vec<ObjectStoreIndex>>,
        }

        /**
         * Object store index.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ObjectStoreIndex {
            /**
             * Object store index.
            */
            name: Option<String>,
            /**
             * Object store index.
            */
            #[serde(rename = "keyPath")]
            keypath: Option<KeyPath>,
            /**
             * Object store index.
            */
            unique: Option<bool>,
            /**
             * Object store index.
            */
            #[serde(rename = "multiEntry")]
            multientry: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum KeyType {
            #[serde(rename = "number")]
            Number,

            #[serde(rename = "string")]
            String,

            #[serde(rename = "date")]
            Date,

            #[serde(rename = "array")]
            Array,
        }

        /**
         * Key.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Key {
            /**
             * Key.
            */
            #[serde(rename = "type")]
            r#type: Option<KeyType>,
            /**
             * Key.
            */
            number: f64,
            /**
             * Key.
            */
            string: String,
            /**
             * Key.
            */
            date: f64,
            /**
             * Key.
            */
            array: Vec<Key>,
        }

        /**
         * Key range.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct KeyRange {
            /**
             * Key range.
            */
            lower: Key,
            /**
             * Key range.
            */
            upper: Key,
            /**
             * Key range.
            */
            #[serde(rename = "lowerOpen")]
            loweropen: Option<bool>,
            /**
             * Key range.
            */
            #[serde(rename = "upperOpen")]
            upperopen: Option<bool>,
        }

        /**
         * Data entry.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct DataEntry {
            /**
             * Data entry.
            */
            key: Option<runtime::RemoteObject>,
            /**
             * Data entry.
            */
            #[serde(rename = "primaryKey")]
            primarykey: Option<runtime::RemoteObject>,
            /**
             * Data entry.
            */
            value: Option<runtime::RemoteObject>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum KeyPathType {
            #[serde(rename = "null")]
            Null,

            #[serde(rename = "string")]
            String,

            #[serde(rename = "array")]
            Array,
        }

        /**
         * Key path.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct KeyPath {
            /**
             * Key path.
            */
            #[serde(rename = "type")]
            r#type: Option<KeyPathType>,
            /**
             * Key path.
            */
            string: String,
            /**
             * Key path.
            */
            array: Vec<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ClearObjectStoreRequest {
            #[serde(rename = "securityOrigin")]
            security_origin: String,

            #[serde(rename = "storageKey")]
            storage_key: String,

            #[serde(rename = "storageBucket")]
            storage_bucket: storage::StorageBucket,

            #[serde(rename = "databaseName")]
            database_name: Option<String>,

            #[serde(rename = "objectStoreName")]
            object_store_name: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct DeleteDatabaseRequest {
            #[serde(rename = "securityOrigin")]
            security_origin: String,

            #[serde(rename = "storageKey")]
            storage_key: String,

            #[serde(rename = "storageBucket")]
            storage_bucket: storage::StorageBucket,

            #[serde(rename = "databaseName")]
            database_name: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct DeleteObjectStoreEntriesRequest {
            #[serde(rename = "securityOrigin")]
            security_origin: String,

            #[serde(rename = "storageKey")]
            storage_key: String,

            #[serde(rename = "storageBucket")]
            storage_bucket: storage::StorageBucket,

            #[serde(rename = "databaseName")]
            database_name: Option<String>,

            #[serde(rename = "objectStoreName")]
            object_store_name: Option<String>,

            #[serde(rename = "keyRange")]
            key_range: Option<KeyRange>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct RequestDataRequest {
            #[serde(rename = "securityOrigin")]
            security_origin: String,

            #[serde(rename = "storageKey")]
            storage_key: String,

            #[serde(rename = "storageBucket")]
            storage_bucket: storage::StorageBucket,

            #[serde(rename = "databaseName")]
            database_name: Option<String>,

            #[serde(rename = "objectStoreName")]
            object_store_name: Option<String>,

            #[serde(rename = "indexName")]
            index_name: Option<String>,

            #[serde(rename = "skipCount")]
            skip_count: Option<Integer>,

            #[serde(rename = "pageSize")]
            page_size: Option<Integer>,

            #[serde(rename = "keyRange")]
            key_range: KeyRange,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct RequestDataResponse {
            #[serde(rename = "objectStoreDataEntries")]
            object_store_data_entries: Option<Vec<DataEntry>>,

            #[serde(rename = "hasMore")]
            has_more: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetMetadataRequest {
            #[serde(rename = "securityOrigin")]
            security_origin: String,

            #[serde(rename = "storageKey")]
            storage_key: String,

            #[serde(rename = "storageBucket")]
            storage_bucket: storage::StorageBucket,

            #[serde(rename = "databaseName")]
            database_name: Option<String>,

            #[serde(rename = "objectStoreName")]
            object_store_name: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetMetadataResponse {
            #[serde(rename = "entriesCount")]
            entries_count: Option<f64>,

            #[serde(rename = "keyGeneratorValue")]
            key_generator_value: Option<f64>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct RequestDatabaseRequest {
            #[serde(rename = "securityOrigin")]
            security_origin: String,

            #[serde(rename = "storageKey")]
            storage_key: String,

            #[serde(rename = "storageBucket")]
            storage_bucket: storage::StorageBucket,

            #[serde(rename = "databaseName")]
            database_name: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct RequestDatabaseResponse {
            #[serde(rename = "databaseWithObjectStores")]
            database_with_object_stores: Option<DatabaseWithObjectStores>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct RequestDatabaseNamesRequest {
            #[serde(rename = "securityOrigin")]
            security_origin: String,

            #[serde(rename = "storageKey")]
            storage_key: String,

            #[serde(rename = "storageBucket")]
            storage_bucket: storage::StorageBucket,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct RequestDatabaseNamesResponse {
            #[serde(rename = "databaseNames")]
            database_names: Option<Vec<String>>,
        }
    }

    pub mod input {
        use super::Integer;
        use serde::{self, Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize)]
        pub struct TouchPoint {
            x: Option<f64>,

            y: Option<f64>,

            #[serde(rename = "radiusX")]
            radius_x: f64,

            #[serde(rename = "radiusY")]
            radius_y: f64,

            #[serde(rename = "rotationAngle")]
            rotation_angle: f64,

            force: f64,

            #[serde(rename = "tangentialPressure")]
            tangential_pressure: f64,

            #[serde(rename = "tiltX")]
            tilt_x: Integer,

            #[serde(rename = "tiltY")]
            tilt_y: Integer,

            twist: Integer,

            id: f64,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum GestureSourceType {
            #[serde(rename = "default")]
            Default,
            #[serde(rename = "touch")]
            Touch,
            #[serde(rename = "mouse")]
            Mouse,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum MouseButton {
            #[serde(rename = "none")]
            None,
            #[serde(rename = "left")]
            Left,
            #[serde(rename = "middle")]
            Middle,
            #[serde(rename = "right")]
            Right,
            #[serde(rename = "back")]
            Back,
            #[serde(rename = "forward")]
            Forward,
        }

        /**
         * UTC time in seconds, counted from January 1, 1970.
        */
        pub type TimeSinceEpoch = f64;

        #[derive(Debug, Serialize, Deserialize)]
        pub struct DragDataItem {
            #[serde(rename = "mimeType")]
            mime_type: Option<String>,

            data: Option<String>,

            title: String,

            #[serde(rename = "baseURL")]
            base_url: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct DragData {
            items: Option<Vec<DragDataItem>>,

            files: Vec<String>,

            #[serde(rename = "dragOperationsMask")]
            dragoperationsmask: Option<Integer>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum DispatchDragEventRequestType {
            #[serde(rename = "dragEnter")]
            DragEnter,

            #[serde(rename = "dragOver")]
            DragOver,

            #[serde(rename = "drop")]
            Drop,

            #[serde(rename = "dragCancel")]
            DragCancel,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct DispatchDragEventRequest {
            #[serde(rename = "type")]
            r#type: Option<DispatchDragEventRequestType>,

            x: Option<f64>,

            y: Option<f64>,

            data: Option<DragData>,

            modifiers: Integer,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum DispatchKeyEventRequestType {
            #[serde(rename = "keyDown")]
            KeyDown,

            #[serde(rename = "keyUp")]
            KeyUp,

            #[serde(rename = "rawKeyDown")]
            RawKeyDown,

            #[serde(rename = "char")]
            Char,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct DispatchKeyEventRequest {
            #[serde(rename = "type")]
            r#type: Option<DispatchKeyEventRequestType>,

            modifiers: Integer,

            timestamp: TimeSinceEpoch,

            text: String,

            #[serde(rename = "unmodifiedText")]
            unmodifiedtext: String,

            #[serde(rename = "keyIdentifier")]
            keyidentifier: String,

            code: String,

            key: String,

            #[serde(rename = "windowsVirtualKeyCode")]
            windowsvirtualkeycode: Integer,

            #[serde(rename = "nativeVirtualKeyCode")]
            nativevirtualkeycode: Integer,

            #[serde(rename = "autoRepeat")]
            autorepeat: bool,

            #[serde(rename = "isKeypad")]
            iskeypad: bool,

            #[serde(rename = "isSystemKey")]
            issystemkey: bool,

            location: Integer,

            commands: Vec<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct InsertTextRequest {
            text: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ImeSetCompositionRequest {
            text: Option<String>,

            #[serde(rename = "selectionStart")]
            selection_start: Option<Integer>,

            #[serde(rename = "selectionEnd")]
            selection_end: Option<Integer>,

            #[serde(rename = "replacementStart")]
            replacement_start: Integer,

            #[serde(rename = "replacementEnd")]
            replacement_end: Integer,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum DispatchMouseEventRequestType {
            #[serde(rename = "mousePressed")]
            MousePressed,

            #[serde(rename = "mouseReleased")]
            MouseReleased,

            #[serde(rename = "mouseMoved")]
            MouseMoved,

            #[serde(rename = "mouseWheel")]
            MouseWheel,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum DispatchMouseEventRequestPointerType {
            #[serde(rename = "mouse")]
            Mouse,

            #[serde(rename = "pen")]
            Pen,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct DispatchMouseEventRequest {
            #[serde(rename = "type")]
            r#type: Option<DispatchMouseEventRequestType>,

            x: Option<f64>,

            y: Option<f64>,

            modifiers: Integer,

            timestamp: TimeSinceEpoch,

            button: MouseButton,

            buttons: Integer,

            #[serde(rename = "clickCount")]
            clickcount: Integer,

            force: f64,

            #[serde(rename = "tangentialPressure")]
            tangentialpressure: f64,

            #[serde(rename = "tiltX")]
            tiltx: Integer,

            #[serde(rename = "tiltY")]
            tilty: Integer,

            twist: Integer,

            #[serde(rename = "deltaX")]
            deltax: f64,

            #[serde(rename = "deltaY")]
            deltay: f64,

            #[serde(rename = "pointerType")]
            pointertype: DispatchMouseEventRequestPointerType,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum DispatchTouchEventRequestType {
            #[serde(rename = "touchStart")]
            TouchStart,

            #[serde(rename = "touchEnd")]
            TouchEnd,

            #[serde(rename = "touchMove")]
            TouchMove,

            #[serde(rename = "touchCancel")]
            TouchCancel,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct DispatchTouchEventRequest {
            #[serde(rename = "type")]
            r#type: Option<DispatchTouchEventRequestType>,

            #[serde(rename = "touchPoints")]
            touchpoints: Option<Vec<TouchPoint>>,

            modifiers: Integer,

            timestamp: TimeSinceEpoch,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum EmulateTouchFromMouseEventRequestType {
            #[serde(rename = "mousePressed")]
            MousePressed,

            #[serde(rename = "mouseReleased")]
            MouseReleased,

            #[serde(rename = "mouseMoved")]
            MouseMoved,

            #[serde(rename = "mouseWheel")]
            MouseWheel,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct EmulateTouchFromMouseEventRequest {
            #[serde(rename = "type")]
            r#type: Option<EmulateTouchFromMouseEventRequestType>,

            x: Option<Integer>,

            y: Option<Integer>,

            button: Option<MouseButton>,

            timestamp: TimeSinceEpoch,

            #[serde(rename = "deltaX")]
            deltax: f64,

            #[serde(rename = "deltaY")]
            deltay: f64,

            modifiers: Integer,

            #[serde(rename = "clickCount")]
            clickcount: Integer,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetIgnoreInputEventsRequest {
            ignore: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetInterceptDragsRequest {
            enabled: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SynthesizePinchGestureRequest {
            x: Option<f64>,

            y: Option<f64>,

            #[serde(rename = "scaleFactor")]
            scale_factor: Option<f64>,

            #[serde(rename = "relativeSpeed")]
            relative_speed: Integer,

            #[serde(rename = "gestureSourceType")]
            gesture_source_type: GestureSourceType,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SynthesizeScrollGestureRequest {
            x: Option<f64>,

            y: Option<f64>,

            #[serde(rename = "xDistance")]
            x_distance: f64,

            #[serde(rename = "yDistance")]
            y_distance: f64,

            #[serde(rename = "xOverscroll")]
            x_overscroll: f64,

            #[serde(rename = "yOverscroll")]
            y_overscroll: f64,

            #[serde(rename = "preventFling")]
            prevent_fling: bool,

            speed: Integer,

            #[serde(rename = "gestureSourceType")]
            gesture_source_type: GestureSourceType,

            #[serde(rename = "repeatCount")]
            repeat_count: Integer,

            #[serde(rename = "repeatDelayMs")]
            repeat_delay_ms: Integer,

            #[serde(rename = "interactionMarkerName")]
            interaction_marker_name: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SynthesizeTapGestureRequest {
            x: Option<f64>,

            y: Option<f64>,

            duration: Integer,

            #[serde(rename = "tapCount")]
            tapcount: Integer,

            #[serde(rename = "gestureSourceType")]
            gesturesourcetype: GestureSourceType,
        }

        /**
         * Emitted only when `Input.setInterceptDrags` is enabled. Use this data with `Input.dispatchDragEvent` to
         * restore normal drag and drop behavior.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct DragInterceptedEvent {
            /**
             * Emitted only when `Input.setInterceptDrags` is enabled. Use this data with `Input.dispatchDragEvent` to
             * restore normal drag and drop behavior.
            */
            data: Option<DragData>,
        }
    }

    pub mod inspector {
        use serde::{self, Deserialize, Serialize};
        /**
         * Fired when remote debugging connection is about to be terminated. Contains detach reason.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct DetachedEvent {
            /**
             * Fired when remote debugging connection is about to be terminated. Contains detach reason.
            */
            reason: Option<String>,
        }
    }

    pub mod layer_tree {
        use super::{dom, Integer};
        use serde::{self, Deserialize, Serialize};
        /**
         * Unique Layer identifier.
        */
        pub type LayerId = String;

        /**
         * Unique snapshot identifier.
        */
        pub type SnapshotId = String;

        #[derive(Debug, Serialize, Deserialize)]
        pub enum ScrollRectType {
            #[serde(rename = "RepaintsOnScroll")]
            RepaintsOnScroll,

            #[serde(rename = "TouchEventHandler")]
            TouchEventHandler,

            #[serde(rename = "WheelEventHandler")]
            WheelEventHandler,
        }

        /**
         * Rectangle where scrolling happens on the main thread.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ScrollRect {
            /**
             * Rectangle where scrolling happens on the main thread.
            */
            rect: Option<dom::Rect>,
            /**
             * Rectangle where scrolling happens on the main thread.
            */
            #[serde(rename = "type")]
            r#type: Option<ScrollRectType>,
        }

        /**
         * Sticky position constraints.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct StickyPositionConstraint {
            /**
             * Sticky position constraints.
            */
            #[serde(rename = "stickyBoxRect")]
            sticky_box_rect: Option<dom::Rect>,
            /**
             * Sticky position constraints.
            */
            #[serde(rename = "containingBlockRect")]
            containing_block_rect: Option<dom::Rect>,
            /**
             * Sticky position constraints.
            */
            #[serde(rename = "nearestLayerShiftingStickyBox")]
            nearest_layer_shifting_sticky_box: LayerId,
            /**
             * Sticky position constraints.
            */
            #[serde(rename = "nearestLayerShiftingContainingBlock")]
            nearest_layer_shifting_containing_block: LayerId,
        }

        /**
         * Serialized fragment of layer picture along with its offset within the layer.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PictureTile {
            /**
             * Serialized fragment of layer picture along with its offset within the layer.
            */
            x: Option<f64>,
            /**
             * Serialized fragment of layer picture along with its offset within the layer.
            */
            y: Option<f64>,
            /**
             * Serialized fragment of layer picture along with its offset within the layer.
            */
            picture: Option<String>,
        }

        /**
         * Information about a compositing layer.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Layer {
            /**
             * Information about a compositing layer.
            */
            #[serde(rename = "layerId")]
            layer_id: Option<LayerId>,
            /**
             * Information about a compositing layer.
            */
            #[serde(rename = "parentLayerId")]
            parent_layer_id: LayerId,
            /**
             * Information about a compositing layer.
            */
            #[serde(rename = "backendNodeId")]
            backend_node_id: dom::BackendNodeId,
            /**
             * Information about a compositing layer.
            */
            #[serde(rename = "offsetX")]
            offset_x: Option<f64>,
            /**
             * Information about a compositing layer.
            */
            #[serde(rename = "offsetY")]
            offset_y: Option<f64>,
            /**
             * Information about a compositing layer.
            */
            width: Option<f64>,
            /**
             * Information about a compositing layer.
            */
            height: Option<f64>,
            /**
             * Information about a compositing layer.
            */
            transform: Vec<f64>,
            /**
             * Information about a compositing layer.
            */
            #[serde(rename = "anchorX")]
            anchor_x: f64,
            /**
             * Information about a compositing layer.
            */
            #[serde(rename = "anchorY")]
            anchor_y: f64,
            /**
             * Information about a compositing layer.
            */
            #[serde(rename = "anchorZ")]
            anchor_z: f64,
            /**
             * Information about a compositing layer.
            */
            #[serde(rename = "paintCount")]
            paint_count: Option<Integer>,
            /**
             * Information about a compositing layer.
            */
            #[serde(rename = "drawsContent")]
            draws_content: Option<bool>,
            /**
             * Information about a compositing layer.
            */
            invisible: bool,
            /**
             * Information about a compositing layer.
            */
            #[serde(rename = "scrollRects")]
            scroll_rects: Vec<ScrollRect>,
            /**
             * Information about a compositing layer.
            */
            #[serde(rename = "stickyPositionConstraint")]
            sticky_position_constraint: StickyPositionConstraint,
        }

        /**
         * Array of timings, one per paint step.
        */
        pub type PaintProfile = Vec<f64>;

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CompositingReasonsRequest {
            #[serde(rename = "layerId")]
            layer_id: Option<LayerId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CompositingReasonsResponse {
            #[serde(rename = "compositingReasons")]
            compositing_reasons: Option<Vec<String>>,

            #[serde(rename = "compositingReasonIds")]
            compositing_reason_ids: Option<Vec<String>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct LoadSnapshotRequest {
            tiles: Option<Vec<PictureTile>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct LoadSnapshotResponse {
            #[serde(rename = "snapshotId")]
            snapshot_id: Option<SnapshotId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct MakeSnapshotRequest {
            #[serde(rename = "layerId")]
            layer_id: Option<LayerId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct MakeSnapshotResponse {
            #[serde(rename = "snapshotId")]
            snapshot_id: Option<SnapshotId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ProfileSnapshotRequest {
            #[serde(rename = "snapshotId")]
            snapshot_id: Option<SnapshotId>,

            #[serde(rename = "minRepeatCount")]
            min_repeat_count: Integer,

            #[serde(rename = "minDuration")]
            min_duration: f64,

            #[serde(rename = "clipRect")]
            clip_rect: dom::Rect,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ProfileSnapshotResponse {
            timings: Option<Vec<PaintProfile>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ReleaseSnapshotRequest {
            #[serde(rename = "snapshotId")]
            snapshot_id: Option<SnapshotId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ReplaySnapshotRequest {
            #[serde(rename = "snapshotId")]
            snapshot_id: Option<SnapshotId>,

            #[serde(rename = "fromStep")]
            from_step: Integer,

            #[serde(rename = "toStep")]
            to_step: Integer,

            scale: f64,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ReplaySnapshotResponse {
            #[serde(rename = "dataURL")]
            data_url: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SnapshotCommandLogRequest {
            #[serde(rename = "snapshotId")]
            snapshot_id: Option<SnapshotId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SnapshotCommandLogResponse {
            #[serde(rename = "commandLog")]
            command_log: Option<Vec<serde_json::Value>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct LayerPaintedEvent {
            #[serde(rename = "layerId")]
            layer_id: Option<LayerId>,

            clip: Option<dom::Rect>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct LayerTreeDidChangeEvent {
            layers: Vec<Layer>,
        }
    }

    /**
     * Provides access to log entries.
    */
    pub mod log {
        use super::{network, runtime, Integer};
        use serde::{self, Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize)]
        pub enum LogEntrySource {
            #[serde(rename = "xml")]
            XML,

            #[serde(rename = "javascript")]
            Javascript,

            #[serde(rename = "network")]
            Network,

            #[serde(rename = "storage")]
            Storage,

            #[serde(rename = "appcache")]
            Appcache,

            #[serde(rename = "rendering")]
            Rendering,

            #[serde(rename = "security")]
            Security,

            #[serde(rename = "deprecation")]
            Deprecation,

            #[serde(rename = "worker")]
            Worker,

            #[serde(rename = "violation")]
            Violation,

            #[serde(rename = "intervention")]
            Intervention,

            #[serde(rename = "recommendation")]
            Recommendation,

            #[serde(rename = "other")]
            Other,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum LogEntryLevel {
            #[serde(rename = "verbose")]
            Verbose,

            #[serde(rename = "info")]
            Info,

            #[serde(rename = "warning")]
            Warning,

            #[serde(rename = "error")]
            Error,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum LogEntryCategory {
            #[serde(rename = "cors")]
            Cors,
        }

        /**
         * Log entry.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct LogEntry {
            /**
             * Log entry.
            */
            source: Option<LogEntrySource>,
            /**
             * Log entry.
            */
            level: Option<LogEntryLevel>,
            /**
             * Log entry.
            */
            text: Option<String>,
            /**
             * Log entry.
            */
            category: LogEntryCategory,
            /**
             * Log entry.
            */
            timestamp: Option<runtime::Timestamp>,
            /**
             * Log entry.
            */
            url: String,
            /**
             * Log entry.
            */
            #[serde(rename = "lineNumber")]
            linenumber: Integer,
            /**
             * Log entry.
            */
            #[serde(rename = "stackTrace")]
            stacktrace: runtime::StackTrace,
            /**
             * Log entry.
            */
            #[serde(rename = "networkRequestId")]
            networkrequestid: network::RequestId,
            /**
             * Log entry.
            */
            #[serde(rename = "workerId")]
            workerid: String,
            /**
             * Log entry.
            */
            args: Vec<runtime::RemoteObject>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum ViolationSettingName {
            #[serde(rename = "longTask")]
            LongTask,

            #[serde(rename = "longLayout")]
            LongLayout,

            #[serde(rename = "blockedEvent")]
            BlockedEvent,

            #[serde(rename = "blockedParser")]
            BlockedParser,

            #[serde(rename = "discouragedAPIUse")]
            DiscouragedAPIUse,

            #[serde(rename = "handler")]
            Handler,

            #[serde(rename = "recurringHandler")]
            RecurringHandler,
        }

        /**
         * Violation configuration setting.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ViolationSetting {
            /**
             * Violation configuration setting.
            */
            name: Option<ViolationSettingName>,
            /**
             * Violation configuration setting.
            */
            threshold: Option<f64>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct StartViolationsReportRequest {
            config: Option<Vec<ViolationSetting>>,
        }

        /**
         * Issued when new message was logged.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EntryAddedEvent {
            /**
             * Issued when new message was logged.
            */
            entry: Option<LogEntry>,
        }
    }

    pub mod memory {
        use super::Integer;
        use serde::{self, Deserialize, Serialize};
        /**
         * Memory pressure level.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum PressureLevel {
            #[serde(rename = "moderate")]
            Moderate,
            #[serde(rename = "critical")]
            Critical,
        }

        /**
         * Heap profile sample.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SamplingProfileNode {
            /**
             * Heap profile sample.
            */
            size: Option<f64>,
            /**
             * Heap profile sample.
            */
            total: Option<f64>,
            /**
             * Heap profile sample.
            */
            stack: Option<Vec<String>>,
        }

        /**
         * Array of heap profile samples.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SamplingProfile {
            /**
             * Array of heap profile samples.
            */
            samples: Option<Vec<SamplingProfileNode>>,
            /**
             * Array of heap profile samples.
            */
            modules: Option<Vec<Module>>,
        }

        /**
         * Executable module information
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Module {
            /**
             * Executable module information
            */
            name: Option<String>,
            /**
             * Executable module information
            */
            uuid: Option<String>,
            /**
             * Executable module information
            */
            #[serde(rename = "baseAddress")]
            baseaddress: Option<String>,
            /**
             * Executable module information
            */
            size: Option<f64>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetDOMCountersResponse {
            documents: Option<Integer>,

            nodes: Option<Integer>,

            #[serde(rename = "jsEventListeners")]
            jseventlisteners: Option<Integer>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetPressureNotificationsSuppressedRequest {
            suppressed: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SimulatePressureNotificationRequest {
            level: Option<PressureLevel>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct StartSamplingRequest {
            #[serde(rename = "samplingInterval")]
            sampling_interval: Integer,

            #[serde(rename = "suppressRandomness")]
            suppress_randomness: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetAllTimeSamplingProfileResponse {
            profile: Option<SamplingProfile>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetBrowserSamplingProfileResponse {
            profile: Option<SamplingProfile>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetSamplingProfileResponse {
            profile: Option<SamplingProfile>,
        }
    }

    /**
     * Network domain allows tracking network activities of the page. It exposes information about http,
     * file, data and other requests and responses, their headers, bodies, timing, etc.
    */
    pub mod network {
        use super::{debugger, emulation, io, network, page, runtime, security, Integer};
        use serde::{self, Deserialize, Serialize};
        use std::collections::HashMap;
        /**
         * Resource type as it was perceived by the rendering engine.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum ResourceType {
            #[serde(rename = "Document")]
            Document,
            #[serde(rename = "Stylesheet")]
            Stylesheet,
            #[serde(rename = "Image")]
            Image,
            #[serde(rename = "Media")]
            Media,
            #[serde(rename = "Font")]
            Font,
            #[serde(rename = "Script")]
            Script,
            #[serde(rename = "TextTrack")]
            TextTrack,
            #[serde(rename = "XHR")]
            XHR,
            #[serde(rename = "Fetch")]
            Fetch,
            #[serde(rename = "Prefetch")]
            Prefetch,
            #[serde(rename = "EventSource")]
            EventSource,
            #[serde(rename = "WebSocket")]
            WebSocket,
            #[serde(rename = "Manifest")]
            Manifest,
            #[serde(rename = "SignedExchange")]
            SignedExchange,
            #[serde(rename = "Ping")]
            Ping,
            #[serde(rename = "CSPViolationReport")]
            CSPViolationReport,
            #[serde(rename = "Preflight")]
            Preflight,
            #[serde(rename = "Other")]
            Other,
        }

        /**
         * Unique loader identifier.
        */
        pub type LoaderId = String;

        /**
         * Unique request identifier.
        */
        pub type RequestId = String;

        /**
         * Unique intercepted request identifier.
        */
        pub type InterceptionId = String;

        /**
         * Network level fetch failure reason.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum ErrorReason {
            #[serde(rename = "Failed")]
            Failed,
            #[serde(rename = "Aborted")]
            Aborted,
            #[serde(rename = "TimedOut")]
            TimedOut,
            #[serde(rename = "AccessDenied")]
            AccessDenied,
            #[serde(rename = "ConnectionClosed")]
            ConnectionClosed,
            #[serde(rename = "ConnectionReset")]
            ConnectionReset,
            #[serde(rename = "ConnectionRefused")]
            ConnectionRefused,
            #[serde(rename = "ConnectionAborted")]
            ConnectionAborted,
            #[serde(rename = "ConnectionFailed")]
            ConnectionFailed,
            #[serde(rename = "NameNotResolved")]
            NameNotResolved,
            #[serde(rename = "InternetDisconnected")]
            InternetDisconnected,
            #[serde(rename = "AddressUnreachable")]
            AddressUnreachable,
            #[serde(rename = "BlockedByClient")]
            BlockedByClient,
            #[serde(rename = "BlockedByResponse")]
            BlockedByResponse,
        }

        /**
         * UTC time in seconds, counted from January 1, 1970.
        */
        pub type TimeSinceEpoch = f64;

        /**
         * Monotonically increasing time in seconds since an arbitrary point in the past.
        */
        pub type MonotonicTime = f64;

        /**
         * Request / response headers as keys / values of JSON object.
        */
        pub type Headers = HashMap<String, String>;
        /**
         * The underlying connection technology that the browser is supposedly using.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum ConnectionType {
            #[serde(rename = "none")]
            None,
            #[serde(rename = "cellular2g")]
            Cellular2g,
            #[serde(rename = "cellular3g")]
            Cellular3g,
            #[serde(rename = "cellular4g")]
            Cellular4g,
            #[serde(rename = "bluetooth")]
            Bluetooth,
            #[serde(rename = "ethernet")]
            Ethernet,
            #[serde(rename = "wifi")]
            Wifi,
            #[serde(rename = "wimax")]
            Wimax,
            #[serde(rename = "other")]
            Other,
        }

        /**
         * Represents the cookie's 'SameSite' status:
         * https://tools.ietf.org/html/draft-west-first-party-cookies
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum CookieSameSite {
            #[serde(rename = "Strict")]
            Strict,
            #[serde(rename = "Lax")]
            Lax,
            #[serde(rename = "None")]
            None,
        }

        /**
         * Represents the cookie's 'Priority' status:
         * https://tools.ietf.org/html/draft-west-cookie-priority-00
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum CookiePriority {
            #[serde(rename = "Low")]
            Low,
            #[serde(rename = "Medium")]
            Medium,
            #[serde(rename = "High")]
            High,
        }

        /**
         * Represents the source scheme of the origin that originally set the cookie.
         * A value of "Unset" allows protocol clients to emulate legacy cookie scope for the scheme.
         * This is a temporary ability and it will be removed in the future.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum CookieSourceScheme {
            #[serde(rename = "Unset")]
            Unset,
            #[serde(rename = "NonSecure")]
            NonSecure,
            #[serde(rename = "Secure")]
            Secure,
        }

        /**
         * Timing information for the request.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ResourceTiming {
            /**
             * Timing information for the request.
            */
            #[serde(rename = "requestTime")]
            request_time: Option<f64>,
            /**
             * Timing information for the request.
            */
            #[serde(rename = "proxyStart")]
            proxy_start: Option<f64>,
            /**
             * Timing information for the request.
            */
            #[serde(rename = "proxyEnd")]
            proxy_end: Option<f64>,
            /**
             * Timing information for the request.
            */
            #[serde(rename = "dnsStart")]
            dns_start: Option<f64>,
            /**
             * Timing information for the request.
            */
            #[serde(rename = "dnsEnd")]
            dns_end: Option<f64>,
            /**
             * Timing information for the request.
            */
            #[serde(rename = "connectStart")]
            connect_start: Option<f64>,
            /**
             * Timing information for the request.
            */
            #[serde(rename = "connectEnd")]
            connect_end: Option<f64>,
            /**
             * Timing information for the request.
            */
            #[serde(rename = "sslStart")]
            ssl_start: Option<f64>,
            /**
             * Timing information for the request.
            */
            #[serde(rename = "sslEnd")]
            ssl_end: Option<f64>,
            /**
             * Timing information for the request.
            */
            #[serde(rename = "workerStart")]
            worker_start: Option<f64>,
            /**
             * Timing information for the request.
            */
            #[serde(rename = "workerReady")]
            worker_ready: Option<f64>,
            /**
             * Timing information for the request.
            */
            #[serde(rename = "workerFetchStart")]
            worker_fetch_start: Option<f64>,
            /**
             * Timing information for the request.
            */
            #[serde(rename = "workerRespondWithSettled")]
            worker_respond_with_settled: Option<f64>,
            /**
             * Timing information for the request.
            */
            #[serde(rename = "sendStart")]
            send_start: Option<f64>,
            /**
             * Timing information for the request.
            */
            #[serde(rename = "sendEnd")]
            send_end: Option<f64>,
            /**
             * Timing information for the request.
            */
            #[serde(rename = "pushStart")]
            push_start: Option<f64>,
            /**
             * Timing information for the request.
            */
            #[serde(rename = "pushEnd")]
            push_end: Option<f64>,
            /**
             * Timing information for the request.
            */
            #[serde(rename = "receiveHeadersStart")]
            receive_headers_start: Option<f64>,
            /**
             * Timing information for the request.
            */
            #[serde(rename = "receiveHeadersEnd")]
            receive_headers_end: Option<f64>,
        }

        /**
         * Loading priority of a resource request.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum ResourcePriority {
            #[serde(rename = "VeryLow")]
            VeryLow,
            #[serde(rename = "Low")]
            Low,
            #[serde(rename = "Medium")]
            Medium,
            #[serde(rename = "High")]
            High,
            #[serde(rename = "VeryHigh")]
            VeryHigh,
        }

        /**
         * Post data entry for HTTP request
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PostDataEntry {
            /**
             * Post data entry for HTTP request
            */
            bytes: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum RequestReferrerPolicy {
            #[serde(rename = "unsafe-url")]
            UnsafeUrl,

            #[serde(rename = "no-referrer-when-downgrade")]
            NoReferrerWhenDowngrade,

            #[serde(rename = "no-referrer")]
            NoReferrer,

            #[serde(rename = "origin")]
            Origin,

            #[serde(rename = "origin-when-cross-origin")]
            OriginWhenCrossOrigin,

            #[serde(rename = "same-origin")]
            SameOrigin,

            #[serde(rename = "strict-origin")]
            StrictOrigin,

            #[serde(rename = "strict-origin-when-cross-origin")]
            StrictOriginWhenCrossOrigin,
        }

        /**
         * HTTP request data.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Request {
            /**
             * HTTP request data.
            */
            url: Option<String>,
            /**
             * HTTP request data.
            */
            #[serde(rename = "urlFragment")]
            url_fragment: String,
            /**
             * HTTP request data.
            */
            method: Option<String>,
            /**
             * HTTP request data.
            */
            headers: Option<Headers>,
            /**
             * HTTP request data.
            */
            #[serde(rename = "postData")]
            post_data: String,
            /**
             * HTTP request data.
            */
            #[serde(rename = "hasPostData")]
            has_post_data: bool,
            /**
             * HTTP request data.
            */
            #[serde(rename = "postDataEntries")]
            post_data_entries: Vec<PostDataEntry>,
            /**
             * HTTP request data.
            */
            #[serde(rename = "mixedContentType")]
            mixed_content_type: security::MixedContentType,
            /**
             * HTTP request data.
            */
            #[serde(rename = "initialPriority")]
            initial_priority: Option<ResourcePriority>,
            /**
             * HTTP request data.
            */
            #[serde(rename = "referrerPolicy")]
            referrer_policy: Option<RequestReferrerPolicy>,
            /**
             * HTTP request data.
            */
            #[serde(rename = "isLinkPreload")]
            is_link_preload: bool,
            /**
             * HTTP request data.
            */
            #[serde(rename = "trustTokenParams")]
            trust_token_params: TrustTokenParams,
            /**
             * HTTP request data.
            */
            #[serde(rename = "isSameSite")]
            is_same_site: bool,
        }

        /**
         * Details of a signed certificate timestamp (SCT).
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SignedCertificateTimestamp {
            /**
             * Details of a signed certificate timestamp (SCT).
            */
            status: Option<String>,
            /**
             * Details of a signed certificate timestamp (SCT).
            */
            origin: Option<String>,
            /**
             * Details of a signed certificate timestamp (SCT).
            */
            #[serde(rename = "logDescription")]
            log_description: Option<String>,
            /**
             * Details of a signed certificate timestamp (SCT).
            */
            #[serde(rename = "logId")]
            log_id: Option<String>,
            /**
             * Details of a signed certificate timestamp (SCT).
            */
            timestamp: Option<f64>,
            /**
             * Details of a signed certificate timestamp (SCT).
            */
            #[serde(rename = "hashAlgorithm")]
            hash_algorithm: Option<String>,
            /**
             * Details of a signed certificate timestamp (SCT).
            */
            #[serde(rename = "signatureAlgorithm")]
            signature_algorithm: Option<String>,
            /**
             * Details of a signed certificate timestamp (SCT).
            */
            #[serde(rename = "signatureData")]
            signature_data: Option<String>,
        }

        /**
         * Security details about a request.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SecurityDetails {
            /**
             * Security details about a request.
            */
            protocol: Option<String>,
            /**
             * Security details about a request.
            */
            #[serde(rename = "keyExchange")]
            key_exchange: Option<String>,
            /**
             * Security details about a request.
            */
            #[serde(rename = "keyExchangeGroup")]
            key_exchange_group: String,
            /**
             * Security details about a request.
            */
            cipher: Option<String>,
            /**
             * Security details about a request.
            */
            mac: String,
            /**
             * Security details about a request.
            */
            #[serde(rename = "certificateId")]
            certificate_id: Option<security::CertificateId>,
            /**
             * Security details about a request.
            */
            #[serde(rename = "subjectName")]
            subject_name: Option<String>,
            /**
             * Security details about a request.
            */
            #[serde(rename = "sanList")]
            san_list: Option<Vec<String>>,
            /**
             * Security details about a request.
            */
            issuer: Option<String>,
            /**
             * Security details about a request.
            */
            #[serde(rename = "validFrom")]
            valid_from: Option<TimeSinceEpoch>,
            /**
             * Security details about a request.
            */
            #[serde(rename = "validTo")]
            valid_to: Option<TimeSinceEpoch>,
            /**
             * Security details about a request.
            */
            #[serde(rename = "signedCertificateTimestampList")]
            signed_certificate_timestamp_list: Option<Vec<SignedCertificateTimestamp>>,
            /**
             * Security details about a request.
            */
            #[serde(rename = "certificateTransparencyCompliance")]
            certificate_transparency_compliance: Option<CertificateTransparencyCompliance>,
            /**
             * Security details about a request.
            */
            #[serde(rename = "serverSignatureAlgorithm")]
            server_signature_algorithm: Integer,
            /**
             * Security details about a request.
            */
            #[serde(rename = "encryptedClientHello")]
            encrypted_client_hello: Option<bool>,
        }

        /**
         * Whether the request complied with Certificate Transparency policy.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum CertificateTransparencyCompliance {
            #[serde(rename = "unknown")]
            Unknown,
            #[serde(rename = "not-compliant")]
            NotCompliant,
            #[serde(rename = "compliant")]
            Compliant,
        }

        /**
         * The reason why request was blocked.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum BlockedReason {
            #[serde(rename = "other")]
            Other,
            #[serde(rename = "csp")]
            Csp,
            #[serde(rename = "mixed-content")]
            MixedContent,
            #[serde(rename = "origin")]
            Origin,
            #[serde(rename = "inspector")]
            Inspector,
            #[serde(rename = "subresource-filter")]
            SubresourceFilter,
            #[serde(rename = "content-type")]
            ContentType,
            #[serde(rename = "coep-frame-resource-needs-coep-header")]
            CoepFrameResourceNeedsCoepHeader,
            #[serde(rename = "coop-sandboxed-iframe-cannot-navigate-to-coop-page")]
            CoopSandboxedIframeCannotNavigateToCoopPage,
            #[serde(rename = "corp-not-same-origin")]
            CorpNotSameOrigin,
            #[serde(rename = "corp-not-same-origin-after-defaulted-to-same-origin-by-coep")]
            CorpNotSameOriginAfterDefaultedToSameOriginByCoep,
            #[serde(rename = "corp-not-same-site")]
            CorpNotSameSite,
        }

        /**
         * The reason why request was blocked.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum CorsError {
            #[serde(rename = "DisallowedByMode")]
            DisallowedByMode,
            #[serde(rename = "InvalidResponse")]
            InvalidResponse,
            #[serde(rename = "WildcardOriginNotAllowed")]
            WildcardOriginNotAllowed,
            #[serde(rename = "MissingAllowOriginHeader")]
            MissingAllowOriginHeader,
            #[serde(rename = "MultipleAllowOriginValues")]
            MultipleAllowOriginValues,
            #[serde(rename = "InvalidAllowOriginValue")]
            InvalidAllowOriginValue,
            #[serde(rename = "AllowOriginMismatch")]
            AllowOriginMismatch,
            #[serde(rename = "InvalidAllowCredentials")]
            InvalidAllowCredentials,
            #[serde(rename = "CorsDisabledScheme")]
            CorsDisabledScheme,
            #[serde(rename = "PreflightInvalidStatus")]
            PreflightInvalidStatus,
            #[serde(rename = "PreflightDisallowedRedirect")]
            PreflightDisallowedRedirect,
            #[serde(rename = "PreflightWildcardOriginNotAllowed")]
            PreflightWildcardOriginNotAllowed,
            #[serde(rename = "PreflightMissingAllowOriginHeader")]
            PreflightMissingAllowOriginHeader,
            #[serde(rename = "PreflightMultipleAllowOriginValues")]
            PreflightMultipleAllowOriginValues,
            #[serde(rename = "PreflightInvalidAllowOriginValue")]
            PreflightInvalidAllowOriginValue,
            #[serde(rename = "PreflightAllowOriginMismatch")]
            PreflightAllowOriginMismatch,
            #[serde(rename = "PreflightInvalidAllowCredentials")]
            PreflightInvalidAllowCredentials,
            #[serde(rename = "PreflightMissingAllowExternal")]
            PreflightMissingAllowExternal,
            #[serde(rename = "PreflightInvalidAllowExternal")]
            PreflightInvalidAllowExternal,
            #[serde(rename = "PreflightMissingAllowPrivateNetwork")]
            PreflightMissingAllowPrivateNetwork,
            #[serde(rename = "PreflightInvalidAllowPrivateNetwork")]
            PreflightInvalidAllowPrivateNetwork,
            #[serde(rename = "InvalidAllowMethodsPreflightResponse")]
            InvalidAllowMethodsPreflightResponse,
            #[serde(rename = "InvalidAllowHeadersPreflightResponse")]
            InvalidAllowHeadersPreflightResponse,
            #[serde(rename = "MethodDisallowedByPreflightResponse")]
            MethodDisallowedByPreflightResponse,
            #[serde(rename = "HeaderDisallowedByPreflightResponse")]
            HeaderDisallowedByPreflightResponse,
            #[serde(rename = "RedirectContainsCredentials")]
            RedirectContainsCredentials,
            #[serde(rename = "InsecurePrivateNetwork")]
            InsecurePrivateNetwork,
            #[serde(rename = "InvalidPrivateNetworkAccess")]
            InvalidPrivateNetworkAccess,
            #[serde(rename = "UnexpectedPrivateNetworkAccess")]
            UnexpectedPrivateNetworkAccess,
            #[serde(rename = "NoCorsRedirectModeNotFollow")]
            NoCorsRedirectModeNotFollow,
            #[serde(rename = "PreflightMissingPrivateNetworkAccessId")]
            PreflightMissingPrivateNetworkAccessId,
            #[serde(rename = "PreflightMissingPrivateNetworkAccessName")]
            PreflightMissingPrivateNetworkAccessName,
            #[serde(rename = "PrivateNetworkAccessPermissionUnavailable")]
            PrivateNetworkAccessPermissionUnavailable,
            #[serde(rename = "PrivateNetworkAccessPermissionDenied")]
            PrivateNetworkAccessPermissionDenied,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CorsErrorStatus {
            #[serde(rename = "corsError")]
            cors_error: Option<CorsError>,

            #[serde(rename = "failedParameter")]
            failed_parameter: Option<String>,
        }

        /**
         * Source of serviceworker response.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum ServiceWorkerResponseSource {
            #[serde(rename = "cache-storage")]
            CacheStorage,
            #[serde(rename = "http-cache")]
            HttpCache,
            #[serde(rename = "fallback-code")]
            FallbackCode,
            #[serde(rename = "network")]
            Network,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum TrustTokenParamsRefreshPolicy {
            #[serde(rename = "UseCached")]
            UseCached,

            #[serde(rename = "Refresh")]
            Refresh,
        }

        /**
         * Determines what type of Trust Token operation is executed and
         * depending on the type, some additional parameters. The values
         * are specified in third_party/blink/renderer/core/fetch/trust_token.idl.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct TrustTokenParams {
            /**
             * Determines what type of Trust Token operation is executed and
             * depending on the type, some additional parameters. The values
             * are specified in third_party/blink/renderer/core/fetch/trust_token.idl.
            */
            operation: Option<TrustTokenOperationType>,
            /**
             * Determines what type of Trust Token operation is executed and
             * depending on the type, some additional parameters. The values
             * are specified in third_party/blink/renderer/core/fetch/trust_token.idl.
            */
            #[serde(rename = "refreshPolicy")]
            refreshpolicy: Option<TrustTokenParamsRefreshPolicy>,
            /**
             * Determines what type of Trust Token operation is executed and
             * depending on the type, some additional parameters. The values
             * are specified in third_party/blink/renderer/core/fetch/trust_token.idl.
            */
            issuers: Vec<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum TrustTokenOperationType {
            #[serde(rename = "Issuance")]
            Issuance,
            #[serde(rename = "Redemption")]
            Redemption,
            #[serde(rename = "Signing")]
            Signing,
        }

        /**
         * The reason why Chrome uses a specific transport protocol for HTTP semantics.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum AlternateProtocolUsage {
            #[serde(rename = "alternativeJobWonWithoutRace")]
            AlternativeJobWonWithoutRace,
            #[serde(rename = "alternativeJobWonRace")]
            AlternativeJobWonRace,
            #[serde(rename = "mainJobWonRace")]
            MainJobWonRace,
            #[serde(rename = "mappingMissing")]
            MappingMissing,
            #[serde(rename = "broken")]
            Broken,
            #[serde(rename = "dnsAlpnH3JobWonWithoutRace")]
            DnsAlpnH3JobWonWithoutRace,
            #[serde(rename = "dnsAlpnH3JobWonRace")]
            DnsAlpnH3JobWonRace,
            #[serde(rename = "unspecifiedReason")]
            UnspecifiedReason,
        }

        /**
         * HTTP response data.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Response {
            /**
             * HTTP response data.
            */
            url: Option<String>,
            /**
             * HTTP response data.
            */
            status: Option<Integer>,
            /**
             * HTTP response data.
            */
            #[serde(rename = "statusText")]
            status_text: Option<String>,
            /**
             * HTTP response data.
            */
            headers: Option<Headers>,
            /**
             * HTTP response data.
            */
            #[serde(rename = "headersText")]
            headers_text: String,
            /**
             * HTTP response data.
            */
            #[serde(rename = "mimeType")]
            mime_type: Option<String>,
            /**
             * HTTP response data.
            */
            #[serde(rename = "requestHeaders")]
            request_headers: Headers,
            /**
             * HTTP response data.
            */
            #[serde(rename = "requestHeadersText")]
            request_headers_text: String,
            /**
             * HTTP response data.
            */
            #[serde(rename = "connectionReused")]
            connection_reused: Option<bool>,
            /**
             * HTTP response data.
            */
            #[serde(rename = "connectionId")]
            connection_id: Option<f64>,
            /**
             * HTTP response data.
            */
            #[serde(rename = "remoteIPAddress")]
            remote_ipaddress: String,
            /**
             * HTTP response data.
            */
            #[serde(rename = "remotePort")]
            remote_port: Integer,
            /**
             * HTTP response data.
            */
            #[serde(rename = "fromDiskCache")]
            from_disk_cache: bool,
            /**
             * HTTP response data.
            */
            #[serde(rename = "fromServiceWorker")]
            from_service_worker: bool,
            /**
             * HTTP response data.
            */
            #[serde(rename = "fromPrefetchCache")]
            from_prefetch_cache: bool,
            /**
             * HTTP response data.
            */
            #[serde(rename = "encodedDataLength")]
            encoded_data_length: Option<f64>,
            /**
             * HTTP response data.
            */
            timing: ResourceTiming,
            /**
             * HTTP response data.
            */
            #[serde(rename = "serviceWorkerResponseSource")]
            service_worker_response_source: ServiceWorkerResponseSource,
            /**
             * HTTP response data.
            */
            #[serde(rename = "responseTime")]
            response_time: TimeSinceEpoch,
            /**
             * HTTP response data.
            */
            #[serde(rename = "cacheStorageCacheName")]
            cache_storage_cache_name: String,
            /**
             * HTTP response data.
            */
            protocol: String,
            /**
             * HTTP response data.
            */
            #[serde(rename = "alternateProtocolUsage")]
            alternate_protocol_usage: AlternateProtocolUsage,
            /**
             * HTTP response data.
            */
            #[serde(rename = "securityState")]
            security_state: Option<security::SecurityState>,
            /**
             * HTTP response data.
            */
            #[serde(rename = "securityDetails")]
            security_details: SecurityDetails,
        }

        /**
         * WebSocket request data.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct WebSocketRequest {
            /**
             * WebSocket request data.
            */
            headers: Option<Headers>,
        }

        /**
         * WebSocket response data.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct WebSocketResponse {
            /**
             * WebSocket response data.
            */
            status: Option<Integer>,
            /**
             * WebSocket response data.
            */
            #[serde(rename = "statusText")]
            status_text: Option<String>,
            /**
             * WebSocket response data.
            */
            headers: Option<Headers>,
            /**
             * WebSocket response data.
            */
            #[serde(rename = "headersText")]
            headers_text: String,
            /**
             * WebSocket response data.
            */
            #[serde(rename = "requestHeaders")]
            request_headers: Headers,
            /**
             * WebSocket response data.
            */
            #[serde(rename = "requestHeadersText")]
            request_headers_text: String,
        }

        /**
         * WebSocket message data. This represents an entire WebSocket message, not just a fragmented frame as the name suggests.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct WebSocketFrame {
            /**
             * WebSocket message data. This represents an entire WebSocket message, not just a fragmented frame as the name suggests.
            */
            opcode: Option<f64>,
            /**
             * WebSocket message data. This represents an entire WebSocket message, not just a fragmented frame as the name suggests.
            */
            mask: Option<bool>,
            /**
             * WebSocket message data. This represents an entire WebSocket message, not just a fragmented frame as the name suggests.
            */
            #[serde(rename = "payloadData")]
            payloaddata: Option<String>,
        }

        /**
         * Information about the cached resource.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct CachedResource {
            /**
             * Information about the cached resource.
            */
            url: Option<String>,
            /**
             * Information about the cached resource.
            */
            #[serde(rename = "type")]
            r#type: Option<ResourceType>,
            /**
             * Information about the cached resource.
            */
            response: Response,
            /**
             * Information about the cached resource.
            */
            #[serde(rename = "bodySize")]
            bodysize: Option<f64>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum InitiatorType {
            #[serde(rename = "parser")]
            Parser,

            #[serde(rename = "script")]
            Script,

            #[serde(rename = "preload")]
            Preload,

            #[serde(rename = "SignedExchange")]
            SignedExchange,

            #[serde(rename = "preflight")]
            Preflight,

            #[serde(rename = "other")]
            Other,
        }

        /**
         * Information about the request initiator.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Initiator {
            /**
             * Information about the request initiator.
            */
            #[serde(rename = "type")]
            r#type: Option<InitiatorType>,
            /**
             * Information about the request initiator.
            */
            stack: runtime::StackTrace,
            /**
             * Information about the request initiator.
            */
            url: String,
            /**
             * Information about the request initiator.
            */
            #[serde(rename = "lineNumber")]
            linenumber: f64,
            /**
             * Information about the request initiator.
            */
            #[serde(rename = "columnNumber")]
            columnnumber: f64,
            /**
             * Information about the request initiator.
            */
            #[serde(rename = "requestId")]
            requestid: RequestId,
        }

        /**
         * Cookie object
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Cookie {
            /**
             * Cookie object
            */
            name: Option<String>,
            /**
             * Cookie object
            */
            value: Option<String>,
            /**
             * Cookie object
            */
            domain: Option<String>,
            /**
             * Cookie object
            */
            path: Option<String>,
            /**
             * Cookie object
            */
            expires: Option<f64>,
            /**
             * Cookie object
            */
            size: Option<Integer>,
            /**
             * Cookie object
            */
            #[serde(rename = "httpOnly")]
            httponly: Option<bool>,
            /**
             * Cookie object
            */
            secure: Option<bool>,
            /**
             * Cookie object
            */
            session: Option<bool>,
            /**
             * Cookie object
            */
            #[serde(rename = "sameSite")]
            samesite: CookieSameSite,
            /**
             * Cookie object
            */
            priority: Option<CookiePriority>,
            /**
             * Cookie object
            */
            #[serde(rename = "sameParty")]
            sameparty: Option<bool>,
            /**
             * Cookie object
            */
            #[serde(rename = "sourceScheme")]
            sourcescheme: Option<CookieSourceScheme>,
            /**
             * Cookie object
            */
            #[serde(rename = "sourcePort")]
            sourceport: Option<Integer>,
            /**
             * Cookie object
            */
            #[serde(rename = "partitionKey")]
            partitionkey: String,
            /**
             * Cookie object
            */
            #[serde(rename = "partitionKeyOpaque")]
            partitionkeyopaque: bool,
        }

        /**
         * Types of reasons why a cookie may not be stored from a response.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum SetCookieBlockedReason {
            #[serde(rename = "SecureOnly")]
            SecureOnly,
            #[serde(rename = "SameSiteStrict")]
            SameSiteStrict,
            #[serde(rename = "SameSiteLax")]
            SameSiteLax,
            #[serde(rename = "SameSiteUnspecifiedTreatedAsLax")]
            SameSiteUnspecifiedTreatedAsLax,
            #[serde(rename = "SameSiteNoneInsecure")]
            SameSiteNoneInsecure,
            #[serde(rename = "UserPreferences")]
            UserPreferences,
            #[serde(rename = "ThirdPartyBlockedInFirstPartySet")]
            ThirdPartyBlockedInFirstPartySet,
            #[serde(rename = "SyntaxError")]
            SyntaxError,
            #[serde(rename = "SchemeNotSupported")]
            SchemeNotSupported,
            #[serde(rename = "OverwriteSecure")]
            OverwriteSecure,
            #[serde(rename = "InvalidDomain")]
            InvalidDomain,
            #[serde(rename = "InvalidPrefix")]
            InvalidPrefix,
            #[serde(rename = "UnknownError")]
            UnknownError,
            #[serde(rename = "SchemefulSameSiteStrict")]
            SchemefulSameSiteStrict,
            #[serde(rename = "SchemefulSameSiteLax")]
            SchemefulSameSiteLax,
            #[serde(rename = "SchemefulSameSiteUnspecifiedTreatedAsLax")]
            SchemefulSameSiteUnspecifiedTreatedAsLax,
            #[serde(rename = "SamePartyFromCrossPartyContext")]
            SamePartyFromCrossPartyContext,
            #[serde(rename = "SamePartyConflictsWithOtherAttributes")]
            SamePartyConflictsWithOtherAttributes,
            #[serde(rename = "NameValuePairExceedsMaxSize")]
            NameValuePairExceedsMaxSize,
            #[serde(rename = "DisallowedCharacter")]
            DisallowedCharacter,
        }

        /**
         * Types of reasons why a cookie may not be sent with a request.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum CookieBlockedReason {
            #[serde(rename = "SecureOnly")]
            SecureOnly,
            #[serde(rename = "NotOnPath")]
            NotOnPath,
            #[serde(rename = "DomainMismatch")]
            DomainMismatch,
            #[serde(rename = "SameSiteStrict")]
            SameSiteStrict,
            #[serde(rename = "SameSiteLax")]
            SameSiteLax,
            #[serde(rename = "SameSiteUnspecifiedTreatedAsLax")]
            SameSiteUnspecifiedTreatedAsLax,
            #[serde(rename = "SameSiteNoneInsecure")]
            SameSiteNoneInsecure,
            #[serde(rename = "UserPreferences")]
            UserPreferences,
            #[serde(rename = "ThirdPartyBlockedInFirstPartySet")]
            ThirdPartyBlockedInFirstPartySet,
            #[serde(rename = "UnknownError")]
            UnknownError,
            #[serde(rename = "SchemefulSameSiteStrict")]
            SchemefulSameSiteStrict,
            #[serde(rename = "SchemefulSameSiteLax")]
            SchemefulSameSiteLax,
            #[serde(rename = "SchemefulSameSiteUnspecifiedTreatedAsLax")]
            SchemefulSameSiteUnspecifiedTreatedAsLax,
            #[serde(rename = "SamePartyFromCrossPartyContext")]
            SamePartyFromCrossPartyContext,
            #[serde(rename = "NameValuePairExceedsMaxSize")]
            NameValuePairExceedsMaxSize,
        }

        /**
         * A cookie which was not stored from a response with the corresponding reason.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct BlockedSetCookieWithReason {
            /**
             * A cookie which was not stored from a response with the corresponding reason.
            */
            #[serde(rename = "blockedReasons")]
            blocked_reasons: Option<Vec<SetCookieBlockedReason>>,
            /**
             * A cookie which was not stored from a response with the corresponding reason.
            */
            #[serde(rename = "cookieLine")]
            cookie_line: Option<String>,
            /**
             * A cookie which was not stored from a response with the corresponding reason.
            */
            cookie: Cookie,
        }

        /**
         * A cookie with was not sent with a request with the corresponding reason.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct BlockedCookieWithReason {
            /**
             * A cookie with was not sent with a request with the corresponding reason.
            */
            #[serde(rename = "blockedReasons")]
            blocked_reasons: Option<Vec<CookieBlockedReason>>,
            /**
             * A cookie with was not sent with a request with the corresponding reason.
            */
            cookie: Option<Cookie>,
        }

        /**
         * Cookie parameter object
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct CookieParam {
            /**
             * Cookie parameter object
            */
            name: Option<String>,
            /**
             * Cookie parameter object
            */
            value: Option<String>,
            /**
             * Cookie parameter object
            */
            url: String,
            /**
             * Cookie parameter object
            */
            domain: String,
            /**
             * Cookie parameter object
            */
            path: String,
            /**
             * Cookie parameter object
            */
            secure: bool,
            /**
             * Cookie parameter object
            */
            #[serde(rename = "httpOnly")]
            httponly: bool,
            /**
             * Cookie parameter object
            */
            #[serde(rename = "sameSite")]
            samesite: CookieSameSite,
            /**
             * Cookie parameter object
            */
            expires: TimeSinceEpoch,
            /**
             * Cookie parameter object
            */
            priority: CookiePriority,
            /**
             * Cookie parameter object
            */
            #[serde(rename = "sameParty")]
            sameparty: bool,
            /**
             * Cookie parameter object
            */
            #[serde(rename = "sourceScheme")]
            sourcescheme: CookieSourceScheme,
            /**
             * Cookie parameter object
            */
            #[serde(rename = "sourcePort")]
            sourceport: Integer,
            /**
             * Cookie parameter object
            */
            #[serde(rename = "partitionKey")]
            partitionkey: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum AuthChallengeSource {
            #[serde(rename = "Server")]
            Server,

            #[serde(rename = "Proxy")]
            Proxy,
        }

        /**
         * Authorization challenge for HTTP status code 401 or 407.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AuthChallenge {
            /**
             * Authorization challenge for HTTP status code 401 or 407.
            */
            source: AuthChallengeSource,
            /**
             * Authorization challenge for HTTP status code 401 or 407.
            */
            origin: Option<String>,
            /**
             * Authorization challenge for HTTP status code 401 or 407.
            */
            scheme: Option<String>,
            /**
             * Authorization challenge for HTTP status code 401 or 407.
            */
            realm: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum AuthChallengeResponseResponse {
            #[serde(rename = "Default")]
            Default,

            #[serde(rename = "CancelAuth")]
            CancelAuth,

            #[serde(rename = "ProvideCredentials")]
            ProvideCredentials,
        }

        /**
         * Response to an AuthChallenge.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AuthChallengeResponse {
            /**
             * Response to an AuthChallenge.
            */
            response: Option<AuthChallengeResponseResponse>,
            /**
             * Response to an AuthChallenge.
            */
            username: String,
            /**
             * Response to an AuthChallenge.
            */
            password: String,
        }

        /**
         * Stages of the interception to begin intercepting. Request will intercept before the request is
         * sent. Response will intercept after the response is received.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum InterceptionStage {
            #[serde(rename = "Request")]
            Request,
            #[serde(rename = "HeadersReceived")]
            HeadersReceived,
        }

        /**
         * Request pattern for interception.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct RequestPattern {
            /**
             * Request pattern for interception.
            */
            #[serde(rename = "urlPattern")]
            url_pattern: String,
            /**
             * Request pattern for interception.
            */
            #[serde(rename = "resourceType")]
            resource_type: ResourceType,
            /**
             * Request pattern for interception.
            */
            #[serde(rename = "interceptionStage")]
            interception_stage: InterceptionStage,
        }

        /**
         * Information about a signed exchange signature.
         * https://wicg.github.io/webpackage/draft-yasskin-httpbis-origin-signed-exchanges-impl.html#rfc.section.3.1
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SignedExchangeSignature {
            /**
             * Information about a signed exchange signature.
             * https://wicg.github.io/webpackage/draft-yasskin-httpbis-origin-signed-exchanges-impl.html#rfc.section.3.1
            */
            label: Option<String>,
            /**
             * Information about a signed exchange signature.
             * https://wicg.github.io/webpackage/draft-yasskin-httpbis-origin-signed-exchanges-impl.html#rfc.section.3.1
            */
            signature: Option<String>,
            /**
             * Information about a signed exchange signature.
             * https://wicg.github.io/webpackage/draft-yasskin-httpbis-origin-signed-exchanges-impl.html#rfc.section.3.1
            */
            integrity: Option<String>,
            /**
             * Information about a signed exchange signature.
             * https://wicg.github.io/webpackage/draft-yasskin-httpbis-origin-signed-exchanges-impl.html#rfc.section.3.1
            */
            #[serde(rename = "certUrl")]
            certurl: String,
            /**
             * Information about a signed exchange signature.
             * https://wicg.github.io/webpackage/draft-yasskin-httpbis-origin-signed-exchanges-impl.html#rfc.section.3.1
            */
            #[serde(rename = "certSha256")]
            certsha256: String,
            /**
             * Information about a signed exchange signature.
             * https://wicg.github.io/webpackage/draft-yasskin-httpbis-origin-signed-exchanges-impl.html#rfc.section.3.1
            */
            #[serde(rename = "validityUrl")]
            validityurl: Option<String>,
            /**
             * Information about a signed exchange signature.
             * https://wicg.github.io/webpackage/draft-yasskin-httpbis-origin-signed-exchanges-impl.html#rfc.section.3.1
            */
            date: Option<Integer>,
            /**
             * Information about a signed exchange signature.
             * https://wicg.github.io/webpackage/draft-yasskin-httpbis-origin-signed-exchanges-impl.html#rfc.section.3.1
            */
            expires: Option<Integer>,
            /**
             * Information about a signed exchange signature.
             * https://wicg.github.io/webpackage/draft-yasskin-httpbis-origin-signed-exchanges-impl.html#rfc.section.3.1
            */
            certificates: Vec<String>,
        }

        /**
         * Information about a signed exchange header.
         * https://wicg.github.io/webpackage/draft-yasskin-httpbis-origin-signed-exchanges-impl.html#cbor-representation
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SignedExchangeHeader {
            /**
             * Information about a signed exchange header.
             * https://wicg.github.io/webpackage/draft-yasskin-httpbis-origin-signed-exchanges-impl.html#cbor-representation
            */
            #[serde(rename = "requestUrl")]
            request_url: Option<String>,
            /**
             * Information about a signed exchange header.
             * https://wicg.github.io/webpackage/draft-yasskin-httpbis-origin-signed-exchanges-impl.html#cbor-representation
            */
            #[serde(rename = "responseCode")]
            response_code: Option<Integer>,
            /**
             * Information about a signed exchange header.
             * https://wicg.github.io/webpackage/draft-yasskin-httpbis-origin-signed-exchanges-impl.html#cbor-representation
            */
            #[serde(rename = "responseHeaders")]
            response_headers: Option<Headers>,
            /**
             * Information about a signed exchange header.
             * https://wicg.github.io/webpackage/draft-yasskin-httpbis-origin-signed-exchanges-impl.html#cbor-representation
            */
            signatures: Option<Vec<SignedExchangeSignature>>,
            /**
             * Information about a signed exchange header.
             * https://wicg.github.io/webpackage/draft-yasskin-httpbis-origin-signed-exchanges-impl.html#cbor-representation
            */
            #[serde(rename = "headerIntegrity")]
            header_integrity: Option<String>,
        }

        /**
         * Field type for a signed exchange related error.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum SignedExchangeErrorField {
            #[serde(rename = "signatureSig")]
            SignatureSig,
            #[serde(rename = "signatureIntegrity")]
            SignatureIntegrity,
            #[serde(rename = "signatureCertUrl")]
            SignatureCertUrl,
            #[serde(rename = "signatureCertSha256")]
            SignatureCertSha256,
            #[serde(rename = "signatureValidityUrl")]
            SignatureValidityUrl,
            #[serde(rename = "signatureTimestamps")]
            SignatureTimestamps,
        }

        /**
         * Information about a signed exchange response.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SignedExchangeError {
            /**
             * Information about a signed exchange response.
            */
            message: Option<String>,
            /**
             * Information about a signed exchange response.
            */
            #[serde(rename = "signatureIndex")]
            signature_index: Integer,
            /**
             * Information about a signed exchange response.
            */
            #[serde(rename = "errorField")]
            error_field: SignedExchangeErrorField,
        }

        /**
         * Information about a signed exchange response.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SignedExchangeInfo {
            /**
             * Information about a signed exchange response.
            */
            #[serde(rename = "outerResponse")]
            outer_response: Option<Response>,
            /**
             * Information about a signed exchange response.
            */
            header: SignedExchangeHeader,
            /**
             * Information about a signed exchange response.
            */
            #[serde(rename = "securityDetails")]
            security_details: SecurityDetails,
            /**
             * Information about a signed exchange response.
            */
            errors: Vec<SignedExchangeError>,
        }

        /**
         * List of content encodings supported by the backend.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum ContentEncoding {
            #[serde(rename = "deflate")]
            Deflate,
            #[serde(rename = "gzip")]
            Gzip,
            #[serde(rename = "br")]
            Br,
            #[serde(rename = "zstd")]
            Zstd,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum PrivateNetworkRequestPolicy {
            #[serde(rename = "Allow")]
            Allow,
            #[serde(rename = "BlockFromInsecureToMorePrivate")]
            BlockFromInsecureToMorePrivate,
            #[serde(rename = "WarnFromInsecureToMorePrivate")]
            WarnFromInsecureToMorePrivate,
            #[serde(rename = "PreflightBlock")]
            PreflightBlock,
            #[serde(rename = "PreflightWarn")]
            PreflightWarn,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum IPAddressSpace {
            #[serde(rename = "Local")]
            Local,
            #[serde(rename = "Private")]
            Private,
            #[serde(rename = "Public")]
            Public,
            #[serde(rename = "Unknown")]
            Unknown,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ConnectTiming {
            #[serde(rename = "requestTime")]
            request_time: Option<f64>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ClientSecurityState {
            #[serde(rename = "initiatorIsSecureContext")]
            initiator_is_secure_context: Option<bool>,

            #[serde(rename = "initiatorIPAddressSpace")]
            initiator_ipaddress_space: Option<IPAddressSpace>,

            #[serde(rename = "privateNetworkRequestPolicy")]
            private_network_request_policy: Option<PrivateNetworkRequestPolicy>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum CrossOriginOpenerPolicyValue {
            #[serde(rename = "SameOrigin")]
            SameOrigin,
            #[serde(rename = "SameOriginAllowPopups")]
            SameOriginAllowPopups,
            #[serde(rename = "RestrictProperties")]
            RestrictProperties,
            #[serde(rename = "UnsafeNone")]
            UnsafeNone,
            #[serde(rename = "SameOriginPlusCoep")]
            SameOriginPlusCoep,
            #[serde(rename = "RestrictPropertiesPlusCoep")]
            RestrictPropertiesPlusCoep,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CrossOriginOpenerPolicyStatus {
            value: Option<CrossOriginOpenerPolicyValue>,

            #[serde(rename = "reportOnlyValue")]
            report_only_value: Option<CrossOriginOpenerPolicyValue>,

            #[serde(rename = "reportingEndpoint")]
            reporting_endpoint: String,

            #[serde(rename = "reportOnlyReportingEndpoint")]
            report_only_reporting_endpoint: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum CrossOriginEmbedderPolicyValue {
            #[serde(rename = "None")]
            None,
            #[serde(rename = "Credentialless")]
            Credentialless,
            #[serde(rename = "RequireCorp")]
            RequireCorp,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CrossOriginEmbedderPolicyStatus {
            value: Option<CrossOriginEmbedderPolicyValue>,

            #[serde(rename = "reportOnlyValue")]
            report_only_value: Option<CrossOriginEmbedderPolicyValue>,

            #[serde(rename = "reportingEndpoint")]
            reporting_endpoint: String,

            #[serde(rename = "reportOnlyReportingEndpoint")]
            report_only_reporting_endpoint: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum ContentSecurityPolicySource {
            #[serde(rename = "HTTP")]
            HTTP,
            #[serde(rename = "Meta")]
            Meta,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ContentSecurityPolicyStatus {
            #[serde(rename = "effectiveDirectives")]
            effective_directives: Option<String>,

            #[serde(rename = "isEnforced")]
            is_enforced: Option<bool>,

            source: Option<ContentSecurityPolicySource>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SecurityIsolationStatus {
            coop: CrossOriginOpenerPolicyStatus,

            coep: CrossOriginEmbedderPolicyStatus,

            csp: Vec<ContentSecurityPolicyStatus>,
        }

        /**
         * The status of a Reporting API report.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum ReportStatus {
            #[serde(rename = "Queued")]
            Queued,
            #[serde(rename = "Pending")]
            Pending,
            #[serde(rename = "MarkedForRemoval")]
            MarkedForRemoval,
            #[serde(rename = "Success")]
            Success,
        }

        pub type ReportId = String;

        /**
         * An object representing a report generated by the Reporting API.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ReportingApiReport {
            /**
             * An object representing a report generated by the Reporting API.
            */
            id: Option<ReportId>,
            /**
             * An object representing a report generated by the Reporting API.
            */
            #[serde(rename = "initiatorUrl")]
            initiatorurl: Option<String>,
            /**
             * An object representing a report generated by the Reporting API.
            */
            destination: Option<String>,
            /**
             * An object representing a report generated by the Reporting API.
            */
            #[serde(rename = "type")]
            r#type: Option<String>,
            /**
             * An object representing a report generated by the Reporting API.
            */
            timestamp: Option<network::TimeSinceEpoch>,
            /**
             * An object representing a report generated by the Reporting API.
            */
            depth: Option<Integer>,
            /**
             * An object representing a report generated by the Reporting API.
            */
            #[serde(rename = "completedAttempts")]
            completedattempts: Option<Integer>,
            /**
             * An object representing a report generated by the Reporting API.
            */
            body: Option<serde_json::Value>,
            /**
             * An object representing a report generated by the Reporting API.
            */
            status: Option<ReportStatus>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ReportingApiEndpoint {
            url: Option<String>,

            #[serde(rename = "groupName")]
            groupname: Option<String>,
        }

        /**
         * An object providing the result of a network resource load.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct LoadNetworkResourcePageResult {
            /**
             * An object providing the result of a network resource load.
            */
            success: Option<bool>,
            /**
             * An object providing the result of a network resource load.
            */
            #[serde(rename = "netError")]
            neterror: f64,
            /**
             * An object providing the result of a network resource load.
            */
            #[serde(rename = "netErrorName")]
            neterrorname: String,
            /**
             * An object providing the result of a network resource load.
            */
            #[serde(rename = "httpStatusCode")]
            httpstatuscode: f64,
            /**
             * An object providing the result of a network resource load.
            */
            stream: io::StreamHandle,
            /**
             * An object providing the result of a network resource load.
            */
            headers: network::Headers,
        }

        /**
         * An options object that may be extended later to better support CORS,
         * CORB and streaming.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct LoadNetworkResourceOptions {
            /**
             * An options object that may be extended later to better support CORS,
             * CORB and streaming.
            */
            #[serde(rename = "disableCache")]
            disable_cache: Option<bool>,
            /**
             * An options object that may be extended later to better support CORS,
             * CORB and streaming.
            */
            #[serde(rename = "includeCredentials")]
            include_credentials: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetAcceptedEncodingsRequest {
            encodings: Option<Vec<ContentEncoding>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CanClearBrowserCacheResponse {
            result: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CanClearBrowserCookiesResponse {
            result: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CanEmulateNetworkConditionsResponse {
            result: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ContinueInterceptedRequestRequest {
            #[serde(rename = "interceptionId")]
            interception_id: Option<InterceptionId>,

            #[serde(rename = "errorReason")]
            error_reason: ErrorReason,

            #[serde(rename = "rawResponse")]
            raw_response: String,

            url: String,

            method: String,

            #[serde(rename = "postData")]
            post_data: String,

            headers: Headers,

            #[serde(rename = "authChallengeResponse")]
            auth_challenge_response: AuthChallengeResponse,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct DeleteCookiesRequest {
            name: Option<String>,

            url: String,

            domain: String,

            path: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct EmulateNetworkConditionsRequest {
            offline: Option<bool>,

            latency: Option<f64>,

            #[serde(rename = "downloadThroughput")]
            download_throughput: Option<f64>,

            #[serde(rename = "uploadThroughput")]
            upload_throughput: Option<f64>,

            #[serde(rename = "connectionType")]
            connection_type: ConnectionType,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct EnableRequest {
            #[serde(rename = "maxTotalBufferSize")]
            max_total_buffer_size: Integer,

            #[serde(rename = "maxResourceBufferSize")]
            max_resource_buffer_size: Integer,

            #[serde(rename = "maxPostDataSize")]
            max_post_data_size: Integer,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetAllCookiesResponse {
            cookies: Option<Vec<Cookie>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetCertificateRequest {
            origin: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetCertificateResponse {
            #[serde(rename = "tableNames")]
            table_names: Option<Vec<String>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetCookiesRequest {
            urls: Vec<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetCookiesResponse {
            cookies: Option<Vec<Cookie>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetResponseBodyRequest {
            #[serde(rename = "requestId")]
            request_id: Option<RequestId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetResponseBodyResponse {
            body: Option<String>,

            #[serde(rename = "base64Encoded")]
            base64encoded: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetRequestPostDataRequest {
            #[serde(rename = "requestId")]
            request_id: Option<RequestId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetRequestPostDataResponse {
            #[serde(rename = "postData")]
            post_data: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetResponseBodyForInterceptionRequest {
            #[serde(rename = "interceptionId")]
            interception_id: Option<InterceptionId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetResponseBodyForInterceptionResponse {
            body: Option<String>,

            #[serde(rename = "base64Encoded")]
            base64encoded: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct TakeResponseBodyForInterceptionAsStreamRequest {
            #[serde(rename = "interceptionId")]
            interception_id: Option<InterceptionId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct TakeResponseBodyForInterceptionAsStreamResponse {
            stream: Option<io::StreamHandle>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ReplayXHRRequest {
            #[serde(rename = "requestId")]
            request_id: Option<RequestId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SearchInResponseBodyRequest {
            #[serde(rename = "requestId")]
            request_id: Option<RequestId>,

            query: Option<String>,

            #[serde(rename = "caseSensitive")]
            case_sensitive: bool,

            #[serde(rename = "isRegex")]
            is_regex: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SearchInResponseBodyResponse {
            result: Option<Vec<debugger::SearchMatch>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetBlockedURLsRequest {
            urls: Option<Vec<String>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetBypassServiceWorkerRequest {
            bypass: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetCacheDisabledRequest {
            #[serde(rename = "cacheDisabled")]
            cache_disabled: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetCookieRequest {
            name: Option<String>,

            value: Option<String>,

            url: String,

            domain: String,

            path: String,

            secure: bool,

            #[serde(rename = "httpOnly")]
            httponly: bool,

            #[serde(rename = "sameSite")]
            samesite: CookieSameSite,

            expires: TimeSinceEpoch,

            priority: CookiePriority,

            #[serde(rename = "sameParty")]
            sameparty: bool,

            #[serde(rename = "sourceScheme")]
            sourcescheme: CookieSourceScheme,

            #[serde(rename = "sourcePort")]
            sourceport: Integer,

            #[serde(rename = "partitionKey")]
            partitionkey: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetCookieResponse {
            success: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetCookiesRequest {
            cookies: Option<Vec<CookieParam>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetExtraHTTPHeadersRequest {
            headers: Option<Headers>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetAttachDebugStackRequest {
            enabled: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetRequestInterceptionRequest {
            patterns: Option<Vec<RequestPattern>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetUserAgentOverrideRequest {
            #[serde(rename = "userAgent")]
            user_agent: Option<String>,

            #[serde(rename = "acceptLanguage")]
            accept_language: String,

            platform: String,

            #[serde(rename = "userAgentMetadata")]
            user_agent_metadata: emulation::UserAgentMetadata,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetSecurityIsolationStatusRequest {
            #[serde(rename = "frameId")]
            frame_id: page::FrameId,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetSecurityIsolationStatusResponse {
            status: Option<SecurityIsolationStatus>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct EnableReportingApiRequest {
            enable: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct LoadNetworkResourceRequest {
            #[serde(rename = "frameId")]
            frameid: page::FrameId,

            url: Option<String>,

            options: Option<LoadNetworkResourceOptions>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct LoadNetworkResourceResponse {
            resource: Option<LoadNetworkResourcePageResult>,
        }

        /**
         * Fired when data chunk was received over the network.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct DataReceivedEvent {
            /**
             * Fired when data chunk was received over the network.
            */
            #[serde(rename = "requestId")]
            request_id: Option<RequestId>,
            /**
             * Fired when data chunk was received over the network.
            */
            timestamp: Option<MonotonicTime>,
            /**
             * Fired when data chunk was received over the network.
            */
            #[serde(rename = "dataLength")]
            data_length: Option<Integer>,
            /**
             * Fired when data chunk was received over the network.
            */
            #[serde(rename = "encodedDataLength")]
            encoded_data_length: Option<Integer>,
        }

        /**
         * Fired when EventSource message is received.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct EventSourceMessageReceivedEvent {
            /**
             * Fired when EventSource message is received.
            */
            #[serde(rename = "requestId")]
            request_id: Option<RequestId>,
            /**
             * Fired when EventSource message is received.
            */
            timestamp: Option<MonotonicTime>,
            /**
             * Fired when EventSource message is received.
            */
            #[serde(rename = "eventName")]
            event_name: Option<String>,
            /**
             * Fired when EventSource message is received.
            */
            #[serde(rename = "eventId")]
            event_id: Option<String>,
            /**
             * Fired when EventSource message is received.
            */
            data: Option<String>,
        }

        /**
         * Fired when HTTP request has failed to load.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct LoadingFailedEvent {
            /**
             * Fired when HTTP request has failed to load.
            */
            #[serde(rename = "requestId")]
            request_id: Option<RequestId>,
            /**
             * Fired when HTTP request has failed to load.
            */
            timestamp: Option<MonotonicTime>,
            /**
             * Fired when HTTP request has failed to load.
            */
            #[serde(rename = "type")]
            r#type: Option<ResourceType>,
            /**
             * Fired when HTTP request has failed to load.
            */
            #[serde(rename = "errorText")]
            error_text: Option<String>,
            /**
             * Fired when HTTP request has failed to load.
            */
            canceled: bool,
            /**
             * Fired when HTTP request has failed to load.
            */
            #[serde(rename = "blockedReason")]
            blocked_reason: BlockedReason,
            /**
             * Fired when HTTP request has failed to load.
            */
            #[serde(rename = "corsErrorStatus")]
            cors_error_status: CorsErrorStatus,
        }

        /**
         * Fired when HTTP request has finished loading.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct LoadingFinishedEvent {
            /**
             * Fired when HTTP request has finished loading.
            */
            #[serde(rename = "requestId")]
            request_id: Option<RequestId>,
            /**
             * Fired when HTTP request has finished loading.
            */
            timestamp: Option<MonotonicTime>,
            /**
             * Fired when HTTP request has finished loading.
            */
            #[serde(rename = "encodedDataLength")]
            encoded_data_length: Option<f64>,
        }

        /**
         * Details of an intercepted HTTP request, which must be either allowed, blocked, modified or
         * mocked.
         * Deprecated, use Fetch.requestPaused instead.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct RequestInterceptedEvent {
            /**
             * Details of an intercepted HTTP request, which must be either allowed, blocked, modified or
             * mocked.
             * Deprecated, use Fetch.requestPaused instead.
            */
            #[serde(rename = "interceptionId")]
            interception_id: Option<InterceptionId>,
            /**
             * Details of an intercepted HTTP request, which must be either allowed, blocked, modified or
             * mocked.
             * Deprecated, use Fetch.requestPaused instead.
            */
            request: Option<Request>,
            /**
             * Details of an intercepted HTTP request, which must be either allowed, blocked, modified or
             * mocked.
             * Deprecated, use Fetch.requestPaused instead.
            */
            #[serde(rename = "frameId")]
            frame_id: Option<page::FrameId>,
            /**
             * Details of an intercepted HTTP request, which must be either allowed, blocked, modified or
             * mocked.
             * Deprecated, use Fetch.requestPaused instead.
            */
            #[serde(rename = "resourceType")]
            resource_type: Option<ResourceType>,
            /**
             * Details of an intercepted HTTP request, which must be either allowed, blocked, modified or
             * mocked.
             * Deprecated, use Fetch.requestPaused instead.
            */
            #[serde(rename = "isNavigationRequest")]
            is_navigation_request: Option<bool>,
            /**
             * Details of an intercepted HTTP request, which must be either allowed, blocked, modified or
             * mocked.
             * Deprecated, use Fetch.requestPaused instead.
            */
            #[serde(rename = "isDownload")]
            is_download: bool,
            /**
             * Details of an intercepted HTTP request, which must be either allowed, blocked, modified or
             * mocked.
             * Deprecated, use Fetch.requestPaused instead.
            */
            #[serde(rename = "redirectUrl")]
            redirect_url: String,
            /**
             * Details of an intercepted HTTP request, which must be either allowed, blocked, modified or
             * mocked.
             * Deprecated, use Fetch.requestPaused instead.
            */
            #[serde(rename = "authChallenge")]
            auth_challenge: AuthChallenge,
            /**
             * Details of an intercepted HTTP request, which must be either allowed, blocked, modified or
             * mocked.
             * Deprecated, use Fetch.requestPaused instead.
            */
            #[serde(rename = "responseErrorReason")]
            response_error_reason: ErrorReason,
            /**
             * Details of an intercepted HTTP request, which must be either allowed, blocked, modified or
             * mocked.
             * Deprecated, use Fetch.requestPaused instead.
            */
            #[serde(rename = "responseStatusCode")]
            response_status_code: Integer,
            /**
             * Details of an intercepted HTTP request, which must be either allowed, blocked, modified or
             * mocked.
             * Deprecated, use Fetch.requestPaused instead.
            */
            #[serde(rename = "responseHeaders")]
            response_headers: Headers,
            /**
             * Details of an intercepted HTTP request, which must be either allowed, blocked, modified or
             * mocked.
             * Deprecated, use Fetch.requestPaused instead.
            */
            #[serde(rename = "requestId")]
            request_id: RequestId,
        }

        /**
         * Fired if request ended up loading from cache.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct RequestServedFromCacheEvent {
            /**
             * Fired if request ended up loading from cache.
            */
            #[serde(rename = "requestId")]
            request_id: Option<RequestId>,
        }

        /**
         * Fired when page is about to send HTTP request.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct RequestWillBeSentEvent {
            /**
             * Fired when page is about to send HTTP request.
            */
            #[serde(rename = "requestId")]
            request_id: Option<RequestId>,
            /**
             * Fired when page is about to send HTTP request.
            */
            #[serde(rename = "loaderId")]
            loader_id: Option<LoaderId>,
            /**
             * Fired when page is about to send HTTP request.
            */
            #[serde(rename = "documentURL")]
            document_url: Option<String>,
            /**
             * Fired when page is about to send HTTP request.
            */
            request: Option<Request>,
            /**
             * Fired when page is about to send HTTP request.
            */
            timestamp: Option<MonotonicTime>,
            /**
             * Fired when page is about to send HTTP request.
            */
            #[serde(rename = "wallTime")]
            wall_time: Option<TimeSinceEpoch>,
            /**
             * Fired when page is about to send HTTP request.
            */
            initiator: Option<Initiator>,
            /**
             * Fired when page is about to send HTTP request.
            */
            #[serde(rename = "redirectHasExtraInfo")]
            redirect_has_extra_info: Option<bool>,
            /**
             * Fired when page is about to send HTTP request.
            */
            #[serde(rename = "redirectResponse")]
            redirect_response: Response,
            /**
             * Fired when page is about to send HTTP request.
            */
            #[serde(rename = "type")]
            r#type: ResourceType,
            /**
             * Fired when page is about to send HTTP request.
            */
            #[serde(rename = "frameId")]
            frame_id: page::FrameId,
            /**
             * Fired when page is about to send HTTP request.
            */
            #[serde(rename = "hasUserGesture")]
            has_user_gesture: bool,
        }

        /**
         * Fired when resource loading priority is changed
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ResourceChangedPriorityEvent {
            /**
             * Fired when resource loading priority is changed
            */
            #[serde(rename = "requestId")]
            request_id: Option<RequestId>,
            /**
             * Fired when resource loading priority is changed
            */
            #[serde(rename = "newPriority")]
            new_priority: Option<ResourcePriority>,
            /**
             * Fired when resource loading priority is changed
            */
            timestamp: Option<MonotonicTime>,
        }

        /**
         * Fired when a signed exchange was received over the network
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SignedExchangeReceivedEvent {
            /**
             * Fired when a signed exchange was received over the network
            */
            #[serde(rename = "requestId")]
            request_id: Option<RequestId>,
            /**
             * Fired when a signed exchange was received over the network
            */
            info: Option<SignedExchangeInfo>,
        }

        /**
         * Fired when HTTP response is available.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ResponseReceivedEvent {
            /**
             * Fired when HTTP response is available.
            */
            #[serde(rename = "requestId")]
            request_id: Option<RequestId>,
            /**
             * Fired when HTTP response is available.
            */
            #[serde(rename = "loaderId")]
            loader_id: Option<LoaderId>,
            /**
             * Fired when HTTP response is available.
            */
            timestamp: Option<MonotonicTime>,
            /**
             * Fired when HTTP response is available.
            */
            #[serde(rename = "type")]
            r#type: Option<ResourceType>,
            /**
             * Fired when HTTP response is available.
            */
            response: Option<Response>,
            /**
             * Fired when HTTP response is available.
            */
            #[serde(rename = "hasExtraInfo")]
            has_extra_info: Option<bool>,
            /**
             * Fired when HTTP response is available.
            */
            #[serde(rename = "frameId")]
            frame_id: page::FrameId,
        }

        /**
         * Fired when WebSocket is closed.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct WebSocketClosedEvent {
            /**
             * Fired when WebSocket is closed.
            */
            #[serde(rename = "requestId")]
            request_id: Option<RequestId>,
            /**
             * Fired when WebSocket is closed.
            */
            timestamp: Option<MonotonicTime>,
        }

        /**
         * Fired upon WebSocket creation.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct WebSocketCreatedEvent {
            /**
             * Fired upon WebSocket creation.
            */
            #[serde(rename = "requestId")]
            requestid: Option<RequestId>,
            /**
             * Fired upon WebSocket creation.
            */
            url: Option<String>,
            /**
             * Fired upon WebSocket creation.
            */
            initiator: Initiator,
        }

        /**
         * Fired when WebSocket message error occurs.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct WebSocketFrameErrorEvent {
            /**
             * Fired when WebSocket message error occurs.
            */
            #[serde(rename = "requestId")]
            request_id: Option<RequestId>,
            /**
             * Fired when WebSocket message error occurs.
            */
            timestamp: Option<MonotonicTime>,
            /**
             * Fired when WebSocket message error occurs.
            */
            #[serde(rename = "errorMessage")]
            error_message: Option<String>,
        }

        /**
         * Fired when WebSocket message is received.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct WebSocketFrameReceivedEvent {
            /**
             * Fired when WebSocket message is received.
            */
            #[serde(rename = "requestId")]
            requestid: Option<RequestId>,
            /**
             * Fired when WebSocket message is received.
            */
            timestamp: Option<MonotonicTime>,
            /**
             * Fired when WebSocket message is received.
            */
            response: Option<WebSocketFrame>,
        }

        /**
         * Fired when WebSocket message is sent.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct WebSocketFrameSentEvent {
            /**
             * Fired when WebSocket message is sent.
            */
            #[serde(rename = "requestId")]
            requestid: Option<RequestId>,
            /**
             * Fired when WebSocket message is sent.
            */
            timestamp: Option<MonotonicTime>,
            /**
             * Fired when WebSocket message is sent.
            */
            response: Option<WebSocketFrame>,
        }

        /**
         * Fired when WebSocket handshake response becomes available.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct WebSocketHandshakeResponseReceivedEvent {
            /**
             * Fired when WebSocket handshake response becomes available.
            */
            #[serde(rename = "requestId")]
            requestid: Option<RequestId>,
            /**
             * Fired when WebSocket handshake response becomes available.
            */
            timestamp: Option<MonotonicTime>,
            /**
             * Fired when WebSocket handshake response becomes available.
            */
            response: Option<WebSocketResponse>,
        }

        /**
         * Fired when WebSocket is about to initiate handshake.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct WebSocketWillSendHandshakeRequestEvent {
            /**
             * Fired when WebSocket is about to initiate handshake.
            */
            #[serde(rename = "requestId")]
            request_id: Option<RequestId>,
            /**
             * Fired when WebSocket is about to initiate handshake.
            */
            timestamp: Option<MonotonicTime>,
            /**
             * Fired when WebSocket is about to initiate handshake.
            */
            #[serde(rename = "wallTime")]
            wall_time: Option<TimeSinceEpoch>,
            /**
             * Fired when WebSocket is about to initiate handshake.
            */
            request: Option<WebSocketRequest>,
        }

        /**
         * Fired upon WebTransport creation.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct WebTransportCreatedEvent {
            /**
             * Fired upon WebTransport creation.
            */
            #[serde(rename = "transportId")]
            transportid: Option<RequestId>,
            /**
             * Fired upon WebTransport creation.
            */
            url: Option<String>,
            /**
             * Fired upon WebTransport creation.
            */
            timestamp: Option<MonotonicTime>,
            /**
             * Fired upon WebTransport creation.
            */
            initiator: Initiator,
        }

        /**
         * Fired when WebTransport handshake is finished.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct WebTransportConnectionEstablishedEvent {
            /**
             * Fired when WebTransport handshake is finished.
            */
            #[serde(rename = "transportId")]
            transport_id: Option<RequestId>,
            /**
             * Fired when WebTransport handshake is finished.
            */
            timestamp: Option<MonotonicTime>,
        }

        /**
         * Fired when WebTransport is disposed.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct WebTransportClosedEvent {
            /**
             * Fired when WebTransport is disposed.
            */
            #[serde(rename = "transportId")]
            transport_id: Option<RequestId>,
            /**
             * Fired when WebTransport is disposed.
            */
            timestamp: Option<MonotonicTime>,
        }

        /**
         * Fired when additional information about a requestWillBeSent event is available from the
         * network stack. Not every requestWillBeSent event will have an additional
         * requestWillBeSentExtraInfo fired for it, and there is no guarantee whether requestWillBeSent
         * or requestWillBeSentExtraInfo will be fired first for the same request.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct RequestWillBeSentExtraInfoEvent {
            /**
             * Fired when additional information about a requestWillBeSent event is available from the
             * network stack. Not every requestWillBeSent event will have an additional
             * requestWillBeSentExtraInfo fired for it, and there is no guarantee whether requestWillBeSent
             * or requestWillBeSentExtraInfo will be fired first for the same request.
            */
            #[serde(rename = "requestId")]
            request_id: Option<RequestId>,
            /**
             * Fired when additional information about a requestWillBeSent event is available from the
             * network stack. Not every requestWillBeSent event will have an additional
             * requestWillBeSentExtraInfo fired for it, and there is no guarantee whether requestWillBeSent
             * or requestWillBeSentExtraInfo will be fired first for the same request.
            */
            #[serde(rename = "associatedCookies")]
            associated_cookies: Option<Vec<BlockedCookieWithReason>>,
            /**
             * Fired when additional information about a requestWillBeSent event is available from the
             * network stack. Not every requestWillBeSent event will have an additional
             * requestWillBeSentExtraInfo fired for it, and there is no guarantee whether requestWillBeSent
             * or requestWillBeSentExtraInfo will be fired first for the same request.
            */
            headers: Option<Headers>,
            /**
             * Fired when additional information about a requestWillBeSent event is available from the
             * network stack. Not every requestWillBeSent event will have an additional
             * requestWillBeSentExtraInfo fired for it, and there is no guarantee whether requestWillBeSent
             * or requestWillBeSentExtraInfo will be fired first for the same request.
            */
            #[serde(rename = "connectTiming")]
            connect_timing: Option<ConnectTiming>,
            /**
             * Fired when additional information about a requestWillBeSent event is available from the
             * network stack. Not every requestWillBeSent event will have an additional
             * requestWillBeSentExtraInfo fired for it, and there is no guarantee whether requestWillBeSent
             * or requestWillBeSentExtraInfo will be fired first for the same request.
            */
            #[serde(rename = "clientSecurityState")]
            client_security_state: ClientSecurityState,
            /**
             * Fired when additional information about a requestWillBeSent event is available from the
             * network stack. Not every requestWillBeSent event will have an additional
             * requestWillBeSentExtraInfo fired for it, and there is no guarantee whether requestWillBeSent
             * or requestWillBeSentExtraInfo will be fired first for the same request.
            */
            #[serde(rename = "siteHasCookieInOtherPartition")]
            site_has_cookie_in_other_partition: bool,
        }

        /**
         * Fired when additional information about a responseReceived event is available from the network
         * stack. Not every responseReceived event will have an additional responseReceivedExtraInfo for
         * it, and responseReceivedExtraInfo may be fired before or after responseReceived.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ResponseReceivedExtraInfoEvent {
            /**
             * Fired when additional information about a responseReceived event is available from the network
             * stack. Not every responseReceived event will have an additional responseReceivedExtraInfo for
             * it, and responseReceivedExtraInfo may be fired before or after responseReceived.
            */
            #[serde(rename = "requestId")]
            request_id: Option<RequestId>,
            /**
             * Fired when additional information about a responseReceived event is available from the network
             * stack. Not every responseReceived event will have an additional responseReceivedExtraInfo for
             * it, and responseReceivedExtraInfo may be fired before or after responseReceived.
            */
            #[serde(rename = "blockedCookies")]
            blocked_cookies: Option<Vec<BlockedSetCookieWithReason>>,
            /**
             * Fired when additional information about a responseReceived event is available from the network
             * stack. Not every responseReceived event will have an additional responseReceivedExtraInfo for
             * it, and responseReceivedExtraInfo may be fired before or after responseReceived.
            */
            headers: Option<Headers>,
            /**
             * Fired when additional information about a responseReceived event is available from the network
             * stack. Not every responseReceived event will have an additional responseReceivedExtraInfo for
             * it, and responseReceivedExtraInfo may be fired before or after responseReceived.
            */
            #[serde(rename = "resourceIPAddressSpace")]
            resource_ipaddress_space: Option<IPAddressSpace>,
            /**
             * Fired when additional information about a responseReceived event is available from the network
             * stack. Not every responseReceived event will have an additional responseReceivedExtraInfo for
             * it, and responseReceivedExtraInfo may be fired before or after responseReceived.
            */
            #[serde(rename = "statusCode")]
            status_code: Option<Integer>,
            /**
             * Fired when additional information about a responseReceived event is available from the network
             * stack. Not every responseReceived event will have an additional responseReceivedExtraInfo for
             * it, and responseReceivedExtraInfo may be fired before or after responseReceived.
            */
            #[serde(rename = "headersText")]
            headers_text: String,
            /**
             * Fired when additional information about a responseReceived event is available from the network
             * stack. Not every responseReceived event will have an additional responseReceivedExtraInfo for
             * it, and responseReceivedExtraInfo may be fired before or after responseReceived.
            */
            #[serde(rename = "cookiePartitionKey")]
            cookie_partition_key: String,
            /**
             * Fired when additional information about a responseReceived event is available from the network
             * stack. Not every responseReceived event will have an additional responseReceivedExtraInfo for
             * it, and responseReceivedExtraInfo may be fired before or after responseReceived.
            */
            #[serde(rename = "cookiePartitionKeyOpaque")]
            cookie_partition_key_opaque: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum TrustTokenOperationDoneEventStatus {
            #[serde(rename = "Ok")]
            Ok,

            #[serde(rename = "InvalidArgument")]
            InvalidArgument,

            #[serde(rename = "MissingIssuerKeys")]
            MissingIssuerKeys,

            #[serde(rename = "FailedPrecondition")]
            FailedPrecondition,

            #[serde(rename = "ResourceExhausted")]
            ResourceExhausted,

            #[serde(rename = "AlreadyExists")]
            AlreadyExists,

            #[serde(rename = "Unavailable")]
            Unavailable,

            #[serde(rename = "Unauthorized")]
            Unauthorized,

            #[serde(rename = "BadResponse")]
            BadResponse,

            #[serde(rename = "InternalError")]
            InternalError,

            #[serde(rename = "UnknownError")]
            UnknownError,

            #[serde(rename = "FulfilledLocally")]
            FulfilledLocally,
        }

        /**
         * Fired exactly once for each Trust Token operation. Depending on
         * the type of the operation and whether the operation succeeded or
         * failed, the event is fired before the corresponding request was sent
         * or after the response was received.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct TrustTokenOperationDoneEvent {
            /**
             * Fired exactly once for each Trust Token operation. Depending on
             * the type of the operation and whether the operation succeeded or
             * failed, the event is fired before the corresponding request was sent
             * or after the response was received.
            */
            status: Option<TrustTokenOperationDoneEventStatus>,
            /**
             * Fired exactly once for each Trust Token operation. Depending on
             * the type of the operation and whether the operation succeeded or
             * failed, the event is fired before the corresponding request was sent
             * or after the response was received.
            */
            #[serde(rename = "type")]
            r#type: Option<TrustTokenOperationType>,
            /**
             * Fired exactly once for each Trust Token operation. Depending on
             * the type of the operation and whether the operation succeeded or
             * failed, the event is fired before the corresponding request was sent
             * or after the response was received.
            */
            #[serde(rename = "requestId")]
            request_id: Option<RequestId>,
            /**
             * Fired exactly once for each Trust Token operation. Depending on
             * the type of the operation and whether the operation succeeded or
             * failed, the event is fired before the corresponding request was sent
             * or after the response was received.
            */
            #[serde(rename = "topLevelOrigin")]
            top_level_origin: String,
            /**
             * Fired exactly once for each Trust Token operation. Depending on
             * the type of the operation and whether the operation succeeded or
             * failed, the event is fired before the corresponding request was sent
             * or after the response was received.
            */
            #[serde(rename = "issuerOrigin")]
            issuer_origin: String,
            /**
             * Fired exactly once for each Trust Token operation. Depending on
             * the type of the operation and whether the operation succeeded or
             * failed, the event is fired before the corresponding request was sent
             * or after the response was received.
            */
            #[serde(rename = "issuedTokenCount")]
            issued_token_count: Integer,
        }

        /**
         * Fired once when parsing the .wbn file has succeeded.
         * The event contains the information about the web bundle contents.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SubresourceWebBundleMetadataReceivedEvent {
            /**
             * Fired once when parsing the .wbn file has succeeded.
             * The event contains the information about the web bundle contents.
            */
            #[serde(rename = "requestId")]
            request_id: Option<RequestId>,
            /**
             * Fired once when parsing the .wbn file has succeeded.
             * The event contains the information about the web bundle contents.
            */
            urls: Option<Vec<String>>,
        }

        /**
         * Fired once when parsing the .wbn file has failed.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SubresourceWebBundleMetadataErrorEvent {
            /**
             * Fired once when parsing the .wbn file has failed.
            */
            #[serde(rename = "requestId")]
            request_id: Option<RequestId>,
            /**
             * Fired once when parsing the .wbn file has failed.
            */
            #[serde(rename = "errorMessage")]
            error_message: Option<String>,
        }

        /**
         * Fired when handling requests for resources within a .wbn file.
         * Note: this will only be fired for resources that are requested by the webpage.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SubresourceWebBundleInnerResponseParsedEvent {
            /**
             * Fired when handling requests for resources within a .wbn file.
             * Note: this will only be fired for resources that are requested by the webpage.
            */
            #[serde(rename = "innerRequestId")]
            inner_request_id: Option<RequestId>,
            /**
             * Fired when handling requests for resources within a .wbn file.
             * Note: this will only be fired for resources that are requested by the webpage.
            */
            #[serde(rename = "innerRequestURL")]
            inner_request_url: Option<String>,
            /**
             * Fired when handling requests for resources within a .wbn file.
             * Note: this will only be fired for resources that are requested by the webpage.
            */
            #[serde(rename = "bundleRequestId")]
            bundle_request_id: RequestId,
        }

        /**
         * Fired when request for resources within a .wbn file failed.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SubresourceWebBundleInnerResponseErrorEvent {
            /**
             * Fired when request for resources within a .wbn file failed.
            */
            #[serde(rename = "innerRequestId")]
            inner_request_id: Option<RequestId>,
            /**
             * Fired when request for resources within a .wbn file failed.
            */
            #[serde(rename = "innerRequestURL")]
            inner_request_url: Option<String>,
            /**
             * Fired when request for resources within a .wbn file failed.
            */
            #[serde(rename = "errorMessage")]
            error_message: Option<String>,
            /**
             * Fired when request for resources within a .wbn file failed.
            */
            #[serde(rename = "bundleRequestId")]
            bundle_request_id: RequestId,
        }

        /**
         * Is sent whenever a new report is added.
         * And after 'enableReportingApi' for all existing reports.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ReportingApiReportAddedEvent {
            /**
             * Is sent whenever a new report is added.
             * And after 'enableReportingApi' for all existing reports.
            */
            report: Option<ReportingApiReport>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ReportingApiReportUpdatedEvent {
            report: Option<ReportingApiReport>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ReportingApiEndpointsChangedForOriginEvent {
            origin: Option<String>,

            endpoints: Option<Vec<ReportingApiEndpoint>>,
        }
    }

    /**
     * This domain provides various functionality related to drawing atop the inspected page.
    */
    pub mod overlay {
        use super::{dom, page, runtime, Integer};
        use serde::{self, Deserialize, Serialize};
        /**
         * Configuration data for drawing the source order of an elements children.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SourceOrderConfig {
            /**
             * Configuration data for drawing the source order of an elements children.
            */
            #[serde(rename = "parentOutlineColor")]
            parent_outline_color: Option<dom::RGBA>,
            /**
             * Configuration data for drawing the source order of an elements children.
            */
            #[serde(rename = "childOutlineColor")]
            child_outline_color: Option<dom::RGBA>,
        }

        /**
         * Configuration data for the highlighting of Grid elements.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct GridHighlightConfig {
            /**
             * Configuration data for the highlighting of Grid elements.
            */
            #[serde(rename = "showGridExtensionLines")]
            show_grid_extension_lines: bool,
            /**
             * Configuration data for the highlighting of Grid elements.
            */
            #[serde(rename = "showPositiveLineNumbers")]
            show_positive_line_numbers: bool,
            /**
             * Configuration data for the highlighting of Grid elements.
            */
            #[serde(rename = "showNegativeLineNumbers")]
            show_negative_line_numbers: bool,
            /**
             * Configuration data for the highlighting of Grid elements.
            */
            #[serde(rename = "showAreaNames")]
            show_area_names: bool,
            /**
             * Configuration data for the highlighting of Grid elements.
            */
            #[serde(rename = "showLineNames")]
            show_line_names: bool,
            /**
             * Configuration data for the highlighting of Grid elements.
            */
            #[serde(rename = "showTrackSizes")]
            show_track_sizes: bool,
            /**
             * Configuration data for the highlighting of Grid elements.
            */
            #[serde(rename = "gridBorderColor")]
            grid_border_color: dom::RGBA,
            /**
             * Configuration data for the highlighting of Grid elements.
            */
            #[serde(rename = "cellBorderColor")]
            cell_border_color: dom::RGBA,
            /**
             * Configuration data for the highlighting of Grid elements.
            */
            #[serde(rename = "rowLineColor")]
            row_line_color: dom::RGBA,
            /**
             * Configuration data for the highlighting of Grid elements.
            */
            #[serde(rename = "columnLineColor")]
            column_line_color: dom::RGBA,
            /**
             * Configuration data for the highlighting of Grid elements.
            */
            #[serde(rename = "gridBorderDash")]
            grid_border_dash: bool,
            /**
             * Configuration data for the highlighting of Grid elements.
            */
            #[serde(rename = "cellBorderDash")]
            cell_border_dash: bool,
            /**
             * Configuration data for the highlighting of Grid elements.
            */
            #[serde(rename = "rowLineDash")]
            row_line_dash: bool,
            /**
             * Configuration data for the highlighting of Grid elements.
            */
            #[serde(rename = "columnLineDash")]
            column_line_dash: bool,
            /**
             * Configuration data for the highlighting of Grid elements.
            */
            #[serde(rename = "rowGapColor")]
            row_gap_color: dom::RGBA,
            /**
             * Configuration data for the highlighting of Grid elements.
            */
            #[serde(rename = "rowHatchColor")]
            row_hatch_color: dom::RGBA,
            /**
             * Configuration data for the highlighting of Grid elements.
            */
            #[serde(rename = "columnGapColor")]
            column_gap_color: dom::RGBA,
            /**
             * Configuration data for the highlighting of Grid elements.
            */
            #[serde(rename = "columnHatchColor")]
            column_hatch_color: dom::RGBA,
            /**
             * Configuration data for the highlighting of Grid elements.
            */
            #[serde(rename = "areaBorderColor")]
            area_border_color: dom::RGBA,
            /**
             * Configuration data for the highlighting of Grid elements.
            */
            #[serde(rename = "gridBackgroundColor")]
            grid_background_color: dom::RGBA,
        }

        /**
         * Configuration data for the highlighting of Flex container elements.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FlexContainerHighlightConfig {
            /**
             * Configuration data for the highlighting of Flex container elements.
            */
            #[serde(rename = "containerBorder")]
            container_border: LineStyle,
            /**
             * Configuration data for the highlighting of Flex container elements.
            */
            #[serde(rename = "lineSeparator")]
            line_separator: LineStyle,
            /**
             * Configuration data for the highlighting of Flex container elements.
            */
            #[serde(rename = "itemSeparator")]
            item_separator: LineStyle,
            /**
             * Configuration data for the highlighting of Flex container elements.
            */
            #[serde(rename = "mainDistributedSpace")]
            main_distributed_space: BoxStyle,
            /**
             * Configuration data for the highlighting of Flex container elements.
            */
            #[serde(rename = "crossDistributedSpace")]
            cross_distributed_space: BoxStyle,
            /**
             * Configuration data for the highlighting of Flex container elements.
            */
            #[serde(rename = "rowGapSpace")]
            row_gap_space: BoxStyle,
            /**
             * Configuration data for the highlighting of Flex container elements.
            */
            #[serde(rename = "columnGapSpace")]
            column_gap_space: BoxStyle,
            /**
             * Configuration data for the highlighting of Flex container elements.
            */
            #[serde(rename = "crossAlignment")]
            cross_alignment: LineStyle,
        }

        /**
         * Configuration data for the highlighting of Flex item elements.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FlexItemHighlightConfig {
            /**
             * Configuration data for the highlighting of Flex item elements.
            */
            #[serde(rename = "baseSizeBox")]
            base_size_box: BoxStyle,
            /**
             * Configuration data for the highlighting of Flex item elements.
            */
            #[serde(rename = "baseSizeBorder")]
            base_size_border: LineStyle,
            /**
             * Configuration data for the highlighting of Flex item elements.
            */
            #[serde(rename = "flexibilityArrow")]
            flexibility_arrow: LineStyle,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum LineStylePattern {
            #[serde(rename = "dashed")]
            Dashed,

            #[serde(rename = "dotted")]
            Dotted,
        }

        /**
         * Style information for drawing a line.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct LineStyle {
            /**
             * Style information for drawing a line.
            */
            color: dom::RGBA,
            /**
             * Style information for drawing a line.
            */
            pattern: LineStylePattern,
        }

        /**
         * Style information for drawing a box.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct BoxStyle {
            /**
             * Style information for drawing a box.
            */
            #[serde(rename = "fillColor")]
            fill_color: dom::RGBA,
            /**
             * Style information for drawing a box.
            */
            #[serde(rename = "hatchColor")]
            hatch_color: dom::RGBA,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum ContrastAlgorithm {
            #[serde(rename = "aa")]
            Aa,
            #[serde(rename = "aaa")]
            Aaa,
            #[serde(rename = "apca")]
            Apca,
        }

        /**
         * Configuration data for the highlighting of page elements.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct HighlightConfig {
            /**
             * Configuration data for the highlighting of page elements.
            */
            #[serde(rename = "showInfo")]
            show_info: bool,
            /**
             * Configuration data for the highlighting of page elements.
            */
            #[serde(rename = "showStyles")]
            show_styles: bool,
            /**
             * Configuration data for the highlighting of page elements.
            */
            #[serde(rename = "showRulers")]
            show_rulers: bool,
            /**
             * Configuration data for the highlighting of page elements.
            */
            #[serde(rename = "showAccessibilityInfo")]
            show_accessibility_info: bool,
            /**
             * Configuration data for the highlighting of page elements.
            */
            #[serde(rename = "showExtensionLines")]
            show_extension_lines: bool,
            /**
             * Configuration data for the highlighting of page elements.
            */
            #[serde(rename = "contentColor")]
            content_color: dom::RGBA,
            /**
             * Configuration data for the highlighting of page elements.
            */
            #[serde(rename = "paddingColor")]
            padding_color: dom::RGBA,
            /**
             * Configuration data for the highlighting of page elements.
            */
            #[serde(rename = "borderColor")]
            border_color: dom::RGBA,
            /**
             * Configuration data for the highlighting of page elements.
            */
            #[serde(rename = "marginColor")]
            margin_color: dom::RGBA,
            /**
             * Configuration data for the highlighting of page elements.
            */
            #[serde(rename = "eventTargetColor")]
            event_target_color: dom::RGBA,
            /**
             * Configuration data for the highlighting of page elements.
            */
            #[serde(rename = "shapeColor")]
            shape_color: dom::RGBA,
            /**
             * Configuration data for the highlighting of page elements.
            */
            #[serde(rename = "shapeMarginColor")]
            shape_margin_color: dom::RGBA,
            /**
             * Configuration data for the highlighting of page elements.
            */
            #[serde(rename = "cssGridColor")]
            css_grid_color: dom::RGBA,
            /**
             * Configuration data for the highlighting of page elements.
            */
            #[serde(rename = "colorFormat")]
            color_format: ColorFormat,
            /**
             * Configuration data for the highlighting of page elements.
            */
            #[serde(rename = "gridHighlightConfig")]
            grid_highlight_config: GridHighlightConfig,
            /**
             * Configuration data for the highlighting of page elements.
            */
            #[serde(rename = "flexContainerHighlightConfig")]
            flex_container_highlight_config: FlexContainerHighlightConfig,
            /**
             * Configuration data for the highlighting of page elements.
            */
            #[serde(rename = "flexItemHighlightConfig")]
            flex_item_highlight_config: FlexItemHighlightConfig,
            /**
             * Configuration data for the highlighting of page elements.
            */
            #[serde(rename = "contrastAlgorithm")]
            contrast_algorithm: ContrastAlgorithm,
            /**
             * Configuration data for the highlighting of page elements.
            */
            #[serde(rename = "containerQueryContainerHighlightConfig")]
            container_query_container_highlight_config: ContainerQueryContainerHighlightConfig,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum ColorFormat {
            #[serde(rename = "rgb")]
            Rgb,
            #[serde(rename = "hsl")]
            Hsl,
            #[serde(rename = "hwb")]
            Hwb,
            #[serde(rename = "hex")]
            Hex,
        }

        /**
         * Configurations for Persistent Grid Highlight
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct GridNodeHighlightConfig {
            /**
             * Configurations for Persistent Grid Highlight
            */
            #[serde(rename = "gridHighlightConfig")]
            grid_highlight_config: Option<GridHighlightConfig>,
            /**
             * Configurations for Persistent Grid Highlight
            */
            #[serde(rename = "nodeId")]
            node_id: Option<dom::NodeId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct FlexNodeHighlightConfig {
            #[serde(rename = "flexContainerHighlightConfig")]
            flex_container_highlight_config: Option<FlexContainerHighlightConfig>,

            #[serde(rename = "nodeId")]
            node_id: Option<dom::NodeId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ScrollSnapContainerHighlightConfig {
            #[serde(rename = "snapportBorder")]
            snapport_border: LineStyle,

            #[serde(rename = "snapAreaBorder")]
            snap_area_border: LineStyle,

            #[serde(rename = "scrollMarginColor")]
            scroll_margin_color: dom::RGBA,

            #[serde(rename = "scrollPaddingColor")]
            scroll_padding_color: dom::RGBA,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ScrollSnapHighlightConfig {
            #[serde(rename = "scrollSnapContainerHighlightConfig")]
            scroll_snap_container_highlight_config: Option<ScrollSnapContainerHighlightConfig>,

            #[serde(rename = "nodeId")]
            node_id: Option<dom::NodeId>,
        }

        /**
         * Configuration for dual screen hinge
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct HingeConfig {
            /**
             * Configuration for dual screen hinge
            */
            rect: Option<dom::Rect>,
            /**
             * Configuration for dual screen hinge
            */
            #[serde(rename = "contentColor")]
            content_color: dom::RGBA,
            /**
             * Configuration for dual screen hinge
            */
            #[serde(rename = "outlineColor")]
            outline_color: dom::RGBA,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ContainerQueryHighlightConfig {
            #[serde(rename = "containerQueryContainerHighlightConfig")]
            container_query_container_highlight_config:
                Option<ContainerQueryContainerHighlightConfig>,

            #[serde(rename = "nodeId")]
            node_id: Option<dom::NodeId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ContainerQueryContainerHighlightConfig {
            #[serde(rename = "containerBorder")]
            container_border: LineStyle,

            #[serde(rename = "descendantBorder")]
            descendant_border: LineStyle,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct IsolatedElementHighlightConfig {
            #[serde(rename = "isolationModeHighlightConfig")]
            isolation_mode_highlight_config: Option<IsolationModeHighlightConfig>,

            #[serde(rename = "nodeId")]
            node_id: Option<dom::NodeId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct IsolationModeHighlightConfig {
            #[serde(rename = "resizerColor")]
            resizer_color: dom::RGBA,

            #[serde(rename = "resizerHandleColor")]
            resizer_handle_color: dom::RGBA,

            #[serde(rename = "maskColor")]
            mask_color: dom::RGBA,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum InspectMode {
            #[serde(rename = "searchForNode")]
            SearchForNode,
            #[serde(rename = "searchForUAShadowDOM")]
            SearchForUAShadowDOM,
            #[serde(rename = "captureAreaScreenshot")]
            CaptureAreaScreenshot,
            #[serde(rename = "showDistances")]
            ShowDistances,
            #[serde(rename = "none")]
            None,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetHighlightObjectForTestRequest {
            #[serde(rename = "nodeId")]
            node_id: Option<dom::NodeId>,

            #[serde(rename = "includeDistance")]
            include_distance: bool,

            #[serde(rename = "includeStyle")]
            include_style: bool,

            #[serde(rename = "colorFormat")]
            color_format: ColorFormat,

            #[serde(rename = "showAccessibilityInfo")]
            show_accessibility_info: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetHighlightObjectForTestResponse {
            highlight: Option<serde_json::Value>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetGridHighlightObjectsForTestRequest {
            #[serde(rename = "nodeIds")]
            node_ids: Option<Vec<dom::NodeId>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetGridHighlightObjectsForTestResponse {
            highlights: Option<serde_json::Value>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetSourceOrderHighlightObjectForTestRequest {
            #[serde(rename = "nodeId")]
            node_id: Option<dom::NodeId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetSourceOrderHighlightObjectForTestResponse {
            highlight: Option<serde_json::Value>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct HighlightFrameRequest {
            #[serde(rename = "frameId")]
            frame_id: Option<page::FrameId>,

            #[serde(rename = "contentColor")]
            content_color: dom::RGBA,

            #[serde(rename = "contentOutlineColor")]
            content_outline_color: dom::RGBA,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct HighlightNodeRequest {
            #[serde(rename = "highlightConfig")]
            highlight_config: Option<HighlightConfig>,

            #[serde(rename = "nodeId")]
            node_id: dom::NodeId,

            #[serde(rename = "backendNodeId")]
            backend_node_id: dom::BackendNodeId,

            #[serde(rename = "objectId")]
            object_id: runtime::RemoteObjectId,

            selector: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct HighlightQuadRequest {
            quad: Option<dom::Quad>,

            color: dom::RGBA,

            #[serde(rename = "outlineColor")]
            outlinecolor: dom::RGBA,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct HighlightRectRequest {
            x: Option<Integer>,

            y: Option<Integer>,

            width: Option<Integer>,

            height: Option<Integer>,

            color: dom::RGBA,

            #[serde(rename = "outlineColor")]
            outlinecolor: dom::RGBA,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct HighlightSourceOrderRequest {
            #[serde(rename = "sourceOrderConfig")]
            source_order_config: Option<SourceOrderConfig>,

            #[serde(rename = "nodeId")]
            node_id: dom::NodeId,

            #[serde(rename = "backendNodeId")]
            backend_node_id: dom::BackendNodeId,

            #[serde(rename = "objectId")]
            object_id: runtime::RemoteObjectId,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetInspectModeRequest {
            mode: Option<InspectMode>,

            #[serde(rename = "highlightConfig")]
            highlightconfig: HighlightConfig,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetShowAdHighlightsRequest {
            show: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetPausedInDebuggerMessageRequest {
            message: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetShowDebugBordersRequest {
            show: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetShowFPSCounterRequest {
            show: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetShowGridOverlaysRequest {
            #[serde(rename = "gridNodeHighlightConfigs")]
            grid_node_highlight_configs: Option<Vec<GridNodeHighlightConfig>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetShowFlexOverlaysRequest {
            #[serde(rename = "flexNodeHighlightConfigs")]
            flex_node_highlight_configs: Option<Vec<FlexNodeHighlightConfig>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetShowScrollSnapOverlaysRequest {
            #[serde(rename = "scrollSnapHighlightConfigs")]
            scroll_snap_highlight_configs: Option<Vec<ScrollSnapHighlightConfig>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetShowContainerQueryOverlaysRequest {
            #[serde(rename = "containerQueryHighlightConfigs")]
            container_query_highlight_configs: Option<Vec<ContainerQueryHighlightConfig>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetShowPaintRectsRequest {
            result: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetShowLayoutShiftRegionsRequest {
            result: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetShowScrollBottleneckRectsRequest {
            show: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetShowHitTestBordersRequest {
            show: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetShowWebVitalsRequest {
            show: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetShowViewportSizeOnResizeRequest {
            show: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetShowHingeRequest {
            #[serde(rename = "hingeConfig")]
            hinge_config: HingeConfig,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetShowIsolatedElementsRequest {
            #[serde(rename = "isolatedElementHighlightConfigs")]
            isolated_element_highlight_configs: Option<Vec<IsolatedElementHighlightConfig>>,
        }

        /**
         * Fired when the node should be inspected. This happens after call to `setInspectMode` or when
         * user manually inspects an element.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InspectNodeRequestedEvent {
            /**
             * Fired when the node should be inspected. This happens after call to `setInspectMode` or when
             * user manually inspects an element.
            */
            #[serde(rename = "backendNodeId")]
            backend_node_id: Option<dom::BackendNodeId>,
        }

        /**
         * Fired when the node should be highlighted. This happens after call to `setInspectMode`.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct NodeHighlightRequestedEvent {
            /**
             * Fired when the node should be highlighted. This happens after call to `setInspectMode`.
            */
            #[serde(rename = "nodeId")]
            node_id: Option<dom::NodeId>,
        }

        /**
         * Fired when user asks to capture screenshot of some area on the page.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ScreenshotRequestedEvent {
            /**
             * Fired when user asks to capture screenshot of some area on the page.
            */
            viewport: Option<page::Viewport>,
        }
    }

    /**
     * Actions and events related to the inspected page belong to the page domain.
    */
    pub mod page {
        use super::{debugger, dom, emulation, io, network, runtime, Integer};
        use serde::{self, Deserialize, Serialize};
        /**
         * Unique frame identifier.
        */
        pub type FrameId = String;

        /**
         * Indicates whether a frame has been identified as an ad.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum AdFrameType {
            #[serde(rename = "none")]
            None,
            #[serde(rename = "child")]
            Child,
            #[serde(rename = "root")]
            Root,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum AdFrameExplanation {
            #[serde(rename = "ParentIsAd")]
            ParentIsAd,
            #[serde(rename = "CreatedByAdScript")]
            CreatedByAdScript,
            #[serde(rename = "MatchedBlockingRule")]
            MatchedBlockingRule,
        }

        /**
         * Indicates whether a frame has been identified as an ad and why.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AdFrameStatus {
            /**
             * Indicates whether a frame has been identified as an ad and why.
            */
            #[serde(rename = "adFrameType")]
            ad_frame_type: Option<AdFrameType>,
            /**
             * Indicates whether a frame has been identified as an ad and why.
            */
            explanations: Vec<AdFrameExplanation>,
        }

        /**
         * Identifies the bottom-most script which caused the frame to be labelled
         * as an ad.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AdScriptId {
            /**
             * Identifies the bottom-most script which caused the frame to be labelled
             * as an ad.
            */
            #[serde(rename = "scriptId")]
            script_id: Option<runtime::ScriptId>,
            /**
             * Identifies the bottom-most script which caused the frame to be labelled
             * as an ad.
            */
            #[serde(rename = "debuggerId")]
            debugger_id: Option<runtime::UniqueDebuggerId>,
        }

        /**
         * Indicates whether the frame is a secure context and why it is the case.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum SecureContextType {
            #[serde(rename = "Secure")]
            Secure,
            #[serde(rename = "SecureLocalhost")]
            SecureLocalhost,
            #[serde(rename = "InsecureScheme")]
            InsecureScheme,
            #[serde(rename = "InsecureAncestor")]
            InsecureAncestor,
        }

        /**
         * Indicates whether the frame is cross-origin isolated and why it is the case.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum CrossOriginIsolatedContextType {
            #[serde(rename = "Isolated")]
            Isolated,
            #[serde(rename = "NotIsolated")]
            NotIsolated,
            #[serde(rename = "NotIsolatedFeatureDisabled")]
            NotIsolatedFeatureDisabled,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum GatedAPIFeatures {
            #[serde(rename = "SharedArrayBuffers")]
            SharedArrayBuffers,
            #[serde(rename = "SharedArrayBuffersTransferAllowed")]
            SharedArrayBuffersTransferAllowed,
            #[serde(rename = "PerformanceMeasureMemory")]
            PerformanceMeasureMemory,
            #[serde(rename = "PerformanceProfile")]
            PerformanceProfile,
        }

        /**
         * All Permissions Policy features. This enum should match the one defined
         * in third_party/blink/renderer/core/permissions_policy/permissions_policy_features.json5.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum PermissionsPolicyFeature {
            #[serde(rename = "accelerometer")]
            Accelerometer,
            #[serde(rename = "ambient-light-sensor")]
            AmbientLightSensor,
            #[serde(rename = "attribution-reporting")]
            AttributionReporting,
            #[serde(rename = "autoplay")]
            Autoplay,
            #[serde(rename = "bluetooth")]
            Bluetooth,
            #[serde(rename = "browsing-topics")]
            BrowsingTopics,
            #[serde(rename = "camera")]
            Camera,
            #[serde(rename = "ch-dpr")]
            ChDpr,
            #[serde(rename = "ch-device-memory")]
            ChDeviceMemory,
            #[serde(rename = "ch-downlink")]
            ChDownlink,
            #[serde(rename = "ch-ect")]
            ChEct,
            #[serde(rename = "ch-prefers-color-scheme")]
            ChPrefersColorScheme,
            #[serde(rename = "ch-prefers-reduced-motion")]
            ChPrefersReducedMotion,
            #[serde(rename = "ch-rtt")]
            ChRtt,
            #[serde(rename = "ch-save-data")]
            ChSaveData,
            #[serde(rename = "ch-ua")]
            ChUa,
            #[serde(rename = "ch-ua-arch")]
            ChUaArch,
            #[serde(rename = "ch-ua-bitness")]
            ChUaBitness,
            #[serde(rename = "ch-ua-platform")]
            ChUaPlatform,
            #[serde(rename = "ch-ua-model")]
            ChUaModel,
            #[serde(rename = "ch-ua-mobile")]
            ChUaMobile,
            #[serde(rename = "ch-ua-form-factor")]
            ChUaFormFactor,
            #[serde(rename = "ch-ua-full-version")]
            ChUaFullVersion,
            #[serde(rename = "ch-ua-full-version-list")]
            ChUaFullVersionList,
            #[serde(rename = "ch-ua-platform-version")]
            ChUaPlatformVersion,
            #[serde(rename = "ch-ua-wow64")]
            ChUaWow64,
            #[serde(rename = "ch-viewport-height")]
            ChViewportHeight,
            #[serde(rename = "ch-viewport-width")]
            ChViewportWidth,
            #[serde(rename = "ch-width")]
            ChWidth,
            #[serde(rename = "clipboard-read")]
            ClipboardRead,
            #[serde(rename = "clipboard-write")]
            ClipboardWrite,
            #[serde(rename = "compute-pressure")]
            ComputePressure,
            #[serde(rename = "cross-origin-isolated")]
            CrossOriginIsolated,
            #[serde(rename = "direct-sockets")]
            DirectSockets,
            #[serde(rename = "display-capture")]
            DisplayCapture,
            #[serde(rename = "document-domain")]
            DocumentDomain,
            #[serde(rename = "encrypted-media")]
            EncryptedMedia,
            #[serde(rename = "execution-while-out-of-viewport")]
            ExecutionWhileOutOfViewport,
            #[serde(rename = "execution-while-not-rendered")]
            ExecutionWhileNotRendered,
            #[serde(rename = "focus-without-user-activation")]
            FocusWithoutUserActivation,
            #[serde(rename = "fullscreen")]
            Fullscreen,
            #[serde(rename = "frobulate")]
            Frobulate,
            #[serde(rename = "gamepad")]
            Gamepad,
            #[serde(rename = "geolocation")]
            Geolocation,
            #[serde(rename = "gyroscope")]
            Gyroscope,
            #[serde(rename = "hid")]
            Hid,
            #[serde(rename = "identity-credentials-get")]
            IdentityCredentialsGet,
            #[serde(rename = "idle-detection")]
            IdleDetection,
            #[serde(rename = "interest-cohort")]
            InterestCohort,
            #[serde(rename = "join-ad-interest-group")]
            JoinAdInterestGroup,
            #[serde(rename = "keyboard-map")]
            KeyboardMap,
            #[serde(rename = "local-fonts")]
            LocalFonts,
            #[serde(rename = "magnetometer")]
            Magnetometer,
            #[serde(rename = "microphone")]
            Microphone,
            #[serde(rename = "midi")]
            Midi,
            #[serde(rename = "otp-credentials")]
            OtpCredentials,
            #[serde(rename = "payment")]
            Payment,
            #[serde(rename = "picture-in-picture")]
            PictureInPicture,
            #[serde(rename = "private-aggregation")]
            PrivateAggregation,
            #[serde(rename = "private-state-token-issuance")]
            PrivateStateTokenIssuance,
            #[serde(rename = "private-state-token-redemption")]
            PrivateStateTokenRedemption,
            #[serde(rename = "publickey-credentials-get")]
            PublickeyCredentialsGet,
            #[serde(rename = "run-ad-auction")]
            RunAdAuction,
            #[serde(rename = "screen-wake-lock")]
            ScreenWakeLock,
            #[serde(rename = "serial")]
            Serial,
            #[serde(rename = "shared-autofill")]
            SharedAutofill,
            #[serde(rename = "shared-storage")]
            SharedStorage,
            #[serde(rename = "shared-storage-select-url")]
            SharedStorageSelectUrl,
            #[serde(rename = "smart-card")]
            SmartCard,
            #[serde(rename = "storage-access")]
            StorageAccess,
            #[serde(rename = "sync-xhr")]
            SyncXhr,
            #[serde(rename = "unload")]
            Unload,
            #[serde(rename = "usb")]
            Usb,
            #[serde(rename = "vertical-scroll")]
            VerticalScroll,
            #[serde(rename = "web-share")]
            WebShare,
            #[serde(rename = "window-management")]
            WindowManagement,
            #[serde(rename = "window-placement")]
            WindowPlacement,
            #[serde(rename = "xr-spatial-tracking")]
            XrSpatialTracking,
        }

        /**
         * Reason for a permissions policy feature to be disabled.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum PermissionsPolicyBlockReason {
            #[serde(rename = "Header")]
            Header,
            #[serde(rename = "IframeAttribute")]
            IframeAttribute,
            #[serde(rename = "InFencedFrameTree")]
            InFencedFrameTree,
            #[serde(rename = "InIsolatedApp")]
            InIsolatedApp,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct PermissionsPolicyBlockLocator {
            #[serde(rename = "frameId")]
            frame_id: Option<FrameId>,

            #[serde(rename = "blockReason")]
            block_reason: Option<PermissionsPolicyBlockReason>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct PermissionsPolicyFeatureState {
            feature: Option<PermissionsPolicyFeature>,

            allowed: Option<bool>,

            locator: PermissionsPolicyBlockLocator,
        }

        /**
         * Origin Trial(https://www.chromium.org/blink/origin-trials) support.
         * Status for an Origin Trial token.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum OriginTrialTokenStatus {
            #[serde(rename = "Success")]
            Success,
            #[serde(rename = "NotSupported")]
            NotSupported,
            #[serde(rename = "Insecure")]
            Insecure,
            #[serde(rename = "Expired")]
            Expired,
            #[serde(rename = "WrongOrigin")]
            WrongOrigin,
            #[serde(rename = "InvalidSignature")]
            InvalidSignature,
            #[serde(rename = "Malformed")]
            Malformed,
            #[serde(rename = "WrongVersion")]
            WrongVersion,
            #[serde(rename = "FeatureDisabled")]
            FeatureDisabled,
            #[serde(rename = "TokenDisabled")]
            TokenDisabled,
            #[serde(rename = "FeatureDisabledForUser")]
            FeatureDisabledForUser,
            #[serde(rename = "UnknownTrial")]
            UnknownTrial,
        }

        /**
         * Status for an Origin Trial.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum OriginTrialStatus {
            #[serde(rename = "Enabled")]
            Enabled,
            #[serde(rename = "ValidTokenNotProvided")]
            ValidTokenNotProvided,
            #[serde(rename = "OSNotSupported")]
            OSNotSupported,
            #[serde(rename = "TrialNotAllowed")]
            TrialNotAllowed,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum OriginTrialUsageRestriction {
            #[serde(rename = "None")]
            None,
            #[serde(rename = "Subset")]
            Subset,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct OriginTrialToken {
            origin: Option<String>,

            #[serde(rename = "matchSubDomains")]
            match_sub_domains: Option<bool>,

            #[serde(rename = "trialName")]
            trial_name: Option<String>,

            #[serde(rename = "expiryTime")]
            expiry_time: Option<network::TimeSinceEpoch>,

            #[serde(rename = "isThirdParty")]
            is_third_party: Option<bool>,

            #[serde(rename = "usageRestriction")]
            usage_restriction: Option<OriginTrialUsageRestriction>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct OriginTrialTokenWithStatus {
            #[serde(rename = "rawTokenText")]
            raw_token_text: Option<String>,

            #[serde(rename = "parsedToken")]
            parsed_token: OriginTrialToken,

            status: Option<OriginTrialTokenStatus>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct OriginTrial {
            #[serde(rename = "trialName")]
            trial_name: Option<String>,

            status: Option<OriginTrialStatus>,

            #[serde(rename = "tokensWithStatus")]
            tokens_with_status: Option<Vec<OriginTrialTokenWithStatus>>,
        }

        /**
         * Information about the Frame on the page.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Frame {
            /**
             * Information about the Frame on the page.
            */
            id: Option<FrameId>,
            /**
             * Information about the Frame on the page.
            */
            #[serde(rename = "parentId")]
            parent_id: FrameId,
            /**
             * Information about the Frame on the page.
            */
            #[serde(rename = "loaderId")]
            loader_id: Option<network::LoaderId>,
            /**
             * Information about the Frame on the page.
            */
            name: String,
            /**
             * Information about the Frame on the page.
            */
            url: Option<String>,
            /**
             * Information about the Frame on the page.
            */
            #[serde(rename = "urlFragment")]
            url_fragment: String,
            /**
             * Information about the Frame on the page.
            */
            #[serde(rename = "domainAndRegistry")]
            domain_and_registry: Option<String>,
            /**
             * Information about the Frame on the page.
            */
            #[serde(rename = "securityOrigin")]
            security_origin: Option<String>,
            /**
             * Information about the Frame on the page.
            */
            #[serde(rename = "mimeType")]
            mime_type: Option<String>,
            /**
             * Information about the Frame on the page.
            */
            #[serde(rename = "unreachableUrl")]
            unreachable_url: String,
            /**
             * Information about the Frame on the page.
            */
            #[serde(rename = "adFrameStatus")]
            ad_frame_status: AdFrameStatus,
            /**
             * Information about the Frame on the page.
            */
            #[serde(rename = "secureContextType")]
            secure_context_type: Option<SecureContextType>,
            /**
             * Information about the Frame on the page.
            */
            #[serde(rename = "crossOriginIsolatedContextType")]
            cross_origin_isolated_context_type: Option<CrossOriginIsolatedContextType>,
            /**
             * Information about the Frame on the page.
            */
            #[serde(rename = "gatedAPIFeatures")]
            gated_apifeatures: Option<Vec<GatedAPIFeatures>>,
        }

        /**
         * Information about the Resource on the page.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FrameResource {
            /**
             * Information about the Resource on the page.
            */
            url: Option<String>,
            /**
             * Information about the Resource on the page.
            */
            #[serde(rename = "type")]
            r#type: Option<network::ResourceType>,
            /**
             * Information about the Resource on the page.
            */
            #[serde(rename = "mimeType")]
            mimetype: Option<String>,
            /**
             * Information about the Resource on the page.
            */
            #[serde(rename = "lastModified")]
            lastmodified: network::TimeSinceEpoch,
            /**
             * Information about the Resource on the page.
            */
            #[serde(rename = "contentSize")]
            contentsize: f64,
            /**
             * Information about the Resource on the page.
            */
            failed: bool,
            /**
             * Information about the Resource on the page.
            */
            canceled: bool,
        }

        /**
         * Information about the Frame hierarchy along with their cached resources.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FrameResourceTree {
            /**
             * Information about the Frame hierarchy along with their cached resources.
            */
            frame: Option<Frame>,
            /**
             * Information about the Frame hierarchy along with their cached resources.
            */
            #[serde(rename = "childFrames")]
            childframes: Vec<FrameResourceTree>,
            /**
             * Information about the Frame hierarchy along with their cached resources.
            */
            resources: Option<Vec<FrameResource>>,
        }

        /**
         * Information about the Frame hierarchy.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FrameTree {
            /**
             * Information about the Frame hierarchy.
            */
            frame: Option<Frame>,
            /**
             * Information about the Frame hierarchy.
            */
            #[serde(rename = "childFrames")]
            childframes: Vec<FrameTree>,
        }

        /**
         * Unique script identifier.
        */
        pub type ScriptIdentifier = String;

        /**
         * Transition type.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum TransitionType {
            #[serde(rename = "link")]
            Link,
            #[serde(rename = "typed")]
            Typed,
            #[serde(rename = "address_bar")]
            AddressBar,
            #[serde(rename = "auto_bookmark")]
            AutoBookmark,
            #[serde(rename = "auto_subframe")]
            AutoSubframe,
            #[serde(rename = "manual_subframe")]
            ManualSubframe,
            #[serde(rename = "generated")]
            Generated,
            #[serde(rename = "auto_toplevel")]
            AutoToplevel,
            #[serde(rename = "form_submit")]
            FormSubmit,
            #[serde(rename = "reload")]
            Reload,
            #[serde(rename = "keyword")]
            Keyword,
            #[serde(rename = "keyword_generated")]
            KeywordGenerated,
            #[serde(rename = "other")]
            Other,
        }

        /**
         * Navigation history entry.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct NavigationEntry {
            /**
             * Navigation history entry.
            */
            id: Option<Integer>,
            /**
             * Navigation history entry.
            */
            url: Option<String>,
            /**
             * Navigation history entry.
            */
            #[serde(rename = "userTypedURL")]
            usertypedurl: Option<String>,
            /**
             * Navigation history entry.
            */
            title: Option<String>,
            /**
             * Navigation history entry.
            */
            #[serde(rename = "transitionType")]
            transitiontype: Option<TransitionType>,
        }

        /**
         * Screencast frame metadata.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ScreencastFrameMetadata {
            /**
             * Screencast frame metadata.
            */
            #[serde(rename = "offsetTop")]
            offset_top: Option<f64>,
            /**
             * Screencast frame metadata.
            */
            #[serde(rename = "pageScaleFactor")]
            page_scale_factor: Option<f64>,
            /**
             * Screencast frame metadata.
            */
            #[serde(rename = "deviceWidth")]
            device_width: Option<f64>,
            /**
             * Screencast frame metadata.
            */
            #[serde(rename = "deviceHeight")]
            device_height: Option<f64>,
            /**
             * Screencast frame metadata.
            */
            #[serde(rename = "scrollOffsetX")]
            scroll_offset_x: Option<f64>,
            /**
             * Screencast frame metadata.
            */
            #[serde(rename = "scrollOffsetY")]
            scroll_offset_y: Option<f64>,
            /**
             * Screencast frame metadata.
            */
            timestamp: network::TimeSinceEpoch,
        }

        /**
         * Javascript dialog type.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum DialogType {
            #[serde(rename = "alert")]
            Alert,
            #[serde(rename = "confirm")]
            Confirm,
            #[serde(rename = "prompt")]
            Prompt,
            #[serde(rename = "beforeunload")]
            Beforeunload,
        }

        /**
         * Error while paring app manifest.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AppManifestError {
            /**
             * Error while paring app manifest.
            */
            message: Option<String>,
            /**
             * Error while paring app manifest.
            */
            critical: Option<Integer>,
            /**
             * Error while paring app manifest.
            */
            line: Option<Integer>,
            /**
             * Error while paring app manifest.
            */
            column: Option<Integer>,
        }

        /**
         * Parsed app manifest properties.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AppManifestParsedProperties {
            /**
             * Parsed app manifest properties.
            */
            scope: Option<String>,
        }

        /**
         * Layout viewport position and dimensions.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct LayoutViewport {
            /**
             * Layout viewport position and dimensions.
            */
            #[serde(rename = "pageX")]
            page_x: Option<Integer>,
            /**
             * Layout viewport position and dimensions.
            */
            #[serde(rename = "pageY")]
            page_y: Option<Integer>,
            /**
             * Layout viewport position and dimensions.
            */
            #[serde(rename = "clientWidth")]
            client_width: Option<Integer>,
            /**
             * Layout viewport position and dimensions.
            */
            #[serde(rename = "clientHeight")]
            client_height: Option<Integer>,
        }

        /**
         * Visual viewport position, dimensions, and scale.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct VisualViewport {
            /**
             * Visual viewport position, dimensions, and scale.
            */
            #[serde(rename = "offsetX")]
            offset_x: Option<f64>,
            /**
             * Visual viewport position, dimensions, and scale.
            */
            #[serde(rename = "offsetY")]
            offset_y: Option<f64>,
            /**
             * Visual viewport position, dimensions, and scale.
            */
            #[serde(rename = "pageX")]
            page_x: Option<f64>,
            /**
             * Visual viewport position, dimensions, and scale.
            */
            #[serde(rename = "pageY")]
            page_y: Option<f64>,
            /**
             * Visual viewport position, dimensions, and scale.
            */
            #[serde(rename = "clientWidth")]
            client_width: Option<f64>,
            /**
             * Visual viewport position, dimensions, and scale.
            */
            #[serde(rename = "clientHeight")]
            client_height: Option<f64>,
            /**
             * Visual viewport position, dimensions, and scale.
            */
            scale: Option<f64>,
            /**
             * Visual viewport position, dimensions, and scale.
            */
            zoom: f64,
        }

        /**
         * Viewport for capturing screenshot.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Viewport {
            /**
             * Viewport for capturing screenshot.
            */
            x: Option<f64>,
            /**
             * Viewport for capturing screenshot.
            */
            y: Option<f64>,
            /**
             * Viewport for capturing screenshot.
            */
            width: Option<f64>,
            /**
             * Viewport for capturing screenshot.
            */
            height: Option<f64>,
            /**
             * Viewport for capturing screenshot.
            */
            scale: Option<f64>,
        }

        /**
         * Generic font families collection.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FontFamilies {
            /**
             * Generic font families collection.
            */
            standard: String,
            /**
             * Generic font families collection.
            */
            fixed: String,
            /**
             * Generic font families collection.
            */
            serif: String,
            /**
             * Generic font families collection.
            */
            #[serde(rename = "sansSerif")]
            sansserif: String,
            /**
             * Generic font families collection.
            */
            cursive: String,
            /**
             * Generic font families collection.
            */
            fantasy: String,
            /**
             * Generic font families collection.
            */
            math: String,
        }

        /**
         * Font families collection for a script.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ScriptFontFamilies {
            /**
             * Font families collection for a script.
            */
            script: Option<String>,
            /**
             * Font families collection for a script.
            */
            #[serde(rename = "fontFamilies")]
            fontfamilies: Option<FontFamilies>,
        }

        /**
         * Default font sizes.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FontSizes {
            /**
             * Default font sizes.
            */
            standard: Integer,
            /**
             * Default font sizes.
            */
            fixed: Integer,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum ClientNavigationReason {
            #[serde(rename = "formSubmissionGet")]
            FormSubmissionGet,
            #[serde(rename = "formSubmissionPost")]
            FormSubmissionPost,
            #[serde(rename = "httpHeaderRefresh")]
            HttpHeaderRefresh,
            #[serde(rename = "scriptInitiated")]
            ScriptInitiated,
            #[serde(rename = "metaTagRefresh")]
            MetaTagRefresh,
            #[serde(rename = "pageBlockInterstitial")]
            PageBlockInterstitial,
            #[serde(rename = "reload")]
            Reload,
            #[serde(rename = "anchorClick")]
            AnchorClick,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum ClientNavigationDisposition {
            #[serde(rename = "currentTab")]
            CurrentTab,
            #[serde(rename = "newTab")]
            NewTab,
            #[serde(rename = "newWindow")]
            NewWindow,
            #[serde(rename = "download")]
            Download,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct InstallabilityErrorArgument {
            name: Option<String>,

            value: Option<String>,
        }

        /**
         * The installability error
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InstallabilityError {
            /**
             * The installability error
            */
            #[serde(rename = "errorId")]
            error_id: Option<String>,
            /**
             * The installability error
            */
            #[serde(rename = "errorArguments")]
            error_arguments: Option<Vec<InstallabilityErrorArgument>>,
        }

        /**
         * The referring-policy used for the navigation.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum ReferrerPolicy {
            #[serde(rename = "noReferrer")]
            NoReferrer,
            #[serde(rename = "noReferrerWhenDowngrade")]
            NoReferrerWhenDowngrade,
            #[serde(rename = "origin")]
            Origin,
            #[serde(rename = "originWhenCrossOrigin")]
            OriginWhenCrossOrigin,
            #[serde(rename = "sameOrigin")]
            SameOrigin,
            #[serde(rename = "strictOrigin")]
            StrictOrigin,
            #[serde(rename = "strictOriginWhenCrossOrigin")]
            StrictOriginWhenCrossOrigin,
            #[serde(rename = "unsafeUrl")]
            UnsafeUrl,
        }

        /**
         * Per-script compilation cache parameters for `Page.produceCompilationCache`
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct CompilationCacheParams {
            /**
             * Per-script compilation cache parameters for `Page.produceCompilationCache`
            */
            url: Option<String>,
            /**
             * Per-script compilation cache parameters for `Page.produceCompilationCache`
            */
            eager: bool,
        }

        /**
         * Enum of possible auto-reponse for permisison / prompt dialogs.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum AutoResponseMode {
            #[serde(rename = "none")]
            None,
            #[serde(rename = "autoAccept")]
            AutoAccept,
            #[serde(rename = "autoReject")]
            AutoReject,
            #[serde(rename = "autoOptOut")]
            AutoOptOut,
        }

        /**
         * The type of a frameNavigated event.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum NavigationType {
            #[serde(rename = "Navigation")]
            Navigation,
            #[serde(rename = "BackForwardCacheRestore")]
            BackForwardCacheRestore,
        }

        /**
         * List of not restored reasons for back-forward cache.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum BackForwardCacheNotRestoredReason {
            #[serde(rename = "NotPrimaryMainFrame")]
            NotPrimaryMainFrame,
            #[serde(rename = "BackForwardCacheDisabled")]
            BackForwardCacheDisabled,
            #[serde(rename = "RelatedActiveContentsExist")]
            RelatedActiveContentsExist,
            #[serde(rename = "HTTPStatusNotOK")]
            HTTPStatusNotOK,
            #[serde(rename = "SchemeNotHTTPOrHTTPS")]
            SchemeNotHTTPOrHTTPS,
            #[serde(rename = "Loading")]
            Loading,
            #[serde(rename = "WasGrantedMediaAccess")]
            WasGrantedMediaAccess,
            #[serde(rename = "DisableForRenderFrameHostCalled")]
            DisableForRenderFrameHostCalled,
            #[serde(rename = "DomainNotAllowed")]
            DomainNotAllowed,
            #[serde(rename = "HTTPMethodNotGET")]
            HTTPMethodNotGET,
            #[serde(rename = "SubframeIsNavigating")]
            SubframeIsNavigating,
            #[serde(rename = "Timeout")]
            Timeout,
            #[serde(rename = "CacheLimit")]
            CacheLimit,
            #[serde(rename = "JavaScriptExecution")]
            JavaScriptExecution,
            #[serde(rename = "RendererProcessKilled")]
            RendererProcessKilled,
            #[serde(rename = "RendererProcessCrashed")]
            RendererProcessCrashed,
            #[serde(rename = "SchedulerTrackedFeatureUsed")]
            SchedulerTrackedFeatureUsed,
            #[serde(rename = "ConflictingBrowsingInstance")]
            ConflictingBrowsingInstance,
            #[serde(rename = "CacheFlushed")]
            CacheFlushed,
            #[serde(rename = "ServiceWorkerVersionActivation")]
            ServiceWorkerVersionActivation,
            #[serde(rename = "SessionRestored")]
            SessionRestored,
            #[serde(rename = "ServiceWorkerPostMessage")]
            ServiceWorkerPostMessage,
            #[serde(rename = "EnteredBackForwardCacheBeforeServiceWorkerHostAdded")]
            EnteredBackForwardCacheBeforeServiceWorkerHostAdded,
            #[serde(rename = "RenderFrameHostReused_SameSite")]
            RenderFrameHostReusedSameSite,
            #[serde(rename = "RenderFrameHostReused_CrossSite")]
            RenderFrameHostReusedCrossSite,
            #[serde(rename = "ServiceWorkerClaim")]
            ServiceWorkerClaim,
            #[serde(rename = "IgnoreEventAndEvict")]
            IgnoreEventAndEvict,
            #[serde(rename = "HaveInnerContents")]
            HaveInnerContents,
            #[serde(rename = "TimeoutPuttingInCache")]
            TimeoutPuttingInCache,
            #[serde(rename = "BackForwardCacheDisabledByLowMemory")]
            BackForwardCacheDisabledByLowMemory,
            #[serde(rename = "BackForwardCacheDisabledByCommandLine")]
            BackForwardCacheDisabledByCommandLine,
            #[serde(rename = "NetworkRequestDatapipeDrainedAsBytesConsumer")]
            NetworkRequestDatapipeDrainedAsBytesConsumer,
            #[serde(rename = "NetworkRequestRedirected")]
            NetworkRequestRedirected,
            #[serde(rename = "NetworkRequestTimeout")]
            NetworkRequestTimeout,
            #[serde(rename = "NetworkExceedsBufferLimit")]
            NetworkExceedsBufferLimit,
            #[serde(rename = "NavigationCancelledWhileRestoring")]
            NavigationCancelledWhileRestoring,
            #[serde(rename = "NotMostRecentNavigationEntry")]
            NotMostRecentNavigationEntry,
            #[serde(rename = "BackForwardCacheDisabledForPrerender")]
            BackForwardCacheDisabledForPrerender,
            #[serde(rename = "UserAgentOverrideDiffers")]
            UserAgentOverrideDiffers,
            #[serde(rename = "ForegroundCacheLimit")]
            ForegroundCacheLimit,
            #[serde(rename = "BrowsingInstanceNotSwapped")]
            BrowsingInstanceNotSwapped,
            #[serde(rename = "BackForwardCacheDisabledForDelegate")]
            BackForwardCacheDisabledForDelegate,
            #[serde(rename = "UnloadHandlerExistsInMainFrame")]
            UnloadHandlerExistsInMainFrame,
            #[serde(rename = "UnloadHandlerExistsInSubFrame")]
            UnloadHandlerExistsInSubFrame,
            #[serde(rename = "ServiceWorkerUnregistration")]
            ServiceWorkerUnregistration,
            #[serde(rename = "CacheControlNoStore")]
            CacheControlNoStore,
            #[serde(rename = "CacheControlNoStoreCookieModified")]
            CacheControlNoStoreCookieModified,
            #[serde(rename = "CacheControlNoStoreHTTPOnlyCookieModified")]
            CacheControlNoStoreHTTPOnlyCookieModified,
            #[serde(rename = "NoResponseHead")]
            NoResponseHead,
            #[serde(rename = "Unknown")]
            Unknown,
            #[serde(rename = "ActivationNavigationsDisallowedForBug1234857")]
            ActivationNavigationsDisallowedForBug1234857,
            #[serde(rename = "ErrorDocument")]
            ErrorDocument,
            #[serde(rename = "FencedFramesEmbedder")]
            FencedFramesEmbedder,
            #[serde(rename = "CookieDisabled")]
            CookieDisabled,
            #[serde(rename = "HTTPAuthRequired")]
            HTTPAuthRequired,
            #[serde(rename = "CookieFlushed")]
            CookieFlushed,
            #[serde(rename = "WebSocket")]
            WebSocket,
            #[serde(rename = "WebTransport")]
            WebTransport,
            #[serde(rename = "WebRTC")]
            WebRTC,
            #[serde(rename = "MainResourceHasCacheControlNoStore")]
            MainResourceHasCacheControlNoStore,
            #[serde(rename = "MainResourceHasCacheControlNoCache")]
            MainResourceHasCacheControlNoCache,
            #[serde(rename = "SubresourceHasCacheControlNoStore")]
            SubresourceHasCacheControlNoStore,
            #[serde(rename = "SubresourceHasCacheControlNoCache")]
            SubresourceHasCacheControlNoCache,
            #[serde(rename = "ContainsPlugins")]
            ContainsPlugins,
            #[serde(rename = "DocumentLoaded")]
            DocumentLoaded,
            #[serde(rename = "DedicatedWorkerOrWorklet")]
            DedicatedWorkerOrWorklet,
            #[serde(rename = "OutstandingNetworkRequestOthers")]
            OutstandingNetworkRequestOthers,
            #[serde(rename = "RequestedMIDIPermission")]
            RequestedMIDIPermission,
            #[serde(rename = "RequestedAudioCapturePermission")]
            RequestedAudioCapturePermission,
            #[serde(rename = "RequestedVideoCapturePermission")]
            RequestedVideoCapturePermission,
            #[serde(rename = "RequestedBackForwardCacheBlockedSensors")]
            RequestedBackForwardCacheBlockedSensors,
            #[serde(rename = "RequestedBackgroundWorkPermission")]
            RequestedBackgroundWorkPermission,
            #[serde(rename = "BroadcastChannel")]
            BroadcastChannel,
            #[serde(rename = "WebXR")]
            WebXR,
            #[serde(rename = "SharedWorker")]
            SharedWorker,
            #[serde(rename = "WebLocks")]
            WebLocks,
            #[serde(rename = "WebHID")]
            WebHID,
            #[serde(rename = "WebShare")]
            WebShare,
            #[serde(rename = "RequestedStorageAccessGrant")]
            RequestedStorageAccessGrant,
            #[serde(rename = "WebNfc")]
            WebNfc,
            #[serde(rename = "OutstandingNetworkRequestFetch")]
            OutstandingNetworkRequestFetch,
            #[serde(rename = "OutstandingNetworkRequestXHR")]
            OutstandingNetworkRequestXHR,
            #[serde(rename = "AppBanner")]
            AppBanner,
            #[serde(rename = "Printing")]
            Printing,
            #[serde(rename = "WebDatabase")]
            WebDatabase,
            #[serde(rename = "PictureInPicture")]
            PictureInPicture,
            #[serde(rename = "Portal")]
            Portal,
            #[serde(rename = "SpeechRecognizer")]
            SpeechRecognizer,
            #[serde(rename = "IdleManager")]
            IdleManager,
            #[serde(rename = "PaymentManager")]
            PaymentManager,
            #[serde(rename = "SpeechSynthesis")]
            SpeechSynthesis,
            #[serde(rename = "KeyboardLock")]
            KeyboardLock,
            #[serde(rename = "WebOTPService")]
            WebOTPService,
            #[serde(rename = "OutstandingNetworkRequestDirectSocket")]
            OutstandingNetworkRequestDirectSocket,
            #[serde(rename = "InjectedJavascript")]
            InjectedJavascript,
            #[serde(rename = "InjectedStyleSheet")]
            InjectedStyleSheet,
            #[serde(rename = "KeepaliveRequest")]
            KeepaliveRequest,
            #[serde(rename = "IndexedDBEvent")]
            IndexedDBEvent,
            #[serde(rename = "Dummy")]
            Dummy,
            #[serde(rename = "JsNetworkRequestReceivedCacheControlNoStoreResource")]
            JsNetworkRequestReceivedCacheControlNoStoreResource,
            #[serde(rename = "WebRTCSticky")]
            WebRTCSticky,
            #[serde(rename = "WebTransportSticky")]
            WebTransportSticky,
            #[serde(rename = "WebSocketSticky")]
            WebSocketSticky,
            #[serde(rename = "ContentSecurityHandler")]
            ContentSecurityHandler,
            #[serde(rename = "ContentWebAuthenticationAPI")]
            ContentWebAuthenticationAPI,
            #[serde(rename = "ContentFileChooser")]
            ContentFileChooser,
            #[serde(rename = "ContentSerial")]
            ContentSerial,
            #[serde(rename = "ContentFileSystemAccess")]
            ContentFileSystemAccess,
            #[serde(rename = "ContentMediaDevicesDispatcherHost")]
            ContentMediaDevicesDispatcherHost,
            #[serde(rename = "ContentWebBluetooth")]
            ContentWebBluetooth,
            #[serde(rename = "ContentWebUSB")]
            ContentWebUSB,
            #[serde(rename = "ContentMediaSessionService")]
            ContentMediaSessionService,
            #[serde(rename = "ContentScreenReader")]
            ContentScreenReader,
            #[serde(rename = "EmbedderPopupBlockerTabHelper")]
            EmbedderPopupBlockerTabHelper,
            #[serde(rename = "EmbedderSafeBrowsingTriggeredPopupBlocker")]
            EmbedderSafeBrowsingTriggeredPopupBlocker,
            #[serde(rename = "EmbedderSafeBrowsingThreatDetails")]
            EmbedderSafeBrowsingThreatDetails,
            #[serde(rename = "EmbedderAppBannerManager")]
            EmbedderAppBannerManager,
            #[serde(rename = "EmbedderDomDistillerViewerSource")]
            EmbedderDomDistillerViewerSource,
            #[serde(rename = "EmbedderDomDistillerSelfDeletingRequestDelegate")]
            EmbedderDomDistillerSelfDeletingRequestDelegate,
            #[serde(rename = "EmbedderOomInterventionTabHelper")]
            EmbedderOomInterventionTabHelper,
            #[serde(rename = "EmbedderOfflinePage")]
            EmbedderOfflinePage,
            #[serde(rename = "EmbedderChromePasswordManagerClientBindCredentialManager")]
            EmbedderChromePasswordManagerClientBindCredentialManager,
            #[serde(rename = "EmbedderPermissionRequestManager")]
            EmbedderPermissionRequestManager,
            #[serde(rename = "EmbedderModalDialog")]
            EmbedderModalDialog,
            #[serde(rename = "EmbedderExtensions")]
            EmbedderExtensions,
            #[serde(rename = "EmbedderExtensionMessaging")]
            EmbedderExtensionMessaging,
            #[serde(rename = "EmbedderExtensionMessagingForOpenPort")]
            EmbedderExtensionMessagingForOpenPort,
            #[serde(rename = "EmbedderExtensionSentMessageToCachedFrame")]
            EmbedderExtensionSentMessageToCachedFrame,
        }

        /**
         * Types of not restored reasons for back-forward cache.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum BackForwardCacheNotRestoredReasonType {
            #[serde(rename = "SupportPending")]
            SupportPending,
            #[serde(rename = "PageSupportNeeded")]
            PageSupportNeeded,
            #[serde(rename = "Circumstantial")]
            Circumstantial,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct BackForwardCacheNotRestoredExplanation {
            #[serde(rename = "type")]
            r#type: Option<BackForwardCacheNotRestoredReasonType>,

            reason: Option<BackForwardCacheNotRestoredReason>,

            context: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct BackForwardCacheNotRestoredExplanationTree {
            url: Option<String>,

            explanations: Option<Vec<BackForwardCacheNotRestoredExplanation>>,

            children: Option<Vec<BackForwardCacheNotRestoredExplanationTree>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct AddScriptToEvaluateOnLoadRequest {
            #[serde(rename = "scriptSource")]
            script_source: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct AddScriptToEvaluateOnLoadResponse {
            identifier: Option<ScriptIdentifier>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct AddScriptToEvaluateOnNewDocumentRequest {
            source: Option<String>,

            #[serde(rename = "worldName")]
            world_name: String,

            #[serde(rename = "includeCommandLineAPI")]
            include_command_line_api: bool,

            #[serde(rename = "runImmediately")]
            run_immediately: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct AddScriptToEvaluateOnNewDocumentResponse {
            identifier: Option<ScriptIdentifier>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum CaptureScreenshotRequestFormat {
            #[serde(rename = "jpeg")]
            Jpeg,

            #[serde(rename = "png")]
            Png,

            #[serde(rename = "webp")]
            Webp,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CaptureScreenshotRequest {
            format: CaptureScreenshotRequestFormat,

            quality: Integer,

            clip: Viewport,

            #[serde(rename = "fromSurface")]
            fromsurface: bool,

            #[serde(rename = "captureBeyondViewport")]
            capturebeyondviewport: bool,

            #[serde(rename = "optimizeForSpeed")]
            optimizeforspeed: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CaptureScreenshotResponse {
            data: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum CaptureSnapshotRequestFormat {
            #[serde(rename = "mhtml")]
            MHTML,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CaptureSnapshotRequest {
            format: CaptureScreenshotRequestFormat,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CaptureSnapshotResponse {
            data: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CreateIsolatedWorldRequest {
            #[serde(rename = "frameId")]
            frame_id: Option<FrameId>,

            #[serde(rename = "worldName")]
            world_name: String,

            #[serde(rename = "grantUniveralAccess")]
            grant_univeral_access: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CreateIsolatedWorldResponse {
            #[serde(rename = "executionContextId")]
            execution_context_id: Option<runtime::ExecutionContextId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct DeleteCookieRequest {
            #[serde(rename = "cookieName")]
            cookie_name: Option<String>,

            url: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetAppManifestResponse {
            url: Option<String>,

            errors: Option<Vec<AppManifestError>>,

            data: String,

            parsed: AppManifestParsedProperties,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetInstallabilityErrorsResponse {
            #[serde(rename = "installabilityErrors")]
            installability_errors: Option<Vec<InstallabilityError>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetManifestIconsResponse {
            #[serde(rename = "primaryIcon")]
            primary_icon: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetAppIdResponse {
            #[serde(rename = "appId")]
            app_id: String,

            #[serde(rename = "recommendedId")]
            recommended_id: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetAdScriptIdRequest {
            #[serde(rename = "frameId")]
            frame_id: Option<FrameId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetAdScriptIdResponse {
            #[serde(rename = "adScriptId")]
            ad_script_id: AdScriptId,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetCookiesResponse {
            cookies: Option<Vec<network::Cookie>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetFrameTreeResponse {
            #[serde(rename = "frameTree")]
            frame_tree: Option<FrameTree>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetLayoutMetricsResponse {
            #[serde(rename = "layoutViewport")]
            layout_viewport: Option<LayoutViewport>,

            #[serde(rename = "visualViewport")]
            visual_viewport: Option<VisualViewport>,

            #[serde(rename = "contentSize")]
            content_size: Option<dom::Rect>,

            #[serde(rename = "cssLayoutViewport")]
            css_layout_viewport: Option<LayoutViewport>,

            #[serde(rename = "cssVisualViewport")]
            css_visual_viewport: Option<VisualViewport>,

            #[serde(rename = "cssContentSize")]
            css_content_size: Option<dom::Rect>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetNavigationHistoryResponse {
            #[serde(rename = "currentIndex")]
            current_index: Option<Integer>,

            entries: Option<Vec<NavigationEntry>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetResourceContentRequest {
            #[serde(rename = "frameId")]
            frame_id: Option<FrameId>,

            url: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetResourceContentResponse {
            content: Option<String>,

            #[serde(rename = "base64Encoded")]
            base64encoded: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetResourceTreeResponse {
            #[serde(rename = "frameTree")]
            frame_tree: Option<FrameResourceTree>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct HandleJavaScriptDialogRequest {
            accept: Option<bool>,

            #[serde(rename = "promptText")]
            prompttext: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct NavigateRequest {
            url: Option<String>,

            referrer: String,

            #[serde(rename = "transitionType")]
            transition_type: TransitionType,

            #[serde(rename = "frameId")]
            frame_id: FrameId,

            #[serde(rename = "referrerPolicy")]
            referrer_policy: ReferrerPolicy,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct NavigateResponse {
            #[serde(rename = "frameId")]
            frame_id: Option<FrameId>,

            #[serde(rename = "loaderId")]
            loader_id: network::LoaderId,

            #[serde(rename = "errorText")]
            error_text: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct NavigateToHistoryEntryRequest {
            #[serde(rename = "entryId")]
            entry_id: Option<Integer>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum PrintToPDFRequestTransferMode {
            #[serde(rename = "ReturnAsBase64")]
            ReturnAsBase64,

            #[serde(rename = "ReturnAsStream")]
            ReturnAsStream,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct PrintToPDFRequest {
            landscape: bool,

            #[serde(rename = "displayHeaderFooter")]
            display_header_footer: bool,

            #[serde(rename = "printBackground")]
            print_background: bool,

            scale: f64,

            #[serde(rename = "paperWidth")]
            paper_width: f64,

            #[serde(rename = "paperHeight")]
            paper_height: f64,

            #[serde(rename = "marginTop")]
            margin_top: f64,

            #[serde(rename = "marginBottom")]
            margin_bottom: f64,

            #[serde(rename = "marginLeft")]
            margin_left: f64,

            #[serde(rename = "marginRight")]
            margin_right: f64,

            #[serde(rename = "pageRanges")]
            page_ranges: String,

            #[serde(rename = "headerTemplate")]
            header_template: String,

            #[serde(rename = "footerTemplate")]
            footer_template: String,

            #[serde(rename = "preferCSSPageSize")]
            prefer_csspage_size: bool,

            #[serde(rename = "transferMode")]
            transfer_mode: PrintToPDFRequestTransferMode,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct PrintToPDFResponse {
            data: Option<String>,

            stream: io::StreamHandle,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ReloadRequest {
            #[serde(rename = "ignoreCache")]
            ignore_cache: bool,

            #[serde(rename = "scriptToEvaluateOnLoad")]
            script_to_evaluate_on_load: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct RemoveScriptToEvaluateOnLoadRequest {
            identifier: Option<ScriptIdentifier>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct RemoveScriptToEvaluateOnNewDocumentRequest {
            identifier: Option<ScriptIdentifier>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ScreencastFrameAckRequest {
            #[serde(rename = "sessionId")]
            session_id: Option<Integer>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SearchInResourceRequest {
            #[serde(rename = "frameId")]
            frame_id: Option<FrameId>,

            url: Option<String>,

            query: Option<String>,

            #[serde(rename = "caseSensitive")]
            case_sensitive: bool,

            #[serde(rename = "isRegex")]
            is_regex: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SearchInResourceResponse {
            result: Option<Vec<debugger::SearchMatch>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetAdBlockingEnabledRequest {
            enabled: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetBypassCSPRequest {
            enabled: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetPermissionsPolicyStateRequest {
            #[serde(rename = "frameId")]
            frame_id: Option<FrameId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetPermissionsPolicyStateResponse {
            states: Option<Vec<PermissionsPolicyFeatureState>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetOriginTrialsRequest {
            #[serde(rename = "frameId")]
            frame_id: Option<FrameId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetOriginTrialsResponse {
            #[serde(rename = "originTrials")]
            origin_trials: Option<Vec<OriginTrial>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetDeviceMetricsOverrideRequest {
            width: Option<Integer>,

            height: Option<Integer>,

            #[serde(rename = "deviceScaleFactor")]
            device_scale_factor: Option<f64>,

            mobile: Option<bool>,

            scale: f64,

            #[serde(rename = "screenWidth")]
            screen_width: Integer,

            #[serde(rename = "screenHeight")]
            screen_height: Integer,

            #[serde(rename = "positionX")]
            position_x: Integer,

            #[serde(rename = "positionY")]
            position_y: Integer,

            #[serde(rename = "dontSetVisibleSize")]
            dont_set_visible_size: bool,

            #[serde(rename = "screenOrientation")]
            screen_orientation: emulation::ScreenOrientation,

            viewport: Viewport,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetDeviceOrientationOverrideRequest {
            alpha: Option<f64>,

            beta: Option<f64>,

            gamma: Option<f64>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetFontFamiliesRequest {
            #[serde(rename = "fontFamilies")]
            font_families: Option<FontFamilies>,

            #[serde(rename = "forScripts")]
            for_scripts: Vec<ScriptFontFamilies>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetFontSizesRequest {
            #[serde(rename = "fontSizes")]
            font_sizes: Option<FontSizes>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetDocumentContentRequest {
            #[serde(rename = "frameId")]
            frame_id: Option<FrameId>,

            html: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum SetDownloadBehaviorRequestBehavior {
            #[serde(rename = "deny")]
            Deny,

            #[serde(rename = "allow")]
            Allow,

            #[serde(rename = "default")]
            Default,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetDownloadBehaviorRequest {
            behavior: Option<SetDownloadBehaviorRequestBehavior>,

            #[serde(rename = "downloadPath")]
            downloadpath: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetGeolocationOverrideRequest {
            latitude: f64,

            longitude: f64,

            accuracy: f64,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetLifecycleEventsEnabledRequest {
            enabled: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum SetTouchEmulationEnabledRequestConfiguration {
            #[serde(rename = "mobile")]
            Mobile,

            #[serde(rename = "desktop")]
            Desktop,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetTouchEmulationEnabledRequest {
            enabled: Option<bool>,

            configuration: SetTouchEmulationEnabledRequestConfiguration,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum StartScreencastRequestFormat {
            #[serde(rename = "jpeg")]
            Jpeg,

            #[serde(rename = "png")]
            Png,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct StartScreencastRequest {
            format: StartScreencastRequestFormat,

            quality: Integer,

            #[serde(rename = "maxWidth")]
            max_width: Integer,

            #[serde(rename = "maxHeight")]
            max_height: Integer,

            #[serde(rename = "everyNthFrame")]
            every_nth_frame: Integer,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum SetWebLifecycleStateRequestState {
            #[serde(rename = "frozen")]
            Frozen,

            #[serde(rename = "active")]
            Active,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetWebLifecycleStateRequest {
            state: Option<SetWebLifecycleStateRequestState>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ProduceCompilationCacheRequest {
            scripts: Option<Vec<CompilationCacheParams>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct AddCompilationCacheRequest {
            url: Option<String>,

            data: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetSPCTransactionModeRequest {
            mode: Option<AutoResponseMode>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetRPHRegistrationModeRequest {
            mode: Option<AutoResponseMode>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GenerateTestReportRequest {
            message: Option<String>,

            group: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetInterceptFileChooserDialogRequest {
            enabled: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetPrerenderingAllowedRequest {
            #[serde(rename = "isAllowed")]
            is_allowed: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct DomContentEventFiredEvent {
            timestamp: Option<network::MonotonicTime>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum FileChooserOpenedEventMode {
            #[serde(rename = "selectSingle")]
            SelectSingle,

            #[serde(rename = "selectMultiple")]
            SelectMultiple,
        }

        /**
         * Emitted only when `page.interceptFileChooser` is enabled.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FileChooserOpenedEvent {
            /**
             * Emitted only when `page.interceptFileChooser` is enabled.
            */
            #[serde(rename = "frameId")]
            frame_id: Option<FrameId>,
            /**
             * Emitted only when `page.interceptFileChooser` is enabled.
            */
            mode: Option<FileChooserOpenedEventMode>,
            /**
             * Emitted only when `page.interceptFileChooser` is enabled.
            */
            #[serde(rename = "backendNodeId")]
            backend_node_id: dom::BackendNodeId,
        }

        /**
         * Fired when frame has been attached to its parent.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FrameAttachedEvent {
            /**
             * Fired when frame has been attached to its parent.
            */
            #[serde(rename = "frameId")]
            frame_id: Option<FrameId>,
            /**
             * Fired when frame has been attached to its parent.
            */
            #[serde(rename = "parentFrameId")]
            parent_frame_id: Option<FrameId>,
            /**
             * Fired when frame has been attached to its parent.
            */
            stack: runtime::StackTrace,
        }

        /**
         * Fired when frame no longer has a scheduled navigation.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FrameClearedScheduledNavigationEvent {
            /**
             * Fired when frame no longer has a scheduled navigation.
            */
            #[serde(rename = "frameId")]
            frame_id: Option<FrameId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum FrameDetachedEventReason {
            #[serde(rename = "remove")]
            Remove,

            #[serde(rename = "swap")]
            Swap,
        }

        /**
         * Fired when frame has been detached from its parent.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FrameDetachedEvent {
            /**
             * Fired when frame has been detached from its parent.
            */
            #[serde(rename = "frameId")]
            frame_id: Option<FrameId>,
            /**
             * Fired when frame has been detached from its parent.
            */
            reason: Option<FrameDetachedEventReason>,
        }

        /**
         * Fired once navigation of the frame has completed. Frame is now associated with the new loader.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FrameNavigatedEvent {
            /**
             * Fired once navigation of the frame has completed. Frame is now associated with the new loader.
            */
            frame: Option<Frame>,
            /**
             * Fired once navigation of the frame has completed. Frame is now associated with the new loader.
            */
            #[serde(rename = "type")]
            r#type: Option<NavigationType>,
        }

        /**
         * Fired when opening document to write to.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct DocumentOpenedEvent {
            /**
             * Fired when opening document to write to.
            */
            frame: Option<Frame>,
        }

        /**
         * Fired when a renderer-initiated navigation is requested.
         * Navigation may still be cancelled after the event is issued.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FrameRequestedNavigationEvent {
            /**
             * Fired when a renderer-initiated navigation is requested.
             * Navigation may still be cancelled after the event is issued.
            */
            #[serde(rename = "frameId")]
            frameid: Option<FrameId>,
            /**
             * Fired when a renderer-initiated navigation is requested.
             * Navigation may still be cancelled after the event is issued.
            */
            reason: Option<ClientNavigationReason>,
            /**
             * Fired when a renderer-initiated navigation is requested.
             * Navigation may still be cancelled after the event is issued.
            */
            url: Option<String>,
            /**
             * Fired when a renderer-initiated navigation is requested.
             * Navigation may still be cancelled after the event is issued.
            */
            disposition: Option<ClientNavigationDisposition>,
        }

        /**
         * Fired when frame schedules a potential navigation.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FrameScheduledNavigationEvent {
            /**
             * Fired when frame schedules a potential navigation.
            */
            #[serde(rename = "frameId")]
            frameid: Option<FrameId>,
            /**
             * Fired when frame schedules a potential navigation.
            */
            delay: Option<f64>,
            /**
             * Fired when frame schedules a potential navigation.
            */
            reason: Option<ClientNavigationReason>,
            /**
             * Fired when frame schedules a potential navigation.
            */
            url: Option<String>,
        }

        /**
         * Fired when frame has started loading.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FrameStartedLoadingEvent {
            /**
             * Fired when frame has started loading.
            */
            #[serde(rename = "frameId")]
            frame_id: Option<FrameId>,
        }

        /**
         * Fired when frame has stopped loading.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FrameStoppedLoadingEvent {
            /**
             * Fired when frame has stopped loading.
            */
            #[serde(rename = "frameId")]
            frame_id: Option<FrameId>,
        }

        /**
         * Fired when page is about to start a download.
         * Deprecated. Use Browser.downloadWillBegin instead.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct DownloadWillBeginEvent {
            /**
             * Fired when page is about to start a download.
             * Deprecated. Use Browser.downloadWillBegin instead.
            */
            #[serde(rename = "frameId")]
            frame_id: Option<FrameId>,
            /**
             * Fired when page is about to start a download.
             * Deprecated. Use Browser.downloadWillBegin instead.
            */
            guid: Option<String>,
            /**
             * Fired when page is about to start a download.
             * Deprecated. Use Browser.downloadWillBegin instead.
            */
            url: Option<String>,
            /**
             * Fired when page is about to start a download.
             * Deprecated. Use Browser.downloadWillBegin instead.
            */
            #[serde(rename = "suggestedFilename")]
            suggested_filename: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum DownloadProgressEventState {
            #[serde(rename = "inProgress")]
            InProgress,

            #[serde(rename = "completed")]
            Completed,

            #[serde(rename = "canceled")]
            Canceled,
        }

        /**
         * Fired when download makes progress. Last call has |done| == true.
         * Deprecated. Use Browser.downloadProgress instead.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct DownloadProgressEvent {
            /**
             * Fired when download makes progress. Last call has |done| == true.
             * Deprecated. Use Browser.downloadProgress instead.
            */
            guid: Option<String>,
            /**
             * Fired when download makes progress. Last call has |done| == true.
             * Deprecated. Use Browser.downloadProgress instead.
            */
            #[serde(rename = "totalBytes")]
            totalbytes: Option<f64>,
            /**
             * Fired when download makes progress. Last call has |done| == true.
             * Deprecated. Use Browser.downloadProgress instead.
            */
            #[serde(rename = "receivedBytes")]
            receivedbytes: Option<f64>,
            /**
             * Fired when download makes progress. Last call has |done| == true.
             * Deprecated. Use Browser.downloadProgress instead.
            */
            state: Option<DownloadProgressEventState>,
        }

        /**
         * Fired when a JavaScript initiated dialog (alert, confirm, prompt, or onbeforeunload) has been
         * closed.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct JavascriptDialogClosedEvent {
            /**
             * Fired when a JavaScript initiated dialog (alert, confirm, prompt, or onbeforeunload) has been
             * closed.
            */
            result: Option<bool>,
            /**
             * Fired when a JavaScript initiated dialog (alert, confirm, prompt, or onbeforeunload) has been
             * closed.
            */
            #[serde(rename = "userInput")]
            userinput: Option<String>,
        }

        /**
         * Fired when a JavaScript initiated dialog (alert, confirm, prompt, or onbeforeunload) is about to
         * open.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct JavascriptDialogOpeningEvent {
            /**
             * Fired when a JavaScript initiated dialog (alert, confirm, prompt, or onbeforeunload) is about to
             * open.
            */
            url: Option<String>,
            /**
             * Fired when a JavaScript initiated dialog (alert, confirm, prompt, or onbeforeunload) is about to
             * open.
            */
            message: Option<String>,
            /**
             * Fired when a JavaScript initiated dialog (alert, confirm, prompt, or onbeforeunload) is about to
             * open.
            */
            #[serde(rename = "type")]
            r#type: Option<DialogType>,
            /**
             * Fired when a JavaScript initiated dialog (alert, confirm, prompt, or onbeforeunload) is about to
             * open.
            */
            #[serde(rename = "hasBrowserHandler")]
            hasbrowserhandler: Option<bool>,
            /**
             * Fired when a JavaScript initiated dialog (alert, confirm, prompt, or onbeforeunload) is about to
             * open.
            */
            #[serde(rename = "defaultPrompt")]
            defaultprompt: String,
        }

        /**
         * Fired for top level page lifecycle events such as navigation, load, paint, etc.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct LifecycleEventEvent {
            /**
             * Fired for top level page lifecycle events such as navigation, load, paint, etc.
            */
            #[serde(rename = "frameId")]
            frame_id: Option<FrameId>,
            /**
             * Fired for top level page lifecycle events such as navigation, load, paint, etc.
            */
            #[serde(rename = "loaderId")]
            loader_id: Option<network::LoaderId>,
            /**
             * Fired for top level page lifecycle events such as navigation, load, paint, etc.
            */
            name: Option<String>,
            /**
             * Fired for top level page lifecycle events such as navigation, load, paint, etc.
            */
            timestamp: Option<network::MonotonicTime>,
        }

        /**
         * Fired for failed bfcache history navigations if BackForwardCache feature is enabled. Do
         * not assume any ordering with the Page.frameNavigated event. This event is fired only for
         * main-frame history navigation where the document changes (non-same-document navigations),
         * when bfcache navigation fails.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct BackForwardCacheNotUsedEvent {
            /**
             * Fired for failed bfcache history navigations if BackForwardCache feature is enabled. Do
             * not assume any ordering with the Page.frameNavigated event. This event is fired only for
             * main-frame history navigation where the document changes (non-same-document navigations),
             * when bfcache navigation fails.
            */
            #[serde(rename = "loaderId")]
            loader_id: Option<network::LoaderId>,
            /**
             * Fired for failed bfcache history navigations if BackForwardCache feature is enabled. Do
             * not assume any ordering with the Page.frameNavigated event. This event is fired only for
             * main-frame history navigation where the document changes (non-same-document navigations),
             * when bfcache navigation fails.
            */
            #[serde(rename = "frameId")]
            frame_id: Option<FrameId>,
            /**
             * Fired for failed bfcache history navigations if BackForwardCache feature is enabled. Do
             * not assume any ordering with the Page.frameNavigated event. This event is fired only for
             * main-frame history navigation where the document changes (non-same-document navigations),
             * when bfcache navigation fails.
            */
            #[serde(rename = "notRestoredExplanations")]
            not_restored_explanations: Option<Vec<BackForwardCacheNotRestoredExplanation>>,
            /**
             * Fired for failed bfcache history navigations if BackForwardCache feature is enabled. Do
             * not assume any ordering with the Page.frameNavigated event. This event is fired only for
             * main-frame history navigation where the document changes (non-same-document navigations),
             * when bfcache navigation fails.
            */
            #[serde(rename = "notRestoredExplanationsTree")]
            not_restored_explanations_tree: BackForwardCacheNotRestoredExplanationTree,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct LoadEventFiredEvent {
            timestamp: Option<network::MonotonicTime>,
        }

        /**
         * Fired when same-document navigation happens, e.g. due to history API usage or anchor navigation.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct NavigatedWithinDocumentEvent {
            /**
             * Fired when same-document navigation happens, e.g. due to history API usage or anchor navigation.
            */
            #[serde(rename = "frameId")]
            frame_id: Option<FrameId>,
            /**
             * Fired when same-document navigation happens, e.g. due to history API usage or anchor navigation.
            */
            url: Option<String>,
        }

        /**
         * Compressed image data requested by the `startScreencast`.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ScreencastFrameEvent {
            /**
             * Compressed image data requested by the `startScreencast`.
            */
            data: Option<String>,
            /**
             * Compressed image data requested by the `startScreencast`.
            */
            metadata: Option<ScreencastFrameMetadata>,
            /**
             * Compressed image data requested by the `startScreencast`.
            */
            #[serde(rename = "sessionId")]
            sessionid: Option<Integer>,
        }

        /**
         * Fired when the page with currently enabled screencast was shown or hidden `.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ScreencastVisibilityChangedEvent {
            /**
             * Fired when the page with currently enabled screencast was shown or hidden `.
            */
            visible: Option<bool>,
        }

        /**
         * Fired when a new window is going to be opened, via window.open(), link click, form submission,
         * etc.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct WindowOpenEvent {
            /**
             * Fired when a new window is going to be opened, via window.open(), link click, form submission,
             * etc.
            */
            url: Option<String>,
            /**
             * Fired when a new window is going to be opened, via window.open(), link click, form submission,
             * etc.
            */
            #[serde(rename = "windowName")]
            window_name: Option<String>,
            /**
             * Fired when a new window is going to be opened, via window.open(), link click, form submission,
             * etc.
            */
            #[serde(rename = "windowFeatures")]
            window_features: Option<Vec<String>>,
            /**
             * Fired when a new window is going to be opened, via window.open(), link click, form submission,
             * etc.
            */
            #[serde(rename = "userGesture")]
            user_gesture: Option<bool>,
        }

        /**
         * Issued for every compilation cache generated. Is only available
         * if Page.setGenerateCompilationCache is enabled.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct CompilationCacheProducedEvent {
            /**
             * Issued for every compilation cache generated. Is only available
             * if Page.setGenerateCompilationCache is enabled.
            */
            url: Option<String>,
            /**
             * Issued for every compilation cache generated. Is only available
             * if Page.setGenerateCompilationCache is enabled.
            */
            data: Option<String>,
        }
    }

    pub mod performance {
        use serde::{self, Deserialize, Serialize};
        /**
         * Run-time execution metric.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Metric {
            /**
             * Run-time execution metric.
            */
            name: Option<String>,
            /**
             * Run-time execution metric.
            */
            value: Option<f64>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum EnableRequestTimeDomain {
            #[serde(rename = "timeTicks")]
            TimeTicks,

            #[serde(rename = "threadTicks")]
            ThreadTicks,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct EnableRequest {
            #[serde(rename = "timeDomain")]
            time_domain: EnableRequestTimeDomain,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum SetTimeDomainRequestTimeDomain {
            #[serde(rename = "timeTicks")]
            TimeTicks,

            #[serde(rename = "threadTicks")]
            ThreadTicks,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetTimeDomainRequest {
            #[serde(rename = "timeDomain")]
            time_domain: Option<SetTimeDomainRequestTimeDomain>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetMetricsResponse {
            metrics: Option<Vec<Metric>>,
        }

        /**
         * Current values of the metrics.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct MetricsEvent {
            /**
             * Current values of the metrics.
            */
            metrics: Option<Vec<Metric>>,
            /**
             * Current values of the metrics.
            */
            title: Option<String>,
        }
    }

    /**
     * Reporting of performance timeline events, as specified in
     * https://w3c.github.io/performance-timeline/#dom-performanceobserver.
    */
    pub mod performance_timeline {
        use super::{dom, network, page,};
        use serde::{self, Deserialize, Serialize};
        /**
         * See https://github.com/WICG/LargestContentfulPaint and largest_contentful_paint.idl
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct LargestContentfulPaint {
            /**
             * See https://github.com/WICG/LargestContentfulPaint and largest_contentful_paint.idl
            */
            #[serde(rename = "renderTime")]
            render_time: Option<network::TimeSinceEpoch>,
            /**
             * See https://github.com/WICG/LargestContentfulPaint and largest_contentful_paint.idl
            */
            #[serde(rename = "loadTime")]
            load_time: Option<network::TimeSinceEpoch>,
            /**
             * See https://github.com/WICG/LargestContentfulPaint and largest_contentful_paint.idl
            */
            size: Option<f64>,
            /**
             * See https://github.com/WICG/LargestContentfulPaint and largest_contentful_paint.idl
            */
            #[serde(rename = "elementId")]
            element_id: String,
            /**
             * See https://github.com/WICG/LargestContentfulPaint and largest_contentful_paint.idl
            */
            url: String,
            /**
             * See https://github.com/WICG/LargestContentfulPaint and largest_contentful_paint.idl
            */
            #[serde(rename = "nodeId")]
            node_id: dom::BackendNodeId,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct LayoutShiftAttribution {
            #[serde(rename = "previousRect")]
            previous_rect: Option<dom::Rect>,

            #[serde(rename = "currentRect")]
            current_rect: Option<dom::Rect>,

            #[serde(rename = "nodeId")]
            node_id: dom::BackendNodeId,
        }

        /**
         * See https://wicg.github.io/layout-instability/#sec-layout-shift and layout_shift.idl
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct LayoutShift {
            /**
             * See https://wicg.github.io/layout-instability/#sec-layout-shift and layout_shift.idl
            */
            value: Option<f64>,
            /**
             * See https://wicg.github.io/layout-instability/#sec-layout-shift and layout_shift.idl
            */
            #[serde(rename = "hadRecentInput")]
            hadrecentinput: Option<bool>,
            /**
             * See https://wicg.github.io/layout-instability/#sec-layout-shift and layout_shift.idl
            */
            #[serde(rename = "lastInputTime")]
            lastinputtime: Option<network::TimeSinceEpoch>,
            /**
             * See https://wicg.github.io/layout-instability/#sec-layout-shift and layout_shift.idl
            */
            sources: Option<Vec<LayoutShiftAttribution>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct TimelineEvent {
            #[serde(rename = "frameId")]
            frameid: Option<page::FrameId>,

            #[serde(rename = "type")]
            r#type: Option<String>,

            name: Option<String>,

            time: Option<network::TimeSinceEpoch>,

            duration: f64,

            #[serde(rename = "lcpDetails")]
            lcpdetails: LargestContentfulPaint,

            #[serde(rename = "layoutShiftDetails")]
            layoutshiftdetails: LayoutShift,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct EnableRequest {
            #[serde(rename = "eventTypes")]
            event_types: Option<Vec<String>>,
        }

        /**
         * Sent when a performance timeline event is added. See reportPerformanceTimeline method.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct TimelineEventAddedEvent {
            /**
             * Sent when a performance timeline event is added. See reportPerformanceTimeline method.
            */
            event: Option<TimelineEvent>,
        }
    }

    /**
     * Security
    */
    pub mod security {
        use super::{network, Integer};
        use serde::{self, Deserialize, Serialize};
        /**
         * An internal certificate ID value.
        */
        pub type CertificateId = Integer;

        /**
         * A description of mixed content (HTTP resources on HTTPS pages), as defined by
         * https://www.w3.org/TR/mixed-content/#categories
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum MixedContentType {
            #[serde(rename = "blockable")]
            Blockable,
            #[serde(rename = "optionally-blockable")]
            OptionallyBlockable,
            #[serde(rename = "none")]
            None,
        }

        /**
         * The security level of a page or resource.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum SecurityState {
            #[serde(rename = "unknown")]
            Unknown,
            #[serde(rename = "neutral")]
            Neutral,
            #[serde(rename = "insecure")]
            Insecure,
            #[serde(rename = "secure")]
            Secure,
            #[serde(rename = "info")]
            Info,
            #[serde(rename = "insecure-broken")]
            InsecureBroken,
        }

        /**
         * Details about the security state of the page certificate.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct CertificateSecurityState {
            /**
             * Details about the security state of the page certificate.
            */
            protocol: Option<String>,
            /**
             * Details about the security state of the page certificate.
            */
            #[serde(rename = "keyExchange")]
            key_exchange: Option<String>,
            /**
             * Details about the security state of the page certificate.
            */
            #[serde(rename = "keyExchangeGroup")]
            key_exchange_group: String,
            /**
             * Details about the security state of the page certificate.
            */
            cipher: Option<String>,
            /**
             * Details about the security state of the page certificate.
            */
            mac: String,
            /**
             * Details about the security state of the page certificate.
            */
            certificate: Option<Vec<String>>,
            /**
             * Details about the security state of the page certificate.
            */
            #[serde(rename = "subjectName")]
            subject_name: Option<String>,
            /**
             * Details about the security state of the page certificate.
            */
            issuer: Option<String>,
            /**
             * Details about the security state of the page certificate.
            */
            #[serde(rename = "validFrom")]
            valid_from: Option<network::TimeSinceEpoch>,
            /**
             * Details about the security state of the page certificate.
            */
            #[serde(rename = "validTo")]
            valid_to: Option<network::TimeSinceEpoch>,
            /**
             * Details about the security state of the page certificate.
            */
            #[serde(rename = "certificateNetworkError")]
            certificate_network_error: String,
            /**
             * Details about the security state of the page certificate.
            */
            #[serde(rename = "certificateHasWeakSignature")]
            certificate_has_weak_signature: Option<bool>,
            /**
             * Details about the security state of the page certificate.
            */
            #[serde(rename = "certificateHasSha1Signature")]
            certificate_has_sha1signature: Option<bool>,
            /**
             * Details about the security state of the page certificate.
            */
            #[serde(rename = "modernSSL")]
            modern_ssl: Option<bool>,
            /**
             * Details about the security state of the page certificate.
            */
            #[serde(rename = "obsoleteSslProtocol")]
            obsolete_ssl_protocol: Option<bool>,
            /**
             * Details about the security state of the page certificate.
            */
            #[serde(rename = "obsoleteSslKeyExchange")]
            obsolete_ssl_key_exchange: Option<bool>,
            /**
             * Details about the security state of the page certificate.
            */
            #[serde(rename = "obsoleteSslCipher")]
            obsolete_ssl_cipher: Option<bool>,
            /**
             * Details about the security state of the page certificate.
            */
            #[serde(rename = "obsoleteSslSignature")]
            obsolete_ssl_signature: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum SafetyTipStatus {
            #[serde(rename = "badReputation")]
            BadReputation,
            #[serde(rename = "lookalike")]
            Lookalike,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SafetyTipInfo {
            #[serde(rename = "safetyTipStatus")]
            safety_tip_status: Option<SafetyTipStatus>,

            #[serde(rename = "safeUrl")]
            safe_url: String,
        }

        /**
         * Security state information about the page.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct VisibleSecurityState {
            /**
             * Security state information about the page.
            */
            #[serde(rename = "securityState")]
            security_state: Option<SecurityState>,
            /**
             * Security state information about the page.
            */
            #[serde(rename = "certificateSecurityState")]
            certificate_security_state: CertificateSecurityState,
            /**
             * Security state information about the page.
            */
            #[serde(rename = "safetyTipInfo")]
            safety_tip_info: SafetyTipInfo,
            /**
             * Security state information about the page.
            */
            #[serde(rename = "securityStateIssueIds")]
            security_state_issue_ids: Option<Vec<String>>,
        }

        /**
         * An explanation of an factor contributing to the security state.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SecurityStateExplanation {
            /**
             * An explanation of an factor contributing to the security state.
            */
            #[serde(rename = "securityState")]
            securitystate: Option<SecurityState>,
            /**
             * An explanation of an factor contributing to the security state.
            */
            title: Option<String>,
            /**
             * An explanation of an factor contributing to the security state.
            */
            summary: Option<String>,
            /**
             * An explanation of an factor contributing to the security state.
            */
            description: Option<String>,
            /**
             * An explanation of an factor contributing to the security state.
            */
            #[serde(rename = "mixedContentType")]
            mixedcontenttype: Option<MixedContentType>,
            /**
             * An explanation of an factor contributing to the security state.
            */
            certificate: Option<Vec<String>>,
            /**
             * An explanation of an factor contributing to the security state.
            */
            recommendations: Vec<String>,
        }

        /**
         * Information about insecure content on the page.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InsecureContentStatus {
            /**
             * Information about insecure content on the page.
            */
            #[serde(rename = "ranMixedContent")]
            ran_mixed_content: Option<bool>,
            /**
             * Information about insecure content on the page.
            */
            #[serde(rename = "displayedMixedContent")]
            displayed_mixed_content: Option<bool>,
            /**
             * Information about insecure content on the page.
            */
            #[serde(rename = "containedMixedForm")]
            contained_mixed_form: Option<bool>,
            /**
             * Information about insecure content on the page.
            */
            #[serde(rename = "ranContentWithCertErrors")]
            ran_content_with_cert_errors: Option<bool>,
            /**
             * Information about insecure content on the page.
            */
            #[serde(rename = "displayedContentWithCertErrors")]
            displayed_content_with_cert_errors: Option<bool>,
            /**
             * Information about insecure content on the page.
            */
            #[serde(rename = "ranInsecureContentStyle")]
            ran_insecure_content_style: Option<SecurityState>,
            /**
             * Information about insecure content on the page.
            */
            #[serde(rename = "displayedInsecureContentStyle")]
            displayed_insecure_content_style: Option<SecurityState>,
        }

        /**
         * The action to take when a certificate error occurs. continue will continue processing the
         * request and cancel will cancel the request.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum CertificateErrorAction {
            #[serde(rename = "continue")]
            Continue,
            #[serde(rename = "cancel")]
            Cancel,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetIgnoreCertificateErrorsRequest {
            ignore: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct HandleCertificateErrorRequest {
            #[serde(rename = "eventId")]
            event_id: Option<Integer>,

            action: Option<CertificateErrorAction>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetOverrideCertificateErrorsRequest {
            #[serde(rename = "override")]
            r#override: Option<bool>,
        }

        /**
         * There is a certificate error. If overriding certificate errors is enabled, then it should be
         * handled with the `handleCertificateError` command. Note: this event does not fire if the
         * certificate error has been allowed internally. Only one client per target should override
         * certificate errors at the same time.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct CertificateErrorEvent {
            /**
             * There is a certificate error. If overriding certificate errors is enabled, then it should be
             * handled with the `handleCertificateError` command. Note: this event does not fire if the
             * certificate error has been allowed internally. Only one client per target should override
             * certificate errors at the same time.
            */
            #[serde(rename = "eventId")]
            event_id: Option<Integer>,
            /**
             * There is a certificate error. If overriding certificate errors is enabled, then it should be
             * handled with the `handleCertificateError` command. Note: this event does not fire if the
             * certificate error has been allowed internally. Only one client per target should override
             * certificate errors at the same time.
            */
            #[serde(rename = "errorType")]
            error_type: Option<String>,
            /**
             * There is a certificate error. If overriding certificate errors is enabled, then it should be
             * handled with the `handleCertificateError` command. Note: this event does not fire if the
             * certificate error has been allowed internally. Only one client per target should override
             * certificate errors at the same time.
            */
            #[serde(rename = "requestURL")]
            request_url: Option<String>,
        }

        /**
         * The security state of the page changed.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct VisibleSecurityStateChangedEvent {
            /**
             * The security state of the page changed.
            */
            #[serde(rename = "visibleSecurityState")]
            visible_security_state: Option<VisibleSecurityState>,
        }

        /**
         * The security state of the page changed. No longer being sent.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SecurityStateChangedEvent {
            /**
             * The security state of the page changed. No longer being sent.
            */
            #[serde(rename = "securityState")]
            security_state: Option<SecurityState>,
            /**
             * The security state of the page changed. No longer being sent.
            */
            #[serde(rename = "schemeIsCryptographic")]
            scheme_is_cryptographic: Option<bool>,
            /**
             * The security state of the page changed. No longer being sent.
            */
            explanations: Option<Vec<SecurityStateExplanation>>,
            /**
             * The security state of the page changed. No longer being sent.
            */
            #[serde(rename = "insecureContentStatus")]
            insecure_content_status: Option<InsecureContentStatus>,
            /**
             * The security state of the page changed. No longer being sent.
            */
            summary: String,
        }
    }

    pub mod service_worker {
        use super::{target, Integer};
        use serde::{self, Deserialize, Serialize};

        pub type RegistrationID = String;

        /**
         * ServiceWorker registration.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ServiceWorkerRegistration {
            /**
             * ServiceWorker registration.
            */
            #[serde(rename = "registrationId")]
            registration_id: Option<RegistrationID>,
            /**
             * ServiceWorker registration.
            */
            #[serde(rename = "scopeURL")]
            scope_url: Option<String>,
            /**
             * ServiceWorker registration.
            */
            #[serde(rename = "isDeleted")]
            is_deleted: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum ServiceWorkerVersionRunningStatus {
            #[serde(rename = "stopped")]
            Stopped,
            #[serde(rename = "starting")]
            Starting,
            #[serde(rename = "running")]
            Running,
            #[serde(rename = "stopping")]
            Stopping,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum ServiceWorkerVersionStatus {
            #[serde(rename = "new")]
            New,
            #[serde(rename = "installing")]
            Installing,
            #[serde(rename = "installed")]
            Installed,
            #[serde(rename = "activating")]
            Activating,
            #[serde(rename = "activated")]
            Activated,
            #[serde(rename = "redundant")]
            Redundant,
        }

        /**
         * ServiceWorker version.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ServiceWorkerVersion {
            /**
             * ServiceWorker version.
            */
            #[serde(rename = "versionId")]
            version_id: Option<String>,
            /**
             * ServiceWorker version.
            */
            #[serde(rename = "registrationId")]
            registration_id: Option<RegistrationID>,
            /**
             * ServiceWorker version.
            */
            #[serde(rename = "scriptURL")]
            script_url: Option<String>,
            /**
             * ServiceWorker version.
            */
            #[serde(rename = "runningStatus")]
            running_status: Option<ServiceWorkerVersionRunningStatus>,
            /**
             * ServiceWorker version.
            */
            status: Option<ServiceWorkerVersionStatus>,
            /**
             * ServiceWorker version.
            */
            #[serde(rename = "scriptLastModified")]
            script_last_modified: f64,
            /**
             * ServiceWorker version.
            */
            #[serde(rename = "scriptResponseTime")]
            script_response_time: f64,
            /**
             * ServiceWorker version.
            */
            #[serde(rename = "controlledClients")]
            controlled_clients: Vec<target::TargetID>,
            /**
             * ServiceWorker version.
            */
            #[serde(rename = "targetId")]
            target_id: target::TargetID,
        }

        /**
         * ServiceWorker error message.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ServiceWorkerErrorMessage {
            /**
             * ServiceWorker error message.
            */
            #[serde(rename = "errorMessage")]
            error_message: Option<String>,
            /**
             * ServiceWorker error message.
            */
            #[serde(rename = "registrationId")]
            registration_id: Option<RegistrationID>,
            /**
             * ServiceWorker error message.
            */
            #[serde(rename = "versionId")]
            version_id: Option<String>,
            /**
             * ServiceWorker error message.
            */
            #[serde(rename = "sourceURL")]
            source_url: Option<String>,
            /**
             * ServiceWorker error message.
            */
            #[serde(rename = "lineNumber")]
            line_number: Option<Integer>,
            /**
             * ServiceWorker error message.
            */
            #[serde(rename = "columnNumber")]
            column_number: Option<Integer>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct DeliverPushMessageRequest {
            origin: Option<String>,

            #[serde(rename = "registrationId")]
            registrationid: Option<RegistrationID>,

            data: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct DispatchSyncEventRequest {
            origin: Option<String>,

            #[serde(rename = "registrationId")]
            registrationid: Option<RegistrationID>,

            tag: Option<String>,

            #[serde(rename = "lastChance")]
            lastchance: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct DispatchPeriodicSyncEventRequest {
            origin: Option<String>,

            #[serde(rename = "registrationId")]
            registrationid: Option<RegistrationID>,

            tag: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct InspectWorkerRequest {
            #[serde(rename = "versionId")]
            version_id: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetForceUpdateOnPageLoadRequest {
            #[serde(rename = "forceUpdateOnPageLoad")]
            force_update_on_page_load: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SkipWaitingRequest {
            #[serde(rename = "scopeURL")]
            scope_url: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct StartWorkerRequest {
            #[serde(rename = "scopeURL")]
            scope_url: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct StopWorkerRequest {
            #[serde(rename = "versionId")]
            version_id: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct UnregisterRequest {
            #[serde(rename = "scopeURL")]
            scope_url: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct UpdateRegistrationRequest {
            #[serde(rename = "scopeURL")]
            scope_url: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct WorkerErrorReportedEvent {
            #[serde(rename = "errorMessage")]
            error_message: Option<ServiceWorkerErrorMessage>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct WorkerRegistrationUpdatedEvent {
            registrations: Option<Vec<ServiceWorkerRegistration>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct WorkerVersionUpdatedEvent {
            versions: Option<Vec<ServiceWorkerVersion>>,
        }
    }

    pub mod storage {
        use super::{browser, network, page, Integer};
        use serde::{self, Deserialize, Serialize};

        pub type SerializedStorageKey = String;

        /**
         * Enum of possible storage types.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum StorageType {
            #[serde(rename = "appcache")]
            Appcache,
            #[serde(rename = "cookies")]
            Cookies,
            #[serde(rename = "file_systems")]
            FileSystems,
            #[serde(rename = "indexeddb")]
            Indexeddb,
            #[serde(rename = "local_storage")]
            LocalStorage,
            #[serde(rename = "shader_cache")]
            ShaderCache,
            #[serde(rename = "websql")]
            Websql,
            #[serde(rename = "service_workers")]
            ServiceWorkers,
            #[serde(rename = "cache_storage")]
            CacheStorage,
            #[serde(rename = "interest_groups")]
            InterestGroups,
            #[serde(rename = "shared_storage")]
            SharedStorage,
            #[serde(rename = "storage_buckets")]
            StorageBuckets,
            #[serde(rename = "all")]
            All,
            #[serde(rename = "other")]
            Other,
        }

        /**
         * Usage for a storage type.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct UsageForType {
            /**
             * Usage for a storage type.
            */
            #[serde(rename = "storageType")]
            storage_type: Option<StorageType>,
            /**
             * Usage for a storage type.
            */
            usage: Option<f64>,
        }

        /**
         * Pair of issuer origin and number of available (signed, but not used) Trust
         * Tokens from that issuer.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct TrustTokens {
            /**
             * Pair of issuer origin and number of available (signed, but not used) Trust
             * Tokens from that issuer.
            */
            #[serde(rename = "issuerOrigin")]
            issuer_origin: Option<String>,
            /**
             * Pair of issuer origin and number of available (signed, but not used) Trust
             * Tokens from that issuer.
            */
            count: Option<f64>,
        }

        /**
         * Enum of interest group access types.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum InterestGroupAccessType {
            #[serde(rename = "join")]
            Join,
            #[serde(rename = "leave")]
            Leave,
            #[serde(rename = "update")]
            Update,
            #[serde(rename = "loaded")]
            Loaded,
            #[serde(rename = "bid")]
            Bid,
            #[serde(rename = "win")]
            Win,
        }

        /**
         * Ad advertising element inside an interest group.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InterestGroupAd {
            /**
             * Ad advertising element inside an interest group.
            */
            #[serde(rename = "renderUrl")]
            render_url: Option<String>,
            /**
             * Ad advertising element inside an interest group.
            */
            metadata: String,
        }

        /**
         * The full details of an interest group.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InterestGroupDetails {
            /**
             * The full details of an interest group.
            */
            #[serde(rename = "ownerOrigin")]
            owner_origin: Option<String>,
            /**
             * The full details of an interest group.
            */
            name: Option<String>,
            /**
             * The full details of an interest group.
            */
            #[serde(rename = "expirationTime")]
            expiration_time: Option<network::TimeSinceEpoch>,
            /**
             * The full details of an interest group.
            */
            #[serde(rename = "joiningOrigin")]
            joining_origin: Option<String>,
            /**
             * The full details of an interest group.
            */
            #[serde(rename = "biddingUrl")]
            bidding_url: String,
            /**
             * The full details of an interest group.
            */
            #[serde(rename = "biddingWasmHelperUrl")]
            bidding_wasm_helper_url: String,
            /**
             * The full details of an interest group.
            */
            #[serde(rename = "updateUrl")]
            update_url: String,
            /**
             * The full details of an interest group.
            */
            #[serde(rename = "trustedBiddingSignalsUrl")]
            trusted_bidding_signals_url: String,
            /**
             * The full details of an interest group.
            */
            #[serde(rename = "trustedBiddingSignalsKeys")]
            trusted_bidding_signals_keys: Option<Vec<String>>,
            /**
             * The full details of an interest group.
            */
            #[serde(rename = "userBiddingSignals")]
            user_bidding_signals: String,
            /**
             * The full details of an interest group.
            */
            ads: Option<Vec<InterestGroupAd>>,
            /**
             * The full details of an interest group.
            */
            #[serde(rename = "adComponents")]
            ad_components: Option<Vec<InterestGroupAd>>,
        }

        /**
         * Enum of shared storage access types.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum SharedStorageAccessType {
            #[serde(rename = "documentAddModule")]
            DocumentAddModule,
            #[serde(rename = "documentSelectURL")]
            DocumentSelectURL,
            #[serde(rename = "documentRun")]
            DocumentRun,
            #[serde(rename = "documentSet")]
            DocumentSet,
            #[serde(rename = "documentAppend")]
            DocumentAppend,
            #[serde(rename = "documentDelete")]
            DocumentDelete,
            #[serde(rename = "documentClear")]
            DocumentClear,
            #[serde(rename = "workletSet")]
            WorkletSet,
            #[serde(rename = "workletAppend")]
            WorkletAppend,
            #[serde(rename = "workletDelete")]
            WorkletDelete,
            #[serde(rename = "workletClear")]
            WorkletClear,
            #[serde(rename = "workletGet")]
            WorkletGet,
            #[serde(rename = "workletKeys")]
            WorkletKeys,
            #[serde(rename = "workletEntries")]
            WorkletEntries,
            #[serde(rename = "workletLength")]
            WorkletLength,
            #[serde(rename = "workletRemainingBudget")]
            WorkletRemainingBudget,
        }

        /**
         * Struct for a single key-value pair in an origin's shared storage.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SharedStorageEntry {
            /**
             * Struct for a single key-value pair in an origin's shared storage.
            */
            key: Option<String>,
            /**
             * Struct for a single key-value pair in an origin's shared storage.
            */
            value: Option<String>,
        }

        /**
         * Details for an origin's shared storage.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SharedStorageMetadata {
            /**
             * Details for an origin's shared storage.
            */
            #[serde(rename = "creationTime")]
            creation_time: Option<network::TimeSinceEpoch>,
            /**
             * Details for an origin's shared storage.
            */
            length: Option<Integer>,
            /**
             * Details for an origin's shared storage.
            */
            #[serde(rename = "remainingBudget")]
            remaining_budget: Option<f64>,
        }

        /**
         * Pair of reporting metadata details for a candidate URL for `selectURL()`.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SharedStorageReportingMetadata {
            /**
             * Pair of reporting metadata details for a candidate URL for `selectURL()`.
            */
            #[serde(rename = "eventType")]
            event_type: Option<String>,
            /**
             * Pair of reporting metadata details for a candidate URL for `selectURL()`.
            */
            #[serde(rename = "reportingUrl")]
            reporting_url: Option<String>,
        }

        /**
         * Bundles a candidate URL with its reporting metadata.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SharedStorageUrlWithMetadata {
            /**
             * Bundles a candidate URL with its reporting metadata.
            */
            url: Option<String>,
            /**
             * Bundles a candidate URL with its reporting metadata.
            */
            #[serde(rename = "reportingMetadata")]
            reportingmetadata: Option<Vec<SharedStorageReportingMetadata>>,
        }

        /**
         * Bundles the parameters for shared storage access events whose
         * presence/absence can vary according to SharedStorageAccessType.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SharedStorageAccessParams {
            /**
             * Bundles the parameters for shared storage access events whose
             * presence/absence can vary according to SharedStorageAccessType.
            */
            #[serde(rename = "scriptSourceUrl")]
            script_source_url: String,
            /**
             * Bundles the parameters for shared storage access events whose
             * presence/absence can vary according to SharedStorageAccessType.
            */
            #[serde(rename = "operationName")]
            operation_name: String,
            /**
             * Bundles the parameters for shared storage access events whose
             * presence/absence can vary according to SharedStorageAccessType.
            */
            #[serde(rename = "serializedData")]
            serialized_data: String,
            /**
             * Bundles the parameters for shared storage access events whose
             * presence/absence can vary according to SharedStorageAccessType.
            */
            #[serde(rename = "urlsWithMetadata")]
            urls_with_metadata: Vec<SharedStorageUrlWithMetadata>,
            /**
             * Bundles the parameters for shared storage access events whose
             * presence/absence can vary according to SharedStorageAccessType.
            */
            key: String,
            /**
             * Bundles the parameters for shared storage access events whose
             * presence/absence can vary according to SharedStorageAccessType.
            */
            value: String,
            /**
             * Bundles the parameters for shared storage access events whose
             * presence/absence can vary according to SharedStorageAccessType.
            */
            #[serde(rename = "ignoreIfPresent")]
            ignore_if_present: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum StorageBucketsDurability {
            #[serde(rename = "relaxed")]
            Relaxed,
            #[serde(rename = "strict")]
            Strict,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct StorageBucket {
            #[serde(rename = "storageKey")]
            storage_key: Option<SerializedStorageKey>,

            name: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct StorageBucketInfo {
            bucket: Option<StorageBucket>,

            id: Option<String>,

            expiration: Option<network::TimeSinceEpoch>,

            quota: Option<f64>,

            persistent: Option<bool>,

            durability: Option<StorageBucketsDurability>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum AttributionReportingSourceType {
            #[serde(rename = "navigation")]
            Navigation,
            #[serde(rename = "event")]
            Event,
        }

        pub type UnsignedInt64AsBase10 = String;

        pub type UnsignedInt128AsBase16 = String;

        pub type SignedInt64AsBase10 = String;

        #[derive(Debug, Serialize, Deserialize)]
        pub struct AttributionReportingFilterDataEntry {
            key: Option<String>,

            values: Option<Vec<String>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct AttributionReportingAggregationKeysEntry {
            key: Option<String>,

            value: Option<UnsignedInt128AsBase16>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct AttributionReportingSourceRegistration {
            time: Option<network::TimeSinceEpoch>,

            expiry: Integer,

            #[serde(rename = "eventReportWindow")]
            event_report_window: Integer,

            #[serde(rename = "aggregatableReportWindow")]
            aggregatable_report_window: Integer,

            #[serde(rename = "type")]
            r#type: Option<AttributionReportingSourceType>,

            #[serde(rename = "sourceOrigin")]
            source_origin: Option<String>,

            #[serde(rename = "reportingOrigin")]
            reporting_origin: Option<String>,

            #[serde(rename = "destinationSites")]
            destination_sites: Option<Vec<String>>,

            #[serde(rename = "eventId")]
            event_id: Option<UnsignedInt64AsBase10>,

            priority: Option<SignedInt64AsBase10>,

            #[serde(rename = "filterData")]
            filter_data: Option<Vec<AttributionReportingFilterDataEntry>>,

            #[serde(rename = "aggregationKeys")]
            aggregation_keys: Option<Vec<AttributionReportingAggregationKeysEntry>>,

            #[serde(rename = "debugKey")]
            debug_key: UnsignedInt64AsBase10,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum AttributionReportingSourceRegistrationResult {
            #[serde(rename = "success")]
            Success,
            #[serde(rename = "internalError")]
            InternalError,
            #[serde(rename = "insufficientSourceCapacity")]
            InsufficientSourceCapacity,
            #[serde(rename = "insufficientUniqueDestinationCapacity")]
            InsufficientUniqueDestinationCapacity,
            #[serde(rename = "excessiveReportingOrigins")]
            ExcessiveReportingOrigins,
            #[serde(rename = "prohibitedByBrowserPolicy")]
            ProhibitedByBrowserPolicy,
            #[serde(rename = "successNoised")]
            SuccessNoised,
            #[serde(rename = "destinationReportingLimitReached")]
            DestinationReportingLimitReached,
            #[serde(rename = "destinationGlobalLimitReached")]
            DestinationGlobalLimitReached,
            #[serde(rename = "destinationBothLimitsReached")]
            DestinationBothLimitsReached,
            #[serde(rename = "reportingOriginsPerSiteLimitReached")]
            ReportingOriginsPerSiteLimitReached,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetStorageKeyForFrameRequest {
            #[serde(rename = "frameId")]
            frame_id: Option<page::FrameId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetStorageKeyForFrameResponse {
            #[serde(rename = "storageKey")]
            storage_key: Option<SerializedStorageKey>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ClearDataForOriginRequest {
            origin: Option<String>,

            #[serde(rename = "storageTypes")]
            storagetypes: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ClearDataForStorageKeyRequest {
            #[serde(rename = "storageKey")]
            storage_key: Option<String>,

            #[serde(rename = "storageTypes")]
            storage_types: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetCookiesRequest {
            #[serde(rename = "browserContextId")]
            browser_context_id: browser::BrowserContextID,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetCookiesResponse {
            cookies: Option<Vec<network::Cookie>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetCookiesRequest {
            cookies: Option<Vec<network::CookieParam>>,

            #[serde(rename = "browserContextId")]
            browsercontextid: browser::BrowserContextID,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ClearCookiesRequest {
            #[serde(rename = "browserContextId")]
            browser_context_id: browser::BrowserContextID,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetUsageAndQuotaRequest {
            origin: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetUsageAndQuotaResponse {
            usage: Option<f64>,

            quota: Option<f64>,

            #[serde(rename = "overrideActive")]
            overrideactive: Option<bool>,

            #[serde(rename = "usageBreakdown")]
            usagebreakdown: Option<Vec<UsageForType>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct OverrideQuotaForOriginRequest {
            origin: Option<String>,

            #[serde(rename = "quotaSize")]
            quotasize: f64,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct TrackCacheStorageForOriginRequest {
            origin: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct TrackCacheStorageForStorageKeyRequest {
            #[serde(rename = "storageKey")]
            storage_key: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct TrackIndexedDBForOriginRequest {
            origin: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct TrackIndexedDBForStorageKeyRequest {
            #[serde(rename = "storageKey")]
            storage_key: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct UntrackCacheStorageForOriginRequest {
            origin: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct UntrackCacheStorageForStorageKeyRequest {
            #[serde(rename = "storageKey")]
            storage_key: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct UntrackIndexedDBForOriginRequest {
            origin: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct UntrackIndexedDBForStorageKeyRequest {
            #[serde(rename = "storageKey")]
            storage_key: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetTrustTokensResponse {
            tokens: Option<Vec<TrustTokens>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ClearTrustTokensRequest {
            #[serde(rename = "issuerOrigin")]
            issuer_origin: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ClearTrustTokensResponse {
            #[serde(rename = "didDeleteTokens")]
            did_delete_tokens: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetInterestGroupDetailsRequest {
            #[serde(rename = "ownerOrigin")]
            owner_origin: Option<String>,

            name: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetInterestGroupDetailsResponse {
            details: Option<InterestGroupDetails>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetInterestGroupTrackingRequest {
            enable: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetSharedStorageMetadataRequest {
            #[serde(rename = "ownerOrigin")]
            owner_origin: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetSharedStorageMetadataResponse {
            metadata: Option<SharedStorageMetadata>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetSharedStorageEntriesRequest {
            #[serde(rename = "ownerOrigin")]
            owner_origin: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetSharedStorageEntriesResponse {
            entries: Option<Vec<SharedStorageEntry>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetSharedStorageEntryRequest {
            #[serde(rename = "ownerOrigin")]
            owner_origin: Option<String>,

            key: Option<String>,

            value: Option<String>,

            #[serde(rename = "ignoreIfPresent")]
            ignore_if_present: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct DeleteSharedStorageEntryRequest {
            #[serde(rename = "ownerOrigin")]
            owner_origin: Option<String>,

            key: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ClearSharedStorageEntriesRequest {
            #[serde(rename = "ownerOrigin")]
            owner_origin: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ResetSharedStorageBudgetRequest {
            #[serde(rename = "ownerOrigin")]
            owner_origin: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetSharedStorageTrackingRequest {
            enable: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetStorageBucketTrackingRequest {
            #[serde(rename = "storageKey")]
            storage_key: Option<String>,

            enable: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct DeleteStorageBucketRequest {
            bucket: Option<StorageBucket>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct RunBounceTrackingMitigationsResponse {
            #[serde(rename = "deletedSites")]
            deleted_sites: Option<Vec<String>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetAttributionReportingLocalTestingModeRequest {
            enabled: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetAttributionReportingTrackingRequest {
            enable: Option<bool>,
        }

        /**
         * A cache's contents have been modified.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct CacheStorageContentUpdatedEvent {
            /**
             * A cache's contents have been modified.
            */
            origin: Option<String>,
            /**
             * A cache's contents have been modified.
            */
            #[serde(rename = "storageKey")]
            storage_key: Option<String>,
            /**
             * A cache's contents have been modified.
            */
            #[serde(rename = "bucketId")]
            bucket_id: Option<String>,
            /**
             * A cache's contents have been modified.
            */
            #[serde(rename = "cacheName")]
            cache_name: Option<String>,
        }

        /**
         * A cache has been added/deleted.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct CacheStorageListUpdatedEvent {
            /**
             * A cache has been added/deleted.
            */
            origin: Option<String>,
            /**
             * A cache has been added/deleted.
            */
            #[serde(rename = "storageKey")]
            storage_key: Option<String>,
            /**
             * A cache has been added/deleted.
            */
            #[serde(rename = "bucketId")]
            bucket_id: Option<String>,
        }

        /**
         * The origin's IndexedDB object store has been modified.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct IndexedDBContentUpdatedEvent {
            /**
             * The origin's IndexedDB object store has been modified.
            */
            origin: Option<String>,
            /**
             * The origin's IndexedDB object store has been modified.
            */
            #[serde(rename = "storageKey")]
            storage_key: Option<String>,
            /**
             * The origin's IndexedDB object store has been modified.
            */
            #[serde(rename = "bucketId")]
            bucket_id: Option<String>,
            /**
             * The origin's IndexedDB object store has been modified.
            */
            #[serde(rename = "databaseName")]
            database_name: Option<String>,
            /**
             * The origin's IndexedDB object store has been modified.
            */
            #[serde(rename = "objectStoreName")]
            object_store_name: Option<String>,
        }

        /**
         * The origin's IndexedDB database list has been modified.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct IndexedDBListUpdatedEvent {
            /**
             * The origin's IndexedDB database list has been modified.
            */
            origin: Option<String>,
            /**
             * The origin's IndexedDB database list has been modified.
            */
            #[serde(rename = "storageKey")]
            storage_key: Option<String>,
            /**
             * The origin's IndexedDB database list has been modified.
            */
            #[serde(rename = "bucketId")]
            bucket_id: Option<String>,
        }

        /**
         * One of the interest groups was accessed by the associated page.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct InterestGroupAccessedEvent {
            /**
             * One of the interest groups was accessed by the associated page.
            */
            #[serde(rename = "accessTime")]
            access_time: Option<network::TimeSinceEpoch>,
            /**
             * One of the interest groups was accessed by the associated page.
            */
            #[serde(rename = "type")]
            r#type: Option<InterestGroupAccessType>,
            /**
             * One of the interest groups was accessed by the associated page.
            */
            #[serde(rename = "ownerOrigin")]
            owner_origin: Option<String>,
            /**
             * One of the interest groups was accessed by the associated page.
            */
            name: Option<String>,
        }

        /**
         * Shared storage was accessed by the associated page.
         * The following parameters are included in all events.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SharedStorageAccessedEvent {
            /**
             * Shared storage was accessed by the associated page.
             * The following parameters are included in all events.
            */
            #[serde(rename = "accessTime")]
            access_time: Option<network::TimeSinceEpoch>,
            /**
             * Shared storage was accessed by the associated page.
             * The following parameters are included in all events.
            */
            #[serde(rename = "type")]
            r#type: Option<SharedStorageAccessType>,
            /**
             * Shared storage was accessed by the associated page.
             * The following parameters are included in all events.
            */
            #[serde(rename = "mainFrameId")]
            main_frame_id: Option<page::FrameId>,
            /**
             * Shared storage was accessed by the associated page.
             * The following parameters are included in all events.
            */
            #[serde(rename = "ownerOrigin")]
            owner_origin: Option<String>,
            /**
             * Shared storage was accessed by the associated page.
             * The following parameters are included in all events.
            */
            params: Option<SharedStorageAccessParams>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct StorageBucketCreatedOrUpdatedEvent {
            #[serde(rename = "bucketInfo")]
            bucket_info: Option<StorageBucketInfo>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct StorageBucketDeletedEvent {
            #[serde(rename = "bucketId")]
            bucket_id: Option<String>,
        }

        /**
         * TODO(crbug.com/1458532): Add other Attribution Reporting events, e.g.
         * trigger registration.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AttributionReportingSourceRegisteredEvent {
            /**
             * TODO(crbug.com/1458532): Add other Attribution Reporting events, e.g.
             * trigger registration.
            */
            registration: Option<AttributionReportingSourceRegistration>,
            /**
             * TODO(crbug.com/1458532): Add other Attribution Reporting events, e.g.
             * trigger registration.
            */
            result: Option<AttributionReportingSourceRegistrationResult>,
        }
    }

    /**
     * The SystemInfo domain defines methods and events for querying low-level system information.
    */
    pub mod system_info {
        use super::Integer;
        use serde::{self, Deserialize, Serialize};
        /**
         * Describes a single graphics processor (GPU).
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct GPUDevice {
            /**
             * Describes a single graphics processor (GPU).
            */
            #[serde(rename = "vendorId")]
            vendor_id: Option<f64>,
            /**
             * Describes a single graphics processor (GPU).
            */
            #[serde(rename = "deviceId")]
            device_id: Option<f64>,
            /**
             * Describes a single graphics processor (GPU).
            */
            #[serde(rename = "subSysId")]
            sub_sys_id: f64,
            /**
             * Describes a single graphics processor (GPU).
            */
            revision: f64,
            /**
             * Describes a single graphics processor (GPU).
            */
            #[serde(rename = "vendorString")]
            vendor_string: Option<String>,
            /**
             * Describes a single graphics processor (GPU).
            */
            #[serde(rename = "deviceString")]
            device_string: Option<String>,
            /**
             * Describes a single graphics processor (GPU).
            */
            #[serde(rename = "driverVendor")]
            driver_vendor: Option<String>,
            /**
             * Describes a single graphics processor (GPU).
            */
            #[serde(rename = "driverVersion")]
            driver_version: Option<String>,
        }

        /**
         * Describes the width and height dimensions of an entity.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Size {
            /**
             * Describes the width and height dimensions of an entity.
            */
            width: Option<Integer>,
            /**
             * Describes the width and height dimensions of an entity.
            */
            height: Option<Integer>,
        }

        /**
         * Describes a supported video decoding profile with its associated minimum and
         * maximum resolutions.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct VideoDecodeAcceleratorCapability {
            /**
             * Describes a supported video decoding profile with its associated minimum and
             * maximum resolutions.
            */
            profile: Option<String>,
            /**
             * Describes a supported video decoding profile with its associated minimum and
             * maximum resolutions.
            */
            #[serde(rename = "maxResolution")]
            max_resolution: Option<Size>,
            /**
             * Describes a supported video decoding profile with its associated minimum and
             * maximum resolutions.
            */
            #[serde(rename = "minResolution")]
            min_resolution: Option<Size>,
        }

        /**
         * Describes a supported video encoding profile with its associated maximum
         * resolution and maximum framerate.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct VideoEncodeAcceleratorCapability {
            /**
             * Describes a supported video encoding profile with its associated maximum
             * resolution and maximum framerate.
            */
            profile: Option<String>,
            /**
             * Describes a supported video encoding profile with its associated maximum
             * resolution and maximum framerate.
            */
            #[serde(rename = "maxResolution")]
            max_resolution: Option<Size>,
            /**
             * Describes a supported video encoding profile with its associated maximum
             * resolution and maximum framerate.
            */
            #[serde(rename = "maxFramerateNumerator")]
            max_framerate_numerator: Option<Integer>,
            /**
             * Describes a supported video encoding profile with its associated maximum
             * resolution and maximum framerate.
            */
            #[serde(rename = "maxFramerateDenominator")]
            max_framerate_denominator: Option<Integer>,
        }

        /**
         * YUV subsampling type of the pixels of a given image.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum SubsamplingFormat {
            #[serde(rename = "yuv420")]
            Yuv420,
            #[serde(rename = "yuv422")]
            Yuv422,
            #[serde(rename = "yuv444")]
            Yuv444,
        }

        /**
         * Image format of a given image.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum ImageType {
            #[serde(rename = "jpeg")]
            Jpeg,
            #[serde(rename = "webp")]
            Webp,
            #[serde(rename = "unknown")]
            Unknown,
        }

        /**
         * Describes a supported image decoding profile with its associated minimum and
         * maximum resolutions and subsampling.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ImageDecodeAcceleratorCapability {
            /**
             * Describes a supported image decoding profile with its associated minimum and
             * maximum resolutions and subsampling.
            */
            #[serde(rename = "imageType")]
            image_type: Option<ImageType>,
            /**
             * Describes a supported image decoding profile with its associated minimum and
             * maximum resolutions and subsampling.
            */
            #[serde(rename = "maxDimensions")]
            max_dimensions: Option<Size>,
            /**
             * Describes a supported image decoding profile with its associated minimum and
             * maximum resolutions and subsampling.
            */
            #[serde(rename = "minDimensions")]
            min_dimensions: Option<Size>,
            /**
             * Describes a supported image decoding profile with its associated minimum and
             * maximum resolutions and subsampling.
            */
            subsamplings: Option<Vec<SubsamplingFormat>>,
        }

        /**
         * Provides information about the GPU(s) on the system.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct GPUInfo {
            /**
             * Provides information about the GPU(s) on the system.
            */
            devices: Option<Vec<GPUDevice>>,
            /**
             * Provides information about the GPU(s) on the system.
            */
            #[serde(rename = "auxAttributes")]
            aux_attributes: serde_json::Value,
            /**
             * Provides information about the GPU(s) on the system.
            */
            #[serde(rename = "featureStatus")]
            feature_status: serde_json::Value,
            /**
             * Provides information about the GPU(s) on the system.
            */
            #[serde(rename = "driverBugWorkarounds")]
            driver_bug_workarounds: Option<Vec<String>>,
            /**
             * Provides information about the GPU(s) on the system.
            */
            #[serde(rename = "videoDecoding")]
            video_decoding: Option<Vec<VideoDecodeAcceleratorCapability>>,
            /**
             * Provides information about the GPU(s) on the system.
            */
            #[serde(rename = "videoEncoding")]
            video_encoding: Option<Vec<VideoEncodeAcceleratorCapability>>,
            /**
             * Provides information about the GPU(s) on the system.
            */
            #[serde(rename = "imageDecoding")]
            image_decoding: Option<Vec<ImageDecodeAcceleratorCapability>>,
        }

        /**
         * Represents process info.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ProcessInfo {
            /**
             * Represents process info.
            */
            #[serde(rename = "type")]
            r#type: Option<String>,
            /**
             * Represents process info.
            */
            id: Option<Integer>,
            /**
             * Represents process info.
            */
            #[serde(rename = "cpuTime")]
            cputime: Option<f64>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetInfoResponse {
            gpu: Option<GPUInfo>,

            #[serde(rename = "modelName")]
            model_name: Option<String>,

            #[serde(rename = "modelVersion")]
            model_version: Option<String>,

            #[serde(rename = "commandLine")]
            command_line: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetFeatureStateRequest {
            #[serde(rename = "featureState")]
            feature_state: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetFeatureStateResponse {
            #[serde(rename = "featureEnabled")]
            feature_enabled: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetProcessInfoResponse {
            #[serde(rename = "processInfo")]
            process_info: Option<Vec<ProcessInfo>>,
        }
    }

    /**
     * Supports additional targets discovery and allows to attach to them.
    */
    pub mod target {
        use super::{browser, page, Integer};
        use serde::{self, Deserialize, Serialize};

        pub type TargetID = String;

        /**
         * Unique identifier of attached debugging session.
        */
        pub type SessionID = String;

        #[derive(Debug, Serialize, Deserialize)]
        pub struct TargetInfo {
            #[serde(rename = "targetId")]
            target_id: Option<TargetID>,

            #[serde(rename = "type")]
            r#type: Option<String>,

            title: Option<String>,

            url: Option<String>,

            attached: Option<bool>,

            #[serde(rename = "openerId")]
            opener_id: TargetID,

            #[serde(rename = "canAccessOpener")]
            can_access_opener: Option<bool>,

            #[serde(rename = "openerFrameId")]
            opener_frame_id: page::FrameId,

            #[serde(rename = "browserContextId")]
            browser_context_id: browser::BrowserContextID,

            subtype: String,
        }

        /**
         * A filter used by target query/discovery/auto-attach operations.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct FilterEntry {
            /**
             * A filter used by target query/discovery/auto-attach operations.
            */
            exclude: bool,
            /**
             * A filter used by target query/discovery/auto-attach operations.
            */
            #[serde(rename = "type")]
            r#type: String,
        }

        /**
         * The entries in TargetFilter are matched sequentially against targets and
         * the first entry that matches determines if the target is included or not,
         * depending on the value of `exclude` field in the entry.
         * If filter is not specified, the one assumed is
         * [{type: "browser", exclude: true}, {type: "tab", exclude: true}, {}]
         * (i.e. include everything but `browser` and `tab`).
        */
        pub type TargetFilter = Vec<FilterEntry>;

        #[derive(Debug, Serialize, Deserialize)]
        pub struct RemoteLocation {
            host: Option<String>,

            port: Option<Integer>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ActivateTargetRequest {
            #[serde(rename = "targetId")]
            target_id: Option<TargetID>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct AttachToTargetRequest {
            #[serde(rename = "targetId")]
            target_id: Option<TargetID>,

            flatten: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct AttachToTargetResponse {
            #[serde(rename = "sessionId")]
            session_id: Option<SessionID>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct AttachToBrowserTargetResponse {
            #[serde(rename = "sessionId")]
            session_id: Option<SessionID>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CloseTargetRequest {
            #[serde(rename = "targetId")]
            target_id: Option<TargetID>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CloseTargetResponse {
            success: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ExposeDevToolsProtocolRequest {
            #[serde(rename = "targetId")]
            target_id: Option<TargetID>,

            #[serde(rename = "bindingName")]
            binding_name: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CreateBrowserContextRequest {
            #[serde(rename = "disposeOnDetach")]
            dispose_on_detach: bool,

            #[serde(rename = "proxyServer")]
            proxy_server: String,

            #[serde(rename = "proxyBypassList")]
            proxy_bypass_list: String,

            #[serde(rename = "originsWithUniversalNetworkAccess")]
            origins_with_universal_network_access: Vec<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CreateBrowserContextResponse {
            #[serde(rename = "browserContextId")]
            browser_context_id: Option<browser::BrowserContextID>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetBrowserContextsResponse {
            #[serde(rename = "browserContextIds")]
            browser_context_ids: Option<Vec<browser::BrowserContextID>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CreateTargetRequest {
            url: Option<String>,

            width: Integer,

            height: Integer,

            #[serde(rename = "browserContextId")]
            browsercontextid: browser::BrowserContextID,

            #[serde(rename = "enableBeginFrameControl")]
            enablebeginframecontrol: bool,

            #[serde(rename = "newWindow")]
            newwindow: bool,

            background: bool,

            #[serde(rename = "forTab")]
            fortab: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CreateTargetResponse {
            #[serde(rename = "targetId")]
            target_id: Option<TargetID>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct DetachFromTargetRequest {
            #[serde(rename = "sessionId")]
            session_id: SessionID,

            #[serde(rename = "targetId")]
            target_id: TargetID,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct DisposeBrowserContextRequest {
            #[serde(rename = "browserContextId")]
            browser_context_id: Option<browser::BrowserContextID>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetTargetInfoRequest {
            #[serde(rename = "targetId")]
            target_id: TargetID,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetTargetInfoResponse {
            #[serde(rename = "targetInfo")]
            target_info: Option<TargetInfo>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetTargetsRequest {
            filter: TargetFilter,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetTargetsResponse {
            #[serde(rename = "targetInfos")]
            target_infos: Option<Vec<TargetInfo>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SendMessageToTargetRequest {
            message: Option<String>,

            #[serde(rename = "sessionId")]
            session_id: SessionID,

            #[serde(rename = "targetId")]
            target_id: TargetID,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetAutoAttachRequest {
            #[serde(rename = "autoAttach")]
            auto_attach: Option<bool>,

            #[serde(rename = "waitForDebuggerOnStart")]
            wait_for_debugger_on_start: Option<bool>,

            flatten: bool,

            filter: TargetFilter,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct AutoAttachRelatedRequest {
            #[serde(rename = "targetId")]
            target_id: Option<TargetID>,

            #[serde(rename = "waitForDebuggerOnStart")]
            wait_for_debugger_on_start: Option<bool>,

            filter: TargetFilter,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetDiscoverTargetsRequest {
            discover: Option<bool>,

            filter: TargetFilter,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetRemoteLocationsRequest {
            locations: Option<Vec<RemoteLocation>>,
        }

        /**
         * Issued when attached to target because of auto-attach or `attachToTarget` command.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AttachedToTargetEvent {
            /**
             * Issued when attached to target because of auto-attach or `attachToTarget` command.
            */
            #[serde(rename = "sessionId")]
            session_id: Option<SessionID>,
            /**
             * Issued when attached to target because of auto-attach or `attachToTarget` command.
            */
            #[serde(rename = "targetInfo")]
            target_info: Option<TargetInfo>,
            /**
             * Issued when attached to target because of auto-attach or `attachToTarget` command.
            */
            #[serde(rename = "waitingForDebugger")]
            waiting_for_debugger: Option<bool>,
        }

        /**
         * Issued when detached from target for any reason (including `detachFromTarget` command). Can be
         * issued multiple times per target if multiple sessions have been attached to it.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct DetachedFromTargetEvent {
            /**
             * Issued when detached from target for any reason (including `detachFromTarget` command). Can be
             * issued multiple times per target if multiple sessions have been attached to it.
            */
            #[serde(rename = "sessionId")]
            session_id: Option<SessionID>,
            /**
             * Issued when detached from target for any reason (including `detachFromTarget` command). Can be
             * issued multiple times per target if multiple sessions have been attached to it.
            */
            #[serde(rename = "targetId")]
            target_id: TargetID,
        }

        /**
         * Notifies about a new protocol message received from the session (as reported in
         * `attachedToTarget` event).
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ReceivedMessageFromTargetEvent {
            /**
             * Notifies about a new protocol message received from the session (as reported in
             * `attachedToTarget` event).
            */
            #[serde(rename = "sessionId")]
            session_id: Option<SessionID>,
            /**
             * Notifies about a new protocol message received from the session (as reported in
             * `attachedToTarget` event).
            */
            message: Option<String>,
            /**
             * Notifies about a new protocol message received from the session (as reported in
             * `attachedToTarget` event).
            */
            #[serde(rename = "targetId")]
            target_id: TargetID,
        }

        /**
         * Issued when a possible inspection target is created.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct TargetCreatedEvent {
            /**
             * Issued when a possible inspection target is created.
            */
            #[serde(rename = "targetInfo")]
            target_info: Option<TargetInfo>,
        }

        /**
         * Issued when a target is destroyed.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct TargetDestroyedEvent {
            /**
             * Issued when a target is destroyed.
            */
            #[serde(rename = "targetId")]
            target_id: Option<TargetID>,
        }

        /**
         * Issued when a target has crashed.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct TargetCrashedEvent {
            /**
             * Issued when a target has crashed.
            */
            #[serde(rename = "targetId")]
            target_id: Option<TargetID>,
            /**
             * Issued when a target has crashed.
            */
            status: Option<String>,
            /**
             * Issued when a target has crashed.
            */
            #[serde(rename = "errorCode")]
            error_code: Option<Integer>,
        }

        /**
         * Issued when some information about a target has changed. This only happens between
         * `targetCreated` and `targetDestroyed`.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct TargetInfoChangedEvent {
            /**
             * Issued when some information about a target has changed. This only happens between
             * `targetCreated` and `targetDestroyed`.
            */
            #[serde(rename = "targetInfo")]
            target_info: Option<TargetInfo>,
        }
    }

    /**
     * The Tethering domain defines methods and events for browser port binding.
    */
    pub mod tethering {
        use super::Integer;
        use serde::{self, Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize)]
        pub struct BindRequest {
            port: Option<Integer>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct UnbindRequest {
            port: Option<Integer>,
        }

        /**
         * Informs that port was successfully bound and got a specified connection id.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AcceptedEvent {
            /**
             * Informs that port was successfully bound and got a specified connection id.
            */
            port: Option<Integer>,
            /**
             * Informs that port was successfully bound and got a specified connection id.
            */
            #[serde(rename = "connectionId")]
            connectionid: Option<String>,
        }
    }

    pub mod tracing {
        use super::io;
        use serde::{self, Deserialize, Serialize};
        use std::collections::HashMap;
        /**
         * Configuration for memory dump. Used only when "memory-infra" category is enabled.
        */
        pub type MemoryDumpConfig = HashMap<String, String>;

        #[derive(Debug, Serialize, Deserialize)]
        pub enum TraceConfigRecordMode {
            #[serde(rename = "recordUntilFull")]
            RecordUntilFull,

            #[serde(rename = "recordContinuously")]
            RecordContinuously,

            #[serde(rename = "recordAsMuchAsPossible")]
            RecordAsMuchAsPossible,

            #[serde(rename = "echoToConsole")]
            EchoToConsole,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct TraceConfig {
            #[serde(rename = "recordMode")]
            record_mode: TraceConfigRecordMode,

            #[serde(rename = "traceBufferSizeInKb")]
            trace_buffer_size_in_kb: f64,

            #[serde(rename = "enableSampling")]
            enable_sampling: bool,

            #[serde(rename = "enableSystrace")]
            enable_systrace: bool,

            #[serde(rename = "enableArgumentFilter")]
            enable_argument_filter: bool,

            #[serde(rename = "includedCategories")]
            included_categories: Vec<String>,

            #[serde(rename = "excludedCategories")]
            excluded_categories: Vec<String>,

            #[serde(rename = "syntheticDelays")]
            synthetic_delays: Vec<String>,

            #[serde(rename = "memoryDumpConfig")]
            memory_dump_config: MemoryDumpConfig,
        }

        /**
         * Data format of a trace. Can be either the legacy JSON format or the
         * protocol buffer format. Note that the JSON format will be deprecated soon.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum StreamFormat {
            #[serde(rename = "json")]
            Json,
            #[serde(rename = "proto")]
            Proto,
        }

        /**
         * Compression type to use for traces returned via streams.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum StreamCompression {
            #[serde(rename = "none")]
            None,
            #[serde(rename = "gzip")]
            Gzip,
        }

        /**
         * Details exposed when memory request explicitly declared.
         * Keep consistent with memory_dump_request_args.h and
         * memory_instrumentation.mojom
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum MemoryDumpLevelOfDetail {
            #[serde(rename = "background")]
            Background,
            #[serde(rename = "light")]
            Light,
            #[serde(rename = "detailed")]
            Detailed,
        }

        /**
         * Backend type to use for tracing. `chrome` uses the Chrome-integrated
         * tracing service and is supported on all platforms. `system` is only
         * supported on Chrome OS and uses the Perfetto system tracing service.
         * `auto` chooses `system` when the perfettoConfig provided to Tracing.start
         * specifies at least one non-Chrome data source; otherwise uses `chrome`.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum TracingBackend {
            #[serde(rename = "auto")]
            Auto,
            #[serde(rename = "chrome")]
            Chrome,
            #[serde(rename = "system")]
            System,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetCategoriesResponse {
            categories: Option<Vec<String>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct RecordClockSyncMarkerRequest {
            #[serde(rename = "syncId")]
            sync_id: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct RequestMemoryDumpRequest {
            deterministic: bool,

            #[serde(rename = "levelOfDetail")]
            levelofdetail: MemoryDumpLevelOfDetail,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct RequestMemoryDumpResponse {
            #[serde(rename = "dumpGuid")]
            dump_guid: Option<String>,

            success: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum StartRequestTransferMode {
            #[serde(rename = "ReportEvents")]
            ReportEvents,

            #[serde(rename = "ReturnAsStream")]
            ReturnAsStream,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct StartRequest {
            categories: String,

            options: String,

            #[serde(rename = "bufferUsageReportingInterval")]
            buffer_usage_reporting_interval: f64,

            #[serde(rename = "transferMode")]
            transfer_mode: StartRequestTransferMode,

            #[serde(rename = "streamFormat")]
            stream_format: StreamFormat,

            #[serde(rename = "streamCompression")]
            stream_compression: StreamCompression,

            #[serde(rename = "traceConfig")]
            trace_config: TraceConfig,

            #[serde(rename = "perfettoConfig")]
            perfetto_config: String,

            #[serde(rename = "tracingBackend")]
            tracing_backend: TracingBackend,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct BufferUsageEvent {
            #[serde(rename = "percentFull")]
            percent_full: f64,

            #[serde(rename = "eventCount")]
            event_count: f64,

            value: f64,
        }

        /**
         * Contains a bucket of collected trace events. When tracing is stopped collected events will be
         * sent as a sequence of dataCollected events followed by tracingComplete event.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct DataCollectedEvent {
            /**
             * Contains a bucket of collected trace events. When tracing is stopped collected events will be
             * sent as a sequence of dataCollected events followed by tracingComplete event.
            */
            value: Option<Vec<serde_json::Value>>,
        }

        /**
         * Signals that tracing is stopped and there is no trace buffers pending flush, all data were
         * delivered via dataCollected events.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct TracingCompleteEvent {
            /**
             * Signals that tracing is stopped and there is no trace buffers pending flush, all data were
             * delivered via dataCollected events.
            */
            #[serde(rename = "dataLossOccurred")]
            data_loss_occurred: Option<bool>,
            /**
             * Signals that tracing is stopped and there is no trace buffers pending flush, all data were
             * delivered via dataCollected events.
            */
            stream: io::StreamHandle,
            /**
             * Signals that tracing is stopped and there is no trace buffers pending flush, all data were
             * delivered via dataCollected events.
            */
            #[serde(rename = "traceFormat")]
            trace_format: StreamFormat,
            /**
             * Signals that tracing is stopped and there is no trace buffers pending flush, all data were
             * delivered via dataCollected events.
            */
            #[serde(rename = "streamCompression")]
            stream_compression: StreamCompression,
        }
    }

    /**
     * A domain for letting clients substitute browser's network layer with client code.
    */
    pub mod fetch {
        use super::{io, network, page, Integer};
        use serde::{self, Deserialize, Serialize};
        /**
         * Unique request identifier.
        */
        pub type RequestId = String;

        /**
         * Stages of the request to handle. Request will intercept before the request is
         * sent. Response will intercept after the response is received (but before response
         * body is received).
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum RequestStage {
            #[serde(rename = "Request")]
            Request,
            #[serde(rename = "Response")]
            Response,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct RequestPattern {
            #[serde(rename = "urlPattern")]
            url_pattern: String,

            #[serde(rename = "resourceType")]
            resource_type: network::ResourceType,

            #[serde(rename = "requestStage")]
            request_stage: RequestStage,
        }

        /**
         * Response HTTP header entry
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct HeaderEntry {
            /**
             * Response HTTP header entry
            */
            name: Option<String>,
            /**
             * Response HTTP header entry
            */
            value: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum AuthChallengeSource {
            #[serde(rename = "Server")]
            Server,

            #[serde(rename = "Proxy")]
            Proxy,
        }

        /**
         * Authorization challenge for HTTP status code 401 or 407.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AuthChallenge {
            /**
             * Authorization challenge for HTTP status code 401 or 407.
            */
            source: AuthChallengeSource,
            /**
             * Authorization challenge for HTTP status code 401 or 407.
            */
            origin: Option<String>,
            /**
             * Authorization challenge for HTTP status code 401 or 407.
            */
            scheme: Option<String>,
            /**
             * Authorization challenge for HTTP status code 401 or 407.
            */
            realm: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum AuthChallengeResponseResponse {
            #[serde(rename = "Default")]
            Default,

            #[serde(rename = "CancelAuth")]
            CancelAuth,

            #[serde(rename = "ProvideCredentials")]
            ProvideCredentials,
        }

        /**
         * Response to an AuthChallenge.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AuthChallengeResponse {
            /**
             * Response to an AuthChallenge.
            */
            response: Option<AuthChallengeResponseResponse>,
            /**
             * Response to an AuthChallenge.
            */
            username: String,
            /**
             * Response to an AuthChallenge.
            */
            password: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct EnableRequest {
            patterns: Vec<RequestPattern>,

            #[serde(rename = "handleAuthRequests")]
            handleauthrequests: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct FailRequestRequest {
            #[serde(rename = "requestId")]
            request_id: Option<RequestId>,

            #[serde(rename = "errorReason")]
            error_reason: Option<network::ErrorReason>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct FulfillRequestRequest {
            #[serde(rename = "requestId")]
            request_id: Option<RequestId>,

            #[serde(rename = "responseCode")]
            response_code: Option<Integer>,

            #[serde(rename = "responseHeaders")]
            response_headers: Vec<HeaderEntry>,

            #[serde(rename = "binaryResponseHeaders")]
            binary_response_headers: String,

            body: String,

            #[serde(rename = "responsePhrase")]
            response_phrase: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ContinueRequestRequest {
            #[serde(rename = "requestId")]
            request_id: Option<RequestId>,

            url: String,

            method: String,

            #[serde(rename = "postData")]
            post_data: String,

            headers: Vec<HeaderEntry>,

            #[serde(rename = "interceptResponse")]
            intercept_response: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ContinueWithAuthRequest {
            #[serde(rename = "requestId")]
            request_id: Option<RequestId>,

            #[serde(rename = "authChallengeResponse")]
            auth_challenge_response: Option<AuthChallengeResponse>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ContinueResponseRequest {
            #[serde(rename = "requestId")]
            request_id: Option<RequestId>,

            #[serde(rename = "responseCode")]
            response_code: Integer,

            #[serde(rename = "responsePhrase")]
            response_phrase: String,

            #[serde(rename = "responseHeaders")]
            response_headers: Vec<HeaderEntry>,

            #[serde(rename = "binaryResponseHeaders")]
            binary_response_headers: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetResponseBodyRequest {
            #[serde(rename = "requestId")]
            request_id: Option<RequestId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetResponseBodyResponse {
            body: Option<String>,

            #[serde(rename = "base64Encoded")]
            base64encoded: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct TakeResponseBodyAsStreamRequest {
            #[serde(rename = "requestId")]
            request_id: Option<RequestId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct TakeResponseBodyAsStreamResponse {
            stream: Option<io::StreamHandle>,
        }

        /**
         * Issued when the domain is enabled and the request URL matches the
         * specified filter. The request is paused until the client responds
         * with one of continueRequest, failRequest or fulfillRequest.
         * The stage of the request can be determined by presence of responseErrorReason
         * and responseStatusCode -- the request is at the response stage if either
         * of these fields is present and in the request stage otherwise.
         * Redirect responses and subsequent requests are reported similarly to regular
         * responses and requests. Redirect responses may be distinguished by the value
         * of `responseStatusCode` (which is one of 301, 302, 303, 307, 308) along with
         * presence of the `location` header. Requests resulting from a redirect will
         * have `redirectedRequestId` field set.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct RequestPausedEvent {
            /**
             * Issued when the domain is enabled and the request URL matches the
             * specified filter. The request is paused until the client responds
             * with one of continueRequest, failRequest or fulfillRequest.
             * The stage of the request can be determined by presence of responseErrorReason
             * and responseStatusCode -- the request is at the response stage if either
             * of these fields is present and in the request stage otherwise.
             * Redirect responses and subsequent requests are reported similarly to regular
             * responses and requests. Redirect responses may be distinguished by the value
             * of `responseStatusCode` (which is one of 301, 302, 303, 307, 308) along with
             * presence of the `location` header. Requests resulting from a redirect will
             * have `redirectedRequestId` field set.
            */
            #[serde(rename = "requestId")]
            request_id: Option<RequestId>,
            /**
             * Issued when the domain is enabled and the request URL matches the
             * specified filter. The request is paused until the client responds
             * with one of continueRequest, failRequest or fulfillRequest.
             * The stage of the request can be determined by presence of responseErrorReason
             * and responseStatusCode -- the request is at the response stage if either
             * of these fields is present and in the request stage otherwise.
             * Redirect responses and subsequent requests are reported similarly to regular
             * responses and requests. Redirect responses may be distinguished by the value
             * of `responseStatusCode` (which is one of 301, 302, 303, 307, 308) along with
             * presence of the `location` header. Requests resulting from a redirect will
             * have `redirectedRequestId` field set.
            */
            request: Option<network::Request>,
            /**
             * Issued when the domain is enabled and the request URL matches the
             * specified filter. The request is paused until the client responds
             * with one of continueRequest, failRequest or fulfillRequest.
             * The stage of the request can be determined by presence of responseErrorReason
             * and responseStatusCode -- the request is at the response stage if either
             * of these fields is present and in the request stage otherwise.
             * Redirect responses and subsequent requests are reported similarly to regular
             * responses and requests. Redirect responses may be distinguished by the value
             * of `responseStatusCode` (which is one of 301, 302, 303, 307, 308) along with
             * presence of the `location` header. Requests resulting from a redirect will
             * have `redirectedRequestId` field set.
            */
            #[serde(rename = "frameId")]
            frame_id: Option<page::FrameId>,
            /**
             * Issued when the domain is enabled and the request URL matches the
             * specified filter. The request is paused until the client responds
             * with one of continueRequest, failRequest or fulfillRequest.
             * The stage of the request can be determined by presence of responseErrorReason
             * and responseStatusCode -- the request is at the response stage if either
             * of these fields is present and in the request stage otherwise.
             * Redirect responses and subsequent requests are reported similarly to regular
             * responses and requests. Redirect responses may be distinguished by the value
             * of `responseStatusCode` (which is one of 301, 302, 303, 307, 308) along with
             * presence of the `location` header. Requests resulting from a redirect will
             * have `redirectedRequestId` field set.
            */
            #[serde(rename = "resourceType")]
            resource_type: Option<network::ResourceType>,
            /**
             * Issued when the domain is enabled and the request URL matches the
             * specified filter. The request is paused until the client responds
             * with one of continueRequest, failRequest or fulfillRequest.
             * The stage of the request can be determined by presence of responseErrorReason
             * and responseStatusCode -- the request is at the response stage if either
             * of these fields is present and in the request stage otherwise.
             * Redirect responses and subsequent requests are reported similarly to regular
             * responses and requests. Redirect responses may be distinguished by the value
             * of `responseStatusCode` (which is one of 301, 302, 303, 307, 308) along with
             * presence of the `location` header. Requests resulting from a redirect will
             * have `redirectedRequestId` field set.
            */
            #[serde(rename = "responseErrorReason")]
            response_error_reason: network::ErrorReason,
            /**
             * Issued when the domain is enabled and the request URL matches the
             * specified filter. The request is paused until the client responds
             * with one of continueRequest, failRequest or fulfillRequest.
             * The stage of the request can be determined by presence of responseErrorReason
             * and responseStatusCode -- the request is at the response stage if either
             * of these fields is present and in the request stage otherwise.
             * Redirect responses and subsequent requests are reported similarly to regular
             * responses and requests. Redirect responses may be distinguished by the value
             * of `responseStatusCode` (which is one of 301, 302, 303, 307, 308) along with
             * presence of the `location` header. Requests resulting from a redirect will
             * have `redirectedRequestId` field set.
            */
            #[serde(rename = "responseStatusCode")]
            response_status_code: Integer,
            /**
             * Issued when the domain is enabled and the request URL matches the
             * specified filter. The request is paused until the client responds
             * with one of continueRequest, failRequest or fulfillRequest.
             * The stage of the request can be determined by presence of responseErrorReason
             * and responseStatusCode -- the request is at the response stage if either
             * of these fields is present and in the request stage otherwise.
             * Redirect responses and subsequent requests are reported similarly to regular
             * responses and requests. Redirect responses may be distinguished by the value
             * of `responseStatusCode` (which is one of 301, 302, 303, 307, 308) along with
             * presence of the `location` header. Requests resulting from a redirect will
             * have `redirectedRequestId` field set.
            */
            #[serde(rename = "responseStatusText")]
            response_status_text: String,
            /**
             * Issued when the domain is enabled and the request URL matches the
             * specified filter. The request is paused until the client responds
             * with one of continueRequest, failRequest or fulfillRequest.
             * The stage of the request can be determined by presence of responseErrorReason
             * and responseStatusCode -- the request is at the response stage if either
             * of these fields is present and in the request stage otherwise.
             * Redirect responses and subsequent requests are reported similarly to regular
             * responses and requests. Redirect responses may be distinguished by the value
             * of `responseStatusCode` (which is one of 301, 302, 303, 307, 308) along with
             * presence of the `location` header. Requests resulting from a redirect will
             * have `redirectedRequestId` field set.
            */
            #[serde(rename = "responseHeaders")]
            response_headers: Vec<HeaderEntry>,
            /**
             * Issued when the domain is enabled and the request URL matches the
             * specified filter. The request is paused until the client responds
             * with one of continueRequest, failRequest or fulfillRequest.
             * The stage of the request can be determined by presence of responseErrorReason
             * and responseStatusCode -- the request is at the response stage if either
             * of these fields is present and in the request stage otherwise.
             * Redirect responses and subsequent requests are reported similarly to regular
             * responses and requests. Redirect responses may be distinguished by the value
             * of `responseStatusCode` (which is one of 301, 302, 303, 307, 308) along with
             * presence of the `location` header. Requests resulting from a redirect will
             * have `redirectedRequestId` field set.
            */
            #[serde(rename = "networkId")]
            network_id: network::RequestId,
            /**
             * Issued when the domain is enabled and the request URL matches the
             * specified filter. The request is paused until the client responds
             * with one of continueRequest, failRequest or fulfillRequest.
             * The stage of the request can be determined by presence of responseErrorReason
             * and responseStatusCode -- the request is at the response stage if either
             * of these fields is present and in the request stage otherwise.
             * Redirect responses and subsequent requests are reported similarly to regular
             * responses and requests. Redirect responses may be distinguished by the value
             * of `responseStatusCode` (which is one of 301, 302, 303, 307, 308) along with
             * presence of the `location` header. Requests resulting from a redirect will
             * have `redirectedRequestId` field set.
            */
            #[serde(rename = "redirectedRequestId")]
            redirected_request_id: RequestId,
        }

        /**
         * Issued when the domain is enabled with handleAuthRequests set to true.
         * The request is paused until client responds with continueWithAuth.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AuthRequiredEvent {
            /**
             * Issued when the domain is enabled with handleAuthRequests set to true.
             * The request is paused until client responds with continueWithAuth.
            */
            #[serde(rename = "requestId")]
            request_id: Option<RequestId>,
            /**
             * Issued when the domain is enabled with handleAuthRequests set to true.
             * The request is paused until client responds with continueWithAuth.
            */
            request: Option<network::Request>,
            /**
             * Issued when the domain is enabled with handleAuthRequests set to true.
             * The request is paused until client responds with continueWithAuth.
            */
            #[serde(rename = "frameId")]
            frame_id: Option<page::FrameId>,
            /**
             * Issued when the domain is enabled with handleAuthRequests set to true.
             * The request is paused until client responds with continueWithAuth.
            */
            #[serde(rename = "resourceType")]
            resource_type: Option<network::ResourceType>,
            /**
             * Issued when the domain is enabled with handleAuthRequests set to true.
             * The request is paused until client responds with continueWithAuth.
            */
            #[serde(rename = "authChallenge")]
            auth_challenge: Option<AuthChallenge>,
        }
    }

    /**
     * This domain allows inspection of Web Audio API.
     * https://webaudio.github.io/web-audio-api/
    */
    pub mod web_audio {
        use serde::{self, Deserialize, Serialize};
        /**
         * An unique ID for a graph object (AudioContext, AudioNode, AudioParam) in Web Audio API
        */
        pub type GraphObjectId = String;

        /**
         * Enum of BaseAudioContext types
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum ContextType {
            #[serde(rename = "realtime")]
            Realtime,
            #[serde(rename = "offline")]
            Offline,
        }

        /**
         * Enum of AudioContextState from the spec
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum ContextState {
            #[serde(rename = "suspended")]
            Suspended,
            #[serde(rename = "running")]
            Running,
            #[serde(rename = "closed")]
            Closed,
        }

        /**
         * Enum of AudioNode types
        */
        pub type NodeType = String;

        /**
         * Enum of AudioNode::ChannelCountMode from the spec
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum ChannelCountMode {
            #[serde(rename = "clamped-max")]
            ClampedMax,
            #[serde(rename = "explicit")]
            Explicit,
            #[serde(rename = "max")]
            Max,
        }

        /**
         * Enum of AudioNode::ChannelInterpretation from the spec
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum ChannelInterpretation {
            #[serde(rename = "discrete")]
            Discrete,
            #[serde(rename = "speakers")]
            Speakers,
        }

        /**
         * Enum of AudioParam types
        */
        pub type ParamType = String;

        /**
         * Enum of AudioParam::AutomationRate from the spec
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum AutomationRate {
            #[serde(rename = "a-rate")]
            ARate,
            #[serde(rename = "k-rate")]
            KRate,
        }

        /**
         * Fields in AudioContext that change in real-time.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ContextRealtimeData {
            /**
             * Fields in AudioContext that change in real-time.
            */
            #[serde(rename = "currentTime")]
            current_time: Option<f64>,
            /**
             * Fields in AudioContext that change in real-time.
            */
            #[serde(rename = "renderCapacity")]
            render_capacity: Option<f64>,
            /**
             * Fields in AudioContext that change in real-time.
            */
            #[serde(rename = "callbackIntervalMean")]
            callback_interval_mean: Option<f64>,
            /**
             * Fields in AudioContext that change in real-time.
            */
            #[serde(rename = "callbackIntervalVariance")]
            callback_interval_variance: Option<f64>,
        }

        /**
         * Protocol object for BaseAudioContext
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct BaseAudioContext {
            /**
             * Protocol object for BaseAudioContext
            */
            #[serde(rename = "contextId")]
            context_id: Option<GraphObjectId>,
            /**
             * Protocol object for BaseAudioContext
            */
            #[serde(rename = "contextType")]
            context_type: Option<ContextType>,
            /**
             * Protocol object for BaseAudioContext
            */
            #[serde(rename = "contextState")]
            context_state: Option<ContextState>,
            /**
             * Protocol object for BaseAudioContext
            */
            #[serde(rename = "realtimeData")]
            realtime_data: ContextRealtimeData,
            /**
             * Protocol object for BaseAudioContext
            */
            #[serde(rename = "callbackBufferSize")]
            callback_buffer_size: Option<f64>,
            /**
             * Protocol object for BaseAudioContext
            */
            #[serde(rename = "maxOutputChannelCount")]
            max_output_channel_count: Option<f64>,
            /**
             * Protocol object for BaseAudioContext
            */
            #[serde(rename = "sampleRate")]
            sample_rate: Option<f64>,
        }

        /**
         * Protocol object for AudioListener
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AudioListener {
            /**
             * Protocol object for AudioListener
            */
            #[serde(rename = "listenerId")]
            listener_id: Option<GraphObjectId>,
            /**
             * Protocol object for AudioListener
            */
            #[serde(rename = "contextId")]
            context_id: Option<GraphObjectId>,
        }

        /**
         * Protocol object for AudioNode
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AudioNode {
            /**
             * Protocol object for AudioNode
            */
            #[serde(rename = "nodeId")]
            node_id: Option<GraphObjectId>,
            /**
             * Protocol object for AudioNode
            */
            #[serde(rename = "contextId")]
            context_id: Option<GraphObjectId>,
            /**
             * Protocol object for AudioNode
            */
            #[serde(rename = "nodeType")]
            node_type: Option<NodeType>,
            /**
             * Protocol object for AudioNode
            */
            #[serde(rename = "numberOfInputs")]
            number_of_inputs: Option<f64>,
            /**
             * Protocol object for AudioNode
            */
            #[serde(rename = "numberOfOutputs")]
            number_of_outputs: Option<f64>,
            /**
             * Protocol object for AudioNode
            */
            #[serde(rename = "channelCount")]
            channel_count: Option<f64>,
            /**
             * Protocol object for AudioNode
            */
            #[serde(rename = "channelCountMode")]
            channel_count_mode: Option<ChannelCountMode>,
            /**
             * Protocol object for AudioNode
            */
            #[serde(rename = "channelInterpretation")]
            channel_interpretation: Option<ChannelInterpretation>,
        }

        /**
         * Protocol object for AudioParam
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AudioParam {
            /**
             * Protocol object for AudioParam
            */
            #[serde(rename = "paramId")]
            param_id: Option<GraphObjectId>,
            /**
             * Protocol object for AudioParam
            */
            #[serde(rename = "nodeId")]
            node_id: Option<GraphObjectId>,
            /**
             * Protocol object for AudioParam
            */
            #[serde(rename = "contextId")]
            context_id: Option<GraphObjectId>,
            /**
             * Protocol object for AudioParam
            */
            #[serde(rename = "paramType")]
            param_type: Option<ParamType>,
            /**
             * Protocol object for AudioParam
            */
            rate: Option<AutomationRate>,
            /**
             * Protocol object for AudioParam
            */
            #[serde(rename = "defaultValue")]
            default_value: Option<f64>,
            /**
             * Protocol object for AudioParam
            */
            #[serde(rename = "minValue")]
            min_value: Option<f64>,
            /**
             * Protocol object for AudioParam
            */
            #[serde(rename = "maxValue")]
            max_value: Option<f64>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetRealtimeDataRequest {
            #[serde(rename = "contextId")]
            context_id: Option<GraphObjectId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetRealtimeDataResponse {
            #[serde(rename = "realtimeData")]
            realtime_data: Option<ContextRealtimeData>,
        }

        /**
         * Notifies that a new BaseAudioContext has been created.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ContextCreatedEvent {
            /**
             * Notifies that a new BaseAudioContext has been created.
            */
            context: Option<BaseAudioContext>,
        }

        /**
         * Notifies that an existing BaseAudioContext will be destroyed.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ContextWillBeDestroyedEvent {
            /**
             * Notifies that an existing BaseAudioContext will be destroyed.
            */
            #[serde(rename = "contextId")]
            context_id: Option<GraphObjectId>,
        }

        /**
         * Notifies that existing BaseAudioContext has changed some properties (id stays the same)..
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct ContextChangedEvent {
            /**
             * Notifies that existing BaseAudioContext has changed some properties (id stays the same)..
            */
            context: Option<BaseAudioContext>,
        }

        /**
         * Notifies that the construction of an AudioListener has finished.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AudioListenerCreatedEvent {
            /**
             * Notifies that the construction of an AudioListener has finished.
            */
            listener: Option<AudioListener>,
        }

        /**
         * Notifies that a new AudioListener has been created.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AudioListenerWillBeDestroyedEvent {
            /**
             * Notifies that a new AudioListener has been created.
            */
            #[serde(rename = "contextId")]
            context_id: Option<GraphObjectId>,
            /**
             * Notifies that a new AudioListener has been created.
            */
            #[serde(rename = "listenerId")]
            listener_id: Option<GraphObjectId>,
        }

        /**
         * Notifies that a new AudioNode has been created.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AudioNodeCreatedEvent {
            /**
             * Notifies that a new AudioNode has been created.
            */
            node: Option<AudioNode>,
        }

        /**
         * Notifies that an existing AudioNode has been destroyed.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AudioNodeWillBeDestroyedEvent {
            /**
             * Notifies that an existing AudioNode has been destroyed.
            */
            #[serde(rename = "contextId")]
            context_id: Option<GraphObjectId>,
            /**
             * Notifies that an existing AudioNode has been destroyed.
            */
            #[serde(rename = "nodeId")]
            node_id: Option<GraphObjectId>,
        }

        /**
         * Notifies that a new AudioParam has been created.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AudioParamCreatedEvent {
            /**
             * Notifies that a new AudioParam has been created.
            */
            param: Option<AudioParam>,
        }

        /**
         * Notifies that an existing AudioParam has been destroyed.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct AudioParamWillBeDestroyedEvent {
            /**
             * Notifies that an existing AudioParam has been destroyed.
            */
            #[serde(rename = "contextId")]
            context_id: Option<GraphObjectId>,
            /**
             * Notifies that an existing AudioParam has been destroyed.
            */
            #[serde(rename = "nodeId")]
            node_id: Option<GraphObjectId>,
            /**
             * Notifies that an existing AudioParam has been destroyed.
            */
            #[serde(rename = "paramId")]
            param_id: Option<GraphObjectId>,
        }

        /**
         * Notifies that two AudioNodes are connected.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct NodesConnectedEvent {
            /**
             * Notifies that two AudioNodes are connected.
            */
            #[serde(rename = "contextId")]
            context_id: Option<GraphObjectId>,
            /**
             * Notifies that two AudioNodes are connected.
            */
            #[serde(rename = "sourceId")]
            source_id: Option<GraphObjectId>,
            /**
             * Notifies that two AudioNodes are connected.
            */
            #[serde(rename = "destinationId")]
            destination_id: Option<GraphObjectId>,
            /**
             * Notifies that two AudioNodes are connected.
            */
            #[serde(rename = "sourceOutputIndex")]
            source_output_index: f64,
            /**
             * Notifies that two AudioNodes are connected.
            */
            #[serde(rename = "destinationInputIndex")]
            destination_input_index: f64,
        }

        /**
         * Notifies that AudioNodes are disconnected. The destination can be null, and it means all the outgoing connections from the source are disconnected.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct NodesDisconnectedEvent {
            /**
             * Notifies that AudioNodes are disconnected. The destination can be null, and it means all the outgoing connections from the source are disconnected.
            */
            #[serde(rename = "contextId")]
            context_id: Option<GraphObjectId>,
            /**
             * Notifies that AudioNodes are disconnected. The destination can be null, and it means all the outgoing connections from the source are disconnected.
            */
            #[serde(rename = "sourceId")]
            source_id: Option<GraphObjectId>,
            /**
             * Notifies that AudioNodes are disconnected. The destination can be null, and it means all the outgoing connections from the source are disconnected.
            */
            #[serde(rename = "destinationId")]
            destination_id: Option<GraphObjectId>,
            /**
             * Notifies that AudioNodes are disconnected. The destination can be null, and it means all the outgoing connections from the source are disconnected.
            */
            #[serde(rename = "sourceOutputIndex")]
            source_output_index: f64,
            /**
             * Notifies that AudioNodes are disconnected. The destination can be null, and it means all the outgoing connections from the source are disconnected.
            */
            #[serde(rename = "destinationInputIndex")]
            destination_input_index: f64,
        }

        /**
         * Notifies that an AudioNode is connected to an AudioParam.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct NodeParamConnectedEvent {
            /**
             * Notifies that an AudioNode is connected to an AudioParam.
            */
            #[serde(rename = "contextId")]
            context_id: Option<GraphObjectId>,
            /**
             * Notifies that an AudioNode is connected to an AudioParam.
            */
            #[serde(rename = "sourceId")]
            source_id: Option<GraphObjectId>,
            /**
             * Notifies that an AudioNode is connected to an AudioParam.
            */
            #[serde(rename = "destinationId")]
            destination_id: Option<GraphObjectId>,
            /**
             * Notifies that an AudioNode is connected to an AudioParam.
            */
            #[serde(rename = "sourceOutputIndex")]
            source_output_index: f64,
        }

        /**
         * Notifies that an AudioNode is disconnected to an AudioParam.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct NodeParamDisconnectedEvent {
            /**
             * Notifies that an AudioNode is disconnected to an AudioParam.
            */
            #[serde(rename = "contextId")]
            context_id: Option<GraphObjectId>,
            /**
             * Notifies that an AudioNode is disconnected to an AudioParam.
            */
            #[serde(rename = "sourceId")]
            source_id: Option<GraphObjectId>,
            /**
             * Notifies that an AudioNode is disconnected to an AudioParam.
            */
            #[serde(rename = "destinationId")]
            destination_id: Option<GraphObjectId>,
            /**
             * Notifies that an AudioNode is disconnected to an AudioParam.
            */
            #[serde(rename = "sourceOutputIndex")]
            source_output_index: f64,
        }
    }

    /**
     * This domain allows configuring virtual authenticators to test the WebAuthn
     * API.
    */
    pub mod web_authn {
        use super::Integer;
        use serde::{self, Deserialize, Serialize};

        pub type AuthenticatorId = String;

        #[derive(Debug, Serialize, Deserialize)]
        pub enum AuthenticatorProtocol {
            #[serde(rename = "u2f")]
            U2f,
            #[serde(rename = "ctap2")]
            Ctap2,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum Ctap2Version {
            #[serde(rename = "ctap2_0")]
            Ctap2_0,
            #[serde(rename = "ctap2_1")]
            Ctap2_1,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum AuthenticatorTransport {
            #[serde(rename = "usb")]
            Usb,
            #[serde(rename = "nfc")]
            Nfc,
            #[serde(rename = "ble")]
            Ble,
            #[serde(rename = "cable")]
            Cable,
            #[serde(rename = "internal")]
            Internal,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct VirtualAuthenticatorOptions {
            protocol: Option<AuthenticatorProtocol>,

            #[serde(rename = "ctap2Version")]
            ctap2version: Ctap2Version,

            transport: Option<AuthenticatorTransport>,

            #[serde(rename = "hasResidentKey")]
            has_resident_key: bool,

            #[serde(rename = "hasUserVerification")]
            has_user_verification: bool,

            #[serde(rename = "hasLargeBlob")]
            has_large_blob: bool,

            #[serde(rename = "hasCredBlob")]
            has_cred_blob: bool,

            #[serde(rename = "hasMinPinLength")]
            has_min_pin_length: bool,

            #[serde(rename = "hasPrf")]
            has_prf: bool,

            #[serde(rename = "automaticPresenceSimulation")]
            automatic_presence_simulation: bool,

            #[serde(rename = "isUserVerified")]
            is_user_verified: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Credential {
            #[serde(rename = "credentialId")]
            credential_id: Option<String>,

            #[serde(rename = "isResidentCredential")]
            is_resident_credential: Option<bool>,

            #[serde(rename = "rpId")]
            rp_id: String,

            #[serde(rename = "privateKey")]
            private_key: Option<String>,

            #[serde(rename = "userHandle")]
            user_handle: String,

            #[serde(rename = "signCount")]
            sign_count: Option<Integer>,

            #[serde(rename = "largeBlob")]
            large_blob: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct EnableRequest {
            #[serde(rename = "enableUI")]
            enable_ui: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct AddVirtualAuthenticatorRequest {
            options: Option<VirtualAuthenticatorOptions>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct AddVirtualAuthenticatorResponse {
            #[serde(rename = "authenticatorId")]
            authenticator_id: Option<AuthenticatorId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetResponseOverrideBitsRequest {
            #[serde(rename = "authenticatorId")]
            authenticator_id: Option<AuthenticatorId>,

            #[serde(rename = "isBogusSignature")]
            is_bogus_signature: bool,

            #[serde(rename = "isBadUV")]
            is_bad_uv: bool,

            #[serde(rename = "isBadUP")]
            is_bad_up: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct RemoveVirtualAuthenticatorRequest {
            #[serde(rename = "authenticatorId")]
            authenticator_id: Option<AuthenticatorId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct AddCredentialRequest {
            #[serde(rename = "authenticatorId")]
            authenticator_id: Option<AuthenticatorId>,

            credential: Option<Credential>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetCredentialRequest {
            #[serde(rename = "authenticatorId")]
            authenticator_id: Option<AuthenticatorId>,

            #[serde(rename = "credentialId")]
            credential_id: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetCredentialResponse {
            credential: Option<Credential>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetCredentialsRequest {
            #[serde(rename = "authenticatorId")]
            authenticator_id: Option<AuthenticatorId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct GetCredentialsResponse {
            credentials: Option<Vec<Credential>>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct RemoveCredentialRequest {
            #[serde(rename = "authenticatorId")]
            authenticator_id: Option<AuthenticatorId>,

            #[serde(rename = "credentialId")]
            credential_id: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct ClearCredentialsRequest {
            #[serde(rename = "authenticatorId")]
            authenticator_id: Option<AuthenticatorId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetUserVerifiedRequest {
            #[serde(rename = "authenticatorId")]
            authenticator_id: Option<AuthenticatorId>,

            #[serde(rename = "isUserVerified")]
            is_user_verified: Option<bool>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SetAutomaticPresenceSimulationRequest {
            #[serde(rename = "authenticatorId")]
            authenticator_id: Option<AuthenticatorId>,

            enabled: Option<bool>,
        }

        /**
         * Triggered when a credential is added to an authenticator.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct CredentialAddedEvent {
            /**
             * Triggered when a credential is added to an authenticator.
            */
            #[serde(rename = "authenticatorId")]
            authenticator_id: Option<AuthenticatorId>,
            /**
             * Triggered when a credential is added to an authenticator.
            */
            credential: Option<Credential>,
        }

        /**
         * Triggered when a credential is used in a webauthn assertion.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct CredentialAssertedEvent {
            /**
             * Triggered when a credential is used in a webauthn assertion.
            */
            #[serde(rename = "authenticatorId")]
            authenticator_id: Option<AuthenticatorId>,
            /**
             * Triggered when a credential is used in a webauthn assertion.
            */
            credential: Option<Credential>,
        }
    }

    /**
     * This domain allows detailed inspection of media elements
    */
    pub mod media {
        use super::Integer;
        use serde::{self, Deserialize, Serialize};
        /**
         * Players will get an ID that is unique within the agent context.
        */
        pub type PlayerId = String;

        pub type Timestamp = f64;

        #[derive(Debug, Serialize, Deserialize)]
        pub enum PlayerMessageLevel {
            #[serde(rename = "error")]
            Error,

            #[serde(rename = "warning")]
            Warning,

            #[serde(rename = "info")]
            Info,

            #[serde(rename = "debug")]
            Debug,
        }

        /**
         * Have one type per entry in MediaLogRecord::Type
         * Corresponds to kMessage
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PlayerMessage {
            /**
             * Have one type per entry in MediaLogRecord::Type
             * Corresponds to kMessage
            */
            level: Option<PlayerMessageLevel>,
            /**
             * Have one type per entry in MediaLogRecord::Type
             * Corresponds to kMessage
            */
            message: Option<String>,
        }

        /**
         * Corresponds to kMediaPropertyChange
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PlayerProperty {
            /**
             * Corresponds to kMediaPropertyChange
            */
            name: Option<String>,
            /**
             * Corresponds to kMediaPropertyChange
            */
            value: Option<String>,
        }

        /**
         * Corresponds to kMediaEventTriggered
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PlayerEvent {
            /**
             * Corresponds to kMediaEventTriggered
            */
            timestamp: Option<Timestamp>,
            /**
             * Corresponds to kMediaEventTriggered
            */
            value: Option<String>,
        }

        /**
         * Represents logged source line numbers reported in an error.
         * NOTE: file and line are from chromium c++ implementation code, not js.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PlayerErrorSourceLocation {
            /**
             * Represents logged source line numbers reported in an error.
             * NOTE: file and line are from chromium c++ implementation code, not js.
            */
            file: Option<String>,
            /**
             * Represents logged source line numbers reported in an error.
             * NOTE: file and line are from chromium c++ implementation code, not js.
            */
            line: Option<Integer>,
        }

        /**
         * Corresponds to kMediaError
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PlayerError {
            /**
             * Corresponds to kMediaError
            */
            #[serde(rename = "errorType")]
            errortype: Option<String>,
            /**
             * Corresponds to kMediaError
            */
            code: Option<Integer>,
            /**
             * Corresponds to kMediaError
            */
            stack: Option<Vec<PlayerErrorSourceLocation>>,
            /**
             * Corresponds to kMediaError
            */
            cause: Option<Vec<PlayerError>>,
            /**
             * Corresponds to kMediaError
            */
            data: Option<serde_json::Value>,
        }

        /**
         * This can be called multiple times, and can be used to set / override /
         * remove player properties. A null propValue indicates removal.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PlayerPropertiesChangedEvent {
            /**
             * This can be called multiple times, and can be used to set / override /
             * remove player properties. A null propValue indicates removal.
            */
            #[serde(rename = "playerId")]
            player_id: Option<PlayerId>,
            /**
             * This can be called multiple times, and can be used to set / override /
             * remove player properties. A null propValue indicates removal.
            */
            properties: Option<Vec<PlayerProperty>>,
        }

        /**
         * Send events as a list, allowing them to be batched on the browser for less
         * congestion. If batched, events must ALWAYS be in chronological order.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PlayerEventsAddedEvent {
            /**
             * Send events as a list, allowing them to be batched on the browser for less
             * congestion. If batched, events must ALWAYS be in chronological order.
            */
            #[serde(rename = "playerId")]
            player_id: Option<PlayerId>,
            /**
             * Send events as a list, allowing them to be batched on the browser for less
             * congestion. If batched, events must ALWAYS be in chronological order.
            */
            events: Option<Vec<PlayerEvent>>,
        }

        /**
         * Send a list of any messages that need to be delivered.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PlayerMessagesLoggedEvent {
            /**
             * Send a list of any messages that need to be delivered.
            */
            #[serde(rename = "playerId")]
            player_id: Option<PlayerId>,
            /**
             * Send a list of any messages that need to be delivered.
            */
            messages: Option<Vec<PlayerMessage>>,
        }

        /**
         * Send a list of any errors that need to be delivered.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PlayerErrorsRaisedEvent {
            /**
             * Send a list of any errors that need to be delivered.
            */
            #[serde(rename = "playerId")]
            player_id: Option<PlayerId>,
            /**
             * Send a list of any errors that need to be delivered.
            */
            errors: Option<Vec<PlayerError>>,
        }

        /**
         * Called whenever a player is created, or when a new agent joins and receives
         * a list of active players. If an agent is restored, it will receive the full
         * list of player ids and all events again.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PlayersCreatedEvent {
            /**
             * Called whenever a player is created, or when a new agent joins and receives
             * a list of active players. If an agent is restored, it will receive the full
             * list of player ids and all events again.
            */
            players: Option<Vec<PlayerId>>,
        }
    }

    pub mod device_access {
        use serde::{self, Deserialize, Serialize};
        /**
         * Device request id.
        */
        pub type RequestId = String;

        /**
         * A device id.
        */
        pub type DeviceId = String;

        /**
         * Device information displayed in a user prompt to select a device.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PromptDevice {
            /**
             * Device information displayed in a user prompt to select a device.
            */
            id: Option<DeviceId>,
            /**
             * Device information displayed in a user prompt to select a device.
            */
            name: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SelectPromptRequest {
            id: Option<RequestId>,

            #[serde(rename = "deviceId")]
            deviceid: Option<DeviceId>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CancelPromptRequest {
            id: Option<RequestId>,
        }

        /**
         * A device request opened a user prompt to select a device. Respond with the
         * selectPrompt or cancelPrompt command.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct DeviceRequestPromptedEvent {
            /**
             * A device request opened a user prompt to select a device. Respond with the
             * selectPrompt or cancelPrompt command.
            */
            id: Option<RequestId>,
            /**
             * A device request opened a user prompt to select a device. Respond with the
             * selectPrompt or cancelPrompt command.
            */
            devices: Option<Vec<PromptDevice>>,
        }
    }

    pub mod preload {
        use super::{dom, network, page};
        use serde::{self, Deserialize, Serialize};
        /**
         * Unique id
        */
        pub type RuleSetId = String;

        /**
         * Corresponds to SpeculationRuleSet
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct RuleSet {
            /**
             * Corresponds to SpeculationRuleSet
            */
            id: Option<RuleSetId>,
            /**
             * Corresponds to SpeculationRuleSet
            */
            #[serde(rename = "loaderId")]
            loader_id: Option<network::LoaderId>,
            /**
             * Corresponds to SpeculationRuleSet
            */
            #[serde(rename = "sourceText")]
            source_text: Option<String>,
            /**
             * Corresponds to SpeculationRuleSet
            */
            #[serde(rename = "backendNodeId")]
            backend_node_id: dom::BackendNodeId,
            /**
             * Corresponds to SpeculationRuleSet
            */
            url: String,
            /**
             * Corresponds to SpeculationRuleSet
            */
            #[serde(rename = "requestId")]
            request_id: network::RequestId,
            /**
             * Corresponds to SpeculationRuleSet
            */
            #[serde(rename = "errorType")]
            error_type: RuleSetErrorType,
            /**
             * Corresponds to SpeculationRuleSet
            */
            #[serde(rename = "errorMessage")]
            error_message: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum RuleSetErrorType {
            #[serde(rename = "SourceIsNotJsonObject")]
            SourceIsNotJsonObject,
            #[serde(rename = "InvalidRulesSkipped")]
            InvalidRulesSkipped,
        }

        /**
         * The type of preloading attempted. It corresponds to
         * mojom::SpeculationAction (although PrefetchWithSubresources is omitted as it
         * isn't being used by clients).
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum SpeculationAction {
            #[serde(rename = "Prefetch")]
            Prefetch,
            #[serde(rename = "Prerender")]
            Prerender,
        }

        /**
         * Corresponds to mojom::SpeculationTargetHint.
         * See https://github.com/WICG/nav-speculation/blob/main/triggers.md#window-name-targeting-hints
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum SpeculationTargetHint {
            #[serde(rename = "Blank")]
            Blank,
            #[serde(rename = "Self")]
            Self_,
        }

        /**
         * A key that identifies a preloading attempt.
         *
         * The url used is the url specified by the trigger (i.e. the initial URL), and
         * not the final url that is navigated to. For example, prerendering allows
         * same-origin main frame navigations during the attempt, but the attempt is
         * still keyed with the initial URL.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PreloadingAttemptKey {
            /**
             * A key that identifies a preloading attempt.
             *
             * The url used is the url specified by the trigger (i.e. the initial URL), and
             * not the final url that is navigated to. For example, prerendering allows
             * same-origin main frame navigations during the attempt, but the attempt is
             * still keyed with the initial URL.
            */
            #[serde(rename = "loaderId")]
            loader_id: Option<network::LoaderId>,
            /**
             * A key that identifies a preloading attempt.
             *
             * The url used is the url specified by the trigger (i.e. the initial URL), and
             * not the final url that is navigated to. For example, prerendering allows
             * same-origin main frame navigations during the attempt, but the attempt is
             * still keyed with the initial URL.
            */
            action: Option<SpeculationAction>,
            /**
             * A key that identifies a preloading attempt.
             *
             * The url used is the url specified by the trigger (i.e. the initial URL), and
             * not the final url that is navigated to. For example, prerendering allows
             * same-origin main frame navigations during the attempt, but the attempt is
             * still keyed with the initial URL.
            */
            url: Option<String>,
            /**
             * A key that identifies a preloading attempt.
             *
             * The url used is the url specified by the trigger (i.e. the initial URL), and
             * not the final url that is navigated to. For example, prerendering allows
             * same-origin main frame navigations during the attempt, but the attempt is
             * still keyed with the initial URL.
            */
            #[serde(rename = "targetHint")]
            target_hint: SpeculationTargetHint,
        }

        /**
         * Lists sources for a preloading attempt, specifically the ids of rule sets
         * that had a speculation rule that triggered the attempt, and the
         * BackendNodeIds of <a href> or <area href> elements that triggered the
         * attempt (in the case of attempts triggered by a document rule). It is
         * possible for mulitple rule sets and links to trigger a single attempt.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PreloadingAttemptSource {
            /**
             * Lists sources for a preloading attempt, specifically the ids of rule sets
             * that had a speculation rule that triggered the attempt, and the
             * BackendNodeIds of <a href> or <area href> elements that triggered the
             * attempt (in the case of attempts triggered by a document rule). It is
             * possible for mulitple rule sets and links to trigger a single attempt.
            */
            key: Option<PreloadingAttemptKey>,
            /**
             * Lists sources for a preloading attempt, specifically the ids of rule sets
             * that had a speculation rule that triggered the attempt, and the
             * BackendNodeIds of <a href> or <area href> elements that triggered the
             * attempt (in the case of attempts triggered by a document rule). It is
             * possible for mulitple rule sets and links to trigger a single attempt.
            */
            #[serde(rename = "ruleSetIds")]
            rule_set_ids: Option<Vec<RuleSetId>>,
            /**
             * Lists sources for a preloading attempt, specifically the ids of rule sets
             * that had a speculation rule that triggered the attempt, and the
             * BackendNodeIds of <a href> or <area href> elements that triggered the
             * attempt (in the case of attempts triggered by a document rule). It is
             * possible for mulitple rule sets and links to trigger a single attempt.
            */
            #[serde(rename = "nodeIds")]
            node_ids: Option<Vec<dom::BackendNodeId>>,
        }

        /**
         * List of FinalStatus reasons for Prerender2.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum PrerenderFinalStatus {
            #[serde(rename = "Activated")]
            Activated,
            #[serde(rename = "Destroyed")]
            Destroyed,
            #[serde(rename = "LowEndDevice")]
            LowEndDevice,
            #[serde(rename = "InvalidSchemeRedirect")]
            InvalidSchemeRedirect,
            #[serde(rename = "InvalidSchemeNavigation")]
            InvalidSchemeNavigation,
            #[serde(rename = "InProgressNavigation")]
            InProgressNavigation,
            #[serde(rename = "NavigationRequestBlockedByCsp")]
            NavigationRequestBlockedByCsp,
            #[serde(rename = "MainFrameNavigation")]
            MainFrameNavigation,
            #[serde(rename = "MojoBinderPolicy")]
            MojoBinderPolicy,
            #[serde(rename = "RendererProcessCrashed")]
            RendererProcessCrashed,
            #[serde(rename = "RendererProcessKilled")]
            RendererProcessKilled,
            #[serde(rename = "Download")]
            Download,
            #[serde(rename = "TriggerDestroyed")]
            TriggerDestroyed,
            #[serde(rename = "NavigationNotCommitted")]
            NavigationNotCommitted,
            #[serde(rename = "NavigationBadHttpStatus")]
            NavigationBadHttpStatus,
            #[serde(rename = "ClientCertRequested")]
            ClientCertRequested,
            #[serde(rename = "NavigationRequestNetworkError")]
            NavigationRequestNetworkError,
            #[serde(rename = "MaxNumOfRunningPrerendersExceeded")]
            MaxNumOfRunningPrerendersExceeded,
            #[serde(rename = "CancelAllHostsForTesting")]
            CancelAllHostsForTesting,
            #[serde(rename = "DidFailLoad")]
            DidFailLoad,
            #[serde(rename = "Stop")]
            Stop,
            #[serde(rename = "SslCertificateError")]
            SslCertificateError,
            #[serde(rename = "LoginAuthRequested")]
            LoginAuthRequested,
            #[serde(rename = "UaChangeRequiresReload")]
            UaChangeRequiresReload,
            #[serde(rename = "BlockedByClient")]
            BlockedByClient,
            #[serde(rename = "AudioOutputDeviceRequested")]
            AudioOutputDeviceRequested,
            #[serde(rename = "MixedContent")]
            MixedContent,
            #[serde(rename = "TriggerBackgrounded")]
            TriggerBackgrounded,
            #[serde(rename = "MemoryLimitExceeded")]
            MemoryLimitExceeded,
            #[serde(rename = "FailToGetMemoryUsage")]
            FailToGetMemoryUsage,
            #[serde(rename = "DataSaverEnabled")]
            DataSaverEnabled,
            #[serde(rename = "HasEffectiveUrl")]
            HasEffectiveUrl,
            #[serde(rename = "ActivatedBeforeStarted")]
            ActivatedBeforeStarted,
            #[serde(rename = "InactivePageRestriction")]
            InactivePageRestriction,
            #[serde(rename = "StartFailed")]
            StartFailed,
            #[serde(rename = "TimeoutBackgrounded")]
            TimeoutBackgrounded,
            #[serde(rename = "CrossSiteRedirectInInitialNavigation")]
            CrossSiteRedirectInInitialNavigation,
            #[serde(rename = "CrossSiteNavigationInInitialNavigation")]
            CrossSiteNavigationInInitialNavigation,
            #[serde(rename = "SameSiteCrossOriginRedirectNotOptInInInitialNavigation")]
            SameSiteCrossOriginRedirectNotOptInInInitialNavigation,
            #[serde(rename = "SameSiteCrossOriginNavigationNotOptInInInitialNavigation")]
            SameSiteCrossOriginNavigationNotOptInInInitialNavigation,
            #[serde(rename = "ActivationNavigationParameterMismatch")]
            ActivationNavigationParameterMismatch,
            #[serde(rename = "ActivatedInBackground")]
            ActivatedInBackground,
            #[serde(rename = "EmbedderHostDisallowed")]
            EmbedderHostDisallowed,
            #[serde(rename = "ActivationNavigationDestroyedBeforeSuccess")]
            ActivationNavigationDestroyedBeforeSuccess,
            #[serde(rename = "TabClosedByUserGesture")]
            TabClosedByUserGesture,
            #[serde(rename = "TabClosedWithoutUserGesture")]
            TabClosedWithoutUserGesture,
            #[serde(rename = "PrimaryMainFrameRendererProcessCrashed")]
            PrimaryMainFrameRendererProcessCrashed,
            #[serde(rename = "PrimaryMainFrameRendererProcessKilled")]
            PrimaryMainFrameRendererProcessKilled,
            #[serde(rename = "ActivationFramePolicyNotCompatible")]
            ActivationFramePolicyNotCompatible,
            #[serde(rename = "PreloadingDisabled")]
            PreloadingDisabled,
            #[serde(rename = "BatterySaverEnabled")]
            BatterySaverEnabled,
            #[serde(rename = "ActivatedDuringMainFrameNavigation")]
            ActivatedDuringMainFrameNavigation,
            #[serde(rename = "PreloadingUnsupportedByWebContents")]
            PreloadingUnsupportedByWebContents,
            #[serde(rename = "CrossSiteRedirectInMainFrameNavigation")]
            CrossSiteRedirectInMainFrameNavigation,
            #[serde(rename = "CrossSiteNavigationInMainFrameNavigation")]
            CrossSiteNavigationInMainFrameNavigation,
            #[serde(rename = "SameSiteCrossOriginRedirectNotOptInInMainFrameNavigation")]
            SameSiteCrossOriginRedirectNotOptInInMainFrameNavigation,
            #[serde(rename = "SameSiteCrossOriginNavigationNotOptInInMainFrameNavigation")]
            SameSiteCrossOriginNavigationNotOptInInMainFrameNavigation,
            #[serde(rename = "MemoryPressureOnTrigger")]
            MemoryPressureOnTrigger,
            #[serde(rename = "MemoryPressureAfterTriggered")]
            MemoryPressureAfterTriggered,
            #[serde(rename = "PrerenderingDisabledByDevTools")]
            PrerenderingDisabledByDevTools,
            #[serde(rename = "ResourceLoadBlockedByClient")]
            ResourceLoadBlockedByClient,
            #[serde(rename = "SpeculationRuleRemoved")]
            SpeculationRuleRemoved,
            #[serde(rename = "ActivatedWithAuxiliaryBrowsingContexts")]
            ActivatedWithAuxiliaryBrowsingContexts,
        }

        /**
         * Preloading status values, see also PreloadingTriggeringOutcome. This
         * status is shared by prefetchStatusUpdated and prerenderStatusUpdated.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum PreloadingStatus {
            #[serde(rename = "Pending")]
            Pending,
            #[serde(rename = "Running")]
            Running,
            #[serde(rename = "Ready")]
            Ready,
            #[serde(rename = "Success")]
            Success,
            #[serde(rename = "Failure")]
            Failure,
            #[serde(rename = "NotSupported")]
            NotSupported,
        }

        /**
         * TODO(https://crbug.com/1384419): revisit the list of PrefetchStatus and
         * filter out the ones that aren't necessary to the developers.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum PrefetchStatus {
            #[serde(rename = "PrefetchAllowed")]
            PrefetchAllowed,
            #[serde(rename = "PrefetchFailedIneligibleRedirect")]
            PrefetchFailedIneligibleRedirect,
            #[serde(rename = "PrefetchFailedInvalidRedirect")]
            PrefetchFailedInvalidRedirect,
            #[serde(rename = "PrefetchFailedMIMENotSupported")]
            PrefetchFailedMIMENotSupported,
            #[serde(rename = "PrefetchFailedNetError")]
            PrefetchFailedNetError,
            #[serde(rename = "PrefetchFailedNon2XX")]
            PrefetchFailedNon2XX,
            #[serde(rename = "PrefetchFailedPerPageLimitExceeded")]
            PrefetchFailedPerPageLimitExceeded,
            #[serde(rename = "PrefetchEvicted")]
            PrefetchEvicted,
            #[serde(rename = "PrefetchHeldback")]
            PrefetchHeldback,
            #[serde(rename = "PrefetchIneligibleRetryAfter")]
            PrefetchIneligibleRetryAfter,
            #[serde(rename = "PrefetchIsPrivacyDecoy")]
            PrefetchIsPrivacyDecoy,
            #[serde(rename = "PrefetchIsStale")]
            PrefetchIsStale,
            #[serde(rename = "PrefetchNotEligibleBrowserContextOffTheRecord")]
            PrefetchNotEligibleBrowserContextOffTheRecord,
            #[serde(rename = "PrefetchNotEligibleDataSaverEnabled")]
            PrefetchNotEligibleDataSaverEnabled,
            #[serde(rename = "PrefetchNotEligibleExistingProxy")]
            PrefetchNotEligibleExistingProxy,
            #[serde(rename = "PrefetchNotEligibleHostIsNonUnique")]
            PrefetchNotEligibleHostIsNonUnique,
            #[serde(rename = "PrefetchNotEligibleNonDefaultStoragePartition")]
            PrefetchNotEligibleNonDefaultStoragePartition,
            #[serde(rename = "PrefetchNotEligibleSameSiteCrossOriginPrefetchRequiredProxy")]
            PrefetchNotEligibleSameSiteCrossOriginPrefetchRequiredProxy,
            #[serde(rename = "PrefetchNotEligibleSchemeIsNotHttps")]
            PrefetchNotEligibleSchemeIsNotHttps,
            #[serde(rename = "PrefetchNotEligibleUserHasCookies")]
            PrefetchNotEligibleUserHasCookies,
            #[serde(rename = "PrefetchNotEligibleUserHasServiceWorker")]
            PrefetchNotEligibleUserHasServiceWorker,
            #[serde(rename = "PrefetchNotEligibleBatterySaverEnabled")]
            PrefetchNotEligibleBatterySaverEnabled,
            #[serde(rename = "PrefetchNotEligiblePreloadingDisabled")]
            PrefetchNotEligiblePreloadingDisabled,
            #[serde(rename = "PrefetchNotFinishedInTime")]
            PrefetchNotFinishedInTime,
            #[serde(rename = "PrefetchNotStarted")]
            PrefetchNotStarted,
            #[serde(rename = "PrefetchNotUsedCookiesChanged")]
            PrefetchNotUsedCookiesChanged,
            #[serde(rename = "PrefetchProxyNotAvailable")]
            PrefetchProxyNotAvailable,
            #[serde(rename = "PrefetchResponseUsed")]
            PrefetchResponseUsed,
            #[serde(rename = "PrefetchSuccessfulButNotUsed")]
            PrefetchSuccessfulButNotUsed,
            #[serde(rename = "PrefetchNotUsedProbeFailed")]
            PrefetchNotUsedProbeFailed,
        }

        /**
         * Upsert. Currently, it is only emitted when a rule set added.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct RuleSetUpdatedEvent {
            /**
             * Upsert. Currently, it is only emitted when a rule set added.
            */
            #[serde(rename = "ruleSet")]
            rule_set: Option<RuleSet>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct RuleSetRemovedEvent {
            id: Option<RuleSetId>,
        }

        /**
         * Fired when a prerender attempt is completed.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PrerenderAttemptCompletedEvent {
            /**
             * Fired when a prerender attempt is completed.
            */
            key: Option<PreloadingAttemptKey>,
            /**
             * Fired when a prerender attempt is completed.
            */
            #[serde(rename = "initiatingFrameId")]
            initiating_frame_id: Option<page::FrameId>,
            /**
             * Fired when a prerender attempt is completed.
            */
            #[serde(rename = "prerenderingUrl")]
            prerendering_url: Option<String>,
            /**
             * Fired when a prerender attempt is completed.
            */
            #[serde(rename = "finalStatus")]
            final_status: Option<PrerenderFinalStatus>,
            /**
             * Fired when a prerender attempt is completed.
            */
            #[serde(rename = "disallowedApiMethod")]
            disallowed_api_method: String,
        }

        /**
         * Fired when a preload enabled state is updated.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PreloadEnabledStateUpdatedEvent {
            /**
             * Fired when a preload enabled state is updated.
            */
            #[serde(rename = "disabledByPreference")]
            disabled_by_preference: Option<bool>,
            /**
             * Fired when a preload enabled state is updated.
            */
            #[serde(rename = "disabledByDataSaver")]
            disabled_by_data_saver: Option<bool>,
            /**
             * Fired when a preload enabled state is updated.
            */
            #[serde(rename = "disabledByBatterySaver")]
            disabled_by_battery_saver: Option<bool>,
            /**
             * Fired when a preload enabled state is updated.
            */
            #[serde(rename = "disabledByHoldbackPrefetchSpeculationRules")]
            disabled_by_holdback_prefetch_speculation_rules: Option<bool>,
            /**
             * Fired when a preload enabled state is updated.
            */
            #[serde(rename = "disabledByHoldbackPrerenderSpeculationRules")]
            disabled_by_holdback_prerender_speculation_rules: Option<bool>,
        }

        /**
         * Fired when a prefetch attempt is updated.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PrefetchStatusUpdatedEvent {
            /**
             * Fired when a prefetch attempt is updated.
            */
            key: Option<PreloadingAttemptKey>,
            /**
             * Fired when a prefetch attempt is updated.
            */
            #[serde(rename = "initiatingFrameId")]
            initiating_frame_id: Option<page::FrameId>,
            /**
             * Fired when a prefetch attempt is updated.
            */
            #[serde(rename = "prefetchUrl")]
            prefetch_url: Option<String>,
            /**
             * Fired when a prefetch attempt is updated.
            */
            status: Option<PreloadingStatus>,
            /**
             * Fired when a prefetch attempt is updated.
            */
            #[serde(rename = "prefetchStatus")]
            prefetch_status: Option<PrefetchStatus>,
            /**
             * Fired when a prefetch attempt is updated.
            */
            #[serde(rename = "requestId")]
            request_id: Option<network::RequestId>,
        }

        /**
         * Fired when a prerender attempt is updated.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PrerenderStatusUpdatedEvent {
            /**
             * Fired when a prerender attempt is updated.
            */
            key: Option<PreloadingAttemptKey>,
            /**
             * Fired when a prerender attempt is updated.
            */
            status: Option<PreloadingStatus>,
            /**
             * Fired when a prerender attempt is updated.
            */
            #[serde(rename = "prerenderStatus")]
            prerenderstatus: PrerenderFinalStatus,
            /**
             * Fired when a prerender attempt is updated.
            */
            #[serde(rename = "disallowedMojoInterface")]
            disallowedmojointerface: String,
        }

        /**
         * Send a list of sources for all preloading attempts in a document.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct PreloadingAttemptSourcesUpdatedEvent {
            /**
             * Send a list of sources for all preloading attempts in a document.
            */
            #[serde(rename = "loaderId")]
            loader_id: Option<network::LoaderId>,
            /**
             * Send a list of sources for all preloading attempts in a document.
            */
            #[serde(rename = "preloadingAttemptSources")]
            preloading_attempt_sources: Option<Vec<PreloadingAttemptSource>>,
        }
    }

    /**
     * This domain allows interacting with the FedCM dialog.
    */
    pub mod fed_cm {
        use super::Integer;
        use serde::{self, Deserialize, Serialize};
        /**
         * Whether this is a sign-up or sign-in action for this account, i.e.
         * whether this account has ever been used to sign in to this RP before.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum LoginState {
            #[serde(rename = "SignIn")]
            SignIn,
            #[serde(rename = "SignUp")]
            SignUp,
        }

        /**
         * Whether the dialog shown is an account chooser or an auto re-authentication dialog.
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub enum DialogType {
            #[serde(rename = "AccountChooser")]
            AccountChooser,
            #[serde(rename = "AutoReauthn")]
            AutoReauthn,
        }

        /**
         * Corresponds to IdentityRequestAccount
        */
        #[derive(Debug, Serialize, Deserialize)]
        pub struct Account {
            /**
             * Corresponds to IdentityRequestAccount
            */
            #[serde(rename = "accountId")]
            account_id: Option<String>,
            /**
             * Corresponds to IdentityRequestAccount
            */
            email: Option<String>,
            /**
             * Corresponds to IdentityRequestAccount
            */
            name: Option<String>,
            /**
             * Corresponds to IdentityRequestAccount
            */
            #[serde(rename = "givenName")]
            given_name: Option<String>,
            /**
             * Corresponds to IdentityRequestAccount
            */
            #[serde(rename = "pictureUrl")]
            picture_url: Option<String>,
            /**
             * Corresponds to IdentityRequestAccount
            */
            #[serde(rename = "idpConfigUrl")]
            idp_config_url: Option<String>,
            /**
             * Corresponds to IdentityRequestAccount
            */
            #[serde(rename = "idpSigninUrl")]
            idp_signin_url: Option<String>,
            /**
             * Corresponds to IdentityRequestAccount
            */
            #[serde(rename = "loginState")]
            login_state: Option<LoginState>,
            /**
             * Corresponds to IdentityRequestAccount
            */
            #[serde(rename = "termsOfServiceUrl")]
            terms_of_service_url: String,
            /**
             * Corresponds to IdentityRequestAccount
            */
            #[serde(rename = "privacyPolicyUrl")]
            privacy_policy_url: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct EnableRequest {
            #[serde(rename = "disableRejectionDelay")]
            disable_rejection_delay: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SelectAccountRequest {
            #[serde(rename = "dialogId")]
            dialog_id: Option<String>,

            #[serde(rename = "accountIndex")]
            account_index: Option<Integer>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct DismissDialogRequest {
            #[serde(rename = "dialogId")]
            dialog_id: Option<String>,

            #[serde(rename = "triggerCooldown")]
            trigger_cooldown: bool,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct DialogShownEvent {
            #[serde(rename = "dialogId")]
            dialogid: Option<String>,

            #[serde(rename = "dialogType")]
            dialogtype: Option<DialogType>,

            accounts: Option<Vec<Account>>,

            title: Option<String>,

            subtitle: String,
        }
    }
}
