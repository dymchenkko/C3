use std::{
    sync::{Arc, Mutex},
    thread::{self, sleep}, time::Duration,
};
mod counter;
mod circle_buffer;
mod process_data;
use counter::Counter;
use circle_buffer::CircleBuffer;
use process_data::process_data;
use std::borrow::Cow;
fn main() {

    //Zadanie 1
    let counter = Arc::new(Mutex::new(Counter::default()));

    let thread1 = thread::spawn({
        let counter_for_first_thread = counter.clone();
        move || {
            for _ in 0..10 {
                counter_for_first_thread.lock().unwrap().increment();
                println!("Value was incremented inside 1 thread, new value: {:?}", counter_for_first_thread.lock().unwrap());
                sleep(Duration::from_millis(500));
            }
        }
    });

    let thread2 = thread::spawn({
        let counter_for_second_thread = counter.clone();
        move || {
            for _ in 0..10 {
                counter_for_second_thread.lock().unwrap().increment();
                println!("Value was incremented inside 2 thread, new value: {:?}", counter_for_second_thread.lock().unwrap());
                sleep(Duration::from_millis(500));
            }
        }
    });

    let _ = thread1.join();
    let _ = thread2.join();

    println!("************************************************************************");

    //Zadanie 2

    let mut c_buffer: CircleBuffer<u64> = CircleBuffer::new(5);

    for i in 1..16 {
        c_buffer.add(i);
        println!("updated c_buffer: {:?}", c_buffer)
    }

    println!("************************************************************************");

    //Zadanie 3

    // Doesn't process data, so it doesn't clone the string
    let mut sentence = Cow::from("Some sentence with full stop.");
    process_data(&mut sentence);
    println!("result = {}", sentence);

    //Processes data, so it clones the string.
    let mut sentence = Cow::from("Some sentence without full stop");
    process_data(&mut sentence);

    println!("result = {}", sentence);
}