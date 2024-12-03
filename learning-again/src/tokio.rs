use std::{
    future::Future,
    thread::{current, sleep},
    time::Duration,
};
use tokio::task::spawn;

#[tokio::main(flavor = "multi_thread", worker_threads = 5)]
async fn main() {
    let racer1 = F1Racer::new();
    let mut racer2 = F1Racer::new();
    racer2.name = "Max Verstapen".to_string();
    racer2.lap_times.push(78.42);

    let handler1 = spawn(racer1);
    let handler2 = spawn(racer2);

    loop {
        if handler1.is_finished() && handler2.is_finished() {
            println!("All racers have completed the laps!");
            break;
        }
        sleep(Duration::from_millis(200));
    }
}

struct F1Racer {
    name: String,
    total_laps: usize,
    completed_laps: usize,
    fastest_lap_time: f32,
    lap_times: Vec<f32>,
}

impl F1Racer {
    fn new() -> F1Racer {
        F1Racer {
            name: "Charles Leclerc".to_string(),
            total_laps: 5,
            completed_laps: 0,
            fastest_lap_time: 81.43,
            lap_times: vec![80.13, 81.43, 79.2, 81.41, 80.12],
        }
    }

    fn calc_lap(&mut self) {
        println!("{} is doing a lap", self.name);
        let lap_time = self.lap_times.pop();
        if lap_time.is_some() && lap_time.unwrap() < self.fastest_lap_time {
            self.fastest_lap_time = lap_time.unwrap();
        }
        self.completed_laps += 1;
    }
}

impl Future for F1Racer {
    type Output = f32;
    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        println!("Running on thread {:?}", current().id());
        if self.completed_laps < self.total_laps {
            self.get_mut().calc_lap();
            cx.waker().wake_by_ref();
            return std::task::Poll::Pending;
        }
        println!("{} has completed all laps", self.name);
        println!(
            "Fastest lap time for {} is {}",
            self.name, self.fastest_lap_time
        );
        return std::task::Poll::Ready(self.fastest_lap_time);
    }
}
