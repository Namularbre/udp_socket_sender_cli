use std::io;
use std::net::UdpSocket;

fn input_string(message: &str) -> String {
    let mut input = String::new();

    println!("{}", message);

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}

fn input_target() -> String {
    input_string("Enter target ip address and port (ex: 127.0.0.1:12345) :")
}

fn input_message() -> String {
    input_string("Enter your message (Must be in one line): ")
}

fn input_stop() -> String {
    input_string("Stop y=yes: ")
}

fn input_name() -> String {
    input_string("Enter your name: ")
}

fn send_message(target: &String, message: &String, name: &String) -> io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:00000")
        .expect("Failed to bind socket");

    let full_message = format!("[{}]: {}", name, message);

    socket.send_to(full_message.as_bytes(), target)
        .expect("Failed to send message");

    println!("Message sent !");

    Ok(())
}

fn main() -> io::Result<()> {
    println!("Welcome to udp_client");

    let mut stop = false;
    let target = input_target();
    let name = input_name();

    while !stop {
        let message = input_message();

        send_message(&target, &message, &name)
            .expect("Error sending message");

        if input_stop() == "y" {
            stop = true;
        }
    }

    Ok(())
}
