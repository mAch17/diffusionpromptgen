use eframe::egui;
use rand::{thread_rng, Rng};

// create an enum called AttrbuteWeight with two values low and high and implementing Debug
#[derive(Debug, PartialEq,Clone)]
enum AttributeWeight {
    Low,
    High,
}

// Create an enum called CameraAngle implementing Debug and varinats ExtremeCloseUp, CloseUp, MediumShot, OverTheShoulder, LongShot, ExtremeLongShot, FullShot, FullBodyView, POVShot, EyeLevelShot, HighAngleShot, LowAngleShot, DutchAngleShot, DroneShot, GoProShot, FishEyeShot, birdEyeView, RuleOfThirdShot, CandidShot, SilhouetteShot
#[derive(Debug,Clone)]
enum CameraAngle {
    ExtremeCloseUp, 
    CloseUp, 
    MediumShot, 
    OverTheShoulder, 
    LongShot, 
    ExtremeLongShot, 
    FullShot, 
    FullBodyView, 
    POVShot, 
    EyeLevelShot, 
    HighAngleShot, 
    LowAngleShot, 
    DutchAngleShot, 
    DroneShot, 
    GoProShot, 
    FishEyeShot,
    BirdEyeView, 
    RuleOfThirdShot, 
    CandidShot, 
    SilhouetteShot
}

// Create an enum called Look implementing Debug and variants Cinematic, UltraRealistic, FilmGrain, DramaticLighting, GenreHorror, GenreWestern, GenreFantasy, GenreRomantic, GenreAnimation, GenreManga, ActionScene, MotionBlur, DynamicAction, DynamicMotion
#[derive(Debug,Clone)]
enum Cinematography {
    Cinematic, 
    UltraRealistic, 
    FilmGrain, 
    DramaticLighting, 
    GenreHorror, 
    GenreWestern, 
    GenreFantasy, 
    GenreRomantic, 
    GenreAnimation, 
    GenreManga, 
    ActionScene, 
    MotionBlur, 
    DynamicAction, 
    DynamicMotion
}

//Create an attribute called Lighting deriving debug and having variants StudioLights, BrightLights, NeonLights, WarmLights , ColdLights , HighKeyLighting , LowKeyLighting, RimLighting , PracticaLighting , MotivatorLighting , Sunny , GoldenHour , Rainy , Foggy , Night , Afternoon 
#[derive(Debug,Clone)]
enum Lighting {
    StudioLights, 
    BrightLights, 
    NeonLights, 
    WarmLights , 
    ColdLights , 
    HighKeyLighting , 
    LowKeyLighting, 
    RimLighting , 
    PracticalLighting , 
    MotivatorLighting , 
    Sunny , 
    GoldenHour , 
    Rainy , 
    Foggy , 
    Night , 
    Afternoon 
}

// Create an enum CameraStyle deriving debug and having variants ArriAlexa, Super16_VintageFilm, CanonCinemaEOS, SonyCineAlta , QuentinTarantinoStyle , AlfredHitchcockStyle, MartinScorseseStyle , ChristopherNolanStyle , MichaelBayStyle , JohnWooStyle , PeterHyamsStyles , JamesCameronStyle , ElsaGarrisonStyle , WalterIoossStyle , NeilLeiferStyle, CanonEOS_1DXMarkII , GoProHero9Black , SonyAlphaA9II
#[derive(Debug,Clone)]
enum CameraStyle {
    ArriAlexa, 
    Super16_VintageFilm, 
    CanonCinemaEOS, 
    SonyCineAlta , 
    QuentinTarantinoStyle , 
    AlfredHitchcockStyle, 
    MartinScorseseStyle , 
    ChristopherNolanStyle , 
    MichaelBayStyle  , 
    JohnWooStyle  , 
    PeterHyamsStyles , 
    JamesCameronStyle , 
    ElsaGarrisonStyle , 
    WalterIoossStyle , 
    NeilLeiferStyle, 
    CanonEOS_1DXMarkII , 
    GoProHero9Black , 
    SonyAlphaA9II
}


