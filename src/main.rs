extern crate num;
extern crate file_scanner;
extern crate grpc;
extern crate chashmap;
extern crate num_cpus;
extern crate tls_api_native_tls;

mod proto;

use crate::proto::messenger::*;
use crate::proto::messenger_grpc::*;
use chashmap::CHashMap;
use std::io;
use file_scanner::Scanner;
use std::io::Read;

const CHAT_PORT: u16 = 87162;

struct ChatEndpointImpl<'a> {
  user_to_thread_messages_: &'a CHashMap<String, Vec<String>>
}

impl<'a> ChatEndpoint for ChatEndpointImpl<'a> {
  fn send_message(
    &self,
    o: grpc::RequestOptions,
    p: Message
  ) -> ::grpc::SingleResponse<MessageAck> {
    println!("Received message from {}", p.sender);

    self.user_to_thread_messages_.upsert(
      p.sender.clone(),
      Vec::new,
      |vals| {
        vals.push(p.content.clone());
      });

    grpc::SingleResponse::completed(MessageAck::new())
  }
}

fn main() {
  // TODO look for others and add ping messaging and a list of them seperate from teh map
  // TODO make messages more complex type and not just strings, with a read marker etc, then show that in dashboard
  // dont show read messages unless requested in read option
  let user_to_thread_messages = CHashMap::new();

  {
    let mut server: grpc::ServerBuilder<tls_api_native_tls::TlsAcceptor> = grpc::ServerBuilder::new();
    server.http.set_port(CHAT_PORT);
    server.add_service(ChatEndpointServer::new_service_def(
      ChatEndpointImpl { user_to_thread_messages_: &user_to_thread_messages }));
    server.http.set_cpu_pool_threads(num_cpus::get());
    let _server = server.build().expect("server");

    println!("Started A chat bot on port {}", CHAT_PORT);

    loop {
      println!("What would you like to do?");
      println!("1) View/Refresh dashboard\n2) View messages from a [1: user]\n3) Send a [1: user] a [2: message]");

      let mut scanner = Scanner::new(io::stdin());
      let selection = scanner.next_int();

      match selection {
        Some(1) => refresh_dashboard(&user_to_thread_messages),
        Some(2) => try_view_messages_user(&mut scanner, &user_to_thread_messages),
        Some(3) => try_send_message_to_user(&mut scanner),
        None | _ => println!("Please put in a valid choice!\n"),
      }
    }
  }
}

fn refresh_dashboard(messages_map: &CHashMap<String, Vec<String>>) {
  println!("You have messages from the following other users: ");
  for (key, value) in messages_map.iter() {
    println!("From: {}, {} messages", key, value.len());
  }

  println!();
}

fn try_view_messages_user<T: Read + Sized>(mut scanner: &Scanner<T>, messages_map: &CHashMap<String, Vec<String>>) {
  let user = scanner.next().unwrap_or("".to_string());

  if !messages_map.contains_key(&user) {
    println!("No such user {}, please try again, come on man...", user);
    return;
  }

  println!("Here's what {} had to say to you", user);
  for message in messages_map.get(&user).unwrap().iter() {
    println!("{}", message);
  }
}

fn try_send_message_to_user<T: Read + Sized>(mut scanner: &Scanner<T>) {
  unimplemented!();
}
