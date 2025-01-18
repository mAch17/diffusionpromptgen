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
    
    
    
    pub cinematic_look : Option<(&'static str,  egui::ImageSource<'static>,Cinematography)>,
    pub cinematic_look_weight : AttributeWeight,
    

    pub camera_angle : Option<(&'static str,  egui::ImageSource<'static>,CameraAngle)>,
    pub camera_angle_weight : AttributeWeight,

    pub lighting : Option<(&'static str,  egui::ImageSource<'static>,Lighting)>,
    pub lighting_weight : AttributeWeight,

    pub camera_style : Option<(&'static str,  egui::ImageSource<'static>,CameraStyle)>,
    pub camera_style_weight : AttributeWeight,

    pub attribute1 : String,
    pub attribute1_weight : AttributeWeight,

    pub attribute2 : String,
    pub attribute2_weight  : AttributeWeight,

    pub attribute3  : String,
    pub attribute3_weight  : AttributeWeight,

    pub attribute4 : String,
    pub attribute4_weight  : AttributeWeight,

    pub attribute5 : String,
    pub attribute5_weight  : AttributeWeight,


}

impl Default for FOOCUSDiffusionPromptApp {
    fn default() -> Self {
        Self {
            // initialize all fields to their default values:
            subject_description : String::from(""),
            subject_weight : AttributeWeight::Low,
        

            cinematic_look : None,
            cinematic_look_weight  : AttributeWeight::Low,
            camera_angle : None,
            camera_angle_weight   : AttributeWeight::Low,
            lighting   : None,
            lighting_weight : AttributeWeight::Low,
            camera_style    : None,
            camera_style_weight : AttributeWeight::Low,

            attribute1 : String::from(""),
            attribute1_weight   : AttributeWeight::Low,
            attribute2   : String::from(""),
            attribute2_weight    : AttributeWeight::Low,
            attribute3    : String::from(""),
            attribute3_weight     : AttributeWeight::Low,
            attribute4     : String::from(""),
            attribute4_weight      : AttributeWeight::Low,
            attribute5      : String::from(""),
            attribute5_weight       : AttributeWeight::Low,
        }
    }
}

// implement gen_prompt for FOOCUSDiffusionPromptApp returning a prompt as string
fn make_high_weight(prompt:String) -> String {
    format!("({},1.3)",prompt)
    
}