// create a struct FOOCUSDiffusionPrompt with no members and it implements Debug
#[derive(Debug,Clone)]
struct FOOCUSDiffusionPromptApp{
    pub subject_description : String,
    pub subject_weight : AttributeWeight,
    pub subject_attributes : Vec<(String,AttributeWeight)>,
    
    
    pub cinematic_look : Option<(&'static str,  egui::ImageSource<'static>,Cinematography)>,
    pub cinematic_look_weight : AttributeWeight,
    

    pub camera_angle : Option<(&'static str,  egui::ImageSource<'static>,CameraAngle)>,
    pub camera_angle_weight : AttributeWeight,

    pub lighting : Option<(&'static str,  egui::ImageSource<'static>,Lighting)>,
    pub lighting_weight : AttributeWeight,

    pub camera_style : Option<(&'static str,  egui::ImageSource<'static>,CameraStyle)>,
    pub camera_style_weight : AttributeWeight



}

impl Default for FOOCUSDiffusionPromptApp {
    fn default() -> Self {
        Self {
            // initialize all fields to their default values:
            subject_description : String::from(""),
            subject_weight : AttributeWeight::Low,
            subject_attributes : Vec::new(),        

            cinematic_look : None,
            cinematic_look_weight  : AttributeWeight::Low,
            camera_angle : None,
            camera_angle_weight   : AttributeWeight::Low,
            lighting   : None,
            lighting_weight : AttributeWeight::Low,
            camera_style    : None,
            camera_style_weight : AttributeWeight::Low,
        }
    }
}

// implement gen_prompt for FOOCUSDiffusionPromptApp returning a prompt as string

fn gen_prompt( status:FOOCUSDiffusionPromptApp ) -> String {
        // TODO: implement this
        // generate a random integer
        let mut rng = thread_rng();
        return format!("{} {}",rng.gen::<i32>(),status.subject_description);
}


impl eframe::App for FOOCUSDiffusionPromptApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {


            ui.heading("Prompt creator for Fooocus Diffusion");

            ui.horizontal(|ui| {

                let decription_label = ui.label(" Subject decription : ");
                ui.text_edit_singleline(&mut self.subject_description)
                    .labelled_by(decription_label.id);

                let description_weight_label  = ui.label(" Subject Weightage : ");
                ui.radio_value(&mut self.subject_weight, AttributeWeight::Low, "Low").labelled_by(description_weight_label.id);
                ui.radio_value(&mut self.subject_weight, AttributeWeight::High, "High").labelled_by(description_weight_label.id);

            });

            ui.horizontal(|ui| {

                // adding upto 5 additional attrbutes
                let attributes_label = ui.label(" You can add upto 5 attributes of the subject below (all optional) and mention their priority : ");
            });

            ui.horizontal(|ui| {

                // Attribute 1
                let mut attribute1 = String::from("");
                let mut attribute1_weight = AttributeWeight::Low;
                let attribute1_decription_label = ui.label(" Attribute1 [Optional] : ");
                ui.text_edit_singleline(&mut attribute1)
                    .labelled_by(attribute1_decription_label.id);
                let attribute1_description_weight_label  = ui.label(" Subject Weightage : ");
                ui.radio_value(&mut attribute1_weight, AttributeWeight::Low, "Low").labelled_by(attribute1_description_weight_label.id);
                ui.radio_value(&mut attribute1_weight, AttributeWeight::High, "High").labelled_by(attribute1_description_weight_label.id);

                if attribute1 != String::from(""){
                    self.subject_attributes.push( (attribute1, attribute1_weight) );
                }

            });

            ui.horizontal(|ui| {
                // Similarly add Attribute 2
                let mut attribute2 = String::from("");
                let mut attribute2_weight = AttributeWeight::Low;
                let attribute2_decription_label   = ui.label(" Attribute2 [Optional] : ");
                ui.text_edit_singleline(&mut attribute2)
                    .labelled_by(attribute2_decription_label.id);
                let attribute2_description_weight_label   = ui.label(" Subject Weightage  :  ");
                ui.radio_value(&mut attribute2_weight, AttributeWeight::Low, "Low").labelled_by(attribute2_description_weight_label.id);
                ui.radio_value(&mut attribute2_weight, AttributeWeight::High, "High").labelled_by(attribute2_description_weight_label.id);
                
                if attribute2 != String::from(""){
                    self.subject_attributes.push( (attribute2, attribute2_weight) );
                }

            });

