extern crate serialize;
use std::os;
use std::vec;
use std::io::{
	IoError,
	File,
	FileMode,
	FileAccess,
};
use std::collections::{
	HashMap,
	BTreeMap
};
use self::serialize::json;
use self::serialize::json::{
	ToJson,
	Json,
	encode,
};

pub struct World{
	w: u32,
	h: u32,
	pub data: HashMap<(u32,u32), vec::Vec<String>>,
}

impl World{
	pub fn new(width: u32, height: u32) -> Result<World, String> {
		let mut data_map:HashMap<(u32,u32), vec::Vec<String>> = HashMap::<(u32,u32), vec::Vec<String>>::new();
		for x in range(0u32, width) {
			for y in range(0u32, height){
				match data_map.insert((x,y), Vec::new()){
					Some(_)	=> return Err("Was not able to insert empty Vec into HashMap".to_string()),
					None	=>	continue,
				};
			}
		}
		Ok(World{w:width, h:height, data:data_map})
	}

	pub fn from_file(path_to_json_file: Path) -> Result<World, String> {
		let current_base_directory = match os::getcwd() {
			Ok(x) 	=>	x,
			Err(x)	=>	return Err(format!("Could not obtain the current working directory, error: {}", x)),
		};
		let final_load_path = current_base_directory.join(path_to_json_file);
		let file_name = final_load_path.display();
		//open the file
		let mut json_file = match File::open_mode(&final_load_path, FileMode::Open, FileAccess::Read){
			Ok(f)	=>	f,
			Err(e)	=>	return Err(format!("File at path {} has an error of the kind {}", file_name, e.kind)),
		};
		//read the contents
		let json_file_contents = match json_file.read_to_end(){
			Ok(c)	=>	c,
			Err(e)	=>	return Err(format!("File at path {} was not able to be read, reason: {}", file_name, e.kind)),
		};
		//make it into a String object
		let json_string = match String::from_utf8(json_file_contents){
			Ok(s)	=>	s,
			Err(_)	=>	return Err(format!("Vec of u8 from path {} was not able to be converted into a String", file_name)),
		};
		//make it into a JSON enum
		let json_object = match json::from_str(json_string.as_slice()){
			Ok(j)	=>	j,
			Err(e)	=>	return Err(format!("Json file at {} is corrupted, the error is {}", file_name, e)),
		};
		//turn it into something that we can use
		let map = match json_object {
			Json::Object(x) 	=> 	x,
			_			=>	return Err(format!("File at path {} is corrupted", file_name)),
		};
		//get the width Json enum
		let width_json = match map.get(&"width".to_string()) {
			Some(val)	=>	val,
			None		=>	return Err(format!("Json file at {} doesn't have a width field", file_name))
		};
		//get the width
		let width = match width_json{
			&Json::U64(x)	=>	x,
			_		=>	return Err(format!("Json file at path {} the width is not the expected type", file_name)),
		};
		//get height json enum
		let height_json = match map.get(&"height".to_string()) {
			Some(val)	=>	val,
			None		=>	return Err(format!("Json file at {} doesn't have a height field", file_name))
		};
		//get the height
		let height = match height_json{
			&Json::U64(x)	=>	x,
			_		=>	return Err(format!("Json file at path {} the height is not the expected type", file_name)),
		};
		let mut world_map = HashMap::<(u32,u32), Vec<String>>::new();
		for x in range(0, width+1) {
			let map_x_level_json = match map.get(&x.to_string()){
				Some(val)	=>	val,
				None		=>	return Err(format!("Json file at path {} doesn't have objects on x coordinate {}", file_name, x)),
			};
			let map_x_level = match map_x_level_json {
				&Json::Object(ref x) =>	x,
				_				=>	return Err(format!("Json file at path {} the x level at {} is not a json object", file_name, x)),
			};
			for y in range(0, height+1) {
				let map_y_level_json = match map_x_level.get(&y.to_string()) {
					Some(val)	=>	val,
					None		=>	return Err(format!("Json file at path {} doesn't have objects on x coordinate {}", file_name, y)),
				};
				let z_level_json = match map_y_level_json {
					&Json::Array(ref val)	=>	val,
					_						=>	return Err(format!("Json file at path {} at the coordinates ({},{}) is not a json array", file_name, x, y)),
				};
				let mut z_level = Vec::<String>::new();
				for x in z_level_json.iter(){
					z_level.push(match x.clone() {
						Json::String(val)	=>	val,
						_			=>	return Err(format!("Json file at path {} at coordinates ({},{}) there is an item that is not a string", file_name, x, y))
					});
				}
				world_map.insert((x.to_u32().unwrap(), y.to_u32().unwrap()), z_level);
			}
		}
		Ok(World{w:width.to_u32().unwrap(), h:height.to_u32().unwrap(), data:world_map})
	}

