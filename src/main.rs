use iced::widget::{button, column, row, text, scrollable, progress_bar, horizontal_space};
use iced::theme::{self, Theme};
use iced::{Alignment, Element, Command, Application, Length, Settings, Color};
use iced::executor;
use iced::window;
use iced_futures::futures;
use futures::channel::mpsc;

use std::process::Command as stdCommand;
use std::path::{Path};
use std::time::{Duration, Instant};
use std::thread::sleep;

use std::env;

mod get_dirlistr;
mod dirpressr;
mod listpressr;
mod dump_file;
mod rotatepressx;
mod get_winsize;

use get_dirlistr::get_dirlistr;
use dirpressr::dirpressr;
use listpressr::listpressr;
use rotatepressx::rotatepressx;
use get_winsize::get_winsize;

pub fn main() -> iced::Result {
     env::set_var("RUST_BACKTRACE", "1");

     let mut widthxx: u32 = 1350;
     let mut heightxx: u32 = 750;
     let (errcode, errstring, widtho, heighto) = get_winsize();
     if errcode == 0 {
         widthxx = widtho;
         heightxx = heighto;
         println!("{}", errstring);
     } else {
         println!("**ERROR {} get_winsize: {}", errcode, errstring);
     }

     RotateCor::run(Settings {
        window: window::Settings {
            size: (widthxx, heightxx),
            ..window::Settings::default()
        },
        ..Settings::default()
     })


//    RotateCor::run(Settings::default())
}

struct RotateCor {
    dir_value: String,
    mess_color: Color,
    msg_value: String,
    do_progress: bool,
    scrol_value: String,
//    outdir_value: String,
    progval: f32,
    tx_send: mpsc::UnboundedSender<String>,
    rx_receive: mpsc::UnboundedReceiver<String>,
}

#[derive(Debug, Clone)]
enum Message {
    DirPressed,
//    OutDirPressed,
    ListPressed,
    RotatePressed,
    ProgressPressed,
    ProgRtn(Result<Progstart, Error>),
    RotatexFound(Result<Rotatex, Error>),
}

