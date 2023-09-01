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

pub fn get_param_value_new<'v>(command_vector: &'v [(String, String)], argument_to_find: &str) -> Option<&'v String> {
	for command in command_vector.iter() {
		if command.0 == *argument_to_find {
			return Some(&command.1)
		}
	}

	None
}
