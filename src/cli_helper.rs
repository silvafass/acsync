//! **cli_helper** contains simple and useful functions to support simple CLI software
//! to work with command arguments and options.

/// Returns [`Some<String>`] corresponding to the index argument.
///
/// Returns [`None`] if there is no argument value at the given index.
///
/// # Examples
///
/// ```
/// # use acsync::cli_helper;
/// #
/// let args: Vec<String> = "command foo bar --baz=qux --debug"
///                         .split_whitespace()
///                         .map(|value| value.to_string())
///                         .skip(1)
///                         .collect();
///
/// assert_eq!(
///     cli_helper::get_argument(0, &args),
///     Some(&String::from("foo"))
/// );
/// ```
pub fn get_argument(index: usize, args: &[String]) -> Option<&String> {
    args.get(index).filter(|value| !value.starts_with("--"))
}

/// Returns ([`bool`], [`Some<usize>`]) if args contains the option name.
///
/// # Examples
///
/// ```
/// # use acsync::cli_helper;
/// #
/// let args: Vec<String> = "command foo bar --baz=qux --debug"
///                         .split_whitespace()
///                         .map(|value| value.to_string())
///                         .skip(1)
///                         .collect();
///
/// assert_eq!(cli_helper::has_option("debug", &args), (true, Some(3)));
/// ```
pub fn has_option(name: &str, args: &[String]) -> (bool, Option<usize>) {
    let index = args
        .iter()
        .position(|value| value.starts_with(&format!("--{name}")));
    // let a = args.iter().enumerate().filter(|(index, ..)| index != index).map(|(.., value)| value);
    (index.is_some(), index)
}

/// Returns ([`Some<&str>`], [`Some<usize>`]) corresponding to the option name.
///
/// Returns ([`None`], [`None`]) if there is no option value matching the option name.
///
/// # Examples
///
/// ```
/// # use acsync::cli_helper;
/// #
/// let args: Vec<String> = "command foo bar --baz=qux --debug"
///                         .split_whitespace()
///                         .map(|value| value.to_string())
///                         .skip(1)
///                         .collect();
///
/// assert_eq!(
///     cli_helper::get_option_value("baz", &args),
///     (Some("qux"), Some(2))
/// );
/// ```
pub fn get_option_value<'a>(name: &str, args: &'a [String]) -> (Option<&'a str>, Option<usize>) {
    let mut value_index = 0;
    let mut arguments_iter = args.iter().enumerate();
    (
        arguments_iter
            .find(|(.., value)| value.starts_with(&format!("--{name}")))
            .inspect(|(index, ..)| value_index = *index)
            .and_then(|(.., value)| value.strip_prefix(&format!("--{name}=")))
            .or(arguments_iter
                .take(1)
                .find(|(.., value)| !value.starts_with("--"))
                .inspect(|(index, ..)| value_index = *index)
                .map(|(.., value)| &value[..])),
        Some(value_index),
    )
}

pub type Arg<T> = Option<T>;

pub trait ArgsParser {
    fn debug(&self) -> bool;

    fn print_help(&self);

    fn describe(command_name: &str) -> String;

    fn parse_slice(args: &[String]) -> Self;

    fn parse() -> Self
    where
        Self: Sized,
    {
        Self::parse_slice(&std::env::args().skip(1).collect::<Vec<String>>())
    }
}

