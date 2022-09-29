#[derive(Debug)]
enum Genders {
	Female = 1,
	Male = 2,
}

#[derive(Debug, Copy, Clone)]
struct UserId(u64);

#[derive(Debug, Copy, Clone)]
struct TopicId(u64);

#[derive(Debug)]
struct User {
	id: UserId,
	gender: Genders,
	name: String,
}

#[derive(Debug)]
struct Topic {
    id: TopicId,
	name: String,
	owner: UserId,
}

#[derive(Debug)]
enum Event {
    Join((UserId, TopicId)),
	Leave((UserId, TopicId)),
	Message((UserId, TopicId, String)),
}

fn process_event(event: &Event) {
	match event {
		Event::Join((user_id, topic_id)) => println!("User {:?} joined in topic {:?}", user_id, topic_id),
		Event::Leave((user_id, topic_id)) => println!("User {:?} leave topic {:?}", user_id, topic_id),
		Event::Message((user_id, topic_id, message)) => println!("User {:?} say {} in {:?}", user_id, message, topic_id),
	}
}

fn process_leave(event: &Event) {
	if let Event::Leave((user_id, topic_id)) = event {
		println!("User {:?} leave topic {:?}", user_id, topic_id);
	}
}

fn main() {
	let user_one = User {id: UserId(1), name: "userOne".into(), gender: Genders::Female};
	let user_two = User {id: UserId(2), name: "userTwo".into(), gender: Genders::Male};

	let topic_one = Topic {id: TopicId(1), name: "topicOne".into(), owner: user_one.id};

	let join = Event::Join((user_one.id, topic_one.id));
	let leave = Event::Leave((user_one.id, topic_one.id));
	let message = Event::Message((user_two.id, topic_one.id, "Hello".into()));

	process_event(&join);
	process_event(&leave);
	process_event(&message);

	process_leave(&join);
	process_leave(&leave);
}

#[cfg(test)]
mod tests {
	#[test]
	fn test() {
		assert_eq!(2 + 2, 4)
	}
}
