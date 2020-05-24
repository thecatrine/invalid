use wasm_bindgen::prelude::*;

extern crate console_error_panic_hook;

const BOARD_X: u32 = 1080;
const BOARD_Y: u32 = 768;

const MID_SCREEN: u32 = BOARD_X / 2;
const MID_Y: u32 = BOARD_Y / 2;
const TILE_SIZE: i32 = 100;

const SCREEN_DIST: u32 = 10;

const OUTPUT_BUFFER_SIZE: usize = BOARD_X as usize * BOARD_Y as usize * 4;
static mut OUTPUT_BUFFER: [u8; OUTPUT_BUFFER_SIZE] = [0; OUTPUT_BUFFER_SIZE];

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);
}

#[wasm_bindgen]
pub fn get_output_buffer_pointer() -> *const u8 {
    let pointer: *const u8;
    unsafe {
	pointer = OUTPUT_BUFFER.as_ptr();
    }

    return pointer;
}


static mut MAP: [[u32; 3]; 3]= [
    [1, 1, 1],
    [1, 0, 1],
    [1, 2, 1]
];

pub fn get_grid(cx: u32, cy: u32) -> (usize, usize) {
    return ((cx/100) as usize, (cy/100) as usize);
}

pub fn get_map(cx: usize, cy: usize) -> u32 {
    unsafe {
	return MAP[cy][cx];
    }
}


pub fn get_intersection(
    tile_x: usize,
    tile_y: usize,
    cx: i32,
    cy: i32,
    dx: f32,
    dy: f32
) -> (usize, usize, u32, u32, i32, i32, f32) {
    // Returns tile_x, tile_y, cx, cy, dx, dy, dist
    let offset_x = if dx > 0.0 { 1 } else { 0 };
    let offset_y = if dy > 0.0 { 1 } else { 0 };

    let dir_x:i32 = if dx > 0.0 { 1 } else { -1 };
    let dir_y:i32 = if dy > 0.0 { 1 } else { -1 };

    let mut dt_x: f32 = 10000.0;
    let mut dt_y: f32 = 10000.0;
    if dx != 0.0 {
	dt_x = (TILE_SIZE*(tile_x as i32+offset_x) - cx) as f32 / dx;
    }
    if dy != 0.0 {
	dt_y = (TILE_SIZE*(tile_y as i32+offset_y) - cy) as f32 / dy;
    }

    let dt: f32;
    let mut res_tile_x: i32 = tile_x as i32;
    let mut res_tile_y: i32 = tile_y as i32;
    if dt_x.abs() < dt_y.abs() {
	dt = dt_x;
	res_tile_x += dir_x;
    } else {
	dt = dt_y;
	res_tile_y += dir_y;
    }

    let res_cx = cx + (dt*dx) as i32;
    let res_cy = cy + (dt*dy) as i32;

    let res_dx = (dt*dx) as i32;
    let res_dy = (dt*dy) as i32;

    let dist = res_dx.pow(2) +res_dy.pow(2);

    return (
	res_tile_x as usize,
	res_tile_y as usize,
	res_cx as u32,
	res_cy as u32,
	res_dx,
	res_dy,
	dist as f32
    );
}

#[wasm_bindgen]
pub fn generate_board(px: u32, py: u32, angle: f32) {
    console_error_panic_hook::set_once();

    //log("x, y");
    //log_u32(px);
    //log_u32(py);

    for x in 0..BOARD_X {
	let mut color:(u8, u8, u8);	
	
	let adx = ((x as f32 - MID_SCREEN as f32) / MID_SCREEN as f32)
	    * 2.0 * SCREEN_DIST as f32;
	let ady = SCREEN_DIST as f32;

	let render_angle = ady.atan2(adx);
	let total_angle = angle + render_angle;

	let dx = total_angle.cos();
	let dy = total_angle.sin();

	let mut cx = px;
	let mut cy = py;

	let (mut tile_x, mut tile_y) = get_grid(cx, cy);
	//log("grid");
	//log_u32(tile_x as u32);
	//log_u32(tile_y as u32);

	let mut dist = 0.0;
	
	while get_map(tile_x, tile_y) == 0 {
//	    log("Tracing");
	    let ray_trace = get_intersection(
		tile_x,
		tile_y,
		cx as i32,
		cy as i32,
		dx,
		dy,
	    );
	    tile_x = ray_trace.0;
	    tile_y = ray_trace.1;
	    cx = ray_trace.2;
	    cy = ray_trace.3;
	    dist += ray_trace.6.sqrt();
	}

	let wall_height:u32;
	if dist != 0.0 {
	    //log("Actually calculated");
	    //log_u32(dist as u32);
	    let max_wall_height = 768.0;
	    wall_height = ((SCREEN_DIST as f32 / dist) * max_wall_height) as u32;
	} else {
	    wall_height = BOARD_Y;
	}
	    
	// Write data to canvas

	//log("___");
	//log_u32(x);
	//log_u32(wall_height);

	let draw_color;
	if get_map(tile_x, tile_y) == 1 {
	    draw_color = (255, 0, 0);
	} else {
	    draw_color = (0, 255, 0);
	}

	for y in 0..BOARD_Y {
	    if y < MID_Y-wall_height/2 || y > MID_Y+wall_height/2  {
		color = (0, 0, 0);
	    } else {
		color = draw_color;
	    }

	    let square_number: usize = (y * BOARD_X + x) as usize;
	    let index: usize = square_number*4;
	
	    unsafe {
		OUTPUT_BUFFER[index] = color.0;
		OUTPUT_BUFFER[index+1] = color.1;
		OUTPUT_BUFFER[index+2] = color.2;
		OUTPUT_BUFFER[index+3] = 255;
	    }
	}
    }
}

