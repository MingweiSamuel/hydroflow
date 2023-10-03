use hydroflow::scheduled::context::Context;
use lattices::Max;
use regex::Regex;

use crate::protocol::KvsMessage;

pub fn parse_command(line: String, context: &Context) -> Option<KvsMessage> {
    let re = Regex::new(r"([A-z]+)\s+(.+)").unwrap();
    let caps = re.captures(line.as_str())?;

    let binding = caps.get(1).unwrap().as_str().to_uppercase();
    let cmdstr = binding.as_str();
    let args = caps.get(2).unwrap().as_str();
    match cmdstr {
        "PUT" => {
            let (k, v) = args.split_once(',')?;
            let value = v.trim().to_string();
            let value = Max::new(value);
            Some(KvsMessage::Put {
                key: k.trim().to_string(),
                value,
            })
        }
        "GET" => Some(KvsMessage::Get {
            key: args.trim().to_string(),
        }),
        _ => None,
    }
}
