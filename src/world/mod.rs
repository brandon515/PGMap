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
use rustc_serialize::json;
use rustc_serialize::json::{
	ToJson,
	Json,
};
use tile::Type;

#[derive(Clone, PartialEq, Copy)]
pub struct WObject {
    pub uid: u32,
    pub obj: Type,
}

pub struct World{
	w: u32,
	h: u32,
	current_uid: u32,
	data: HashMap<(u32,u32), vec::Vec<WObject>>,
}

impl World{

	/*
	Signature:	new(u32,u32)
	Purpose: 	Creation of a new empty world object
	Inputs: 	Two 32 bit integers, the first is the width and the second is the height
	Outputs: 	A world object with w set to width, h set to height, and data is a map set with width*height number of empty lists
	*/
	pub fn new(width: u32, height: u32) -> Result<World, String> {
		let mut data_map:HashMap<(u32,u32), vec::Vec<WObject>> = HashMap::<(u32,u32), vec::Vec<WObject>>::new();
		for x in (0u32..width) {
			for y in (0u32..height){
				match data_map.insert((x,y), Vec::new()){
					Some(_)	=> return Err("Was not able to insert empty Vec into HashMap".to_string()),
					None	=>	continue,
				};
			}
		}
		Ok(World{w:width, h:height, current_uid: 1, data:data_map})
	}