#[macro_export]
macro_rules! create_args_parser {
    (
        $(@attr #[$enum_meta:meta])?
        $(#[doc = $doc_literal:literal])*
        $vis_enum:vis enum $ident_enum:ident {
            $($(#[doc = $literal_command_description:literal])*
            $ident_command:ident {
                $(
                    $(#[doc = $literal_parameter_description:literal])*
                    $ident_parameter:ident: $ty_parameter:ty
                ),* $(,)?
            }),*  $(,)?
           $(@default $ident_default_command:ident {
                $(
                    $(#[doc = $literal_default_parameter_description:literal])*
                    $ident_default_parameter:ident: $ty_default_parameter:ty
                ),* $(,)?
            } $(,)? )?
        }
    ) => {
        $(#[$enum_meta])?
        $(#[doc = $doc_literal])*
        $vis_enum enum $ident_enum {
            $($(#[doc = $literal_command_description])*
            $ident_command {
                $(
                    $(#[doc = $literal_parameter_description])*
                    $ident_parameter: $ty_parameter,
                )*
                debug: Option<bool>,
            },)*
            $($ident_default_command {
                $(
                    $(#[doc = $literal_default_parameter_description])*
                    $ident_default_parameter: $ty_default_parameter,
                )*
                debug: Option<bool>,
            })?
        }

        impl $crate::cli_helper::ArgsParser for $ident_enum {

            fn debug(&self) -> bool {
                match &self {
                    $($ident_enum::$ident_command { debug, .. } => debug.unwrap_or_default(),)*
                    $($ident_enum::$ident_default_command { debug, .. } => debug.unwrap_or_default(),)?
                }
            }

            fn print_help(&self) {
                match &self {
                    $($ident_enum::$ident_command { .. } => {
                        println!("{}", $ident_enum::describe(stringify!($ident_command)))
                    },)*
                    $($ident_enum::$ident_default_command { .. } => {
                        println!("{}", $ident_enum::describe(stringify!($ident_default_command)))
                    },)?
                };
            }

            fn describe(command_name: &str) -> String {
                let crate_name = env!("CARGO_PKG_NAME");
                let mut description = String::new();

                let mut all_parameters = vec![
                    $($(stringify!($ident_parameter),)*)*
                    $($(stringify!($ident_default_parameter),)*)*
                    "debug",
                ];
                let parameter_width = all_parameters.iter().map(|item| item.len()).max().unwrap() + 2;

                let mut parameter_description_map = std::collections::HashMap::from([
                    ("debug", "Enable debug mode".to_string()),
                ]);

                match command_name {
                    $(stringify!($ident_command) => {
                        let command_name = &stringify!($ident_command).to_lowercase();

                        let mut arg_parameters: Vec<&str> = vec![];
                        let mut opt_parameters: Vec<&str> = vec![];
                        $(
                        let parameter_descriptions: [&str; _] = [$($literal_parameter_description.trim_start(),)*];
                        parameter_description_map.insert(stringify!($ident_parameter), parameter_descriptions.join(" "));
                        if stringify!($ty_parameter).starts_with("Arg") {
                            arg_parameters.push(stringify!($ident_parameter));
                        }
                        else {
                            opt_parameters.push(stringify!($ident_parameter));
                        }
                        )*
                        opt_parameters.push("debug");

                        $(description += &format!("{}\n", $literal_command_description).trim_start();)*
                        description += "\n";
                        description += &format!(
                            "Usage: {} {} [OPTIONS] [ARGS]...\n", crate_name, command_name,
                        ).as_str();
                        if !arg_parameters.is_empty() {
                            description += "\n";
                            description += "Arguments:\n";
                            let parameter_width = parameter_width + 2;
                            for arg_name in &arg_parameters {
                                description += &format!("\t{:<parameter_width$}", arg_name).as_str();
                                description += format!("{}\n", parameter_description_map.get(arg_name).unwrap()).as_str();
                            }
                        }
                        if !opt_parameters.is_empty() {
                            description += "\n";
                            description += "Options:\n";
                            for opt_name in &opt_parameters {
                                description += &format!("\t--{:<parameter_width$}", opt_name).as_str();
                                description += format!("{}\n", parameter_description_map.get(opt_name).unwrap()).as_str();
                            }
                        }

                        description
                    },)*
                    _ => {
                        let command_names: Vec<&str> = vec![$(stringify!($ident_command)),*];
                        $(
                        let command_descriptions: [&str; _] = [$($literal_command_description.trim_start(),)*];
                        parameter_description_map.insert(stringify!($ident_command), command_descriptions.join(" "));
                        )*
                        let mut arg_parameters: Vec<&str> = vec![];
                        let mut opt_parameters: Vec<&str> = vec![];
                        $($(
                        let default_parameter_descriptions: [&str; _] = [$($literal_default_parameter_description.trim_start(),)*];
                        parameter_description_map.insert(stringify!($ident_default_parameter), default_parameter_descriptions.join(" "));
                        if stringify!($ty_default_parameter).starts_with("Arg") {
                            arg_parameters.push(stringify!($ident_default_parameter));
                        }
                        else {
                            opt_parameters.push(stringify!($ident_default_parameter));
                        }
                        )*)*
                        opt_parameters.push("debug");

                        $(description += &format!("{}\n", $doc_literal).trim_start())*;
                        description += "\n";
                        description += &format!(
                            "Usage: {} [COMMAND] [OPTIONS] [ARGS]...\n", crate_name,
                        ).as_str();
                        if !command_names.is_empty() {
                            description += "\n";
                            description += "Commands:\n";
                            let parameter_width = parameter_width + 2;
                            for command_name in &command_names {
                                description += &format!("\t{:<parameter_width$}", command_name.to_lowercase()).as_str();
                                description += format!("{}\n", parameter_description_map.get(command_name).unwrap()).as_str();
                            }
                        }
                        if !arg_parameters.is_empty() {
                            description += "\n";
                            description += "Arguments:\n";
                            let parameter_width = parameter_width + 2;
                            for arg_name in &arg_parameters {
                                description += &format!("\t{:<parameter_width$}", arg_name).as_str();
                                description += format!("{}\n", parameter_description_map.get(arg_name).unwrap()).as_str();
                            }
                        }
                        if !opt_parameters.is_empty() {
                            description += "\n";
                            description += "Options:\n";
                            for opt_name in &opt_parameters {
                                description += &format!("\t--{:<parameter_width$}", opt_name).as_str();
                                description += format!("{}\n", parameter_description_map.get(opt_name).unwrap()).as_str();
                            }
                        }

                        description
                    }
                }
            }

            fn parse_slice(args: &[String]) -> Self {
                let mut indexes_found: std::collections::HashSet<usize>  = std::collections::HashSet::new();

                let debug = if let (has_option, Some(index)) = cli_helper::has_option("debug", &args) {
                    indexes_found.insert(index);
                    has_option
                } else {
                    false
                };

                let command_name_map: std::collections::HashMap<String, &str> = std::collections::HashMap::from([
                    $((stringify!($ident_command).to_lowercase(), stringify!($ident_command)),)*
                ]);
                let command_name = cli_helper::get_argument(0, &args);

                if let (true, ..)  = cli_helper::has_option("help", &args) {
                    println!("{}", $ident_enum::describe(
                        command_name_map.get(command_name.unwrap_or(&"__".to_string())).unwrap_or(&"__")
                    ));
                    std::process::exit(0);
                }

                let mut argument_index = 0;

                let command_names: Vec<&str> = vec![$(stringify!($ident_command)),*];
                if !command_names.is_empty() {
                    argument_index = 1;
                }

                let mut get = |field_name: &str, field_type: &str| {
                    let mut value = None;
                    if (field_type.starts_with("Arg")) {
                        value = cli_helper::get_argument(argument_index, &args).cloned();
                        if value.is_some() {
                            indexes_found.insert(argument_index);
                        }
                        argument_index += 1;
                    }
                    else {
                        let (has_option, option_index) = cli_helper::has_option(field_name, &args);
                        if has_option {
                            let (option_value, option_index) = cli_helper::get_option_value(field_name, &args);
                            if let Some(option_index) = option_index {
                                indexes_found.insert(option_index);
                            }
                            value = option_value.map(String::from);
                        }
                        if value.is_none() {
                            value = Some(has_option.to_string());
                        }
                        if let Some(option_index) = option_index {
                            indexes_found.insert(option_index);
                        }
                    }
                    value
                };

                let command = match command_name {
                    $(Some(command_name) if command_name == &stringify!($ident_command).to_lowercase() => {
                        $ident_enum::$ident_command {
                            $($ident_parameter: match get(stringify!($ident_parameter), stringify!($ty_parameter)) {
                                Some(value) => Some(value.parse().unwrap_or_default()),
                                None => Default::default()
                            },)*
                            debug: Some(debug),
                        }
                    })*
                    $(_ if command_name.is_none() || command_names.is_empty() => $ident_enum::$ident_default_command {
                        $($ident_default_parameter: match get(stringify!($ident_default_parameter), stringify!($ty_default_parameter)) {
                            Some(value) => Some(value.parse().unwrap_or_default()),
                            None => Default::default()
                        },)*
                        debug: Some(debug),
                    },)?
                    _ => {
                        eprintln!("ERROR: Command {:?} not found!", command_name.unwrap_or(&"None".to_string()));
                        std::process::exit(1);
                    }
                };

                indexes_found.insert(0);

                let reaming: Vec<&String> = args
                                        .iter().enumerate()
                                        .filter(|(index, ..)| !indexes_found.contains(index))
                                        .map(|(.., value)| value)
                                        .collect();
                if (!reaming.is_empty()) {
                    eprintln!("ERROR: Not recognized arguments! {:?}", reaming);
                    std::process::exit(1);
                }

                command
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(command_call: &str, skip: usize) -> Vec<String> {
        command_call
            .split_whitespace()
            .map(|value| value.to_string())
            .skip(skip)
            .collect()
    }

    #[test]
    fn it_finds_argument_foo() {
        let args = parse("command foo bar --baz=qux --debug", 1);
        assert_eq!(get_argument(0, &args), Some(&String::from("foo")));
    }

    #[test]
    fn it_does_not_find_argument_foo() {
        let args = parse("command bar --baz=qux --debug", 1);
        assert_ne!(get_argument(0, &args), Some(&String::from("foo")));
    }

    #[test]
    fn it_does_not_find_none_argument() {
        let args = parse("command --baz=qux --debug", 1);
        assert_eq!(get_argument(0, &args), None);
    }

    #[test]
    fn it_finds_debug_option() {
        let args: Vec<String> = parse("command foo bar --baz=qux --debug", 1);
        assert_eq!(has_option("debug", &args), (true, Some(3)));
    }

    #[test]
    fn it_does_not_find_debug_option() {
        let args: Vec<String> = parse("command foo bar --baz=qux", 1);
        assert_eq!(has_option("debug", &args), (false, None));
    }

    #[test]
    fn it_finds_baz_option_value() {
        let args: Vec<String> = parse("command foo bar --baz=qux --debug", 1);
        assert_eq!(get_option_value("baz", &args), (Some("qux"), Some(2)));
    }

    #[test]
    fn it_does_not_find_baz_option_value() {
        let args: Vec<String> = parse("command foo bar --fred=qux --debug", 1);
        assert_ne!(get_option_value("baz", &args), (Some("qux"), Some(2)));
    }
}