            ui.horizontal(|ui| {
                // Similarly add Attribute 3
                let mut attribute3 = String::from("");
                let mut attribute3_weight = AttributeWeight::Low;
                let attribute3_decription_label   = ui.label(" Attribute3 [Optional] : ");
                ui.text_edit_singleline(&mut attribute3)
                    .labelled_by(attribute3_decription_label.id);
                let attribute3_description_weight_label   = ui.label(" Subject Weightage  :  ");
                ui.radio_value(&mut attribute3_weight, AttributeWeight::Low, "Low").labelled_by(attribute3_description_weight_label.id);
                ui.radio_value(&mut attribute3_weight, AttributeWeight::High, "High").labelled_by(attribute3_description_weight_label.id);

                if attribute3 != String::from(""){
                    self.subject_attributes.push( (attribute3, attribute3_weight) );
                }

            });

            ui.horizontal(|ui| {
                // Similarity add attribute 4 and 5
                let mut attribute4 = String::from("");
                let mut attribute4_weight  = AttributeWeight::Low;

                let attribute4_decription_label    = ui.label(" Attribute4 [Optional] : ");
                ui.text_edit_singleline(&mut attribute4)
                  .labelled_by(attribute4_decription_label.id);
                let attribute4_description_weight_label    = ui.label(" Subject Weightage   :   ");
                ui.radio_value(&mut attribute4_weight, AttributeWeight::Low,"Low").labelled_by(attribute4_description_weight_label.id);
                ui.radio_value(&mut attribute4_weight, AttributeWeight::High,"High").labelled_by(attribute4_description_weight_label.id);

                if attribute4 != String::from(""){
                    self.subject_attributes.push( (attribute4, attribute4_weight) );
                }

            });

            ui.horizontal(|ui| {
                let mut attribute5 = String::from("");
                let mut attribute5_weight  = AttributeWeight::Low;  
                let attribute5_decription_label    = ui.label(" Attribute5 [Optional] : ");
                ui.text_edit_singleline(&mut attribute5)
                  .labelled_by(attribute5_decription_label.id);
                let attribute5_description_weight_label    = ui.label(" Subject Weightage   :   ");
                ui.radio_value(&mut attribute5_weight, AttributeWeight::Low,"Low").labelled_by(attribute5_description_weight_label.id);
                ui.radio_value(&mut attribute5_weight, AttributeWeight::High,"High").labelled_by(attribute5_description_weight_label.id);

                if attribute5 != String::from(""){
                    self.subject_attributes.push( (attribute5, attribute5_weight) );
                }

            });

            // Code below for the enum Cinematography

            let cinematography_vec = vec![
               ("Cinematic",egui::include_image!("../assets/ferris.png"),Cinematography::Cinematic),
               // Add new elements with the first being incrementing index, second being "../assets/ferris.png", third being from [   Cinematography::UltraRealistic, Cinematography::FilmGrain, Cinematography::DramaticLighting, Cinematography::GenreHorror, Cinematography::GenreWestern, Cinematography::GenreFantasy, Cinematography::GenreRomantic, Cinematography::GenreAnimation, Cinematography::GenreManga,  Cinematography::ActionScene, Cinematography::MotionBlur, Cinematography::DynamicAction,  Cinematography::DynamicMotion]
               ("Ultra Realistic",egui::include_image!("../assets/ferris.png"),Cinematography::UltraRealistic),
                ("Film Grain",egui::include_image!("../assets/ferris.png"),Cinematography::FilmGrain),
                ("Dramatic Lighting",egui::include_image!("../assets/ferris.png"),Cinematography::DramaticLighting),
                ("Horror",egui::include_image!("../assets/ferris.png"),Cinematography::GenreHorror),
                ("Western",egui::include_image!("../assets/ferris.png"),Cinematography::GenreWestern),
                ("Fantasy",egui::include_image!("../assets/ferris.png"),Cinematography::GenreFantasy),
                ("Romantic",egui::include_image!("../assets/ferris.png"),Cinematography::GenreRomantic),
                ("Animation",egui::include_image!("../assets/ferris.png"),Cinematography::GenreAnimation),
                ("Manga",egui::include_image!("../assets/ferris.png"),Cinematography::GenreManga),
                ("Action Scene",egui::include_image!("../assets/ferris.png"),Cinematography::ActionScene),
                ("Motion Blur",egui::include_image!("../assets/ferris.png"),Cinematography::MotionBlur),
                ("Dynamic Action",egui::include_image!("../assets/ferris.png"),Cinematography::DynamicAction),
                ("Dynamic Motion",egui::include_image!("../assets/ferris.png"),Cinematography::DynamicMotion),
            ];

