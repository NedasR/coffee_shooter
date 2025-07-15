use crate::core::render_manager;
use crate::core::render_manager::RenderManager;
use crate::player::Player;
use crate::systems::input::Input;
use crate::util::im_gui_text_buffer::ImGuiTextBuffer;
use crate::core::texture_manger::TextureManger;
use box2d_rs::b2_body::B2body;
use box2d_rs::b2_body::BodyPtr;
use box2d_rs::b2_body::{B2bodyDef, B2bodyType};
use box2d_rs::b2_fixture::B2fixtureDef;
use box2d_rs::b2_fixture::FixturePtr;
use box2d_rs::b2_math::B2vec2;
use box2d_rs::b2_math::*;
use box2d_rs::b2_shape::B2shapeDynTrait;
use box2d_rs::b2_world::B2world;
use box2d_rs::b2_world::*;
use box2d_rs::b2rs_common::UserDataType;
use box2d_rs::shapes;
use box2d_rs::shapes::b2_polygon_shape;
use imgui::{Context, FontSource};
use raylib::prelude::*;
use raylib_imgui_rs::Renderer;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::default;
use std::fmt;
use std::rc::Rc;

#[derive(Default, Clone, Debug)]
struct MyBodyData {
    pub id: u32,
}

#[derive(Default, Clone, Debug)]
struct MyFixtureData {
    pub label: String,
}

#[derive(Default, Clone, Debug)]
struct MyJointData {
    pub index: i32,
}

#[derive(Default, Clone, Debug)]
struct MyUserDataType;

impl UserDataType for MyUserDataType {
    type Body = MyBodyData;
    type Fixture = MyFixtureData;
    type Joint = MyJointData;
}

pub struct GameContext {
    pub imgui: imgui::Context,
    pub renderer: Renderer,
    pub rt: Rc<RaylibThread>,
    pub rl: Rc<RefCell<RaylibHandle>>,
    pub camera: Camera2D,
    pub collsion_world: Rc<RefCell<B2world<MyUserDataType>>>,
    pub render_manager: RenderManager,
    pub texture_manager: TextureManger,
}

impl GameContext {
    pub fn new() -> Self {
        let (mut rl, thread) = raylib::init()
            .size(640, 480)
            .title("raylib shooter")
            .build();
        let rl = Rc::new(RefCell::new(rl));
        let rt = Rc::new(thread);

        let mut imgui = Context::create();
        imgui
            .fonts()
            .add_font(&[FontSource::DefaultFontData { config: None }]);
        
        let renderer = Renderer::create(&mut imgui, &mut rl.borrow_mut(), &rt);
        let mut G = box2d_rs::b2_math::B2vec2::new(0.0, 0.0);
        let world: Rc<RefCell<B2world<MyUserDataType>>> = B2world::new(G);

        let mut camera = Camera2D {
            offset: Vector2::new(320.0, 240.),
            target: rl.borrow_mut().get_mouse_position(),
            rotation: 0.0,
            zoom: 1.0,
        };

        let mut render_manager = render_manager::RenderManager::new();

        let mut texture_manager = TextureManger::new(rl.clone(),rt.clone());

        let game_context = Self {
            imgui,
            renderer,
            rt,
            rl,
            camera,
            collsion_world: world,
            render_manager,
            texture_manager,
        };
        //b2_abs_mat22(20);
        //Body::SetGravityScale(0.0);

        game_context
    }

