# Argust
A simple command line parser which will accept the vector of program arguments and return a sorted structure which can be used to direct execution of the program.

The command line is divided into three parts - 
* Arguments - args which represent either commands or parameters for the program. e.g. `cp test.rs test2.rs` here "test.rs" and "test2.rs" are the arguements.
* Switches - as the name described. These are switches which can modify the behaviour of program e.g. `cp test.rs test2.rs -f` here "-f" is a switch.
* Options - like switch options are modifiable parameters for the program and can be used to update default settings/ configurations e.g. `cp test.rs test2.rs --buffer-size=2048` here "--buffer-size=2048" updates the default copy buffer to use the value of 2048 instead.

# How to use
The usage is very straight forward.
```rust
    let args: Vec<String> = env::args().skip(1).collect();
    let command_set: ArgContext = commands::parse_args(args, None);

    // Use the values as required
    if let Some(command) = command_set.commands.first() {
        handle_command(command);
    } else {
        basic_commands::help();
    }
```

The `ArgContext` object contains structured information about arguments which were passed. It's defined as-
```rust
pub struct ArgContext {
    pub options: HashMap<String, String>,
    pub switches: Vec<String>,
    pub args: Vec<String>,
}
```

Please note that `options` doesn't necessarily require to be a key value pair. i.e. `cp --version` will return a hashmap with key 'version' and value empty.

It's even possible to override the default parse token, i.e. "-" for switched and "--" for options by using the following object and passing it along with `parse_args` method.
```rust
pub struct ParseToken {
    pub option: String,
    pub option_key: String,
    pub switch: String,
}
```
In the command `testApp hello --print=stdin -f`. "--" is `option`, "=" is `option_key` and "-" is `switch`.