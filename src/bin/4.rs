// --- Day 4: Repose Record ---
// You've sneaked into another supply closet - this time, it's across from the
// prototype suit manufacturing lab. You need to sneak inside and fix the issues
// with the suit, but there's a guard stationed outside the lab, so this is as
// close as you can safely get.
//
// As you search the closet for anything that might help, you discover that
// you're not the first person to want to sneak in. Covering the walls, someone
// has spent an hour starting every midnight for the past few months secretly
// observing this guard post! They've been writing down the ID of the one guard
// on duty that night - the Elves seem to have decided that one guard was enough
// for the overnight shift - as well as when they fall asleep or wake up while
// at their post (your puzzle input).
//
// For example, consider the following records, which have already been
// organized into chronological order:
//
// [1518-11-01 00:00] Guard #10 begins shift
// [1518-11-01 00:05] falls asleep
// [1518-11-01 00:25] wakes up
// [1518-11-01 00:30] falls asleep
// [1518-11-01 00:55] wakes up
// [1518-11-01 23:58] Guard #99 begins shift
// [1518-11-02 00:40] falls asleep
// [1518-11-02 00:50] wakes up
// [1518-11-03 00:05] Guard #10 begins shift
// [1518-11-03 00:24] falls asleep
// [1518-11-03 00:29] wakes up
// [1518-11-04 00:02] Guard #99 begins shift
// [1518-11-04 00:36] falls asleep
// [1518-11-04 00:46] wakes up
// [1518-11-05 00:03] Guard #99 begins shift
// [1518-11-05 00:45] falls asleep
// [1518-11-05 00:55] wakes up
//
// Timestamps are written using year-month-day hour:minute format. The guard
// falling asleep or waking up is always the one whose shift most recently
// started. Because all asleep/awake times are during the midnight hour (00:00 -
// 00:59), only the minute portion (00 - 59) is relevant for those events.
//
// Visually, these records show that the guards are asleep at these times:
//
// Date   ID   Minute
//             000000000011111111112222222222333333333344444444445555555555
//             012345678901234567890123456789012345678901234567890123456789
// 11-01  #10  .....####################.....#########################.....
// 11-02  #99  ........................................##########..........
// 11-03  #10  ........................#####...............................
// 11-04  #99  ....................................##########..............
// 11-05  #99  .............................................##########.....
//
// The columns are Date, which shows the month-day portion of the relevant day;
// ID, which shows the guard on duty that day; and Minute, which shows the
// minutes during which the guard was asleep within the midnight hour. (The
// Minute column's header shows the minute's ten's digit in the first row and
// the one's digit in the second row.) Awake is shown as ., and asleep is shown
// as #.
//
// Note that guards count as asleep on the minute they fall asleep, and they
// count as awake on the minute they wake up. For example, because Guard #10
// wakes up at 00:25 on 1518-11-01, minute 25 is marked as awake.
//
// If you can figure out the guard most likely to be asleep at a specific time,
// you might be able to trick that guard into working tonight so you can have
// the best chance of sneaking in. You have two strategies for choosing the best
// guard/minute combination.
//
// Strategy 1: Find the guard that has the most minutes asleep. What minute does
// that guard spend asleep the most?
//
// In the example above, Guard #10 spent the most minutes asleep, a total of 50
// minutes (20+25+5), while Guard #99 only slept for a total of 30 minutes
// (10+10+10). Guard #10 was asleep most during minute 24 (on two days, whereas
// any other minute the guard was asleep was only seen on one day).
//
// While this example listed the entries in chronological order, your entries
// are in the order you found them. You'll need to organize them before they can
// be analyzed.
//
// What is the ID of the guard you chose multiplied by the minute you chose? (In
// the above example, the answer would be 10 * 24 = 240.)

#[macro_use]
extern crate lazy_static;
extern crate chrono;
extern crate regex;
extern crate utils;

use chrono::prelude::*;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct ShiftTimeline {
    data: Vec<u32>,
}

impl ShiftTimeline {
    fn new() -> ShiftTimeline {
        ShiftTimeline { data: Vec::new() }
    }

    fn sleep_minutes(&self) -> u32 {
        self.data.len() as u32
    }

    fn record_sleep_time(&mut self, asleep_at: Option<NaiveDateTime>, awake_at: NaiveDateTime) {
        match asleep_at {
            Some(asleep_at_time) => {
                let start_min = asleep_at_time.minute();
                let end_min = awake_at.minute();
                for idx in start_min..end_min {
                    self.data.push(idx);
                }
            }
            None => panic!("ShiftTimeline.fill_sleep got None as asleep_at..."),
        }
    }
}

#[derive(Debug)]
enum ActionType {
    Shift,
    Asleep,
    Awake,
}

#[derive(Debug)]
struct Action {
    kind: ActionType,
    time: NaiveDateTime,
    guard_id: Option<u32>,
}

fn parse_action(line: &str) -> Option<Action> {
    lazy_static! {
        static ref ACTION_REGEX: Regex =
            Regex::new(r"\[(.*)\] (Guard|wakes|falls) (#(\d+))?").unwrap();
    }

    let captures = ACTION_REGEX.captures(line)?;
    // println!("{:?}", captures);

    let time_str = &captures[1];
    // println!("{:?}", time_str);

    let time = NaiveDateTime::parse_from_str(time_str, "%Y-%m-%d %H:%M").ok()?;

    let kind = match &captures[2] {
        "Guard" => ActionType::Shift,
        "falls" => ActionType::Asleep,
        "wakes" => ActionType::Awake,
        _ => return None,
    };

    // println!("{:?}", captures.get(4));
    let guard_id = captures.get(4).and_then(|s| s.as_str().parse::<u32>().ok());

    Some(Action {
        kind,
        time,
        guard_id,
    })
}