    pub fn run(&mut self) {
        let mut player = crate::player::Player::new(self, None, "assets/textures/player.png");
        let mut last_time: f32 = 0.0;
        let mut delta_time: f32 = 0.0;
        let mut last_postion = Vector2::new(0.0, 0.0);

        let mut mouse = Vector2::new(0.0, 0.0);

        let mut ptrs: Vec<Rc<RefCell<box2d_rs::b2_fixture::B2fixture<MyUserDataType>>>> =
            Vec::<Rc<RefCell<box2d_rs::b2_fixture::B2fixture<MyUserDataType>>>>::new();

        {
            //let mut w = self.collsion_world.borrow_mut();

            PhysicsBody::new(self, &mut ptrs, B2vec2 { x: 000.0, y: 10.0 }, None);
            PhysicsBody::new(self, &mut ptrs, B2vec2 { x: 500.0, y: 10.0 }, None);

            //if(ptrs.len() > 0)
            //{
            //    let mut body_fixture:std::cell::RefMut<'_, box2d_rs::b2_fixture::B2fixture<MyUserDataType>> = ptrs[0].borrow_mut();
            //    let body = body_fixture.get_body();
            //    body.borrow_mut().apply_force(B2vec2 { x: 900000.0, y: 0.0 }, B2vec2 { x: 0.0, y: 10.0 }, true);
            //}
            println!("Created a dynamic body at (0, 10) with a box fixture");
        }
        let count = self.collsion_world.borrow_mut().get_body_count() as i32;
        let string: &str = &count.to_string();
        let s = format!("Body count: {}", string);
        println!("{}", s);
        //self.collsion_world.borrow_mut().set_debug_draw(debug_draw);
        while !self.rl.borrow_mut().window_should_close() {
            let current_time : f32;
            {
                let mut rlmut = self.rl.borrow_mut();
                current_time = rlmut.get_time() as f32;
                delta_time = current_time - last_time;
                let mut text_buffer = ImGuiTextBuffer::new();
                {
                    last_postion = player.transform.position;
                    player.input_update(&mut rlmut, &mut mouse, &mut delta_time);
                    self.camera.target = player.transform.position;

                    text_buffer.push_text("Player info display below:");
                    text_buffer.push_vector2("Pos", player.transform.position);
                    text_buffer.push_float("DeltaTime", delta_time);
                    let value = Vector2::new(
                        player.transform.position.x - last_postion.x,
                        player.transform.position.y - last_postion.y,
                    );
                    text_buffer.push_vector2("Rate of change", value);
                }

                self.renderer.update(&mut self.imgui, &mut rlmut);
                {
                    update_imgui(&mut self.imgui, text_buffer);
                }
                let mut drawer = rlmut.begin_drawing(&self.rt);
                drawer.clear_background(Color::WHITE);

                if (ptrs.len() > 0) {
                    let mut body_fixture: std::cell::RefMut<
                        '_,
                        box2d_rs::b2_fixture::B2fixture<MyUserDataType>,
                    > = ptrs[0].borrow_mut();
                    let body = body_fixture.get_body();
                    body.borrow_mut().apply_force(
                        B2vec2 { x: 1000.0, y: 0.0 },
                        B2vec2 { x: 0.0, y: 10.0 },
                        true,
                    );
                }

                self.collsion_world.borrow_mut().step(0.02, 1 as i32, 1);

                {
                    let mut camera_mode = drawer.begin_mode2D(self.camera);

                    camera_mode.draw_rectangle(0, 0, 640, 480, Color::GRAY);
                    camera_mode.draw_line_ex(player.transform.position, mouse, 2.0, Color::RED);
                    camera_mode.draw_text("raylib shooter", 12, 12, 20, Color::BLACK);
                    player.draw(&mut camera_mode);
                    mouse = camera_mode
                        .get_screen_to_world2D(camera_mode.get_mouse_position(), self.camera);

                    //let B2vec2 = B2vec2::new(0.0, 0.0);
                    //for val in col_list.iter()
                    //{
                    //    let vertex: B2vec2 = val;
                    //    camera_mode.draw_line_ex(Vector2::new(vertex.x, vertex.y), mouse, 2.0, Color::RED);
                    //}
                    for i in 0..ptrs.len() {
                        let mut body_fixture: std::cell::RefMut<
                            '_,
                            box2d_rs::b2_fixture::B2fixture<MyUserDataType>,
                        > = ptrs[i].borrow_mut();
                        let ply = body_fixture.get_shape();
                        let mut col_list = Vec::<B2vec2>::new();
                        if let Some(poly) = ply.as_polygon() {
                            // Assume poly.m_count is the number of vertices and poly.m_vertices is an array.
                            for i in 0..(poly.m_count as usize) {
                                col_list.push(poly.m_vertices[i]);
                                let vertex: B2vec2 = poly.m_vertices[i];
                                //println!("Vertex {}: ({}, {})", i, vertex.x, vertex.y);
                            }
                        }
                        let body = body_fixture.get_body();
                        let body_position = body.borrow_mut().get_position();
                        if col_list.len() >= 2 {
                            for i in 0..col_list.len() {
                                let current = col_list[i];
                                // Determine the next vertex: if we're at the last element, wrap around to the first.
                                let next = if i + 1 < col_list.len() {
                                    col_list[i + 1]
                                } else {
                                    col_list[0]
                                };
                                camera_mode.draw_line_ex(
                                    Vector2::new(
                                        body_position.x + current.x,
                                        body_position.y + current.y,
                                    ),
                                    Vector2::new(
                                        body_position.x + next.x,
                                        body_position.y + next.y,
                                    ),
                                    2.0,
                                    Color::new(11, 219, 66, 255),
                                );
                                //print!(format!("{} {}",body_position.x + current.x, body_position.y + current.y));
                            }
                        }
                    }
                }
                //drop(drawer);
                self.renderer.render(&mut self.imgui, &mut drawer);
                //let mut drawer = rl.begin_drawing(&thread);
                //let mut camera_mode = drawer.begin_mode2D(camera);
                //drawer.draw_rectangle(0, 0, 640, 480, Color::GRAY);
                //drawer.clear_background(Color::WHITE);
                //drawer.draw_text("raylib shooter", 12, 12, 20, Color::BLACK);
                //player.draw(&mut drawer);

                //renderer.render(imgui, drawer);
            };
            last_time = current_time;
        }
    }
}
fn update_imgui(imgui_cotext: &mut imgui::Context, imgui_buffer: ImGuiTextBuffer) {
    let ui = imgui_cotext.new_frame();
    imgui_buffer.draw(ui);
    //unsafe
    //{
    //    ui.text(format!("{}{}{}{}", "Pos: X:", player.transform.position.x, " Y:",player.transform.position.y));
    //    ui.text(format!("{}{}", "DeltaTime", delta_time));
    //
    //    ui.text(format!("{}{}{}{}", "Rate of change: X:", player.transform.position.x - last_postion.x, " Y:",player.transform.position.y - last_postion.y));
    //}
}

