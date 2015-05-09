extern crate rand;
use self::rand::distributions::{
    IndependentSample,
    Range
};
use self::rand::SeedableRng;
use world::World;
use tile::Type;

enum Direction{
    Horizontal,
    Vertical,
}

//Purpose: creates a horizontal corridor from left to right
fn create_horizontal_corridor(world: &mut World, starting_x: u32, starting_y: u32, length: u32) -> Result<(),String>{
    if starting_x+length>world.w{
        return Err(format!("out of bounds at ({},{}) on the x-axis", starting_x+length, starting_y))
    }
   for x in starting_x..starting_x+length+1{
        let objects_above = match world.objects_at(x, starting_y-1){
            Some(x) =>  x,
            None    =>  return Err(format!("Could not retrieve Vector at ({},{})", x, starting_y-1))
        };
        let objects_place = match world.objects_at(x, starting_y){
            Some(x) =>  x,
            None    =>  return Err(format!("Could not retrieve Vector at ({},{})", x, starting_y))
        };
        let objects_below = match world.objects_at(x, starting_y+1){
            Some(x) =>  x,
            None    =>  return Err(format!("Could not retrieve Vector at ({},{})", x, starting_y+1))
        };
        if objects_above.len() != 0 ||objects_place.len() != 0 ||objects_below.len() != 0{
            return Err(format!("Overlap at ({},{})",x,starting_y));
        }
    }
    for x in starting_x..starting_x+length+1{
        if x == starting_x || x == starting_x+length{
            world.put(Type::VerticalWall,x,starting_y).unwrap();
        }
        else{
            world.put(Type::Floor,x,starting_y).unwrap();
        }
        world.put(Type::HorizontalWall,x,starting_y-1).unwrap();
        world.put(Type::HorizontalWall,x,starting_y+1).unwrap();
    } 
    Ok(())
}

//Purpose: Creates a vertical corridor starting from the top down
fn create_vertical_corridor(world: &mut World, starting_x: u32, starting_y: u32, length: u32) -> Result<(),String>{
    if starting_y+length > world.h{
        return Err(format!("The cooridinates ({},{}) are out of bounds on the y-axis", starting_x, starting_y+length))
    }
    for y in starting_y..starting_y+length+1{
        let objects_left = match world.objects_at(starting_x-1,y){
            Some(x) =>  x,
            None    =>  return Err(format!("Could not retrieve Vector at ({},{})", starting_x-1, y))
        };
        let objects_place = match world.objects_at(starting_x,y){
            Some(x) =>  x,
            None    =>  return Err(format!("Could not retrieve Vector at ({},{})", starting_x, y))
        };
        let objects_right = match world.objects_at(starting_x+1,y){
            Some(x) =>  x,
            None    =>  return Err(format!("Could not retrieve Vector at ({},{})", starting_x+1, y))
        };

        if objects_left.len() != 0 ||objects_place.len() != 0 ||objects_right.len() != 0{
            return Err(format!("Overlap at ({},{})",starting_x,y));
        }
    }
    for y in starting_y..starting_y+length+1{
        if y == starting_y || y == starting_y+length{
            world.put(Type::HorizontalWall,starting_x,y).unwrap();
        }
        else{
            world.put(Type::Floor, starting_x,y).unwrap();
        }
        world.put(Type::VerticalWall, starting_x-1,y).unwrap();
        world.put(Type::VerticalWall, starting_x+1,y).unwrap();
    }
    Ok(())
}

fn create_corridor(world: &mut World, starting_x: u32, starting_y: u32, length: u32, direction: Direction) -> Result<(),String>{
    match direction{
        Direction::Horizontal   =>  create_horizontal_corridor(world,starting_x,starting_y,length),
        Direction::Vertical     =>  create_vertical_corridor(world,starting_x,starting_y,length),
    }
}

fn create_rectangle_room(world: &mut World, upper_left_x: u32, upper_left_y: u32, height: u32, width: u32) -> Result<(), String>{
    for x in upper_left_x..upper_left_x+width{
        for y in upper_left_y..upper_left_y+height{
            let objects = world.objects_at(x,y).unwrap();
            if objects.len() != 0{
                return Err(format!("Overlap at ({},{})",x,y));
            }
        }
    }
    for x in upper_left_x..upper_left_x+width{
        for y in upper_left_y..upper_left_y+height{
            if x == upper_left_x || x==upper_left_x+width-1{
                world.put(Type::HorizontalWall,x,y).unwrap();
            }
            else if y == upper_left_y || y == upper_left_y+width-1{
                world.put(Type::VerticalWall,x,y).unwrap();
            }
            else{
                world.put(Type::Floor,x,y).unwrap();
            }
        }
    }
    Ok(())
}

fn create_diamond_room(world: &mut World, center_x: u32, center_y: u32, radius: u32) -> Result<(), String>{
    if center_x+radius>world.w || center_y+radius>world.h{
        return Err(format!("The diamond room was out of bounds"))
    }
    for x in center_x-radius..center_x+radius+1{
        //distance away from the center
        let y_delta_i = radius as i32 - (center_x as i32 - x as i32).abs();
        let y_delta = y_delta_i.abs() as u32;
        for y in center_y-y_delta..center_y+y_delta+1{
            let objects_place = world.objects_at(x,y).unwrap();
            if objects_place.len() != 0{
                return Err(format!("Overlap at ({},{})",x,y));
            }
        }
    }
    for x in center_x-radius..center_x+radius+1{
        //distance away from the center
        let y_delta_i = radius as i32 - (center_x as i32 - x as i32).abs();
        let y_delta = y_delta_i.abs() as u32;
        for y in center_y-y_delta..center_y+y_delta+1{
            if y == center_y-y_delta || y == center_y+y_delta{
                world.put(Type::VerticalWall,x,y).unwrap();
            }
            else{
                world.put(Type::Floor,x,y).unwrap();
            }
        }
    }
    world.put(Type::VerticalWall,center_x-radius,center_y).unwrap();
    world.put(Type::VerticalWall,center_x+radius,center_y).unwrap();
    world.put(Type::HorizontalWall,center_x,center_y-radius).unwrap();
    world.put(Type::HorizontalWall,center_x,center_y+radius).unwrap();
    Ok(())
}

pub fn create_dungeon(world: &mut World, seed: &[usize]){
    let room_generation_range = Range::new(1u32,100);
    let height_width_range = Range::new(1u32, 20);
    let starting_room_x_range = Range::new(1u32, world.w);
    let starting_room_y_range = Range::new(1u32, world.h);
    let mut rng = rand::StdRng::from_seed(seed);
    let mut current_x = starting_room_x_range.ind_sample(&mut rng);
    let mut current_y = starting_room_y_range.ind_sample(&mut rng);
    create_rectangle_room(world, current_x, current_y, height_width_range.ind_sample(&mut rng), height_width_range.ind_sample(&mut rng)).unwrap();

}
