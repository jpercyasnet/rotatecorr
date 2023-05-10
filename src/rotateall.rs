//----------------- rotate all button start -----------------------------------
    rotateall_button.connect_clicked(glib::clone!(@weak directory_combobox, @weak messageval_label, @weak tree_view, @weak progress_progressbar => move|_| {

        progress_progressbar.set_fraction(0.0);
        while glib::MainContext::pending(&glib::MainContext::default()) {
               glib::MainContext::iteration(&glib::MainContext::default(),true);
        }

        let treemodel = tree_view.model();
        if treemodel == None {
             messageval_label.set_markup("<span color=\"#FF000000\">********* Rotate All: ERROR NOTHING IN LIST **********</span>");
        } else {
            if let Some(cur_dir) = directory_combobox.active_text() {
                let mut numrot = 0;
                let treemodeluw = treemodel.unwrap();
                let mut valid = true;
                let validval = treemodeluw.iter_first().unwrap();
                let mut numrow = 0;
                let numchildren = treemodeluw.iter_n_children(None);
                let mut msgvar = format!(" ");
                let mut numprocess = 0;
                while valid {
                      let treeval = treemodeluw.get_value(&validval,1).get::<String>();
                      let filenameval = treemodeluw.get_value(&validval,0).get::<String>();
                      valid = treemodeluw.iter_next(&validval);
                      let strval = format!("{:?}", treeval);
                      let locind = strval.find("Orientation");
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
                                  numrot = numrot + 1;
//                                  let filenamestr = format!("{:?}", filenameval);
//                                  let fileln = filenamestr.len();
//                                  let fileend = fileln - 3;
//                                  let filestart = 9;
                                  let filenamex = filenameval.unwrap().to_string();
                                  let s1_param = format!("{}/{}", cur_dir, filenamex);
//         				          println!("s1_param: {}", s1_param);

                                  if valid & (numprocess < 4) {
                                      Command::new("/home/jp/gimp.sh")
                                             .arg(&s1_param)
                                             .spawn()
                                             .expect("failed to execute process");
                                      numprocess = numprocess + 1;
                                  } else {
                                      let _output = Command::new("/home/jp/gimp.sh")
                                                               .arg(&s1_param)
                                                               .output()
                                                               .expect("failed to execute process");
                                      numprocess = 0;
                                  }
                              }
                          } else if orient_int == -99 {
                              msgvar = format!(" {} File {:?} orientation value of {:?} is not an integer", msgvar, filenameval, getorient);
                          } else {
                              msgvar = format!(" {} File {:?} orientation value of {} is not positive", msgvar, filenameval, orient_int);
                          }
                      }
                      numrow = numrow + 1;
                      let progressfr: f64 = numrow as f64 / numchildren as f64;
                      progress_progressbar.set_fraction(progressfr);
                      while glib::MainContext::pending(&glib::MainContext::default()) {
                             glib::MainContext::iteration(&glib::MainContext::default(),true);
                      }
                }
                let msgstr = format!("Number of files rotated: {} {}", numrot, msgvar);
                messageval_label.set_text(&msgstr);
            } else {
                messageval_label.set_markup("<span color=\"#FF000000\">********* List: ERROR GETTING DIRECTORY IN COMBOBOX **********</span>");
            }
        }
    }));
//----------------- rotate all button end -----------------------------------

