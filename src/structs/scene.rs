use crate::traits::scene_actor::SceneActor;

pub struct Scene<T: SceneActor> {
	title: String,
	actor: T
}

impl<T> Scene<T> where T: SceneActor {
	
}