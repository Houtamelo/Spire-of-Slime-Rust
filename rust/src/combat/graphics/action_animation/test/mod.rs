#[allow(unused_imports)]
use crate::*;

#[extends(Node)]
pub struct AnimTester {
	#[export_path] button_play: Option<Ref<Button>>,
	#[export_path] caster: Option<Ref<Node2D>>,
	#[export_path] targets: Vec<Ref<Node2D>>,
}

#[methods]
impl AnimTester {
	#[method]
	unsafe fn _ready(&self, #[base] owner: &Node) {
		self.button_play
			.unwrap_manual()
			.connect("pressed", owner.assume_shared(), "_play", VariantArray::new_shared(), Object::CONNECT_DEFERRED)
			.log_if_err();
	}
	
	#[method]
	fn _play(&self) {
		
	}
}