            ui.label(" Cinematography :  ");

            ui.horizontal(|ui| {


                let mut selected_cinematography_index :Option<usize>= None; 
                // Create a button for each image option
                for (index, (string_name,image_path,cinematography)) in cinematography_vec.iter().enumerate() {
                    if ui.button(string_name.clone()).clicked() {
                        selected_cinematography_index = Some(index);
                        self.cinematic_look = Some((string_name.clone(),image_path.clone(),cinematography.clone()));
                    }
                }    
                if ui.button("None").clicked(){
                    self.cinematic_look = None;
                }

            });


            

            // Display the selected image if available
            if let Some(cinematic_look) = self.cinematic_look.clone() {
                        ui.image(cinematic_look.1);
            }
            
            
            ui.horizontal(|ui| {
            let cinematic_look_label  = ui.label(" Cinematography Look Weightage : ");
                ui.radio_value(&mut self.cinematic_look_weight, AttributeWeight::Low, "Low").labelled_by(cinematic_look_label.id);
                ui.radio_value(&mut self.cinematic_look_weight, AttributeWeight::High, "High").labelled_by(cinematic_look_label.id);
            });

            // Camera Angle Selection

            let camera_angle_vec = vec![
                            ("Extreme Close Up",egui::include_image!("../assets/ferris.png"),CameraAngle::ExtremeCloseUp),
                            ("Close Up",egui::include_image!("../assets/ferris.png"),CameraAngle::CloseUp),
                            ("Medium Shot",egui::include_image!("../assets/ferris.png"),CameraAngle::MediumShot),
                            ("Over The Shoulder",egui::include_image!("../assets/ferris.png"),CameraAngle::OverTheShoulder),
                            ("Long Shot",egui::include_image!("../assets/ferris.png"),CameraAngle::LongShot),
                            ("Extreme Long Shot",egui::include_image!("../assets/ferris.png"),CameraAngle::ExtremeLongShot),
                            ("Full Shot Shot",egui::include_image!("../assets/ferris.png"),CameraAngle::FullShot),
                            ("Full Body View",egui::include_image!("../assets/ferris.png"),CameraAngle::FullBodyView),
                            ("POV Shot",egui::include_image!("../assets/ferris.png"),CameraAngle::POVShot),
                            ("Eye Level Shot",egui::include_image!("../assets/ferris.png"),CameraAngle::EyeLevelShot),
                            ("High Angle Shot",egui::include_image!("../assets/ferris.png"),CameraAngle::HighAngleShot),
                            ("Low Angle Shot",egui::include_image!("../assets/ferris.png"),CameraAngle::LowAngleShot),
                            ("Dutch Angle Shot",egui::include_image!("../assets/ferris.png"),CameraAngle::DutchAngleShot),
                            ("Drone Shot",egui::include_image!("../assets/ferris.png"),CameraAngle::DroneShot),
                            ("Gro pro Shot",egui::include_image!("../assets/ferris.png"),CameraAngle::GoProShot),
                            ("Fish eye Shot",egui::include_image!("../assets/ferris.png"),CameraAngle::FishEyeShot),
                            ("Bird eye view",egui::include_image!("../assets/ferris.png"),CameraAngle::BirdEyeView),
                            ("Rule of thirds Shot",egui::include_image!("../assets/ferris.png"),CameraAngle::RuleOfThirdShot),
                            ("Candid Shot",egui::include_image!("../assets/ferris.png"),CameraAngle::CandidShot),
                            ("Silhouette Shot",egui::include_image!("../assets/ferris.png"),CameraAngle::SilhouetteShot),
            ];

            ui.label(" Camera Angle :  ");

