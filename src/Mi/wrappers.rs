use std::io::timer::sleep;
use std::time::duration::Duration;

use std::sync::{Arc, Mutex};

pub struct TaskPool {
    tx: SyncSender<proc():Send>,
}

impl TaskPool {
    pub fn new(tasks: uint) -> TaskPool {
        assert!(tasks > 0);
        let (tx, rx) = sync_channel(tasks);
        let state = Arc::new(Mutex::new(rx));
        for _ in range(0, tasks) {
            let state = state.clone();
            spawn(proc() worker(&*state));
        }
        return TaskPool { tx: tx };
        fn worker(rx: &Mutex<Receiver<proc():Send>>) {
            loop {
                let job = rx.lock().recv_opt();
                match job {
                    Ok(job) => job(),
                    Err(..) => break,
                }
            }
        }
    }
    pub fn execute(&self, job: proc():Send) {
        self.tx.send(job);
    }
}

#[allow(non_snake_case)]
#[inline]
fn λ_fly<Ψ>(animation: Vec<&str>, symbols: int, delay: Duration, Ω: || -> Ψ) -> Ψ {
    let anime : Vec<String> = animation.iter().map(|s| s.to_string()).collect();
    let pool = TaskPool::new(1);
    let mut prefix = "".to_string();
    for _ in range (0, symbols) {
        print!(" ");
        prefix = format!("{:s}\x08", prefix);
    }
    let res = Ω();
    pool.execute(proc() {
        for fly in anime.iter() {
            print!("{:s}{:s}", prefix, fly.as_slice());
            sleep(delay);
        }
    });
    res
}

#[allow(non_snake_case)]
pub fn λButterfly<Ψ>(Ω: || -> Ψ) -> Ψ {
    let animation = vec![ "|", "/","-","\\"];
    λ_fly(animation, 1, Duration::seconds(1), Ω)
}

#[allow(non_snake_case)]
pub fn ξ<Ψ>(Ω: || -> Ψ) -> Ψ {
    let animation = vec![
        "<(^.^<)"
        ,"<(^.^)>"
        ,"(>^.^)>"
        ,"(7^.^)7"
        ,"(>^.^<)"];
    λ_fly(animation, 7, Duration::seconds(2), Ω)
}

#[inline]
#[allow(non_snake_case)]
pub fn λ<Ψ>(Ω: || -> Ψ) -> Ψ {
    println!("_________________________________________________________________________");
    let ret = Ω();
    println!("_________________________________________________________________________");
    ret
}
