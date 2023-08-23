pub fn get_param_value(command_vector: Vec<Vec<(String, String)>>, value_to_find: &str) -> String {
	for command in command_vector.iter() {
		if command[0].0 == value_to_find.to_string() {
			return command[0].1.to_string();
		}
	}
	return String::new();
}