	/*
	Signature:	from_file(Path)
	Purpose:	Creation of a new world object filled with data from a file
	Inputs:		Path object pointing to the file
	Outputs:	A world object filled with the data from the file if successful, a String object explaining the error if not
	*/
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
			Err(e)	=>	return Err(format!("File at path {} has an error of the kind {}", file_name, e.desc)),
		};
		//read the contents
		let json_file_contents = match json_file.read_to_end(){
			Ok(c)	=>	c,
			Err(e)	=>	return Err(format!("File at path {} was not able to be read, reason: {}", file_name, e.desc)),
		};
		//make it into a String object
		let json_string = match String::from_utf8(json_file_contents){
			Ok(s)	=>	s,
			Err(_)	=>	return Err(format!("Vec of u8 from path {} was not able to be converted into a String", file_name)),
		};
		//make it into a JSON enum
		let json_object = match Json::from_str(json_string.as_slice()){
			Ok(j)	=>	j,
			Err(_)	=>	return Err(format!("Json file at {} is corrupted", file_name)),
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
		//get current_uid json enum
		let current_uid_json = match map.get(&"current_uid".to_string()) {
			Some(val)	=>	val,
			None		=>	return Err(format!("Json file at {} doesn't have a current_uid field", file_name))
		};
		//get current_uid from the enum
		let current_uid_obj = match current_uid_json{
			&Json::U64(x)	=>	x,
			_		=>	return Err(format!("Json file at path {} the current_uid is not the expected type", file_name)),
		};

		let mut world_map = HashMap::<(u32,u32), Vec<WObject>>::new();
		for x in (0..width+1) {
			//get the x object and turn it into a BTreeMap
			let map_x_level_json = match map.get(&x.to_string()){
				Some(val)	=>	val,
				None		=>	return Err(format!("Json file at path {} doesn't have objects on x coordinate {}", file_name, x)),
			};
			let map_x_level = match map_x_level_json {
				&Json::Object(ref x) =>	x,
				_				=>	return Err(format!("Json file at path {} the x level at {} is not a json object", file_name, x)),
			};
			//iterate through the x object
			for y in (0..height+1) {
				//extract the y array
				let map_y_level_json = match map_x_level.get(&y.to_string()) {
					Some(val)	=>	val,
					None		=>	return Err(format!("Json file at path {} doesn't have objects on x coordinate {}", file_name, y)),
				};
				let z_level_json = match map_y_level_json {
					&Json::Array(ref val)	=>	val,
					_						=>	return Err(format!("Json file at path {} at the coordinates ({},{}) is not a json array", file_name, x, y)),
				};
				let mut z_level = Vec::<WObject>::new();
				//iterate through the y array
				for x in z_level_json.iter(){
					//extract the object
					let world_object = match x.clone() {
						Json::Object(val)	=>	val,
						_			=>	return Err(format!("Json file at path {} at coordinates ({},{}) there is an item that is not a valid world object", file_name, x, y)),
					};
					let uid_json = match world_object.get(&"uid".to_string()){
						Some(val)	=>	val,
						None		=>	return Err(format!("Json file at path {} at coordinates ({},{}), the item there has no uid", file_name, x, y)),
					};
					let uid_real = match uid_json {
						&Json::U64(ref val) => val,
						_					=>	return Err(format!("Json file at path {} at the coordinates ({},{}) is not a U64", file_name, x, y)),
					};
					let obj_json = match world_object.get(&"obj".to_string()) {
						Some(val) 	=> val,
						None		=> return Err(format!("Json file at path {} at coordinates ({},{}), the item there has no obj", file_name, x, y)),
					};
					let obj_real = match obj_json{
						&Json::String(ref val)	=>	val,
						_						=>	return Err(format!("Json file at path {} at the coordinates ({},{}) is not a String", file_name, x, y)),
					};
					let tile_type = match obj_real.as_slice() {
						"H_WALL" 	=> 	Type::HorizontalWall,
						"V_WALL"	=>	Type::VerticalWall,
						"FLOOR"		=>	Type::Floor,
						"MAIN_CHAR"	=>	Type::MainCharacter,
						tile_str	=>	return Err(format!("Json file at path {} at coordinates ({},{}) is not a valid type, type in tile: {}", file_name, x, y, tile_str)),
					};
					z_level.push(WObject{uid:*uid_real as u32, obj:tile_type})
				}
				world_map.insert((x as u32, y as u32), z_level);
			}
		}
		Ok(World{w:width as u32, h:height as u32, current_uid: current_uid_obj as u32, data:world_map})
	}

	/*
	Signature:	build_room(u32,u32,u32,u32)
	Purpose:	Place a room that use | and - as walls and . as the Floors, rooms can overlap with their exterior walls staying
	Inputs:		Four 32 bit integers, the first 2 are the x,y coordinates, the second 2 are width and height respectively.
	Outputs:	A unit object if successful, a String object explaining the err if not
	POSSIBLY DEFUNCT
	*/
	/*pub fn build_room(&mut self, x: u32, y: u32, w: u32, h: u32) -> Result<(), String>{
		//Error checking, trying to prevent rooms from stretching beyond 
		if x+w > self.w{
			return Err("Right edge of the room goes beyond the edge of the map".to_string());
		}
		if y+h > self.h{
			return Err("Bottom edge of the room goes beyond the edge of the map".to_string());
		}
		//double nested loop to iterate over the area of the room, horizontal row by horizontal row
		for room_y in (y..y+h) {
			for room_x in (x..x+w) {
				//this should never happen but I've been biten in the ass by that saying before so let's just account for every possibility
				let room_z = match self.data.get_mut(&(room_x, room_y)){
					Some(x) => x,
					None	=> return Err(format!("No vec in the coordinates ({},{})",room_x,room_y)),
				};
				//destroy any existing walls, rooms can intersect but they join together, it would be weird to have the walls of one room in the middle of another
				room_z.retain(|x| {
					if x.obj == "|".to_string() || x.obj == "-".to_string(){
						return false;
					}
					true
				});
				//this whole section is just to make sure we dont put walls where there shouldn't be any
				let mut current_cell_is_occupied = false;
				for x in room_z.iter(){
					if x.obj == ".".to_string(){
						current_cell_is_occupied = true;
						break;
					}
				}
				if current_cell_is_occupied{
					continue;
				}
				//fill the room
				match (room_x, room_y) {
					(m_x, _) if m_x == x || m_x == x+w 	=>	room_z.push(WObject{uid:0,obj:"-".to_string()}),
					(_, m_y) if m_y == y || m_y == y+h 	=>	room_z.push(WObject{uid:0,obj:"|".to_string()}),
					(_, _) 								=> 	room_z.push(WObject{uid:0,obj:".".to_string()}),
				}
			}
		}
		//tell everyone it's ok!
		Ok(())
	}*/

	/*
	
	*/
	pub fn save(&self, save_path: Path) -> Result<Path, IoError> {
		let current_base_directory = match os::getcwd() {
			Ok(x) 	=>	x,
			Err(x)	=>	return Err(x),
		};
		let final_save_path = current_base_directory.join(save_path.clone());
		let json_string = json::encode(&self.to_json()).unwrap();
		let mut json_file = match File::open_mode(&final_save_path, FileMode::Truncate, FileAccess::Write){
			Ok(f)	=>	f,
			Err(e)	=>	return Err(e),
		};
		match json_file.write_str(json_string.as_slice()){
			Ok(_)	=>	Ok(save_path),
			Err(e)	=>	Err(e),
		}
	}

	pub fn put(&mut self, obj: Type, x: u32, y: u32) -> Result<u32, String> {
		let z_level = self.data.get_mut(&(x,y));
		let z_level = match z_level {
			Some(val)	=>	val,
			None		=>	return Err(format!("world data could not insert object at coordinates ({},{})", x, y)),
		};
		z_level.push(WObject{uid:self.current_uid, obj:obj});
		self.current_uid=self.current_uid+1;
		Ok(self.current_uid-1)
	}

	pub fn objects_at(&self, x: u32, y: u32) -> Result<vec::Vec<WObject>, String> {
		let objs = self.data.get(&(x,y));
		let mut ret_vec = Vec::new();
		let z_level = match objs {
			Some(val)						=>	val,
			None							=>	return Err(format!("there is no Vector at coordinates ({},{})", x, y)),
		};
		for x in z_level.iter() {
			ret_vec.push(x.clone());
		}
		Ok(ret_vec)
	}

	pub fn where_is(&self, uid: u32) -> Option<(u32,u32)> {
		for (coordinates, vector) in self.data.iter() {
			for val in vector.iter(){
				if val.uid == uid{
					return Some(*coordinates)
				}
			}
		}
		None
	}

	fn retrieve(&mut self, uid: u32) -> Result<WObject, String> {
		let (former_x, former_y) = match self.where_is(uid){
			Some(val)	=>	val,
			None		=>	return Err(format!("No object with the uid {} is in the world during Retrieve where_is search", uid)),
		};
		let mut former_vector = self.data.get_mut(&(former_x, former_y)).unwrap();
		for index in (0..former_vector.len()){
			if former_vector[index].uid == uid{
				return Ok(former_vector.remove(index));
			}
		}
		Err(format!("No object with the uid {} is in the world", uid))
	}

	pub fn translate(&mut self, delta_x: u32, delta_y: u32, uid: u32) -> Result<(), String>{
		let (former_x, former_y) = match self.where_is(uid){
			Some(val)	=>	val,
			None		=>	return Err(format!("No object with the uid {} is in the world during translate where_is search", uid)),
		};
		let object_being_moved = self.retrieve(uid).unwrap();
		let (new_x, new_y) = (former_x+delta_x, former_y+delta_y);
		let mut latter_vector = self.data.get_mut(&(new_x, new_y)).unwrap();
		latter_vector.push(object_being_moved);
		Ok(())
	}

	pub fn destroy(&mut self, uid:u32) -> Result<(), String>{
		self.retrieve(uid).unwrap();
		Ok(())
	}

	pub fn width(&self) -> u32{
		self.w.clone()
	}

	pub fn height(&self) -> u32{
		self.h.clone()
	}

	pub fn number_of_tiles(&self) -> u32{
		self.data.len().clone() as u32
	}
}

