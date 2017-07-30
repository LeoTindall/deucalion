//! Tests for the basic operations on scripting environments

#[test]
fn test_scripting_execution_basic() {
    use scripting::basic::get_scripting_environment;
    // Get an environment to work with
    let mut environment = get_scripting_environment(); 
    // Set a variable x = 2 in the environment
    environment.set("x", 2);
    // Modify the variable with a Lua script; specifically, add one to x
    match environment.execute::<()>("x = x + 1") {
        Ok(v) => println!("Successfully executed the code: {:?}", v),
        Err(e) => panic!("Failed to execute() the code: {:?}", e),

    }
    // Retrieve the variable. Type annotation is required because Lua is dynamically typed.
    let x: i32 = environment.get("x").unwrap();
    assert_eq!(
        x,
        3,
        "Expected Lua code 'x = x + 1' with 'x' set to 2 to result in 3; instead, got {}.",
        x
    );
}