impl Application for RotateCor {
    type Message = Message;
    type Theme = Theme;
    type Flags = ();
    type Executor = executor::Default;
    fn new(_flags: ()) -> (Self, iced::Command<Message>) {
        let (tx_send, rx_receive) = mpsc::unbounded();
        (
            RotateCor {
                dir_value: "no directory".to_string(),
                mess_color: Color::from([0.0, 0.0, 0.0]),
                msg_value: "no message".to_string(),
                do_progress: false,
                scrol_value: " No directory selected \n ".to_string(),
//                outdir_value: "no directory".to_string(),
                progval: 0.0,
                tx_send,
                rx_receive,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Correct Rotation-- iced")
    }

    fn update(&mut self, message: Message) -> Command<Message>  {
        match message {
            Message::DirPressed => {
               let (colorout, errstr, newdir, newliststr) = dirpressr();
               self.scrol_value  = newliststr.to_string();
               self.dir_value = newdir.to_string();
               self.msg_value = errstr.to_string();
               self.mess_color = colorout;
               Command::none()
            }
/*            Message::OutDirPressed => {
               let (colorout, errstr, newdir) = diroutpressx();
               self.outdir_value = newdir.to_string();
               self.msg_value = errstr.to_string();
               self.mess_color = colorout;
               Command::none()
            } */
            Message::ListPressed => {
               let (colorout, errstr, newliststr) = listpressr(self.dir_value.clone());
               self.scrol_value  = newliststr.to_string();
               self.msg_value = errstr.to_string();
               self.mess_color = colorout;
               Command::none()
            }
            Message::RotatePressed => {
               let (errcode, colorout, errstr) = rotatepressx(self.dir_value.clone(), self.scrol_value.clone());
               self.msg_value = errstr.to_string();
               self.mess_color = colorout;
               if errcode == 0 {
                   Command::perform(Rotatex::rotateit(self.dir_value.clone(), self.scrol_value.clone(), self.tx_send.clone()), Message::RotatexFound)

               } else {
                   Command::none()
               }
            }
            Message::ProgressPressed => {
                   self.do_progress = true;
                   Command::perform(Progstart::pstart(), Message::ProgRtn)
            }
            Message::RotatexFound(Ok(copyx)) => {
                self.msg_value = copyx.errval.clone();
                self.mess_color = copyx.errcolor.clone();
                self.do_progress = false;
                self.progval = 0.0;
                Command::none()
            }
            Message::RotatexFound(Err(_error)) => {
                self.msg_value = "error in copyx copyit routine".to_string();
                self.mess_color = Color::from([1.0, 0.0, 0.0]);
                Command::none()
            }
            Message::ProgRtn(Ok(_prx)) => {
              if self.do_progress {
                let mut inputval  = " ".to_string();
                let mut bgotmesg = false;
                while let Ok(Some(input)) = self.rx_receive.try_next() {
                   inputval = input;
                   bgotmesg = true;
                }
                if bgotmesg {
                    let progvec: Vec<&str> = inputval[0..].split("|").collect();
                    let lenpg1 = progvec.len();
                    if lenpg1 == 3 {
                        let prog1 = progvec[0].clone().to_string();
                        if prog1 == "Progress" {
                            let num_int: i32 = progvec[1].clone().parse().unwrap_or(-9999);
                            if num_int == -9999 {
                                println!("progress numeric not numeric: {}", inputval);
                            } else {
                                let dem_int: i32 = progvec[2].clone().parse().unwrap_or(-9999);
                                if dem_int == -9999 {
                                    println!("progress numeric not numeric: {}", inputval);
                                } else {
                                    self.progval = 100.0 * (num_int as f32 / dem_int as f32);
                                    self.msg_value = format!("Convert progress: {}", self.progval);
                                    self.mess_color = Color::from([0.0, 0.0, 1.0]);
                                }
                            }
                        } else {
                            println!("message not progress: {}", inputval);
                        }
                    } else {
                        println!("message not progress: {}", inputval);
                    }
                }             
                Command::perform(Progstart::pstart(), Message::ProgRtn)
              } else {
                Command::none()
              }
            }
            Message::ProgRtn(Err(_error)) => {
                self.msg_value = "error in Progstart::pstart routine".to_string();
                self.mess_color = Color::from([1.0, 0.0, 0.0]);
                Command::none()
            }

        }
    }

    fn view(&self) -> Element<Message> {
        column![
            row![text("Message:").size(30),
                 text(&self.msg_value).size(30).style(*&self.mess_color),
            ].align_items(Alignment::Center).spacing(10).padding(10),
            row![button("Directory Button").on_press(Message::DirPressed),
                 text(&self.dir_value).size(30),
            ].align_items(Alignment::Center).spacing(10).padding(10),
            row![horizontal_space(200), 
                 button("List Orientation Button").on_press(Message::ListPressed),
                 button("Rotate All Button").on_press(Message::RotatePressed),
//                 button("Start Progress Button").on_press(Message::ProgressPressed),
            ].align_items(Alignment::Center).spacing(200).padding(10),
            scrollable(
                column![
                        text(format!("{}",&self.scrol_value))
                ].width(Length::Fill),
            ).height(Length::Fixed(450.0)),

            row![button("Start Progress Button").on_press(Message::ProgressPressed),
                 progress_bar(0.0..=100.0,self.progval),
                 text(format!("{}%", &self.progval)).size(30),

            ].align_items(Alignment::Center).spacing(5).padding(10),


//            progress_bar(0.0..=100.0,self.progval),
         ]
        .padding(10)
        .align_items(Alignment::Start)
        .into()
    }

    fn theme(&self) -> Theme {
//        Theme::Light
          Theme::custom(theme::Palette {
                        background: Color::from_rgb(0.1, 0.5, 0.1),
                        text: Color::BLACK,
                        primary: Color::from_rgb(0.1, 0.7, 0.0),
                        success: Color::from_rgb(0.0, 1.0, 0.0),
                        danger: Color::from_rgb(1.0, 0.0, 0.0),
                    })
               
    }
}
#[derive(Debug, Clone)]
struct Rotatex {
    errcolor: Color,
    errval: String,
}

impl Rotatex {

    async fn rotateit(dir_value: String, mergescrol_value: String, tx_send: mpsc::UnboundedSender<String>,) -> Result<Rotatex, Error> {
     let mut errstring  = " ".to_string();
     let mut colorx = Color::from([0.0, 0.0, 0.0]);
     let mut bolok = true;
     let mut numrow = 0;
     let mut numprocess = 0;
     let mergelistvec: Vec<&str> = mergescrol_value[0..].split("\n").collect();
     let mut lenmg1 = mergelistvec.len();
     lenmg1 = lenmg1 -1;
     let start_time = Instant::now();
     for indl in 0..lenmg1 {
          let str_cur_dirfrom = dir_value.clone();
          let linestr = mergelistvec[indl].clone();
          let lineparse: Vec<&str> = linestr[0..].split(" | ").collect();
          let filefromx = lineparse[0].clone().to_string();
          let fullfrom = str_cur_dirfrom.clone() + "/" + &filefromx[1..];
          if !Path::new(&fullfrom).exists() {
              errstring = format!("********* convert Copy: ERROR {} does not exist **********",fullfrom);
              colorx = Color::from([1.0, 0.0, 0.0]);
              bolok = false;
              break;
          }
          let strval = lineparse[1].clone().to_string();
          let locind = strval.find("orientation");
          if locind != None {
              let start = locind.unwrap();
              let start = start + 13;
              let end = start + 1;
              let getorient = strval.get(start..end);
              let orient_int: i32 = getorient.unwrap().parse().unwrap_or(-99);
              if orient_int > 0 {
                  if (orient_int == 3) | 
                     (orient_int == 6) |
                     (orient_int == 8) {
                      numrow = numrow + 1;
                      if numprocess < 4 {
                          stdCommand::new("/home/jp/gimp.sh")
                             .arg(&fullfrom)
                             .spawn()
                             .expect("failed to execute process");
                          numprocess = numprocess + 1;
                      } else {
                          let _output = stdCommand::new("/home/jp/gimp.sh")
                               .arg(&fullfrom)
                               .output()
                               .expect("failed to execute process");
                          numprocess = 0;
                          let msgx = format!("Progress|{}|{}", numrow, lenmg1);
                          tx_send.unbounded_send(msgx).unwrap();
                      }
                  }
              }
          }
     }
     if bolok {
         let diffx = start_time.elapsed();     
         errstring = format!("rotated {} files in {} seconds", lenmg1, diffx.as_secs());
         colorx = Color::from([0.0, 0.0, 0.0]);
     }
     Ok(Rotatex {
            errcolor: colorx,
            errval: errstring,
        })
    }
}
#[derive(Debug, Clone)]
enum Error {
//    APIError,
}
// loop thru by sleeping for 5 seconds
#[derive(Debug, Clone)]
struct Progstart {
//    errcolor: Color,
//    errval: String,
}

impl Progstart {

    async fn pstart() -> Result<Progstart, Error> {
//     let errstring  = " ".to_string();
//     let colorx = Color::from([0.0, 0.0, 0.0]);
     sleep(Duration::from_secs(5));
     Ok(Progstart {
//            errcolor: colorx,
//            errval: errstring,
        })
    }
}
