use std::default;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{prelude::*, Error, Result};
use toml::{from_str, to_string_pretty};
use std::fs;
const HOME: &'static str = env!("HOME");

use iced::{Align, Column, Container, Element, Length, PickList, Sandbox, Scrollable, Space, Text, pick_list, scrollable};

#[derive(Default, Debug)]
pub struct FontStyle {
    // scroll: scrollable::State,
    pick_list: pick_list::State<Font>,
    selected_font: Font,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    FontSelected(Font),
}

impl Sandbox for FontStyle {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Pick list - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::FontSelected(font) => {
                self.selected_font = font;
                // println!("{:#?}" ,language ); //to print out the language you choose
                create_dir();
                let font_conf = to_string_pretty(&font).unwrap();
                writer("font.conf",&font ).unwrap();

            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let pick_list = PickList::new(
            &mut self.pick_list,
            &Font::ALL[..],
            Some(self.selected_font),
            Message::FontSelected,
        );
        let mut content= Column::new()
            .spacing(10)
            .align_items(Align::Center)
            .push(Text::new("Which is your favorite Fonts?"))
            .push(pick_list);

    //add scroll to content
        // let mut content = Scrollable::new(&mut self.scroll)
        //     .width(Length::Fill)
        //     .align_items(Align::Center)
        //     .spacing(10)
        //     .push(Space::with_height(Length::Units(600)))
        //     .push(Text::new("Which is your favorite Font?"))
        //     .push(pick_list);

        content = content.push(Space::with_height(Length::Units(600)));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Font {
    Monospace,
    Arial,
    Serif,
  
}

impl Font {
    const ALL: [Font; 3] = [
        Font::Arial,
        Font::Serif,
        Font::Monospace,
      
    ];
}

impl Default for Font {
    fn default() -> Font {
        Font::Monospace
    }
}

impl std::fmt::Display for Font {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Font::Monospace => "Monospace",
                Font::Arial => "Arial",
                Font::Serif => "Serif",
             
            }
        )
    }
}

pub fn reader(name: &str) -> Result<String> {
    let path = std::path::Path::new(format!("{}/.config/koompi/font", HOME).as_str()).join(name);

    std::fs::read_to_string(path)
    
}

pub fn writer(name: &str, data: &Font) -> Result<()> {
    
    let path = std::path::Path::new(format!("{}/.config/koompi/font", HOME).as_str()).join(name);
   
    let mut file = File::create(path).unwrap();
    match file.write_all(to_string_pretty(data).unwrap().as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
        }
    
}
pub fn create_dir() -> std::io::Result<()>{
    fs::create_dir_all(format!("{}/.config/koompi/font",HOME))?;
    Ok(())
}


