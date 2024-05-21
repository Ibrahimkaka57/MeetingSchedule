#[warn(dead_code)]
use std::collections::HashMap;
use std::io;
use chrono::NaiveTime;

#[derive(Debug)]
struct Meeting {
    title: String,
    start_time: NaiveTime,
    end_time: NaiveTime,
}

struct ConferenceRoom {
    schedule: HashMap<NaiveTime, Meeting>,
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
        io::stdin().read_line(&mut title).expect("Failed to read input");

        println!("Please enter the start time of the meeting (HH:MM):");
        let mut start_time_input = String::new();
        io::stdin().read_line(&mut start_time_input).expect("Failed to read input");
        let start_time = match NaiveTime::parse_from_str(start_time_input.trim(), "%H:%M") {
            Ok(time) => time,
            Err(_) => return Err("Invalid start time format, please use HH:MM".to_string()),
        };

        println!("Please enter the end time of the meeting (HH:MM):");
        let mut end_time_input = String::new();
        io::stdin().read_line(&mut end_time_input).expect("Failed to read input");
        let end_time = match NaiveTime::parse_from_str(end_time_input.trim(), "%H:%M") {
            Ok(time) => time,
            Err(_) => return Err("Invalid end time format, please use HH:MM".to_string()),
        };

        for (time_slot, meeting) in &self.schedule {
            if (start_time >= *time_slot && start_time < meeting.end_time) ||
                (end_time > *time_slot && end_time <= meeting.end_time) {
                return Err(format!("The meeting \"{}\" conflicts with an existing one", title.trim()));
            }
        }

        self.schedule.insert(start_time, Meeting { title: title.trim().to_string(), start_time, end_time });
        Ok(())
    }

    fn view_free_time_slots(&self) -> Vec<(NaiveTime, NaiveTime)> {
        let end_of_day = NaiveTime::from_hms(23, 59, 59); // Represents the end of the day

        let mut free_slots = vec![];
        let mut last_end_time = NaiveTime::from_hms(0, 0, 0); // Start of the day

        let mut sorted_schedule: Vec<_> = self.schedule.iter().collect();
        sorted_schedule.sort_by_key(|(time_slot, _)| **time_slot);
        for (time_slot, meeting) in sorted_schedule {
            if *time_slot > last_end_time {
                free_slots.push((last_end_time, *time_slot));
            }

            last_end_time = meeting.end_time;
        }

        if last_end_time < end_of_day {
            free_slots.push((last_end_time, end_of_day));
        }

        free_slots
    }

    fn view_all_time_slots(&self) -> Vec<(NaiveTime, NaiveTime, String)> {
        let mut all_slots = vec![];
        for (time_slot, meeting) in &self.schedule {
            all_slots.push((*time_slot, meeting.end_time, meeting.title.clone()));
        }
        all_slots
    }
}

fn main() {
    let mut conference_room = ConferenceRoom::new();
    println!("Enter meeting details (Meeting, Start_time, End_time):");

    match conference_room.schedule_meeting() {
        Ok(_) => println!("Meeting scheduled successfully!"),
        Err(err) => println!("Error: {}", err),
    }

    println!("Free Time Slots:");
    for (start, end) in conference_room.view_free_time_slots() {
        println!("{} - {}", start.format("%H:%M"), end.format("%H:%M"));
    }

    println!("All Time Slots:");
    for (start, end, title) in conference_room.view_all_time_slots() {
        println!("{} - {}: {}", start.format("%H:%M"), end.format("%H:%M"), title);
    }
}