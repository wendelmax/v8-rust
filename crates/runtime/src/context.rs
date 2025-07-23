// Execution context for ECMAScript

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use super::value::Value;
use super::object::Object;
use super::scope::Scope;
use super::environment::Environment;

#[derive(Debug, Clone)]
pub enum ContextType {
    Global,
    Function,
    Eval,
    Module,
}

#[derive(Debug, Clone)]
pub struct Context {
    pub context_type: ContextType,
    pub lexical_environment: Rc<RefCell<Environment>>,
    pub variable_environment: Rc<RefCell<Environment>>,
    pub this_binding: Value,
    pub strict: bool,
    pub function: Option<Rc<RefCell<super::function::Function>>>,
    pub script_or_module: Option<String>,
    pub realm: Option<Rc<RefCell<Realm>>>,
    pub generator: Option<Rc<RefCell<Generator>>>,
}

#[derive(Debug, Clone)]
pub struct Realm {
    pub intrinsics: HashMap<String, Value>,
    pub global_object: Rc<RefCell<Object>>,
    pub global_environment: Rc<RefCell<Environment>>,
    pub template_map: HashMap<String, Value>,
    pub host_defined: Option<Value>,
}

#[derive(Debug, Clone)]
pub struct Generator {
    pub generator_object: Rc<RefCell<Object>>,
    pub generator_context: Rc<RefCell<Context>>,
    pub generator_state: GeneratorState,
    pub generator_queue: Vec<Value>,
}

#[derive(Debug, Clone)]
pub enum GeneratorState {
    SuspendedStart,
    SuspendedYield,
    Executing,
    Completed,
}

impl Context {
    pub fn new(global_object: Rc<RefCell<Object>>) -> Self {
        let global_env = Environment::new_global(global_object.clone());
        
        Self {
            context_type: ContextType::Global,
            lexical_environment: global_env.clone(),
            variable_environment: global_env,
            this_binding: Value::Object(global_object.borrow().clone()),
            strict: false,
            function: None,
            script_or_module: None,
            realm: None,
            generator: None,
        }
    }
    
    pub fn function_context(
        function: Rc<RefCell<super::function::Function>>,
        this_binding: Value,
        lexical_env: Rc<RefCell<Environment>>,
        strict: bool,
    ) -> Self {
        Self {
            context_type: ContextType::Function,
            lexical_environment: lexical_env.clone(),
            variable_environment: lexical_env,
            this_binding,
            strict,
            function: Some(function),
            script_or_module: None,
            realm: None,
            generator: None,
        }
    }
    
    pub fn eval_context(
        calling_context: &Context,
        strict: bool,
    ) -> Self {
        Self {
            context_type: ContextType::Eval,
            lexical_environment: calling_context.lexical_environment.clone(),
            variable_environment: calling_context.variable_environment.clone(),
            this_binding: calling_context.this_binding.clone(),
            strict,
            function: None,
            script_or_module: None,
            realm: calling_context.realm.clone(),
            generator: None,
        }
    }
    
    pub fn module_context(
        module: String,
        lexical_env: Rc<RefCell<Environment>>,
        global_object: Rc<RefCell<Object>>,
    ) -> Self {
        Self {
            context_type: ContextType::Module,
            lexical_environment: lexical_env.clone(),
            variable_environment: lexical_env,
            this_binding: Value::Undefined,
            strict: true,
            function: None,
            script_or_module: Some(module),
            realm: None,
            generator: None,
        }
    }
    
    pub fn get_this_binding(&self) -> Value {
        self.this_binding.clone()
    }
    
    pub fn set_this_binding(&mut self, this: Value) {
        self.this_binding = this;
    }
    
    pub fn get_lexical_environment(&self) -> Rc<RefCell<Environment>> {
        self.lexical_environment.clone()
    }
    
    pub fn set_lexical_environment(&mut self, env: Rc<RefCell<Environment>>) {
        self.lexical_environment = env;
    }
    
    pub fn get_variable_environment(&self) -> Rc<RefCell<Environment>> {
        self.variable_environment.clone()
    }
    
    pub fn set_variable_environment(&mut self, env: Rc<RefCell<Environment>>) {
        self.variable_environment = env;
    }
    
    pub fn is_strict(&self) -> bool {
        self.strict
    }
    
    pub fn set_strict(&mut self, strict: bool) {
        self.strict = strict;
    }
    
    pub fn get_function(&self) -> Option<Rc<RefCell<super::function::Function>>> {
        self.function.clone()
    }
    
    pub fn set_function(&mut self, function: Rc<RefCell<super::function::Function>>) {
        self.function = Some(function);
    }
    
    pub fn get_script_or_module(&self) -> Option<&str> {
        self.script_or_module.as_deref()
    }
    
