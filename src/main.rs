use concurrent_hashmap::*;
use crate::proto_gen::messenger::*;
use crate::proto_gen::messenger_grpc::*;
use file_scanner::Scanner;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};
use std::io;
use std::io::Read;
use std::sync::Arc;

mod proto_gen;

const CHAT_PORT: u16 = 87162;

struct ChatEndpointImpl<'a, 'b> {
  user_to_thread_messages_: &'a ConcHashMap<&'b String, Vec<String>>
}

impl<'a, 'b> ChatEndpoint for ChatEndpointImpl<'a, 'b> {
  fn send_message(
    &mut self,
    ctx: RpcContext,
    req: Message,
    sink: UnarySink<MessageAck>) {
    println!("Received message from {}", req.sender);

    self.user_to_thread_messages_.upsert(
      &req.sender,
      Vec::new(),
      &|vals: &mut Vec<String>| {
        vals.push(req.content.clone());
      });

    let resp = MessageAck::new();
    let f = sink
      .success(resp)
      .map_err(move |e| println!("failed to reply {:?}: {:?}", req, e));
    ctx.spawn(resp);
  }
}

fn main() {
  // TODO look for others and add ping messaging and a list of them seperate from teh map
  // TODO make messages more complex type and not just strings, with a read marker etc, then show that in dashboard
  // dont show read messages unless requested in read option
  let user_to_thread_messages = ConcHashMap::new();

  {
    let env = Arc::new(Environment::new(num_cpus::get()));
    let service = create_chat_endpoint(ChatEndpointImpl { user_to_thread_messages_: &user_to_thread_messages });
    let mut server = ServerBuilder::new(env)
      .register_service(service)
      .bind("localhost", CHAT_PORT)
      .build()
      .unwrap();
    server.start();

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

fn refresh_dashboard(messages_map: &ConcHashMap<&String, Vec<String>>) {
  println!("You have messages from the following other users: ");
  for (&key, &value) in messages_map.iter() {
    println!("From: {}, {} messages", key, value.len());
  }

  println!();
}

fn try_view_messages_user<T: Read + Sized>(mut scanner: &Scanner<T>, messages_map: &ConcHashMap<&String, Vec<String>>) {
  let user = scanner.next().unwrap_or("".to_string());

  println!("Here's what {} had to say to you", user);
  if let Some(accessor) = messages_map.find(&user) {
    for message in accessor.get() {
      println!("{}", message);
    }
  } else {
    println!("No such user {}, please try again, come on man...", user);
  }
}

fn try_send_message_to_user<T: Read + Sized>(mut scanner: &Scanner<T>) {
  unimplemented!();
}