            ui.horizontal(|ui| {


                let mut selected_camera_angle_index :Option<usize>= None; 
                // Create a button for each image option
                for (index, (string_name,image_path,camera_angle)) in camera_angle_vec.iter().enumerate() {
                    if ui.button(string_name.clone()).clicked() {
                        selected_camera_angle_index = Some(index);
                        self.camera_angle = Some((string_name.clone(),image_path.clone(),camera_angle.clone()));
                    }
                }    
                if ui.button("None").clicked(){
                    self.camera_angle = None;
                }

            });

            // Display the selected image if available
            if let Some(camera_angle) = self.camera_angle.clone() {
                            ui.image(camera_angle.1);
            }

            ui.horizontal(|ui| {
                let camera_angle_weight_label  = ui.label(" Camera Angle Weightage : ");
                    ui.radio_value(&mut self.camera_angle_weight, AttributeWeight::Low, "Low").labelled_by(camera_angle_weight_label.id);
                    ui.radio_value(&mut self.camera_angle_weight, AttributeWeight::High, "High").labelled_by(camera_angle_weight_label.id);
                });

            // Selected the camera angle above

            // StudioLights, BrightLights, NeonLights, WarmLights , ColdLights , HighKeyLighting , LowKeyLighting, RimLighting , PracticaLighting , MotivatorLighting , Sunny , GoldenHour , Rainy , Foggy , Night , Afternoon 
            let lighting_vec = vec![
                ("Studio Lights",egui::include_image!("../assets/ferris.png"),Lighting::StudioLights),
                ("Bright Lights",egui::include_image!("../assets/ferris.png"),Lighting::BrightLights),
                ("Neon Lights",egui::include_image!("../assets/ferris.png"),Lighting::NeonLights),
                ("Warm Lights",egui::include_image!("../assets/ferris.png"),Lighting::WarmLights),
                ("Cold Lights",egui::include_image!("../assets/ferris.png"),Lighting::ColdLights),
                ("High Key Lighting",egui::include_image!("../assets/ferris.png"),Lighting::HighKeyLighting),
                ("Low Key Lighting",egui::include_image!("../assets/ferris.png"),Lighting::LowKeyLighting),
                ("Rim Lighting",egui::include_image!("../assets/ferris.png"),Lighting::RimLighting),
                ("Practical Lighting",egui::include_image!("../assets/ferris.png"),Lighting::PracticalLighting),
                ("Motivator Lighting",egui::include_image!("../assets/ferris.png"),Lighting::MotivatorLighting),
                ("Sunny",egui::include_image!("../assets/ferris.png"),Lighting::Sunny),
                ("Golden Hour",egui::include_image!("../assets/ferris.png"),Lighting::GoldenHour),
                ("Rainy",egui::include_image!("../assets/ferris.png"),Lighting::Rainy),
                ("Foggy",egui::include_image!("../assets/ferris.png"),Lighting::Foggy),
                ("Night",egui::include_image!("../assets/ferris.png"),Lighting::Night),
                ("Afternoon",egui::include_image!("../assets/ferris.png"),Lighting::Afternoon)
            ];

            ui.label(" Lighting :  ");

            ui.horizontal(|ui| {


                let mut selected_lighting_index :Option<usize>= None; 
                // Create a button for each image option
                for (index, (string_name,image_path,lighting)) in lighting_vec.iter().enumerate() {
                    if ui.button(string_name.clone()).clicked() {
                        selected_lighting_index = Some(index);
                        self.lighting = Some((string_name.clone(),image_path.clone(),lighting.clone()));
                    }
                }    
                if ui.button("None").clicked(){
                    self.lighting = None;
                }

            });

            // Display the selected image if available
            if let Some(Lighting) = self.lighting.clone() {
                ui.image(Lighting.1);
            }

            ui.horizontal(|ui| {
                let lighting_label  = ui.label(" Lighting Weightage : ");
                    ui.radio_value(&mut self.lighting_weight, AttributeWeight::Low, "Low").labelled_by(lighting_label.id);
                    ui.radio_value(&mut self.lighting_weight, AttributeWeight::High, "High").labelled_by(lighting_label.id);
                });


            // Selected Lighting Above
            
