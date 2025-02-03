use macroquad::prelude::*;
use macroquad::logging::info;
//Might consider doing all my calculations in f64 for more accuracy.
static GRAVITATIONAL_CONSTANT: f32 = 6.67430e-11;
static STEP_SIZE: f32 = 1.0/160.0;
#[derive(Clone)]
struct Object {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    mass: f32,
    radius: f32,
    color: Color,
    trail: [(f32, f32); 200] //would be more memory efficient if I used u16s,
    //but draw_line() requires f32s as parameters. Might make changes in the future
    //where I typecast u16s to f32s should the need arise.
}

fn initialize_object(x: f32, y: f32, dx: f32, dy: f32, mass: f32, radius: f32, color: Color) -> (Object) {
    return Object { x: x, y: y, dx: dx, dy: dy, 
                    mass: mass, radius: radius,
                    color: color, trail: [(x, y); 200] };
}

#[macroquad::main("3BodySim")] //I might want to implement a GUI or maybe put this up as a website with webassembly. Would be nice to be able to choose the time step size
//and configure multiple different settings for the simulation. Most importantly though I just want to implement the other numerical methods.
//Maybe have chosen simulations display side by side.
async fn main() {
    let planet_one = initialize_object(screen_width()/4.0 + screen_width()/2.0, screen_height()/2.0, -0.2, -0.15, 1.0, 10.0, GREEN);
    let planet_two = initialize_object(screen_width()/4.0, screen_height()/2.0, 0.0, 0.0, 5.972e24, 30.0, RED);
    let planet_three  = initialize_object(screen_width()/2.0, screen_height()/4.0 + screen_height()/2.0, 0.0, 0.2, 1.0, 10.0, BLUE);
    let mut planet_vector = vec![planet_one, planet_two, planet_three];
    info!("{}, {}", screen_width(), screen_height());
    loop
    {
        clear_background(BLACK);
        //draw_circle(planet_vector[1].x, planet_vector[1].y, planet_vector[1].radius, RED);
        planet_vector = eulers_method_update(planet_vector);
        planet_vector = trail_manager(planet_vector);
        let fps = get_fps();
        draw_text(
            &format!("FPS: {}", fps),
            10.0,
            10.0,
            20.0,
            WHITE,
    );
        next_frame().await
    }
}

fn trail_manager(mut planet_vector: Vec<Object>) -> Vec<Object>
{
    //iterate through every planet.
    //eject the first coordinate in trail.
    //add the current coordinates of the planet to trail.
    //loop through the entire list, stopping at the second last element. In this loop draw a line between the element being indexed and the element succeeding it.

    //Note for a later change I might want to make: check for a certain speed before drawing the line to not needlessly expend resources, also consider starting outside of the planet's radius.
    planet_vector = planet_vector.into_iter()
    .map(|mut planet|
    {
        planet.trail.rotate_left(1);
        planet.trail[199] = (planet.x, planet.y);
        
        for index in 0..199
        {
            draw_line(planet.trail[index].0, screen_height() - planet.trail[index].1, planet.trail[index + 1].0, screen_height() - planet.trail[index + 1].1, planet.radius/10.0, planet.color);
        }//subtracted the y coordinate from screen_height above because my origin is at the bottom left, not the top left.
        return planet;
    }).collect();
    return planet_vector;
}

fn calculate_forces(mut planet_vector: Vec<Object>) -> Vec<Object>
{
    for (mut index, mut planet) in planet_vector.clone().into_iter().enumerate() { //see if you can get rid of the .clone()
        index += 1;
        if (index > planet_vector.len() - 1)
        {
            break;
        }
        for i in index..planet_vector.len()
        {
            let distance_parameters: [f32; 2] = [planet.x - planet_vector[i].x, planet.y - planet_vector[i].y];
            let radius = ((distance_parameters[0]).powi(2) + (distance_parameters[1]).powi(2)).sqrt()*4e3;
            let force = GRAVITATIONAL_CONSTANT * (planet.mass * planet_vector[i].mass)/(radius.powi(2));
            let force_y: f32;
            let force_x: f32;
            
            match distance_parameters {
                distance_parameters if (distance_parameters[0] == 0.0 && distance_parameters[1] > 0.0) => {force_y = force; force_x = 0.0;},
                distance_parameters if (distance_parameters[0] == 0.0 && distance_parameters[1] < 0.0) => {force_y = -force; force_x = 0.0},
                [..] => {let theta = (distance_parameters[1]).atan2(distance_parameters[0]);
                    force_y = force * theta.sin();
                    force_x = force * theta.cos();
                }
            }
            planet_vector[i - 1].dy = planet.dy - force_y/planet_vector[i - 1].mass * STEP_SIZE * 1.0/4e3;
            planet_vector[i - 1].dx = planet.dx - force_x/planet_vector[i - 1].mass * STEP_SIZE * 1.0/4e3;
            planet_vector[i].dy += force_y/planet_vector[i].mass * STEP_SIZE * 1.0/4e3;
            planet_vector[i].dx += force_x/planet_vector[i].mass * STEP_SIZE * 1.0/4e3;
        }
    }
    return planet_vector;
}