	pub fn put_room(&mut self, x: u32, y: u32, w: u32, h: u32) -> Result<(), String>{
		//Error checking, trying to prevent rooms from stretching beyond 
		if x+w > self.w{
			return Err(String::from_str("Right edge of the room goes beyond the edge of the map"));
		}
		if y+h > self.h{
			return Err(String::from_str("Bottom edge of the room goes beyond the edge of the map"));
		}

		//double nested loop to iterate over the area of the room, horizontal row by horizontal row
		for room_y in range(y, y+h) {
			for room_x in range(x, x+w) {
				//this should never happen but I've been biten in the ass by that saying before so let's just account for every possibility
				let room_z = match self.data.get_mut(&(room_x, room_y)){
					Some(x) => x,
					None	=> return Err(format!("No vec in the coordinates ({},{})",room_x,room_y)),
				};
				//destroy any existing walls, rooms can intersect but they join together, it would be weird to have the walls of one room in the middle of another
				room_z.retain(|x| {
					if x == &"|".to_string() || x == &"-".to_string(){
						return false;
					}
					true
				});
				//this whole section is just to make sure we dont put walls where there shouldn't be any
				let mut current_cell_is_occupied = false;
				for x in room_z.iter(){
					if x == &".".to_string(){
						current_cell_is_occupied = true;
						break;
					}
				}
				if current_cell_is_occupied{
					continue;
				}
				//fill the room
				match (room_x, room_y) {
					(m_x, _) if m_x == x || m_x == x+w 	=>	room_z.push("-".to_string()),
					(_, m_y) if m_y == y || m_y == y+h 	=>	room_z.push("|".to_string()),
					(_, _) 								=> 	room_z.push(".".to_string()),
				}
			}
		}
		//tell everyone it's ok!
		Ok(())
	}

	pub fn save(&self, save_path: Path) -> Result<Path, IoError> {
		let current_base_directory = match os::getcwd() {
			Ok(x) 	=>	x,
			Err(x)	=>	return Err(x),
		};
		let final_save_path = current_base_directory.join(save_path.clone());
		let json_string = encode(&self.to_json());
		let mut json_file = match File::open_mode(&final_save_path, FileMode::Truncate, FileAccess::Write){
			Ok(f)	=>	f,
			Err(e)	=>	return Err(e),
		};
		match json_file.write_str(json_string.as_slice()){
			Ok(_)	=>	Ok(save_path),
			Err(e)	=>	Err(e),
		}
	}

	pub fn put_object(&mut self, obj: String, x: u32, y: u32) -> Result<(), String> {
		let z_level = self.data.get_mut(&(x,y));
		let z_level = match z_level {
			Some(val)	=>	val,
			None		=>	return Err(format!("world data could not insert object ({}) at coordinates ({},{})", obj, x, y)),
		};
		z_level.push(obj);
		Ok(())
	}

	pub fn get_objects(&self, x: u32, y: u32) -> Result<vec::Vec<String>, String> {
		let objs = self.data.get(&(x,y));
		match objs {
			Some(val)						=>	Ok(val.clone()),
			None							=>	Err(format!("there is no Vector at coordinates ({},{})", x, y)),
		}
	}

	pub fn get_width(&self) -> u32{
		self.w.clone()
	}

	pub fn get_height(&self) -> u32{
		self.h.clone()
	}

	pub fn get_number_of_tiles(&self) -> u32{
		self.data.len().to_u32().unwrap()
	}
}

impl ToJson for World {
	fn to_json(&self) -> Json {
		let mut json_file = BTreeMap::new();
		json_file.insert("width".to_string(), Json::U64(self.w.to_u64().expect("Width of world is not able to be converted into a U64")));
		json_file.insert("height".to_string(),Json::U64(self.h.to_u64().expect("Height of world is not able to be converted into a U64")));
		for x in range(0, self.w+1) {
			let mut y_map = BTreeMap::new();
			for y in range(0, self.h+1){
				match self.data.get(&(x,y)){
					Some(val)	=>	y_map.insert(y.to_string(), val.to_json()),
					None		=>	y_map.insert(y.to_string(), Vec::<String>::new().to_json()),
				};
			}
			json_file.insert(x.to_string(), y_map.to_json());
		}
		json_file.to_json()
	}
}