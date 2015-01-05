use world::World;
#[test]
fn empty_creation(){
	let width = 4;
	let height = 12;
	let test_world = World::new(width,height).unwrap();
	assert!(test_world.get_height() == height, "The Height of the world is not set correctly\n\texpected: {}\n\tactual: {}\n", height, test_world.get_height());
	assert!(test_world.get_width() == width, "The Width of the world is not set correctly\n\texpected: {}\n\tactual: {}\n", width, test_world.get_width());
	assert!(test_world.get_number_of_tiles() == width*height, "The dimensions of the world are not correct\n\texpected: {}\n\tactual: {}\n", width*height, test_world.get_number_of_tiles());
}

#[test]
fn put_retrieve(){
	let test_object = String::from_str("@");
	let width = 10;
	let height = 14;
	let x = 5;
	let y = 6;
	let mut test_world = World::new(width,height).unwrap();
	assert!(test_world.put_object(test_object.clone(), x, y) == Ok(()), "World did not put an object in accepted parameters");
	assert!(test_world.put_object(test_object.clone(), width, height) != Ok(()), "World accepted an object beyond accpeted parameters");
	let mut z_level = test_world.get_objects(x, y).unwrap();
	assert!(z_level.pop() == Some(test_object.clone()), "Was not about to retrieve the object put into the world");
}

#[test]
fn save_load(){
	let width = 20;
	let height = 20;
	let test_object = String::from_str("@");
	let x = 4;
	let y = 2;
	let mut save_world = World::new(width, height).unwrap();
	assert!(save_world.put_object(test_object.clone(), x, y) == Ok(()), "World could not put test object");
	let save_path = save_world.save(Path::new("test_file")).unwrap();
	let load_world = World::from_file(save_path).unwrap();
	let mut z_level = load_world.get_objects(x,y).unwrap();
	assert!(load_world.get_width() == width, "World width was not saved correctly");
	assert!(load_world.get_height() == height, "World height was not saved correctly");
	assert!(z_level.pop() == Some(test_object), "World did not have correct test object saved");
}

#[test]
fn room(){
	let width = 10;
	let height = 10;
	let mut test_world = World::new(width, height).unwrap();
	test_world.put_room(0,0,5,5).unwrap();
	for x in range(0, 10u32) {
		for y in range(0, 10u32) {
			let z_level = test_world.get_objects(x,y).unwrap();
			if x <= 4 && y <= 4{
				assert!(z_level.len() == 1, "The room was not placed in a spot it was supposed to be, coordinates ({},{})", x, y);
			}
			else{
				assert!(z_level.len() == 0, "The room was placed in a spot it was not suppoed to be, coordinates ({},{})", x, y);
			}
		}
	}
}