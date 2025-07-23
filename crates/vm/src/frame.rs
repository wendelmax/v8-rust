// Frame system for ECMAScript VM

use super::bytecode::Bytecode;

#[derive(Debug, Clone)]
pub struct Frame {
    pub bytecode: Bytecode,
    pub pc: usize,
    pub locals: Vec<Value>,
    pub arguments: Vec<Value>,
    pub this_value: Option<Value>,
    pub function: Option<FunctionInfo>,
    pub scope_chain: Vec<EnvironmentInfo>,
    pub exception_handler: Option<ExceptionHandler>,
    pub generator_state: Option<GeneratorState>,
}

#[derive(Debug, Clone)]
pub struct FunctionInfo {
    pub name: String,
    pub param_count: usize,
    pub local_count: usize,
    pub max_stack: usize,
    pub strict: bool,
    pub generator: bool,
    pub async: bool,
}

#[derive(Debug, Clone)]
pub struct EnvironmentInfo {
    pub environment_type: EnvironmentType,
    pub object: Option<ObjectInfo>,
    pub scope: Option<ScopeInfo>,
}

#[derive(Debug, Clone)]
pub enum EnvironmentType {
    Global,
    Function,
    Block,
    Catch,
    With,
    Module,
}

#[derive(Debug, Clone)]
pub struct ObjectInfo {
    pub properties: HashMap<String, Value>,
    pub prototype: Option<usize>, // index to another object
}

#[derive(Debug, Clone)]
pub struct ScopeInfo {
    pub bindings: HashMap<String, BindingInfo>,
    pub outer: Option<usize>, // index to outer scope
}

#[derive(Debug, Clone)]
pub struct BindingInfo {
    pub value: Value,
    pub mutable: bool,
    pub deletable: bool,
    pub strict: bool,
}

#[derive(Debug, Clone)]
pub struct ExceptionHandler {
    pub try_pc: usize,
    pub catch_pc: usize,
    pub finally_pc: Option<usize>,
    pub exception_var: Option<String>,
}

#[derive(Debug, Clone)]
pub struct GeneratorState {
    pub state: GeneratorStateType,
    pub yielded_value: Option<Value>,
    pub return_value: Option<Value>,
    pub thrown_value: Option<Value>,
}

#[derive(Debug, Clone)]
pub enum GeneratorStateType {
    SuspendedStart,
    SuspendedYield,
    Executing,
    Completed,
}

impl Frame {
    pub fn new(bytecode: Bytecode, pc: usize) -> Self {
        Self {
            bytecode,
            pc,
            locals: Vec::new(),
            arguments: Vec::new(),
            this_value: None,
            function: None,
            scope_chain: Vec::new(),
            exception_handler: None,
            generator_state: None,
        }
    }
    
    pub fn function_frame(
        bytecode: Bytecode,
        pc: usize,
        function: FunctionInfo,
        arguments: Vec<Value>,
        this_value: Value,
    ) -> Self {
        let mut frame = Self::new(bytecode, pc);
        frame.function = Some(function.clone());
        frame.arguments = arguments;
        frame.this_value = Some(this_value);
        frame.locals = vec![Value::Undefined; function.local_count];
        frame
    }
    
    pub fn get_pc(&self) -> usize {
        self.pc
    }
    
    pub fn set_pc(&mut self, pc: usize) {
        self.pc = pc;
    }
    
    pub fn get_bytecode(&self) -> &Bytecode {
        &self.bytecode
    }
    
    pub fn get_local(&self, index: usize) -> Option<&Value> {
        self.locals.get(index)
    }
    
    pub fn set_local(&mut self, index: usize, value: Value) -> Result<(), String> {
        if index < self.locals.len() {
            self.locals[index] = value;
            Ok(())
        } else {
            Err(format!("Invalid local index: {}", index))
        }
    }
    
    pub fn get_argument(&self, index: usize) -> Option<&Value> {
        self.arguments.get(index)
    }
    
    pub fn set_argument(&mut self, index: usize, value: Value) -> Result<(), String> {
        if index < self.arguments.len() {
            self.arguments[index] = value;
            Ok(())
        } else {
            Err(format!("Invalid argument index: {}", index))
        }
    }
    
    pub fn get_this(&self) -> Option<&Value> {
        self.this_value.as_ref()
    }
    
    pub fn set_this(&mut self, value: Value) {
        self.this_value = Some(value);
    }
    
