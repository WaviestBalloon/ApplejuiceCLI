pub fn get_param_value(command_vector: Vec<Vec<(String, String)>>, value_to_find: &str) -> String {
	for command in command_vector.iter() {
		if command[0].0 == *value_to_find {
			if command[0].1.is_empty() {
				return "blank".to_string();
			} else {
				return command[0].1.to_string();
			}
		}
	}
	String::new()
}

pub fn get_param_value_new<'v>(command_vector: &'v [(String, String)], argument_to_find: &str) -> Option<&'v str> {
	for command in command_vector.iter() {
		if command.0 == *argument_to_find {
			return Some(&command.1)
		}
	}

	None
}

pub fn parse_arguments(args: &[String]) -> Vec<(String, String)> {
	let mut arguments = vec![]; // Collected args and their values
	let mut arg_command = String::new(); // Current command parameter being collected
	let mut arg_command_value = String::new(); // Current command parameter value being collected

	for (index, argument) in args.iter().enumerate() {
		if index == 0 { continue; }

		if argument.contains("--") {
			if arg_command.is_empty() && arg_command_value.is_empty() {
				arg_command = argument.replace("--", "");
			}
		} else if arg_command_value.is_empty() {
			arg_command_value = argument.to_string(); // Construct first argument
		} else {
			arg_command_value = format!("{} {}", arg_command_value, argument); // Construct argument value and concatenate :3
		}

		// If this is the last argument or next argument is a command, push to vec
		if index == args.len() - 1 || args[index + 1].contains("--") {
			arguments.push((arg_command, arg_command_value));
			arg_command = String::new();
			arg_command_value = String::new();
		}
	}

	arguments
}
