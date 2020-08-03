use chrono::{DateTime, TimeZone, Utc};

struct ScheduleEvent {
    title: String,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
}
#[allow(dead_code)]
impl ScheduleEvent {
    fn new(title: String, start: DateTime<Utc>, end: DateTime<Utc>) -> ScheduleEvent {
        ScheduleEvent {
            title: title,
            start_time: start,
            end_time: end,
        }
    }
}

fn find_schedule_clash(events: &[ScheduleEvent]) {
    for (initial_index, initial_event) in events.iter().enumerate() {
        for (comparison_index, comparison_event) in events.iter().enumerate() {
            if initial_index != comparison_index
                && (comparison_event.start_time >= initial_event.start_time
                    && comparison_event.start_time < initial_event.end_time)
            {
                println!(
                    "Meeting Clash Found between {initialEventTitle} and {comparisonEventTitle}.",
                    initialEventTitle = initial_event.title,
                    comparisonEventTitle = comparison_event.title
                );
            }
        }
    }
}

fn main() {
    let events: [ScheduleEvent; 3] = [
        ScheduleEvent::new(
            String::from("Meeting 1"),
            Utc.ymd(2020, 2, 2).and_hms(9, 30, 0),
            Utc.ymd(2020, 2, 2).and_hms(10, 0, 0),
        ),
        ScheduleEvent::new(
            String::from("Meeting 2"),
            Utc.ymd(2020, 2, 2).and_hms(10, 30, 0),
            Utc.ymd(2020, 2, 2).and_hms(11, 30, 0),
        ),
        ScheduleEvent::new(
            String::from("Meeting 3"),
            Utc.ymd(2020, 2, 2).and_hms(9, 45, 0), // Should clash with Meeting 1
            Utc.ymd(2020, 2, 2).and_hms(12, 00, 0), // Should clash with Meeting 2
        ),
    ];

    find_schedule_clash(&events);
}