struct PhysicsBody {
    body_def: B2bodyDef<MyUserDataType>,
    body_handle: Rc<RefCell<B2body<MyUserDataType>>>,
    body_fixture: B2fixtureDef<MyUserDataType>,
}

impl PhysicsBody {
    fn new(
        game_context: &mut GameContext,
        ptrs: &mut Vec<Rc<RefCell<box2d_rs::b2_fixture::B2fixture<MyUserDataType>>>>,
        pos: B2vec2,
        body_settings: Option<BodySettings>,
    ) -> Self {
        let body_settings = body_settings.unwrap_or(BodySettings::default());

        let mut body_def = body_settings.body_settings;
        body_def.body_type = B2bodyType::B2DynamicBody;
        body_def.position = pos;

        let body_handle: Rc<RefCell<B2body<MyUserDataType>>> =
            B2world::create_body(game_context.collsion_world.clone(), &body_def);
        let mut box_shape = body_settings.box_shape_settings;
        box_shape.set_as_box(100.0, 100.0);

        let mut body_fixture = body_settings.fixture_def_settings;
        let shape_ptr: Rc<RefCell<dyn B2shapeDynTrait>> = Rc::new(RefCell::new(box_shape));
        body_fixture.shape = Some(shape_ptr.clone());
        body_fixture.density = 1.0;
        body_fixture.friction = 0.3;

        let _body_fixture = B2body::create_fixture(body_handle.clone(), &body_fixture);
        ptrs.push(_body_fixture.clone());
        Self {
            body_def,
            body_handle,
            body_fixture,
        }
    }
}

struct BodySettings {
    body_settings: B2bodyDef<MyUserDataType>,
    box_shape_settings: b2_polygon_shape::B2polygonShape,
    fixture_def_settings: B2fixtureDef<MyUserDataType>,
}

impl BodySettings {
    //fn new() -> Self { to do make this work
    //    Self { body_settings : B2bodyDef::default(), box_shape_settings : b2_polygon_shape::B2polygonShape::default(), fixture_def_settings : B2fixtureDef::default() }
    fn default() -> Self {
        Self {
            body_settings: B2bodyDef::default(),
            box_shape_settings: b2_polygon_shape::B2polygonShape::default(),
            fixture_def_settings: B2fixtureDef::default(),
        }
    }
}

// Need a system where it would store already used texture so we don't load the same texture over and over again
// Should think about adding an ECS like system for the game

//pub trait GuiDebug {
//    fn add_debug_info(&self, buffer: &mut ImGuiTextBuffer);
//}

// TO DO
// make Collision System for that is optimized with a quadTree for collision checking
// need quadTree to to minimize object collisions against each other
// handle objects switching from one quad node to another quad node

// simple map editor for making the map
// map editor needs to be able to load and save the map to a file
// need to make a struct that contain all the objects that will be added to the scene collider box what texture  size rot position

// implement multiplayer for the game
// need to make a protocol if we want to use a simple lib that just gives us sockets for TCP and UDP
// server-client networking for the game much easier to implement then P2P networking for the game
// server authoritative so players can't cheat server
// make sure not to included non deterministic
// implement AI for the game (Nice to have NPC that can fight with the player)