            // Select camera style below out of     ArriAlexa, Super16_VintageFilm, CanonCinemaEOS, SonyCineAlta , QuentinTarantinoStyle , AlfredHitchcockStyle, MartinScorseseStyle , ChristopherNolanStyle , MichaelBayStyle  , JohnWooStyle  , PeterHyamsStyles , JamesCameronStyle , ElsaGarrisonStyle , WalterIoossStyle , NeilLeiferStyle, CanonEOS_1DXMarkII , GoProHero9Black , SonyAlphaA9II
            let camera_style_vec = vec![
                ("Arri Alexa", egui::include_image!("../assets/ferris.png"), CameraStyle::ArriAlexa),
                ("Super16 Vintage Film", egui::include_image!("../assets/ferris.png"), CameraStyle:: Super16_VintageFilm),
                ("Canon Cinema EOS", egui::include_image!("../assets/ferris.png"), CameraStyle:: CanonCinemaEOS),
                ("Sony Cine Alta ", egui::include_image!("../assets/ferris.png"), CameraStyle:: SonyCineAlta ),
                ("Quentin Tarantino", egui::include_image!("../assets/ferris.png"), CameraStyle:: QuentinTarantinoStyle  ),
                ("Alfred Hitchcock", egui::include_image!("../assets/ferris.png"), CameraStyle:: AlfredHitchcockStyle  ),
                ("Martin Scorsese", egui::include_image!("../assets/ferris.png"), CameraStyle:: MartinScorseseStyle   ),
                ("Christopher Nolan", egui::include_image!("../assets/ferris.png"), CameraStyle:: ChristopherNolanStyle),
                ("Michael Bay",egui::include_image!("../assets/ferris.png"), CameraStyle:: MichaelBayStyle    ),
                ("John Woo",egui::include_image!("../assets/ferris.png"), CameraStyle:: JohnWooStyle     ),
                ("Peter Hyams  ",egui::include_image!("../assets/ferris.png"), CameraStyle:: PeterHyamsStyles  ),
                ("James Cameron",egui::include_image!("../assets/ferris.png"), CameraStyle:: JamesCameronStyle   ),
                ("Elsa Garrison",egui::include_image!("../assets/ferris.png"), CameraStyle:: ElsaGarrisonStyle),
                ("Walter Iooss",egui::include_image!("../assets/ferris.png"), CameraStyle:: WalterIoossStyle    ),
                ("Neil Leifer",egui::include_image!("../assets/ferris.png"), CameraStyle:: NeilLeiferStyle     ),
                ("Canon EOS 1DX Mark II ", egui::include_image!("../assets/ferris.png"), CameraStyle::CanonEOS_1DXMarkII  ),
                ("Sony Alpha A9 II",egui::include_image!("../assets/ferris.png"), CameraStyle:: SonyAlphaA9II),
            ];

            ui.label(" Camera Style :  ");

            ui.horizontal(|ui| {


                let mut selected_camera_style_index :Option<usize>= None; 
                // Create a button for each image option
                for (index, (string_name,image_path,camera_style)) in camera_style_vec.iter().enumerate() {
                    if ui.button(string_name.clone()).clicked() {
                        selected_camera_style_index = Some(index);
                        self.camera_style = Some((string_name.clone(),image_path.clone(),camera_style.clone()));
                    }
                }    
                if ui.button("None").clicked(){
                    self.lighting = None;
                }

            });

            // Display the selected image if available
            if let Some(camera_style) = self.camera_style.clone() {
                ui.image(camera_style.1);
            }
            
            ui.horizontal(|ui| {
                let camera_style_weight_label  = ui.label(" Camera Angle Weightage : ");
                    ui.radio_value(&mut self.camera_style_weight , AttributeWeight::Low, "Low").labelled_by(camera_style_weight_label.id);
                    ui.radio_value(&mut self.camera_style_weight , AttributeWeight::High, "High").labelled_by(camera_style_weight_label.id);
                });

            // Select the camera style above


            ui.horizontal(|ui| {

                let output_text = gen_prompt(self.clone());
                let po = ui.label("Output::");
                ui.label(output_text).labelled_by(po.id);
                

            });
            

            //ui.image(egui::include_image!(
            //    "../assets/ferris.png"
            //));






        });
    }
}

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1920.0, 950.0]),
        ..Default::default()
    };
    eframe::run_native(
        "App to create Image Generation Prompts",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<FOOCUSDiffusionPromptApp>::default())
        }),
    )
}


