use std::thread;
use std::time::{Instant, Duration};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

pub struct Task {
    start_time: Instant,
    pause_start_time: Option<Instant>,
    end_time: Option<Instant>,
    is_paused: bool,
    pause_duration: Duration,
}

impl Task {
    fn total_duration(&self) -> Duration {
        let mut total_duration = self.end_time.unwrap_or_else(|| Instant::now()) - self.start_time;
        if self.is_paused {
            total_duration -= self.pause_duration;
        }
        total_duration
    }
}


pub fn start(tasks: &Arc<Mutex<HashMap<String, Task>>>) {
    println!("Enter task name:");
    let mut task_name = String::new();
    std::io::stdin().read_line(&mut task_name).expect("Failed to read task name");
    let task_name = task_name.trim().to_string();
    let start_time = Instant::now();

    let tasks_clone = Arc::clone(&tasks);
    let task_name_clone = task_name.clone();
    let handle = thread::spawn(move || {
        tasks_clone.lock().unwrap().insert(task_name_clone, Task { start_time, end_time: None, pause_start_time: None, is_paused: false, pause_duration: Duration::from_secs(0) });
    });
    handle.join().unwrap();

    println!("Task '{}' started.", task_name);
}

pub fn pause(tasks: &Arc<Mutex<HashMap<String, Task>>>) {
    println!("Enter task name to pause:");
    let mut task_name_input = String::new();
    std::io::stdin().read_line(&mut task_name_input).expect("Failed to read task name");
    let task_name = task_name_input.trim().to_string();

    let tasks_clone = Arc::clone(&tasks);
    let handle = thread::spawn(move || {
        let mut tasks = tasks_clone.lock().unwrap();
        if let Some(task) = tasks.get_mut(&task_name) {
            if !task.is_paused {
                task.is_paused = true;
                task.pause_duration = Duration::from_secs(0);
                task.pause_start_time = Some(Instant::now());

                println!("Task '{}' paused.", task_name_input.trim_end());
            }
        } else {
            println!("Task not found.");
        }
    });
    handle.join().unwrap();
}

pub fn resume(tasks: &Arc<Mutex<HashMap<String, Task>>>) {
    println!("Enter task name to resume:");
    let mut task_name_input = String::new();
    std::io::stdin().read_line(&mut task_name_input).expect("Failed to read task name");
    let task_name = task_name_input.trim().to_string();
    let pause_end_time = Instant::now();

    let tasks_clone = Arc::clone(&tasks);
    let handle = thread::spawn(move || {
        let mut tasks = tasks_clone.lock().unwrap();
        if let Some(task) = tasks.get_mut(&task_name) {
            if task.is_paused {
                task.is_paused = false;
                task.pause_duration += pause_end_time - task.pause_start_time.unwrap_or_else(|| Instant::now());

                println!("Task '{}' resumed.", task_name_input.trim_end());
            }
        } else {
            println!("Task not found.");
        }
    });
    handle.join().unwrap();
}

pub fn end(tasks: &Arc<Mutex<HashMap<String, Task>>>) {
    println!("Enter task name to end:");
    let mut task_name = String::new();
    std::io::stdin().read_line(&mut task_name).expect("Failed to read task name");
    let task_name = task_name.trim().to_string();
    let end_time = Instant::now();

    let tasks_clone = Arc::clone(&tasks);
    let handle = thread::spawn(move || {
        let mut tasks = tasks_clone.lock().unwrap();
        if let Some(task) = tasks.get_mut(&task_name) {
            task.end_time = Some(end_time);
            let duration = task.total_duration();
            println!("Task '{}' ended. Duration: {:?}", task_name, duration);
        } else {
            println!("Task not found.");
        }
    });
    handle.join().unwrap();
}

pub fn list_all(tasks: &Arc<Mutex<HashMap<String, Task>>>) {
    let tasks = tasks.lock().unwrap();
    println!("All Tasks:");

    for (task_name, task) in tasks.iter() {
        let duration = task.total_duration();
        let status = if task.is_paused { "Paused" } else if task.end_time.is_none() { "Running" } else { "Ended" };
        println!("Task Name: '{}', Status: {}, Total Duration: {:?}", task_name, status, duration);
    }
}