    pub fn get_function(&self) -> Option<&FunctionInfo> {
        self.function.as_ref()
    }
    
    pub fn add_environment(&mut self, env: EnvironmentInfo) -> usize {
        let index = self.scope_chain.len();
        self.scope_chain.push(env);
        index
    }
    
    pub fn get_environment(&self, index: usize) -> Option<&EnvironmentInfo> {
        self.scope_chain.get(index)
    }
    
    pub fn get_environment_mut(&mut self, index: usize) -> Option<&mut EnvironmentInfo> {
        self.scope_chain.get_mut(index)
    }
    
    pub fn resolve_binding(&self, name: &str) -> Option<&Value> {
        for env in self.scope_chain.iter().rev() {
            if let Some(scope) = &env.scope {
                if let Some(binding) = scope.bindings.get(name) {
                    return Some(&binding.value);
                }
            }
            if let Some(obj) = &env.object {
                if let Some(value) = obj.properties.get(name) {
                    return Some(value);
                }
            }
        }
        None
    }
    
    pub fn set_binding(&mut self, name: &str, value: Value) -> Result<(), String> {
        for env in self.scope_chain.iter_mut().rev() {
            if let Some(scope) = &mut env.scope {
                if let Some(binding) = scope.bindings.get_mut(name) {
                    if binding.mutable {
                        binding.value = value;
                        return Ok(());
                    } else {
                        return Err(format!("Cannot assign to immutable binding: {}", name));
                    }
                }
            }
            if let Some(obj) = &mut env.object {
                obj.properties.insert(name.to_string(), value);
                return Ok(());
            }
        }
        Err(format!("Identifier '{}' is not defined", name))
    }
    
    pub fn create_binding(&mut self, name: String, value: Value, mutable: bool, deletable: bool, strict: bool) -> Result<(), String> {
        if let Some(env) = self.scope_chain.last_mut() {
            if let Some(scope) = &mut env.scope {
                if scope.bindings.contains_key(&name) {
                    return Err(format!("Identifier '{}' has already been declared", name));
                }
                scope.bindings.insert(name, BindingInfo {
                    value,
                    mutable,
                    deletable,
                    strict,
                });
                return Ok(());
            }
        }
        Err("No scope available for binding".to_string())
    }
    
    pub fn set_exception_handler(&mut self, handler: ExceptionHandler) {
        self.exception_handler = Some(handler);
    }
    
    pub fn get_exception_handler(&self) -> Option<&ExceptionHandler> {
        self.exception_handler.as_ref()
    }
    
    pub fn set_generator_state(&mut self, state: GeneratorState) {
        self.generator_state = Some(state);
    }
    
    pub fn get_generator_state(&self) -> Option<&GeneratorState> {
        self.generator_state.as_ref()
    }
    
    pub fn get_generator_state_mut(&mut self) -> Option<&mut GeneratorState> {
        self.generator_state.as_mut()
    }
    
    pub fn is_function_frame(&self) -> bool {
        self.function.is_some()
    }
    
    pub fn is_generator_frame(&self) -> bool {
        self.generator_state.is_some()
    }
    
    pub fn is_async_frame(&self) -> bool {
        self.function.as_ref().map(|f| f.async).unwrap_or(false)
    }
    
    pub fn is_strict_frame(&self) -> bool {
        self.function.as_ref().map(|f| f.strict).unwrap_or(false)
    }
    
    pub fn get_local_count(&self) -> usize {
        self.locals.len()
    }
    
    pub fn get_argument_count(&self) -> usize {
        self.arguments.len()
    }
    
    pub fn get_scope_chain_length(&self) -> usize {
        self.scope_chain.len()
    }
}

impl FunctionInfo {
    pub fn new(name: String, param_count: usize, local_count: usize, max_stack: usize) -> Self {
        Self {
            name,
            param_count,
            local_count,
            max_stack,
            strict: false,
            generator: false,
            async: false,
        }
    }
    
    pub fn strict(mut self) -> Self {
        self.strict = true;
        self
    }
    
    pub fn generator(mut self) -> Self {
        self.generator = true;
        self
    }
    
    pub fn async(mut self) -> Self {
        self.async = true;
        self
    }
}

impl EnvironmentInfo {
    pub fn new(environment_type: EnvironmentType) -> Self {
        Self {
            environment_type,
            object: None,
            scope: None,
        }
    }
    
    pub fn with_object(mut self, object: ObjectInfo) -> Self {
        self.object = Some(object);
        self
    }
    
