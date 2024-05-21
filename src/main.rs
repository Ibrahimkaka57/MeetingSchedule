use std::fs::{self, OpenOptions};
use std::io::prelude::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::io;
use serde_json;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Meeting { 
    title: String,
    start_time: u32,
    end_time: u32,
}

#[derive(Deserialize, Serialize)]
struct ConferenceRoom {
    schedule: HashMap<u32, Meeting>,
   
}

impl ConferenceRoom {
    fn new() -> Self {
        ConferenceRoom {
            schedule: HashMap::new(),
           
        }
    }
    
    fn schedule_meeting(&mut self) -> Result<(), String> {
        println!("Please enter the title of the meeting:");
        let mut title = String::new();
        io::stdin()
            .read_line(&mut title)
            .expect("Failed to read input");

        println!("Please enter the start time of the meeting (HHMM):");
        let mut start_time_input = String::new();
        io::stdin()
            .read_line(&mut start_time_input)
            .expect("Failed to read input");
        let start_time: u32 = start_time_input
            .trim()
            .parse()
            .expect("Please enter a valid number");

        println!("Please enter the end time of the meeting (HHMM):");
        let mut end_time_input = String::new();
        io::stdin()
            .read_line(&mut end_time_input)
            .expect("Failed to read input");
        let end_time: u32 = end_time_input
            .trim()
            .parse()
            .expect("Please enter a valid number");

       

        for (time_slot, meeting) in &self.schedule {
            if (start_time >= *time_slot && start_time < meeting.end_time) ||
                (end_time > *time_slot && end_time <= meeting.end_time) {
                return Err(format!("The meeting \"{}\" conflicts with an existing one", title.trim()));
            }
        }

        self.schedule.insert(start_time, Meeting { title: title.trim().to_string(), start_time, end_time });
        Ok(())

    }



    fn view_free_time_slots(&self) -> Vec<(u32, u32)> {
        const END_OF_DAY: u32 = 2400;
    
        let mut free_slots = vec![];
        let mut last_end_time = 0;
    
        let mut sorted_schedule: Vec<_> = self.schedule.iter().collect();
        sorted_schedule.sort_by_key(|(time_slot, _)| *time_slot);
        for (time_slot, meeting) in sorted_schedule {
            if *time_slot > last_end_time {
                free_slots.push((last_end_time, *time_slot));
            }
    
            last_end_time = meeting.end_time;
        }
    
        if last_end_time < END_OF_DAY {
            free_slots.push((last_end_time, END_OF_DAY));
        }
    
        free_slots
    }
    

    fn view_all_time_slots(&self) -> Vec<(u32, u32, String)> {
        let mut all_slots = vec![];
        for (_, meeting) in &self.schedule {
            all_slots.push((meeting.start_time, meeting.end_time, meeting.title.clone()));
        }
        all_slots
    }

    fn load_schedule_from_file(filename: &str) -> Self {
        match fs::read_to_string(filename) {
            Ok(content) => {
                match serde_json::from_str(&content) {
                    Ok(schedule) => schedule,
                    Err(_) => {
                        println!("Error: Invalid JSON format in the file.");
                        ConferenceRoom::new()
                    }
                }
            }
            Err(_) => {
                println!("Error: File not found or could not be read.");
                ConferenceRoom::new()
            }
        }
    }
}


fn main() {
    let mut conference_room = ConferenceRoom::load_schedule_from_file("data.json");

    println!("Free Time Slots:");
    for (start, end) in conference_room.view_free_time_slots() {
        println!("{} - {}", start, end);
    }

    println!("All Time Slots:");
    for (start, end, title) in conference_room.view_all_time_slots() {
        println!("{} - {}: {}", start, end, title);
    }

    println!("Enter meeting details (Meeting, Start_time, End_time):");

    match conference_room.schedule_meeting() {
        Ok(_) => println!("Meeting scheduled successfully!"),
        Err(err) => println!("Error: {}", err),
    }

    let conference_room_ser = serde_json::to_string(&conference_room).expect("Failed to serialize to JSON");
  

    let file_path = "data.json";

    if fs::metadata(&file_path).is_ok() {
        println!("File already exists. Do you want to overwrite it? (y/n)");
        
        let mut response = String::new();
        io::stdin().read_line(&mut response).expect("Failed to read input");

        if response.trim().to_lowercase() != "y" {
            println!("Exiting without overwriting the file.");
            return;
        }
    }    
    let mut file = OpenOptions::new()
    .create(true)
    .write(true)
    .append(true)
    .open(file_path)
    .expect("Failed to open file");

file.write_all(conference_room_ser.as_bytes()).expect("Failed to write to file");
file.flush().expect("Failed to flush file");

    println!("Data has been written to {}", file_path);

}  