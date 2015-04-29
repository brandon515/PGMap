use world::World;
use tile;
use std::path::Path;
#[test]
fn empty_creation(){
	let width = 4;
	let height = 12;
	let test_world = World::new(width,height).unwrap();
	assert!(test_world.height() == height, "The Height of the world is not set correctly\n\texpected: {}\n\tactual: {}\n", height, test_world.height());
	assert!(test_world.width() == width, "The Width of the world is not set correctly\n\texpected: {}\n\tactual: {}\n", width, test_world.width());
	assert!(test_world.number_of_tiles() == width*height, "The dimensions of the world are not correct\n\texpected: {}\n\tactual: {}\n", width*height, test_world.number_of_tiles());
}

#[test]
fn put_retrieve(){
	let test_object = tile::Type::MainCharacter;
	let width = 10;
	let height = 14;
	let x = 5;
	let y = 6;
	let mut test_world = World::new(width,height).unwrap();
	match test_world.put(test_object.clone(), x, y){
		Ok(_)	=>	(),
		Err(x)	=>	panic!("World did not put an object in accepted parameters, reason: {}", x),
	};
	match test_world.put(test_object.clone(), width, height){
		Ok(_)	=>	panic!("World accepted an object beyond accpeted parameters"),
		Err(_)	=>	(),
	};
	let mut z_level = test_world.objects_at(x, y).unwrap();
	assert!(z_level.pop().unwrap().obj == test_object.clone(), "Was not about to retrieve the object put into the world");
}

#[test]
fn save_load(){
	let width = 20;
	let height = 20;
	let test_object = tile::Type::MainCharacter;
	let x = 4;
	let y = 2;
	let mut save_world = World::new(width, height).unwrap();
	match save_world.put(test_object.clone(), x, y){
		Ok(_)	=>	(),
		Err(x)	=>	panic!("World could not put test object, reason: {}", x),
	};
    let save_path = Path::new("test_file");
	save_world.save(save_path).unwrap();
	let load_world = World::from_file(save_path).unwrap();
	let mut z_level = load_world.objects_at(x,y).unwrap();
	assert!(load_world.width() == width, "World width was not saved correctly");
	assert!(load_world.height() == height, "World height was not saved correctly");
	assert!(z_level.pop().unwrap().obj == test_object.clone(), "World did not have correct test object saved");
}

#[test]
fn move_destroy(){
	let test_object = tile::Type::MainCharacter;
	let width = 10;
	let height = 10;
	let first_x = 4;
	let first_y = 5;
	let final_x = 7;
	let final_y = 1;
	let mut test_world = World::new(width, height).unwrap();
	let uid = test_world.put(test_object, first_x, first_y).unwrap();
	test_world.translate(final_x as i32-first_x as i32, final_y as i32-first_y as i32, uid).unwrap();
	assert!(test_world.objects_at(final_x, final_y).unwrap().len() == 1, "Object was not translated correctly");
	test_world.destroy(uid).unwrap();
	assert!(test_world.objects_at(final_x, final_y).unwrap().len() == 0, "Object was not destroyed");
}

/*#[test]
fn room(){
	let width = 10;
	let height = 10;
	let mut test_world = World::new(width, height).unwrap();
	test_world.build_room(0,0,5,5).unwrap();
	for x in range(0, 10u32) {
		for y in range(0, 10u32) {
			let z_level = test_world.objects_at(x,y).unwrap();
			if x <= 4 && y <= 4{
				assert!(z_level.len() == 1, "The room was not placed in a spot it was supposed to be, coordinates ({},{})", x, y);
			}
			else{
				assert!(z_level.len() == 0, "The room was placed in a spot it was not suppoed to be, coordinates ({},{})", x, y);
			}
		}
	}
}*/