    pub fn with_scope(mut self, scope: ScopeInfo) -> Self {
        self.scope = Some(scope);
        self
    }
    
    pub fn is_global_environment(&self) -> bool {
        matches!(self.environment_type, EnvironmentType::Global)
    }
    
    pub fn is_function_environment(&self) -> bool {
        matches!(self.environment_type, EnvironmentType::Function)
    }
    
    pub fn is_block_environment(&self) -> bool {
        matches!(self.environment_type, EnvironmentType::Block)
    }
    
    pub fn is_catch_environment(&self) -> bool {
        matches!(self.environment_type, EnvironmentType::Catch)
    }
    
    pub fn is_with_environment(&self) -> bool {
        matches!(self.environment_type, EnvironmentType::With)
    }
    
    pub fn is_module_environment(&self) -> bool {
        matches!(self.environment_type, EnvironmentType::Module)
    }
}

impl ObjectInfo {
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
            prototype: None,
        }
    }
    
    pub fn with_prototype(mut self, prototype: usize) -> Self {
        self.prototype = Some(prototype);
        self
    }
    
    pub fn set_property(&mut self, name: String, value: Value) {
        self.properties.insert(name, value);
    }
    
    pub fn get_property(&self, name: &str) -> Option<&Value> {
        self.properties.get(name)
    }
    
    pub fn delete_property(&mut self, name: &str) -> bool {
        self.properties.remove(name).is_some()
    }
}

impl ScopeInfo {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
            outer: None,
        }
    }
    
    pub fn with_outer(mut self, outer: usize) -> Self {
        self.outer = Some(outer);
        self
    }
    
    pub fn add_binding(&mut self, name: String, binding: BindingInfo) -> Result<(), String> {
        if self.bindings.contains_key(&name) {
            return Err(format!("Binding '{}' already exists", name));
        }
        self.bindings.insert(name, binding);
        Ok(())
    }
    
    pub fn get_binding(&self, name: &str) -> Option<&BindingInfo> {
        self.bindings.get(name)
    }
    
    pub fn set_binding(&mut self, name: &str, value: Value) -> Result<(), String> {
        if let Some(binding) = self.bindings.get_mut(name) {
            if binding.mutable {
                binding.value = value;
                Ok(())
            } else {
                Err(format!("Cannot assign to immutable binding: {}", name))
            }
        } else {
            Err(format!("Binding '{}' not found", name))
        }
    }
}

impl BindingInfo {
    pub fn new(value: Value, mutable: bool, deletable: bool, strict: bool) -> Self {
        Self {
            value,
            mutable,
            deletable,
            strict,
        }
    }
    
    pub fn immutable(value: Value) -> Self {
        Self {
            value,
            mutable: false,
            deletable: false,
            strict: true,
        }
    }
    
    pub fn mutable(value: Value) -> Self {
        Self {
            value,
            mutable: true,
            deletable: true,
            strict: false,
        }
    }
}

impl ExceptionHandler {
    pub fn new(try_pc: usize, catch_pc: usize) -> Self {
        Self {
            try_pc,
            catch_pc,
            finally_pc: None,
            exception_var: None,
        }
    }
    
    pub fn with_finally(mut self, finally_pc: usize) -> Self {
        self.finally_pc = Some(finally_pc);
        self
    }
    
    pub fn with_exception_var(mut self, var: String) -> Self {
        self.exception_var = Some(var);
        self
    }
}

impl GeneratorState {
    pub fn new() -> Self {
        Self {
            state: GeneratorStateType::SuspendedStart,
            yielded_value: None,
            return_value: None,
            thrown_value: None,
        }
    }
    
    pub fn set_state(&mut self, state: GeneratorStateType) {
        self.state = state;
    }
    
    pub fn set_yielded_value(&mut self, value: Value) {
        self.yielded_value = Some(value);
    }
    
    pub fn set_return_value(&mut self, value: Value) {
        self.return_value = Some(value);
    }
    
    pub fn set_thrown_value(&mut self, value: Value) {
        self.thrown_value = Some(value);
    }
    
    pub fn is_suspended(&self) -> bool {
        matches!(self.state, GeneratorStateType::SuspendedStart | GeneratorStateType::SuspendedYield)
    }
    
    pub fn is_executing(&self) -> bool {
        matches!(self.state, GeneratorStateType::Executing)
    }
    
    pub fn is_completed(&self) -> bool {
        matches!(self.state, GeneratorStateType::Completed)
    }
} 