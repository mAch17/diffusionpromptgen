use eframe::egui;


// create an enum called AttrbuteWeight with two values low and high and implementing Debug
#[derive(Debug, PartialEq)]
enum AttributeWeight {
    Low,
    High,
}

// Create an enum called CameraAngle implementing Debug and varinats ExtremeCloseUp, CloseUp, MediumShot, OverTheShoulder, LongShot, ExtremeLongShot, FullShot, FullBodyView, POVShot, EyeLevelShot, HighAngleShot, LowAngleShot, DutchAngleShot, DroneShot, GoProShot, FishEyeShot, birdEyeView, RuleOfThirdShot, CandidShot, SilhouetteShot
#[derive(Debug)]
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
#[derive(Debug)]
enum Look {
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
#[derive(Debug)]
enum Lighting {
    StudioLights, 
    BrightLights, 
    NeonLights, 
    WarmLights , 
    ColdLights , 
    HighKeyLighting , 
    LowKeyLighting, 
    RimLighting , 
    PracticaLighting , 
    MotivatorLighting , 
    Sunny , 
    GoldenHour , 
    Rainy , 
    Foggy , 
    Night , 
    Afternoon 
}

// Create an enum CameraStyle deriving debug and having variants ArriAlexa, Super16_VintageFilm, CanonCinemaEOS, SonyCineAlta , QuentinTarantinoStyle , AlfredHitchcockStyle, MartinScorseseStyle , ChristopherNolanStyle , MichaelBayStyle , JohnWooStyle , PeterHyamsStyles , JamesCameronStyle , ElsaGarrisonStyle , WalterIoossStyle , NeilLeiferStyle, CanonEOS_1DXMarkII , GoProHero9Black , SonyAlphaA9II
#[derive(Debug)]
enum ImageLook {
    StudioLook, 
    BrightLook, 
    NeonLook, 
    WarmLook , 
    ColdLook , 
    HighKeyLook , 
    LowKeyLook, 
    RimLook , 
    PracticaLook , 
    MotivatorLook , 
    SunnyLook , 
    GoldenHourLook , 
    RainyLook , 
    FoggyLook , 
    NightLook , 
    AfternoonLook 
}


// create a struct FOOCUSDiffusionPrompt with no members and it implements Debug
#[derive(Debug)]
struct FOOCUSDiffusionPromptApp{
    pub subject_description : String,
    pub subject_weight : AttributeWeight,
    pub subject_emotion : Option<String>,
    pub subject_attributes : Vec<(String,AttributeWeight)>,
    
    
    pub cinematic_look : Option<Look>,
    pub cinematic_look_weight : AttributeWeight,
    

    pub camera_angle : Option<CameraAngle>,
    pub camera_angle_weight : AttributeWeight,

    pub lighting : Option<Lighting>,
    pub lighting_weight : AttributeWeight,

    pub look : Option<ImageLook>,
    pub look_weight : AttributeWeight



}

impl Default for FOOCUSDiffusionPromptApp {
    fn default() -> Self {
        Self {
            // initialize all fields to their default values:
            subject_description : String::from(""),
            subject_weight : AttributeWeight::Low,
            subject_emotion : None,
            subject_attributes : Vec::new(),        

            cinematic_look : None,
            cinematic_look_weight  : AttributeWeight::Low,
            camera_angle : None,
            camera_angle_weight   : AttributeWeight::Low,
            lighting   : None,
            lighting_weight : AttributeWeight::Low,
            look    : None,
            look_weight : AttributeWeight::Low,
        }
    }
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
            

            ui.image(egui::include_image!(
                "../assets/ferris.png"
            ));


        });
    }
}

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 500.0]),
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


