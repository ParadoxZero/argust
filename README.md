# Argust
A simple command line parser which will accept the vector of program arguments and return a sorted structure which can be used to direct execution of the program.

Argust is aimed towards smaller utilities that want to avoid the added conplexities due to powerful flexiblity and input validations offered by declarative or derivative arg parsers. This is a quick and simple library to handle basic arg handling needs.

No need to design your input, prepare arguement hierarchy. No specific order and updating the params just require adding the string to a list.
 
> There is no support for string containing non-valid unicode.

In the context of Argust the command line is divided into three parts - 
* Arguments - args which represent either commands or parameters for the program. e.g. `cp test.rs test2.rs` here "test.rs" and "test2.rs" are the arguements.
* Short Parameters - These are single character switches which can modify the behaviour of program e.g. `cp test.rs test2.rs -f` here "-f" is a switch. These can also contain values to pass to program. e.g. `./test -t cpp`, Here cpp can be a value being passed to program.
* Long Paramters - Behaves same as short parameter. Except these are typed out. Short params are generally shortcuts to long_params e.g. `cp test.rs test2.rs --buffer-size=2048` here "--buffer-size=2048" updates the default copy buffer to use the value of 2048 instead.

# How to use

## Basic Usage
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
    pub long_params: HashMap<String, Option<String>>,
    pub short_params: HashMap<String, Option<String>>,
    pub args: Vec<String>,
}
```

Please note that `options` doesn't necessarily require to be a key value pair. i.e. `cp --version` will return a hashmap with key 'version' and value None.

## Configuring parameter style

It's even possible to override the default parse token, i.e. "-" for switch and "--" for options by using the following object and passing it along with `parse_args` method.
```rust
pub struct ParseToken {
    pub option: String,
    pub option_key: String,
    pub switch: String,
}
```
In the command `testApp hello --print=stdin -f`. "--" is `option`, "=" is `option_key` and "-" is `switch`.

The default configuration of parser is - 
```rust
ParseTokens {
    long_token: new_string!("--"),
    long_seperator: new_string!("="),
    short_token: new_string!("-"),
}
```

We need to set the `parameterized_switch_list` field of `ParserConfig` in order to specify which short parameters have values associated with them.

Which means this is how parser will behave - 
`./test -h myval --long=MyLongVal command`
This will generate - `args = ["command"], short=[{"h", "myval"}], long=[{"long","MyLongVal"}]`
The relative order is not important.

This is also valid input as long as `parameterized_switch_list` is configured - `./test -hMyval`

## Accepting space seperated long parameters

In order to modify the parser to accept space seperated long values e.g. `./test --long MyLongValue command` we need to update the config as follows - 
```rust
let mut parse_config = ParserConfig::new();
parse_config.parse_tokens = ParseTokens {
    long_token: String::from("--"),
    long_seperator: String::from(" "),
    short_token: String::from("-"),
};
```

> Please note: If we set `long_seperator` as " ", then it's important to set the `parameterized_option_list` of `ParserConfig` to specify which long parameters will have values. Otherwise these values will be recieved as commands. For any `long_seperator` value other than " ". This field will be ignored since the value will be generated by spliting the parameter.

## Querying the parameters

The call to `parse_args` will return an object of `ArgContext`. This object can be used to extract information about passed arguements e.g.
```rust
let context: ArgContext = commands::parse_args(args, None);
let (isTest, testPath) context.contains_short('t');
if isTest {
    println!("the path is {}", testPath.unwrap());
}
```

Query helper methods reference are - 
```rust
pub fn contains_short(&self, short: char) -> (bool, Option<String>);
pub fn contains_long(&self, long: &str) -> (bool, Option<String>);
pub fn contains(&self, short: Option<char>, long: Option<&str>) -> (bool, Option<String>) ;
```

Other than that, you can also simply query the status directly via the data in `ArgContext`.
```rust
let context: ArgContext = commands::parse_args(args, None);
match context.args.first() {
    "test" => run_test(),
    "validate" => run_validate(),
    _ => print_help()
};
```