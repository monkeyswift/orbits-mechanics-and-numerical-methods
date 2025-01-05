use macroquad::prelude::*;
use macroquad::logging::info;
//Need to do all calculations in f64 then typecast to f32 before passing them into macroquad machinery.
static GRAVITATIONAL_CONSTANT: f32 = 6.67430e-11;
#[derive(Clone)]
struct Object {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    mass: f32,
    radius: f32,
    color: Color,
    trail: Vec<(f32, f32)> //for convenience I'm making this with a vector of f32 tuples, but I can typecase the f32s to u16 and store everything in an array instead of a vector
    //cause the length of the trail will vary with the speed of the object. I'm going to start with a size of 320 as that's double the frame rate, so the line will only be on
    //screen for two seconds.
}

#[macroquad::main("3BodySim")] //I might want to implement a GUI or maybe put this up as a website with webassembly. Would be nice to be able to choose the time step size
//and configure multiple different settings for the simulation. Most importantly though I just want to implement the other numerical methods.
//Maybe have chosen simulations display side by side.
async fn main() {
    let planet_one = Object { x: screen_width()/4.0 + screen_width()/2.0, y: screen_height()/2.0, dx: -0.2, dy: -0.15, mass: 1.0, radius: 10.0, color: GREEN, trail: };//initializing
    // planets like this can be easier if I use some java concepts, will consider later.
    let planet_two = Object { x: screen_width()/4.0, y: screen_height()/2.0, dx: 0.0, dy: 0.0, mass: 5.972e24, radius: 30.0, color: RED, trail:  };
    let planet_three = Object { x: screen_width()/2.0, y: screen_height()/4.0 + screen_height()/2.0, dx: 0.0, dy: 0.2, mass: 1.0, radius: 10.0, color: BLUE, trail: };
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
    planet_vector = planet_vector.into_iter()
    .map(|planet|
    {
        planet.trail/* cant remember the syntax for this */;
        planet.trail...;
        for 0..planet.trail.len() - 1
        {
            draw_line();//check syntax
        }
    }).collect();
    return planet_vector;
}

fn calculate_forces(planet_vector: Vec<Objects>) -> Vec<Objects>
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
            planet_vector[i - 1].dy = planet.dy - force_y/planet_vector[i - 1].mass * 1.0/160.0 * 1.0/4e3;
            planet_vector[i - 1].dx = planet.dx - force_x/planet_vector[i - 1].mass * 1.0/160.0 * 1.0/4e3;
            planet_vector[i].dy += force_y/planet_vector[i].mass * 1.0/160.0 * 1.0/4e3;
            planet_vector[i].dx += force_x/planet_vector[i].mass * 1.0/160.0 * 1.0/4e3;
            
        }
    }
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
            draw_circle(planet.x, screen_height() - planet.y, planet.radius, planet.color);

            return planet;
        }).collect();
    return planet_vector;
}

fn runge_kutta_update(mut planet_vector: Vec<Object>, order: u8) -> Vec<Object>
{
    let time_step: f64 = 1.0/160.0
    for (mut index, mut planet) in planet_vector.clone().into_iter().enumerate() {
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
            let mut force_y: f32;
            let mut force_x: f32;
            //Can I get rid of a lot of this code by just starting off with components?
            //that is next on the list cause it would greatly simplify things.
            match distance_parameters {
                distance_parameters if (distance_parameters[0] == 0.0 && distance_parameters[1] > 0.0) => {force_y = force; force_x = 0.0;},
                distance_parameters if (distance_parameters[0] == 0.0 && distance_parameters[1] < 0.0) => {force_y = -force; force_x = 0.0},
                [..] => {let theta = (distance_parameters[1]).atan2(distance_parameters[0]);
                    force_y = force * theta.sin();
                    force_x = force * theta.cos();
                }
            }
            planet_vector[i - 1].dy = planet.dy - force_y/planet_vector[i - 1].mass * time_step * 1.0/4e3;
            planet_vector[i - 1].dx = planet.dx - force_x/planet_vector[i - 1].mass * time_step * 1.0/4e3;
            planet_vector[i].dy += force_y/planet_vector[i].mass * time_step * 1.0/4e3;
            planet_vector[i].dx += force_x/planet_vector[i].mass * time_step * 1.0/4e3;
            
        }
    }
    planet_vector = planet_vector.into_iter()
        .map(|mut planet| {
            planet.y += planet.dy;
            planet.x += planet.dx;
            draw_circle(planet.x, screen_height() - planet.y, planet.radius, planet.color);

            return planet;
        }).collect();
    return planet_vector;
}

fn hamiltonian_symplectic_integration()
{

}

//Check if symplectic integrators exist for the lagrangian formalism.

fn verlet_integration()
{
    //Might want to do some refactoring and calculate force using a seperate function. Might save headaches.
}

