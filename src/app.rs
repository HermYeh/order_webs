/// We derive Deserialize/Serialize so we can persist app state on shutdown.


use egui::{ColorImage,Label, TextStyle, Ui};
use std::fs;
use egui_extras::RetainedImage;
use std::time::Duration;
use egui_extras::{TableBuilder, Column};
use chrono::{DateTime, Local};
use std::path::Path;
use eframe::{egui};
use egui::{ Id, RichText, TextureHandle, Vec2};
use image;
use std::sync::mpsc::channel;
use reqwest::Client;
use wasm_bindgen_futures::spawn_local;
use crate::order_table;
#[derive(serde::Deserialize, serde::Serialize,Clone)]
#[serde(default)]
pub struct TemplateApp {
    // Example stuff:
    label: String,
    
// This how you opt-out of serialization of a field
    pub order_number: Vec<String>,
    #[serde(skip)]
    pub total_order:Vec<(String, String,bool)>,
    pub order_time:Vec<String>,
    pub selection: usize,
    rows: i32,
    row_index: i32,
    friedbun_count: i32,
    pub payment: Vec<bool>,
    pub scroll_to_row: Option<usize>,
    name:String,
}

fn check_order(template_app:&mut TemplateApp){
     
    if template_app.order_number.len()==4{
        let target_first_value =&template_app.order_number.concat();
        let contains_first_value : Vec<_>= template_app.total_order.iter().enumerate()
        .filter_map(|(index,(first, _,_))|{if first == target_first_value {
                Some(index)
            } else {
                None
            }
        })
        .collect();
        if !contains_first_value.is_empty() {
          
            for index in contains_first_value {
                template_app.total_order.remove(index);
            }
            template_app.order_number.clear();
        } else {
            let time: DateTime<Local> = Local::now();
            println!("{}",time.to_rfc3339().to_string());
            template_app.total_order.push((template_app.order_number.concat(),time.to_rfc3339(),false));

            template_app.order_number.clear();
            
        }
        save_to_remote(template_app.total_order.clone());
      
    };
}
fn buttons(template_app:&mut TemplateApp,ui:&mut Ui){
     let wsize=ui.available_width();
     ui.vertical(|ui| {
               
    ui.horizontal(|ui| {     
            
        for but_index in 1..4{
            let button = ui.add_sized(
                [wsize/3.0,60.0],
                egui::Button::new(but_index.to_string())
            ) ;
            if button.clicked(){
                template_app.selection=999;
                template_app.order_number.push(but_index.to_string());
                check_order(template_app);
            }
        }
       
    });
    ui.horizontal(|ui| {     
        for but_index in 4..7{
            let button = ui.add_sized(
                [wsize/3.0,60.0],
                egui::Button::new(but_index.to_string())
            ) ;

            if button.clicked(){
                template_app.selection=999;
                template_app.order_number.push(but_index.to_string());
                check_order(template_app);
            }
        }
       
    });  
    ui.horizontal(|ui| {     
        for but_index in 7..10{
            let button = ui.add_sized(
                [wsize/3.0,60.0],
                egui::Button::new(but_index.to_string())
            ) ;
            if button.clicked(){
                template_app.selection=999;
                template_app.order_number.push(but_index.to_string());
                check_order(template_app);
            }
        }
       
    });    
    ui.horizontal(|ui| {     
            let button = ui.add_sized(
                [wsize/3.0,60.0],
                egui::Button::new("<".to_string())
            ) ;
            if button.clicked(){
                template_app.order_number.pop();
       
                check_order(template_app);
            }
            let button = ui.add_sized(
                [wsize/3.0,60.0],
                egui::Button::new("0".to_string())
            ) ;
            if button.clicked(){
                template_app.order_number.push("0".to_string());
                template_app.selection=999;
                check_order(template_app);
            }
            let button_c = ui.add_sized(
                [wsize/3.0,60.0],
                egui::Button::new("C".to_string())
            ) ;
            if button_c.clicked(){
                if template_app.selection!=999{
                template_app.total_order.remove(template_app.selection);
                template_app.payment.remove(template_app.selection);
                template_app.payment.push(false);
                template_app.selection=999;
                }else{
                    template_app.order_number.clear();
                }
                save_to_remote(template_app.total_order.clone());
                        }
    });  
});  
}

impl<'a> Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            order_number:Vec::new(),
            total_order:Default::default(),
            order_time:Vec::new(),
            selection: 999,
            rows: 1,
            row_index: 0,
            friedbun_count: 0,
            payment: vec![false;50],
            scroll_to_row: None,
            name:"".to_owned(),
        }
    }
  
}
use reqwest::header::ACCESS_CONTROL_ALLOW_ORIGIN;
impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}
use serde::{Deserialize, Serialize};
#[derive(Serialize)]
struct SaveData<'a> {
    key: &'a str,
    value: &'a str,
}

#[derive(Deserialize)]
struct LoadData {
    value: String,
}
#[derive(Serialize, Deserialize)]
struct Message {
    vector:Vec<(String,String,bool)>,
}
async fn save_to_remote(total_order:Vec<(String, String,bool)>){
    let client = reqwest::Client::new();
    
    let my_data = Message {
        vector: total_order,
    };
    println!("my_data: {:?}",my_data.vector);
    let response = client.post(" https://ts.maya.se:3030/data")
        .json(&my_data)
        .send()
        .await.unwrap();

println!("Response: {:?}", response.text().await.unwrap());

}

async fn load_from_remote() ->  Vec<(String,String,bool)>{
    let my_data = Message {
        vector: Vec::new(),
    };
   
    let client = reqwest::Client::new();
   
    let response =client.get(" https://ts.maya.se:3030/load")
    .json(&my_data)
    .send()
    .await.unwrap();
    let msg=&response.text().await.unwrap();
    println!("Received : {:?}",msg);
        let message: Message = serde_json::from_str(msg).unwrap();
        println!("Received vector: {:?}", message.vector);
      message.vector
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
    
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
     
  
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Save").clicked() {
                let name = self.total_order.clone();

                spawn_local(async move {
                    save_to_remote(name).await;
                });
            }

            if ui.button("Load").clicked() {
                let name_clone = self.name.clone();
                let mut app :std::vec::Vec<(String,String,bool)>=Vec::new();
             async{
                self.total_order = load_from_remote().await;
               
                };
                ()
            }

           
            let body_text_size = TextStyle::Body.resolve(ui.style()).size;
            use egui_extras::{Size, StripBuilder};
            StripBuilder::new(ui)
                .size(Size::remainder()) // for the table
                .size(Size::exact(body_text_size)) // for the source code link
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        egui::ScrollArea::horizontal().show(ui, |ui| {
                            let mut table=order_table::Table::default();
                            table.table_ui(ui,self);
                        });
                    });
                  
                });
         
          
          
            
            
        });
     
  
        
        egui::TopBottomPanel::bottom("bot").show(ctx, |ui| {
         /*    ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui| {
               
            ui.add_space(100.0);
            let time_now: DateTime<Local> = Local::now();
            ui.add(Label::new(egui::RichText::new(time_now.format("%H:%M:%S").to_string()).size(50.0)));
            ui.add_space(100.0);
        }); */
      
            buttons(self, ui)
    
      
    });
  /*    ctx.request_repaint();
     std::thread::sleep(Duration::from_millis(1)); */
    }
}


