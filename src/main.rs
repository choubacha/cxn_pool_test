extern crate r2d2;
extern crate rand;
use std::error;
use std::thread;
use std::fmt;

struct RandomConnection;
impl RandomConnection {
    fn new() -> Option<RandomConnection> {
        if rand::random() {
            Some(RandomConnection)
        } else {
            None
        }
    }

    fn is_valid(&self) -> bool {
        rand::random()
    }

    fn has_broken(&self) -> bool {
        rand::random()
    }
}

#[derive(Debug)]
enum Error {
    ConnectFailed,
    NotValid,
    Broke,
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            &Error::NotValid => "not valid",
            &Error::Broke => "broke",
            &Error::ConnectFailed => "connection failed",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

struct ManageRandom;
impl r2d2::ManageConnection for ManageRandom {
    type Connection = RandomConnection;
    type Error = Error;

    fn connect(&self) -> Result<Self::Connection, Self::Error> {
        if let Some(cxn) = RandomConnection::new() {
            println!("connected!");
            Ok(cxn)
        } else {
            println!("connect failed");
            Err(Error::ConnectFailed)
        }
    }

    fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        if conn.is_valid() {
            println!("valid!");
            Ok(())
        } else {
            println!("not valid");
            Err(Error::NotValid)
        }
    }

    fn has_broken(&self, conn: &mut Self::Connection) -> bool {
        if conn.has_broken() {
            println!("broke");
            true
        } else {
            println!("still around!");
            false
        }
    }
}

fn main() {
    let pool = r2d2::Pool::builder().max_size(10).build(ManageRandom).unwrap();
    let successes = (0..20)
        .map(|_| {
             let pool = pool.clone();
             thread::spawn(move || {
                 let conn = pool.get().unwrap();
                 println!("got connection!");
                 true
             })
        })
        .filter_map(|t| t.join().ok())
        .count();

    println!("We had {} successes", successes);
}
