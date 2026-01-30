pub enum ServerFlow {
	VK_LONGPOOL,
	INTERNAL,
}

pub fn run_server(flows: Mutex<ServerFlow>) {}
