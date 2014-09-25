use std::io::timer::sleep;
use std::time::duration::Duration;

#[allow(non_snake_case)]
#[inline]
fn λ_fly<Ψ>(animation: Vec<String>, symbols: int, delay: Duration, Ω: || -> Ψ) -> Ψ {
    let (tx, rx) = channel();
    spawn(proc() {
        let mut prefix = "".to_string();
        for _ in range (0, symbols) {
            print!(" ");
            prefix = format!("{:s}\x08", prefix);
        }
        while rx.try_recv().is_ok() {
            for fly in animation.iter() {
                print!("{:s}{:s}", prefix, *fly);
                sleep(delay);
            }
        }
    });     let res = Ω();
            tx.send(());
            res
}

#[allow(non_snake_case)]
pub fn λButterfly<Ψ>(Ω: || -> Ψ) -> Ψ {
    let animation = vec![ "|", "/","-","\\"
                        ].iter().map(|s| s.to_string()).collect();
    λ_fly(animation, 1, Duration::seconds(1), Ω)
}

#[allow(non_snake_case)]
pub fn ξ<Ψ>(Ω: || -> Ψ) -> Ψ {
    let animation = vec![
        "<(^.^<)"
        ,"<(^.^)>"
        ,"(>^.^)>"
        ,"(7^.^)7"
        ,"(>^.^<)"].iter().map(|s| s.to_string()).collect();
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