impl ToJson for World {
	fn to_json(&self) -> Json {
		let mut json_file = BTreeMap::new();
		json_file.insert("width".to_string(), Json::U64(self.w as u64));
		json_file.insert("height".to_string(),Json::U64(self.h as u64));
		json_file.insert("current_uid".to_string(), Json::U64(self.current_uid as u64));

		for x in (0..self.w+1) {
			let mut y_map = BTreeMap::new();
			for y in (0..self.h+1){
				match self.data.get(&(x,y)){
					Some(val)	=>	y_map.insert(y.to_string(), val.to_json()),
					None		=>	y_map.insert(y.to_string(), Vec::<WObject>::new().to_json()),
				};
			}
			json_file.insert(x.to_string(), y_map.to_json());
		}
		json_file.to_json()
	}
}

impl ToJson for WObject {
	fn to_json(&self) -> Json {
		let mut json_file = BTreeMap::new();
		json_file.insert("uid".to_string(), self.uid.to_json());
		match self.obj{
			Type::HorizontalWall	=>	json_file.insert("obj".to_string(), "H_WALL".to_json()),
    		Type::VerticalWall		=>	json_file.insert("obj".to_string(), "V_WALL".to_json()),
    		Type::Floor				=>	json_file.insert("obj".to_string(), "FLOOR".to_json()),
    		Type::MainCharacter	=>	json_file.insert("obj".to_string(), "MAIN_CHAR".to_json()),
		};
		json_file.to_json()
	}
}