//1 pixel = 4e3 meters.
fn eulers_method_update(mut planet_vector: Vec<Object>) -> Vec<Object>
{
    //add up all the dy and dx changes, don't actually modify position yet. You'll modify position afterwards.
    //I think the best algorithm here is probably a shrinking list. take one element of the list, perform operations on it and every element on the rest of the list.
    //Remove it from the list. Repeat until you're done with the entire list.
    planet_vector = calculate_forces(planet_vector);
    planet_vector = planet_vector.into_iter()
        .map(|mut planet| {
            planet.y += planet.dy;
            planet.x += planet.dx;
            draw_circle(planet.x, screen_height() - planet.y, planet.radius, planet.color); // I'm subtracting planet.y from screen_height() because my coordinate system's origin
            //is at the bottom left of the screen, while macroquad's is at the top left.

            return planet;
        }).collect();
    return planet_vector;
}

fn calculate_forces_rk4(mut planet_vector: Vec<Object>) -> (Vec<Object>)
{
    for (mut index, mut planet) in planet_vector.clone().into_iter().enumerate() { //see if you can get rid of the .clone()
        index += 1;
        if (index > planet_vector.len() - 1)
        {
            break;
        }
        for i in index..planet_vector.len()
        {
            let distance_parameters: [f32; 2] = [planet.x - planet_vector[i].x, planet.y - planet_vector[i].y];
            let radius = ((distance_parameters[0]).powi(2) + (distance_parameters[1]).powi(2)).sqrt()*4e3;
            let force = GRAVITATIONAL_CONSTANT * (planet.mass * planet_vector[i].mass)/(radius.powi(2));
            let force_y: f32;
            let force_x: f32;
            
            match distance_parameters {
                distance_parameters if (distance_parameters[0] == 0.0 && distance_parameters[1] > 0.0) => {force_y = force; force_x = 0.0;},
                distance_parameters if (distance_parameters[0] == 0.0 && distance_parameters[1] < 0.0) => {force_y = -force; force_x = 0.0},
                [..] => {let theta = (distance_parameters[1]).atan2(distance_parameters[0]);
                    force_y = force * theta.sin();
                    force_x = force * theta.cos();
                }
            }
            planet_vector[i - 1].dy = planet.dy - force_y/planet_vector[i - 1].mass * STEP_SIZE * 1.0/4e3;
            planet_vector[i - 1].dx = planet.dx - force_x/planet_vector[i - 1].mass * STEP_SIZE * 1.0/4e3;
            planet_vector[i].dy += force_y/planet_vector[i].mass * STEP_SIZE * 1.0/4e3;
            planet_vector[i].dx += force_x/planet_vector[i].mass * STEP_SIZE * 1.0/4e3;
            
            
            let k_one_y = planet_vector[i - 1].dy;
            let k_one_x = planet_vector[i - 1].dx;
            

        }
    }
    return planet_vector;
}

fn rk4_update(mut planet_vector: Vec<Object>, order: u8) -> Vec<Object>
{
    planet_vector = calculate_forces_rk4(planet_vector);
    planet_vector = planet_vector.into_iter()
        .map(|mut planet| {
            planet.y += planet.dy;
            planet.x += planet.dx;
            draw_circle(planet.x, screen_height() - planet.y, planet.radius, planet.color); // I'm subtracting planet.y from screen_height() because my coordinate system's origin
            //is at the bottom left of the screen, while macroquad's is at the top left.

            return planet;
        }).collect();
    return planet_vector;
}

fn hamiltonian_symplectic_integration()
{

}

fn verlet_integration()
{
    //Might want to do some refactoring and calculate force using a seperate function. Might save headaches.
}

