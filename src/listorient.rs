    listorient_button.connect_clicked(glib::clone!(@weak directory_combobox, @weak messageval_label, @weak tree_view => move|_| {

        if let Some(cur_dir) = directory_combobox.active_text() {
            let current_dir = PathBuf::from(&cur_dir);
            let new_model = ListStore::new(&[String::static_type(), String::static_type()]);
            let mut filesize;
            let mut numentry = 0;
            for entry1 in fs::read_dir(&current_dir).unwrap() {
                 let entry = entry1.unwrap();
                 if let Ok(metadata) = entry.metadata() {
                     if let Ok(file_name) = entry.file_name().into_string() {
                         if metadata.is_file() {
                             let file_path = entry.path();
                             if let Err(_e) = dump_file(&file_path) {
                             } else {
                                 let file = File::open(file_path).unwrap();
                                 let reader = exif::Reader::new().read_from_container(&mut BufReader::new(&file)).unwrap();
                                 if let Some(field) = reader.get_field(exif::Tag::Orientation, exif::In::PRIMARY) {
                                     if let Some(width) = field.value.get_uint(0) {
                                         filesize = format!("Orientation: {}", width);
                                         if (filesize == "Orientation: 3") |
                                            (filesize == "Orientation: 6") |
                                            (filesize == "Orientation: 8") {
                                             new_model.insert_with_values(None,
                                                   &[(VALUE_COL as u32,&file_name), (IS_DIR_COL as u32, &filesize)]);
                                             numentry = numentry + 1;
                                         }
                                     }
                                 }
                            }
                         }
                     }
                 }
            }
            tree_view.set_model(Some(&new_model));
            if numentry > 0 {
                let msgstr = format!("{} files need rotation correction", numentry);
                messageval_label.set_text(&msgstr);
            } else {
                messageval_label.set_markup("<span color=\"#FF000000\">********* List Orientation: directory has no images to rotate **********</span>");
            }
        } else {
            messageval_label.set_markup("<span color=\"#FF000000\">********* List Orientation: ERROR GETTING DIRECTORY IN COMBOBOX **********</span>");
        }
    }));

//----------------- list orientation button end -----------------------------------