fn get_records(actions: &Vec<Action>) -> HashMap<Option<u32>, Vec<ShiftTimeline>> {
    // Not every action has guard_id initially and since they are sorted by time
    // now and first action in a row for given guard is usually "Shift" and contains
    // guard id. So we can fill guard_id from initial action...
    let mut shift_timeline = ShiftTimeline::new();
    let mut guard_id: Option<u32> = None;
    let mut asleep_at: Option<NaiveDateTime> = None;
    let mut records: HashMap<Option<u32>, Vec<ShiftTimeline>> = HashMap::new();

    for action in actions {
        match action.kind {
            ActionType::Shift => {
                // first lets save previous ShiftTimeline
                records
                    .entry(guard_id)
                    .or_insert(Vec::new())
                    .push(shift_timeline);
                guard_id = action.guard_id;
                shift_timeline = ShiftTimeline::new();
            }
            ActionType::Asleep => asleep_at = Some(action.time),
            ActionType::Awake => {
                let awake_at = action.time;
                shift_timeline.record_sleep_time(asleep_at, awake_at);
            }
        }
    }

    records
}

fn get_sleep_minutes_per_guard_id(
    records: &HashMap<Option<u32>, Vec<ShiftTimeline>>,
) -> HashMap<u32, u32> {
    let mut counters: HashMap<u32, u32> = HashMap::new();

    for key in records.keys().filter(|x| x.is_some()) {
        for shift_timelines in records.get(key) {
            let sleep_minutes = shift_timelines
                .iter()
                .fold(0, |acc, t| acc + t.sleep_minutes());
            counters.insert(key.unwrap(), sleep_minutes);
        }
    }

    counters
}

fn get_kv_for_max_value(hash_map: &HashMap<u32, u32>) -> (u32, u32) {
    let (mut max_value, mut max_key) = (0, 0);

    for (key, val) in hash_map {
        if *val > max_value {
            max_value = *val;
            max_key = *key;
        }
    }

    (max_key, max_value)
}

fn part_one() {
    let input = utils::read_puzzle_input(4);
    let mut actions: Vec<Action> = input.lines().filter_map(parse_action).collect();

    actions.sort_by_key(|a| a.time);

    let records = get_records(&actions);
    let sleep_counts = get_sleep_minutes_per_guard_id(&records);
    let (most_sleeping_guard, _) = get_kv_for_max_value(&sleep_counts);

    let records_of_most_sleeping_guard = records.get(&Some(most_sleeping_guard)).unwrap();
    // let popular_minute = count_most_popular();

    let mut minute_counts: HashMap<u32, u32> = HashMap::new();
    records_of_most_sleeping_guard
        .iter()
        .cloned()
        .map(|h| h.data)
        .flatten()
        .for_each(|el| {
            minute_counts.entry(el).and_modify(|c| *c += 1).or_insert(1);
        });

    let (sleep_mostly_on_minute, _) = get_kv_for_max_value(&minute_counts);

    println!("--- Part 1 ---");
    println!("Most sleeping guard: {:?}", most_sleeping_guard);
    println!("Mostly sleep on minute: {:?}", sleep_mostly_on_minute);
    println!("Result: {}", sleep_mostly_on_minute * most_sleeping_guard);
}

// --- Part Two ---
// Strategy 2: Of all guards, which guard is most frequently asleep on the same minute?
//
// In the example above, Guard #99 spent minute 45 asleep more than any other
// guard or minute - three times in total. (In all other cases, any guard spent
// any minute asleep at most twice.)
//
// What is the ID of the guard you chose multiplied by the minute you chose? (In
// the above example, the answer would be 99 * 45 = 4455.)

fn part_two() {
    let input = utils::read_puzzle_input(4);
    let mut actions: Vec<Action> = input.lines().filter_map(parse_action).collect();
    actions.sort_by_key(|a| a.time);

    let records = get_records(&actions);

    // Option<GuardId> => HashMap<minute, count>
    let mut minute_counters_by_guard: HashMap<Option<u32>, HashMap<u32, u32>> = HashMap::new();

    // Calculate times each guard spent sleeping at each minute
    for (guard_id, shift_timelines) in records {
        let counters = minute_counters_by_guard.entry(guard_id).or_default();
        for timeline in shift_timelines {
            for minute in timeline.data {
                counters.entry(minute).and_modify(|c| *c += 1).or_insert(1);
            }
        }
    }

    // Find minute most popular for sleep in scope of single guard id
    let mut sleephead_id: Option<u32> = None;
    let mut sleepy_minute = 0;
    let mut sleepy_minute_used_times = 0;

    for (guard_id, counters) in minute_counters_by_guard {
        let (current_candidate_to_sleepy_minute, times) = get_kv_for_max_value(&counters);

        if times > sleepy_minute_used_times {
            sleepy_minute = current_candidate_to_sleepy_minute;
            sleephead_id = guard_id;
            sleepy_minute_used_times = times;
        }
    }

    let sleephead = sleephead_id.unwrap();

    println!("--- Part 2 ---");
    println!("Sleephead ID: {}", sleephead);
    println!("Sleepy Minute: {}", sleepy_minute);
    println!("Sleepy Minute Used: {}", sleepy_minute_used_times);
    println!("Solution: {}", sleephead * sleepy_minute);
}

fn main() {
    part_one();
    part_two();
}