    pub fn set_script_or_module(&mut self, script_or_module: String) {
        self.script_or_module = Some(script_or_module);
    }
    
    pub fn get_realm(&self) -> Option<Rc<RefCell<Realm>>> {
        self.realm.clone()
    }
    
    pub fn set_realm(&mut self, realm: Rc<RefCell<Realm>>) {
        self.realm = Some(realm);
    }
    
    pub fn get_generator(&self) -> Option<Rc<RefCell<Generator>>> {
        self.generator.clone()
    }
    
    pub fn set_generator(&mut self, generator: Rc<RefCell<Generator>>) {
        self.generator = Some(generator);
    }
    
    pub fn resolve_binding(&self, name: &str) -> Result<Value, String> {
        self.lexical_environment.borrow().get_binding_value(name)
    }
    
    pub fn get_identifier_reference(&self, name: &str) -> Result<Value, String> {
        self.lexical_environment.borrow().get_binding_value(name)
    }
    
    pub fn has_super_binding(&self) -> bool {
        if let Some(function) = &self.function {
            // TODO: Check if function has super binding
            false
        } else {
            false
        }
    }
    
    pub fn get_this_environment(&self) -> Rc<RefCell<Environment>> {
        // TODO: Find the nearest environment that provides a this binding
        self.lexical_environment.clone()
    }
    
    pub fn get_new_target(&self) -> Option<Value> {
        // TODO: Get the new.target value
        None
    }
    
    pub fn get_global_object(&self) -> Option<Rc<RefCell<Object>>> {
        match &self.context_type {
            ContextType::Global => {
                if let Value::Object(obj) = &self.this_binding {
                    Some(Rc::new(RefCell::new(obj.clone())))
                } else {
                    None
                }
            }
            _ => {
                // Find the global environment in the environment chain
                let mut env = self.lexical_environment.clone();
                loop {
                    let env_ref = env.borrow();
                    if env_ref.is_global_environment() {
                        return Some(env_ref.get_global_object());
                    }
                    if let Some(outer) = env_ref.get_outer_environment() {
                        env = outer;
                    } else {
                        break;
                    }
                }
                None
            }
        }
    }
}

impl Realm {
    pub fn new(global_object: Rc<RefCell<Object>>) -> Self {
        let global_env = Environment::new_global(global_object.clone());
        
        Self {
            intrinsics: HashMap::new(),
            global_object,
            global_environment: global_env,
            template_map: HashMap::new(),
            host_defined: None,
        }
    }
    
    pub fn get_intrinsic(&self, name: &str) -> Option<&Value> {
        self.intrinsics.get(name)
    }
    
    pub fn set_intrinsic(&mut self, name: String, value: Value) {
        self.intrinsics.insert(name, value);
    }
    
    pub fn get_global_object(&self) -> Rc<RefCell<Object>> {
        self.global_object.clone()
    }
    
    pub fn get_global_environment(&self) -> Rc<RefCell<Environment>> {
        self.global_environment.clone()
    }
    
    pub fn get_template_map(&self) -> &HashMap<String, Value> {
        &self.template_map
    }
    
    pub fn set_template_map(&mut self, template_map: HashMap<String, Value>) {
        self.template_map = template_map;
    }
    
    pub fn get_host_defined(&self) -> Option<&Value> {
        self.host_defined.as_ref()
    }
    
    pub fn set_host_defined(&mut self, value: Value) {
        self.host_defined = Some(value);
    }
}

impl Generator {
    pub fn new(
        generator_object: Rc<RefCell<Object>>,
        generator_context: Rc<RefCell<Context>>,
    ) -> Self {
        Self {
            generator_object,
            generator_context,
            generator_state: GeneratorState::SuspendedStart,
            generator_queue: Vec::new(),
        }
    }
    
    pub fn get_generator_object(&self) -> Rc<RefCell<Object>> {
        self.generator_object.clone()
    }
    
    pub fn get_generator_context(&self) -> Rc<RefCell<Context>> {
        self.generator_context.clone()
    }
    
    pub fn get_generator_state(&self) -> &GeneratorState {
        &self.generator_state
    }
    
    pub fn set_generator_state(&mut self, state: GeneratorState) {
        self.generator_state = state;
    }
    
    pub fn get_generator_queue(&self) -> &Vec<Value> {
        &self.generator_queue
    }
    
    pub fn set_generator_queue(&mut self, queue: Vec<Value>) {
        self.generator_queue = queue;
    }
    
    pub fn enqueue(&mut self, value: Value) {
        self.generator_queue.push(value);
    }
    
    pub fn dequeue(&mut self) -> Option<Value> {
        self.generator_queue.pop()
    }
} 