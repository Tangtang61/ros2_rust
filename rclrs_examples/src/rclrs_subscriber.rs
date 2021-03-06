extern crate rclrs;
extern crate std_msgs;

fn topic_callback(msg: &std_msgs::msg::String) {
    println!("I heard: '{}'", msg.data);
}

fn main() {
    rclrs::init().unwrap();

    let mut node = rclrs::create_node("minimal_subscriber");
    let subscription = node.create_subscription::<std_msgs::msg::String>(
        "topic",
        rclrs::qos::QOS_PROFILE_DEFAULT,
        topic_callback,
    );

    rclrs::spin(&node);
}