fn gen_prompt( status:FOOCUSDiffusionPromptApp ) -> String {
        // TODO: implement this
        // generate a random integer
        let mut rng = thread_rng();
        let mut desc = status.subject_description;
        if matches!(status.subject_weight, AttributeWeight::High) {
            desc = make_high_weight(desc);
        }
        
        // Iterate over status.subject_attributes and create an accumulated string with all strings in first element,when the second element of tuple is AttributeWeight::High, apply the make_high_weight function to the first element of tuple function
        let mut attrstr = String::new();
        if status.attribute1!=String::from("") {
            
            if status.attribute1_weight == AttributeWeight::High {
                attrstr = format!("{} {}",attrstr,make_high_weight(status.attribute1));
            } else {
                attrstr = format!("{} {}",attrstr,status.attribute1);
            }
        }
        if status.attribute2!=String::from("") {

            if status.attribute2_weight == AttributeWeight::High {
                attrstr = format!("{} {}",attrstr,make_high_weight(status.attribute2));
            } else {
                attrstr = format!("{} {}",attrstr,status.attribute2);
             }
         }
        if status.attribute3!=String::from("")  {
            if status.attribute3_weight == AttributeWeight::High {
                attrstr = format!("{}  {}",attrstr,make_high_weight(status.attribute3));
            } else {
                attrstr = format!("{}  {}",attrstr,status.attribute3);
            }
        }
        if status.attribute4!=String::from("") {
            if status.attribute4_weight == AttributeWeight::High  {
                attrstr = format!("{}   {}",attrstr,make_high_weight(status.attribute4));
             } else  {
                 attrstr = format!("{}   {}",attrstr,status.attribute4);
             }
        }
        if status.attribute5!=String::from("") {
            if status.attribute5_weight == AttributeWeight::High  {
                attrstr = format!("{}   {}",attrstr,make_high_weight(status.attribute5));
             } else  {
                 attrstr = format!("{}   {}",attrstr,status.attribute5);
             }
        }
        



        return format!("{} {} {}",rng.gen::<i32>(),desc,attrstr);
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

           

            // adding upto 5 additional attrbutes
            let attributes_label = ui.label(" You can add upto 5 attributes of the subject below (all optional) and mention their priority : ");


            ui.horizontal(|ui| {

                // Attribute 1
                
                let attribute1_decription_label = ui.label(" Attribute1 [Optional] : ");
                ui.text_edit_singleline(&mut self.attribute1)
                    .labelled_by(attribute1_decription_label.id);
                let attribute1_description_weight_label  = ui.label(" Subject Weightage : ");
                ui.radio_value(&mut self.attribute1_weight, AttributeWeight::Low, "Low").labelled_by(attribute1_description_weight_label.id);
                ui.radio_value(&mut self.attribute1_weight, AttributeWeight::High, "High").labelled_by(attribute1_description_weight_label.id);



            });

            ui.horizontal(|ui| {
                // Similarly add Attribute 2
                let attribute2_decription_label   = ui.label(" Attribute2 [Optional] : ");
                ui.text_edit_singleline(&mut self.attribute2)
                    .labelled_by(attribute2_decription_label.id);
                let attribute2_description_weight_label   = ui.label(" Subject Weightage  :  ");
                ui.radio_value(&mut self.attribute2_weight, AttributeWeight::Low, "Low").labelled_by(attribute2_description_weight_label.id);
                ui.radio_value(&mut self.attribute2_weight, AttributeWeight::High, "High").labelled_by(attribute2_description_weight_label.id);


            });

            ui.horizontal(|ui| {
                // Similarly add Attribute 3
                let attribute3_decription_label   = ui.label(" Attribute3 [Optional] : ");
                ui.text_edit_singleline(&mut self.attribute3)
                    .labelled_by(attribute3_decription_label.id);
                let attribute3_description_weight_label   = ui.label(" Subject Weightage  :  ");
                ui.radio_value(&mut self.attribute3_weight, AttributeWeight::Low, "Low").labelled_by(attribute3_description_weight_label.id);
                ui.radio_value(&mut self.attribute3_weight, AttributeWeight::High, "High").labelled_by(attribute3_description_weight_label.id);

            });

            ui.horizontal(|ui| {
                // Similarity add attribute 4 and 5

                let attribute4_decription_label    = ui.label(" Attribute4 [Optional] : ");
                ui.text_edit_singleline(&mut self.attribute4)
                  .labelled_by(attribute4_decription_label.id);
                let attribute4_description_weight_label    = ui.label(" Subject Weightage   :   ");
                ui.radio_value(&mut self.attribute4_weight, AttributeWeight::Low,"Low").labelled_by(attribute4_description_weight_label.id);
                ui.radio_value(&mut self.attribute4_weight, AttributeWeight::High,"High").labelled_by(attribute4_description_weight_label.id);

            });

            ui.horizontal(|ui| { 
                let attribute5_decription_label    = ui.label(" Attribute5 [Optional] : ");
                ui.text_edit_singleline(&mut self.attribute5)
                  .labelled_by(attribute5_decription_label.id);
                let attribute5_description_weight_label    = ui.label(" Subject Weightage   :   ");
                ui.radio_value(&mut self.attribute5_weight, AttributeWeight::Low,"Low").labelled_by(attribute5_description_weight_label.id);
                ui.radio_value(&mut self.attribute5_weight, AttributeWeight::High,"High").labelled_by(attribute5_description_weight_label.id);


            });

            // Code below for the enum Cinematography

            let cinematography_vec = vec![
                (0 as usize,"Cinematic",egui::include_image!("../assets/ferris.png"),Cinematography::Cinematic),
               // Add new elements with the first being incrementing index, second being "../assets/ferris.png", third being from [   Cinematography::UltraRealistic, Cinematography::FilmGrain, Cinematography::DramaticLighting, Cinematography::GenreHorror, Cinematography::GenreWestern, Cinematography::GenreFantasy, Cinematography::GenreRomantic, Cinematography::GenreAnimation, Cinematography::GenreManga,  Cinematography::ActionScene, Cinematography::MotionBlur, Cinematography::DynamicAction,  Cinematography::DynamicMotion]
                (1 as usize,"Ultra Realistic",egui::include_image!("../assets/ferris.png"),Cinematography::UltraRealistic),
                (2 as usize, "Film Grain",egui::include_image!("../assets/ferris.png"),Cinematography::FilmGrain),
                (3 as usize, "Dramatic Lighting",egui::include_image!("../assets/ferris.png"),Cinematography::DramaticLighting),
                (4 as usize, "Horror",egui::include_image!("../assets/ferris.png"),Cinematography::GenreHorror),
                (5 as usize, "Western",egui::include_image!("../assets/ferris.png"),Cinematography::GenreWestern),
                (6 as usize, "Fantasy",egui::include_image!("../assets/ferris.png"),Cinematography::GenreFantasy),
                (7 as usize, "Romantic",egui::include_image!("../assets/ferris.png"),Cinematography::GenreRomantic),
                (8 as usize, "Animation",egui::include_image!("../assets/ferris.png"),Cinematography::GenreAnimation),
                (9 as usize, "Manga",egui::include_image!("../assets/ferris.png"),Cinematography::GenreManga),
                (10 as usize,"Action Scene",egui::include_image!("../assets/ferris.png"),Cinematography::ActionScene),
                (11 as usize, "Motion Blur",egui::include_image!("../assets/ferris.png"),Cinematography::MotionBlur),
                (12 as usize, "Dynamic Action",egui::include_image!("../assets/ferris.png"),Cinematography::DynamicAction),
                (13 as usize, "Dynamic Motion",egui::include_image!("../assets/ferris.png"),Cinematography::DynamicMotion),
            ];

            ui.label(" Cinematography :  ");

            ui.horizontal(|ui| {
                ui.vertical(|ui| {


                    let mut selected_cinematography_index :Option<usize>= None; 
                    // Create a button for each image option
                    for chunk in cinematography_vec.chunks(5){
                        ui.horizontal(|ui| {
                            for (index, string_name,image_path,cinematography) in chunk.iter() {
                                if ui.button(string_name.clone()).clicked() {
                                    selected_cinematography_index = Some(*index);
                                    self.cinematic_look = Some((string_name.clone(),image_path.clone(),cinematography.clone()));
                                }
                            }
                        });
                    }    
                    if ui.button("None").clicked(){
                        self.cinematic_look = None;
                    }

                });


                

                // Display the selected image if available
                if let Some(cinematic_look) = self.cinematic_look.clone() {
                            ui.image(cinematic_look.1);
                }
            });
            
            
            ui.horizontal(|ui| {
            let cinematic_look_label  = ui.label(" Cinematography Look Weightage : ");
                ui.radio_value(&mut self.cinematic_look_weight, AttributeWeight::Low, "Low").labelled_by(cinematic_look_label.id);
                ui.radio_value(&mut self.cinematic_look_weight, AttributeWeight::High, "High").labelled_by(cinematic_look_label.id);
            });

            // Camera Angle Selection

            let camera_angle_vec = vec![
                            (0 as usize,"Extreme Close Up",egui::include_image!("../assets/ferris.png"),CameraAngle::ExtremeCloseUp),
                            (1 as usize,"Close Up",egui::include_image!("../assets/ferris.png"),CameraAngle::CloseUp),
                            (2 as usize,"Medium Shot",egui::include_image!("../assets/ferris.png"),CameraAngle::MediumShot),
                            (3 as usize,"Over The Shoulder",egui::include_image!("../assets/ferris.png"),CameraAngle::OverTheShoulder),
                            (4 as usize,"Long Shot",egui::include_image!("../assets/ferris.png"),CameraAngle::LongShot),
                            (5 as usize,"Extreme Long Shot",egui::include_image!("../assets/ferris.png"),CameraAngle::ExtremeLongShot),
                            (6 as usize,"Full Shot Shot",egui::include_image!("../assets/ferris.png"),CameraAngle::FullShot),
                            (7 as usize,"Full Body View",egui::include_image!("../assets/ferris.png"),CameraAngle::FullBodyView),
                            (8 as usize,"POV Shot",egui::include_image!("../assets/ferris.png"),CameraAngle::POVShot),
                            (9 as usize,"Eye Level Shot",egui::include_image!("../assets/ferris.png"),CameraAngle::EyeLevelShot),
                            (10 as usize,"High Angle Shot",egui::include_image!("../assets/ferris.png"),CameraAngle::HighAngleShot),
                            (11 as usize,"Low Angle Shot",egui::include_image!("../assets/ferris.png"),CameraAngle::LowAngleShot),
                            (12 as usize,"Dutch Angle Shot",egui::include_image!("../assets/ferris.png"),CameraAngle::DutchAngleShot),
                            (13 as usize,"Drone Shot",egui::include_image!("../assets/ferris.png"),CameraAngle::DroneShot),
                            (14 as usize,"Gro pro Shot",egui::include_image!("../assets/ferris.png"),CameraAngle::GoProShot),
                            (15 as usize,"Fish eye Shot",egui::include_image!("../assets/ferris.png"),CameraAngle::FishEyeShot),
                            (16 as usize,"Bird eye view",egui::include_image!("../assets/ferris.png"),CameraAngle::BirdEyeView),
                            (17 as usize,"Rule of thirds Shot",egui::include_image!("../assets/ferris.png"),CameraAngle::RuleOfThirdShot),
                            (18 as usize,"Candid Shot",egui::include_image!("../assets/ferris.png"),CameraAngle::CandidShot),
                            (19 as usize,"Silhouette Shot",egui::include_image!("../assets/ferris.png"),CameraAngle::SilhouetteShot),
            ];

            ui.label(" Camera Angle :  ");

            ui.horizontal(|ui| {
                ui.vertical(|ui| {


                    let mut selected_camera_angle_index :Option<usize>= None; 
                    // Create a button for each image option
                    for chunk in camera_angle_vec.chunks(5){
                        ui.horizontal(|ui| {
                            for (index, string_name,image_path,camera_angle) in chunk.iter() {
                                if ui.button(string_name.clone()).clicked() {
                                    selected_camera_angle_index = Some(*index);
                                    self.camera_angle = Some((string_name.clone(),image_path.clone(),camera_angle.clone()));
                                }
                            }
                        });
                    }

                    if ui.button("None").clicked(){
                        self.camera_angle = None;
                    }

                });

                // Display the selected image if available
                if let Some(camera_angle) = self.camera_angle.clone() {
                                ui.image(camera_angle.1);
                }

            });
            ui.horizontal(|ui| {
                let camera_angle_weight_label  = ui.label(" Camera Angle Weightage : ");
                    ui.radio_value(&mut self.camera_angle_weight, AttributeWeight::Low, "Low").labelled_by(camera_angle_weight_label.id);
                    ui.radio_value(&mut self.camera_angle_weight, AttributeWeight::High, "High").labelled_by(camera_angle_weight_label.id);
            });

            // Selected the camera angle above

            // StudioLights, BrightLights, NeonLights, WarmLights , ColdLights , HighKeyLighting , LowKeyLighting, RimLighting , PracticaLighting , MotivatorLighting , Sunny , GoldenHour , Rainy , Foggy , Night , Afternoon 
            let lighting_vec = vec![
                (0 as usize,"Studio Lights",egui::include_image!("../assets/ferris.png"),Lighting::StudioLights),
                (1 as usize,"Bright Lights",egui::include_image!("../assets/ferris.png"),Lighting::BrightLights),
                (2 as usize,"Neon Lights",egui::include_image!("../assets/ferris.png"),Lighting::NeonLights),
                (3 as usize,"Warm Lights",egui::include_image!("../assets/ferris.png"),Lighting::WarmLights),
                (4 as usize,"Cold Lights",egui::include_image!("../assets/ferris.png"),Lighting::ColdLights),
                (5 as usize,"High Key Lighting",egui::include_image!("../assets/ferris.png"),Lighting::HighKeyLighting),
                (6 as usize,"Low Key Lighting",egui::include_image!("../assets/ferris.png"),Lighting::LowKeyLighting),
                (7 as usize,"Rim Lighting",egui::include_image!("../assets/ferris.png"),Lighting::RimLighting),
                (8 as usize,"Practical Lighting",egui::include_image!("../assets/ferris.png"),Lighting::PracticalLighting),
                (9 as usize,"Motivator Lighting",egui::include_image!("../assets/ferris.png"),Lighting::MotivatorLighting),
                (10 as usize,"Sunny",egui::include_image!("../assets/ferris.png"),Lighting::Sunny),
                (11 as usize,"Golden Hour",egui::include_image!("../assets/ferris.png"),Lighting::GoldenHour),
                (12 as usize,"Rainy",egui::include_image!("../assets/ferris.png"),Lighting::Rainy),
                (13 as usize,"Foggy",egui::include_image!("../assets/ferris.png"),Lighting::Foggy),
                (14 as usize,"Night",egui::include_image!("../assets/ferris.png"),Lighting::Night),
                (15 as usize,"Afternoon",egui::include_image!("../assets/ferris.png"),Lighting::Afternoon)
            ];

            ui.label(" Lighting :  ");

            ui.horizontal(|ui| {
                ui.vertical(|ui| {

                    let mut selected_lighting_index :Option<usize>= None; 
                    // Create a button for each image option
                    for chunk in lighting_vec.chunks(5){
                        ui.horizontal(|ui| {
                            for (index, string_name,image_path,lighting) in chunk.iter() {
                                if ui.button(string_name.clone()).clicked() {
                                    selected_lighting_index = Some(*index);
                                    self.lighting = Some((string_name.clone(),image_path.clone(),lighting.clone()));
                                }
                            }
                        });
                    }
                    if ui.button("None").clicked(){
                        self.lighting = None;
                    }

                });

                // Display the selected image if available
                if let Some(Lighting) = self.lighting.clone() {
                    ui.image(Lighting.1);
                }
            });

            ui.horizontal(|ui| {
                let lighting_label  = ui.label(" Lighting Weightage : ");
                    ui.radio_value(&mut self.lighting_weight, AttributeWeight::Low, "Low").labelled_by(lighting_label.id);
                    ui.radio_value(&mut self.lighting_weight, AttributeWeight::High, "High").labelled_by(lighting_label.id);
                });


            // Selected Lighting Above
            
            // Select camera style below out of     ArriAlexa, Super16_VintageFilm, CanonCinemaEOS, SonyCineAlta , QuentinTarantinoStyle , AlfredHitchcockStyle, MartinScorseseStyle , ChristopherNolanStyle , MichaelBayStyle  , JohnWooStyle  , PeterHyamsStyles , JamesCameronStyle , ElsaGarrisonStyle , WalterIoossStyle , NeilLeiferStyle, CanonEOS_1DXMarkII , GoProHero9Black , SonyAlphaA9II
            let camera_style_vec = vec![
                (0 as usize,"Arri Alexa", egui::include_image!("../assets/ferris.png"), CameraStyle::ArriAlexa),
                (1 as usize,"Super16 Vintage Film", egui::include_image!("../assets/ferris.png"), CameraStyle:: Super16_VintageFilm),
                (2 as usize,"Canon Cinema EOS", egui::include_image!("../assets/ferris.png"), CameraStyle:: CanonCinemaEOS),
                (3 as usize,"Sony Cine Alta ", egui::include_image!("../assets/ferris.png"), CameraStyle:: SonyCineAlta ),
                (4 as usize,"Quentin Tarantino", egui::include_image!("../assets/ferris.png"), CameraStyle:: QuentinTarantinoStyle  ),
                (5 as usize,"Alfred Hitchcock", egui::include_image!("../assets/ferris.png"), CameraStyle:: AlfredHitchcockStyle  ),
                (6 as usize,"Martin Scorsese", egui::include_image!("../assets/ferris.png"), CameraStyle:: MartinScorseseStyle   ),
                (7 as usize,"Christopher Nolan", egui::include_image!("../assets/ferris.png"), CameraStyle:: ChristopherNolanStyle),
                (8 as usize,"Michael Bay",egui::include_image!("../assets/ferris.png"), CameraStyle:: MichaelBayStyle    ),
                (9 as usize,"John Woo",egui::include_image!("../assets/ferris.png"), CameraStyle:: JohnWooStyle     ),
                (10 as usize,"Peter Hyams  ",egui::include_image!("../assets/ferris.png"), CameraStyle:: PeterHyamsStyles  ),
                (11 as usize,"James Cameron",egui::include_image!("../assets/ferris.png"), CameraStyle:: JamesCameronStyle   ),
                (12 as usize,"Elsa Garrison",egui::include_image!("../assets/ferris.png"), CameraStyle:: ElsaGarrisonStyle),
                (13 as usize,"Walter Iooss",egui::include_image!("../assets/ferris.png"), CameraStyle:: WalterIoossStyle    ),
                (14 as usize,"Neil Leifer",egui::include_image!("../assets/ferris.png"), CameraStyle:: NeilLeiferStyle     ),
                (15 as usize,"Canon EOS 1DX Mark II ", egui::include_image!("../assets/ferris.png"), CameraStyle::CanonEOS_1DXMarkII  ),
                (16 as usize,"Sony Alpha A9 II",egui::include_image!("../assets/ferris.png"), CameraStyle:: SonyAlphaA9II),
            ];

            ui.label(" Camera Style :  ");

            ui.horizontal(|ui| {
                ui.vertical(|ui| {


                    let mut selected_camera_style_index :Option<usize>= None; 
                    // Create a button for each image option
                    for chunk in camera_style_vec.chunks(5){
                        ui.horizontal(|ui| {
                            for (index, string_name,image_path,camera_style) in chunk.iter() {
                                if ui.button(string_name.clone()).clicked() {
                                    selected_camera_style_index = Some(*index);
                                    self.camera_style = Some((string_name.clone(),image_path.clone(),camera_style.clone()));
                                }
                            }
                        });
                    }
                    if ui.button("None").clicked(){
                        self.lighting = None;
                    }
                

                });

                // Display the selected image if available
                if let Some(camera_style) = self.camera_style.clone() {
                    ui.image(camera_style.1);
                }
            });
            
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


