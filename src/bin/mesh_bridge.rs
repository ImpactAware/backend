extern crate serialport;
extern crate nom;
use std::time::Duration;
use backend::*;
use backend::models::*;

use nom::{
  IResult,
  branch::alt,
  bytes::complete::{tag, take_while},
  combinator::map_res,
};


fn from_decimal(input: &str) -> Result<u32, std::num::ParseIntError> {
  u32::from_str_radix(input, 10)
}

fn is_digit(c: char) -> bool {
  c.is_digit(10)
}

#[derive(Debug)]
enum Command
{
    Vibration {
        device_id: u32,
        count: u32
    },
    Connection {
        device_id: u32,
    },
    Disconnection {
        device_id: u32,
    },
}

fn vibration_parser(input: &str) -> IResult<&str, Command> {
    let (remainder, _) = tag("VIBR ")(input)?;


    let (remainder, first_arg) = map_res(
        take_while(is_digit),
        from_decimal
    )(remainder)?;

    let (tag_remainder, _) = tag(" ")(remainder)?;

    let (remainder, second_arg) = map_res(
        take_while(is_digit),
        from_decimal
    )(tag_remainder)?;

    return Ok((remainder, Command::Vibration {
        count: first_arg,
        device_id: second_arg
    }));
}

fn disconnect_parser(i: &str) -> IResult<&str, Command> {
    let (remainder, _) = tag("DROP ")(i)?;

    let (remainder, first_arg) = map_res(
        take_while(is_digit),
        from_decimal
    )(remainder)?;

    Ok((remainder, Command::Disconnection { device_id: first_arg }))
}

fn connect_parser(i: &str) -> IResult<&str, Command> {
    let (remainder, _) = tag("CONN ")(i)?;

    let (remainder, first_arg) = map_res(
        take_while(is_digit),
        from_decimal
    )(remainder)?;

    Ok((remainder, Command::Connection { device_id: first_arg }))
}

fn parse_command(buffer: &str) -> IResult<&str, Command> {
    alt((vibration_parser, disconnect_parser, connect_parser))(buffer)
}

fn process_command(command: Command) {
    use Command::*;
    let conn = establish_connection();
    match command {
        Vibration { device_id, count } => {
            match nodesdsl::nodes.find(device_id as i64).get_result::<Node>(&conn) {
                Ok(node) => {
                },
                Err(e) => {
                }
            }
        },
        Connection { device_id } => {
        },
        Disconnection { device_id } => {
        }
    }
}

fn main() {
    let mut port = serialport::new("/dev/ttyUSB0", 115_200)
        .timeout(Duration::from_secs(20))
        .open().expect("Failed to open port");

    let mut running_buffer = String::new();

    loop {
        running_buffer = match parse_command(&running_buffer) {
            Ok((remainder, command)) => {
                println!("got command {:?}",command);
                let _ = process_command(command);
                // match null bytes
                match tag::<_, _, ()>("\n")(remainder) {
                    Ok((remainder, _)) => {
                        println!("found newlines");
                        remainder.trim().to_string()
                    },
                    Err(_) => {
                        println!("couldn't find any newlines");
                        remainder.trim().to_string()
                    }
                }
            },
            Err(e) => {
                let mut serial_buf: Vec<u8> = vec![0; 64];

                let mut end = 64;

                if let Ok(_) = port.read(&mut serial_buf) {
                    for i in 0..64 {
                        if serial_buf[i] == 0 {
                            end = i;
                            break;
                        }
                    }

                    let read_str = String::from_utf8_lossy(&serial_buf[0..end]);//.expect("Should get utf8 string from buffer");
                    println!("pushing {:?}, from 0 to {:?}", read_str, end); 


                    running_buffer.push_str(read_str.trim());
                    running_buffer.trim().to_string()
                } else {
                    running_buffer
                }
            }
        }
    }

    
}
// leave the running buffer alone
/*port.read(serial_buf.as_mut_slice()).expect("Found no data!");
let read_str = String::from_utf8(serial_buf.clone()).expect("Should get utf8 string from buffer");
running_buffer.push_str(&read_str);
*/
