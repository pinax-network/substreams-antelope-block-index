use substreams::errors::Error;
use substreams::pb::sf::substreams::index::v1::Keys;
use substreams_antelope::pb::Block;

#[substreams::handlers::map]
fn block_index(block: Block) -> Result<Keys, Error> {
    let mut keys = Keys::default();

    for transaction_trace in block.into_transaction_traces() {
        // action traces
        for action_trace in transaction_trace.action_traces {
            // key format: "receiver=receiver"
            {
                let key = format!("receiver={}", action_trace.receiver);
                if !keys.keys.contains(&key) {
                    keys.keys.push(key);
                }
            }

            let action = match action_trace.action {
                Some(action) => action,
                None => { continue; }
            };
            {
                // key format: "action=account::name"
                let key = format!("action={}::{}", action.account, action.name);
                if !keys.keys.contains(&key) {
                    keys.keys.push(key);
                }
            }
            for authorization in action.authorization {
                // key format: "authorization=actor@permission"
                let key = format!("authorization={}@{}", authorization.actor, authorization.permission);
                if !keys.keys.contains(&key) {
                    keys.keys.push(key);
                }
            }
        }

        // db ops
        for db_op in transaction_trace.db_ops {
            // key format: "db_op=code::table_name"
            let key = format!("db_op={}::{}", db_op.code.clone().to_string(), db_op.table_name.clone().to_string());
            if !keys.keys.contains(&key) {
                keys.keys.push(key);
            }
        }
    }
    Ok(keys)
}
