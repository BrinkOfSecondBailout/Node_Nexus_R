struct Command_Handler {
    cmd_name: String,
    callback_function: Callback,
}

type Callback = fn(client: &Client, arg1: &str, arg2: &str) -> i32;
