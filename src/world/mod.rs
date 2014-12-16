extern crate image;
extern crate serialize;
use std::os;
use std::vec;
use std::io::{
	IoError,
	File,
	FileMode,
	FileAccess
};
use std::collections::{
	HashMap,
	TreeMap
};
use self::serialize::json::{
	ToJson,
	Json,
	decode,
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

	pub fn new_from_file(path_to_json_file: Path) -> Result<World, String> {
		Err("tester".to_string())
	}

	pub fn put_room(&mut self, x: u32, y: u32, w: u32, h: u32) -> Result<(), String>{
		if x+w > self.w{
			return Err(String::from_str("Right edge of the room goes beyond the edge of the map"));
		}
		if y+h > self.h{
			return Err(String::from_str("Bottom edge of the room goes beyond the edge of the map"));
		}
		for room_y in range(y, y+h+1) {
			for room_x in range(x, x+w+1) {

				let room_z = match self.data.get_mut(&(room_x, room_y)){
					Some(x) => x,
					None	=> return Err(format!("No vec in the coordinates ({},{})",room_x,room_y)),
				};
				room_z.retain(|x| {
					if x == &"|".to_string() || x == &"-".to_string(){
						return false;
					}
					true
				});
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
				match (room_x, room_y) {
					(m_x, _) if m_x == x || m_x == x+w 	=>	room_z.push("-".to_string()),
					(_, m_y) if m_y == y || m_y == y+h 	=>	room_z.push("|".to_string()),
					(_, _) 									=> 	room_z.push(".".to_string()),
				}
			}
		}
		Ok(())
	}

	pub fn save(&self, save_path: Path) -> Result<Path, IoError> {
		let current_base_directory = match os::getcwd() {
			Ok(x) 	=>	x,
			Err(x)	=>	return Err(x),
		};
		let final_save_path = current_base_directory.join(save_path);
		let json_string = encode(&self.to_json());
		let mut json_file = match File::open_mode(&final_save_path, FileMode::Truncate, FileAccess::Write){
			Ok(f)	=>	f,
			Err(e)	=>	return Err(e),
		};
		match json_file.write_str(json_string.as_slice()){
			Ok(_)	=>	Ok(final_save_path),
			Err(e)	=>	Err(e),
		}
	}
}

impl ToJson for World {
	fn to_json(&self) -> Json {
		let mut json_file = TreeMap::new();
		json_file.insert("width".to_string(), Json::U64(self.w.to_u64().expect("Width of world is not able to be converted into a U64")));
		json_file.insert("height".to_string(),Json::U64(self.h.to_u64().expect("Height of world is not able to be converted into a U64")));
		for x in range(0, self.w+1) {
			let mut y_map = TreeMap::new();
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